# Hyperliquid Swift Library

A Swift wrapper around the `hyperliquid-rust-sdk` using UniFFI for cross-platform bindings. This library provides Swift developers with access to Hyperliquid's trading APIs including order placement, cancellation, and market data retrieval.

## Features

- **Exchange Client**: Place orders, cancel orders, manage positions
- **Info Client**: Retrieve market data, user states, balances, and order history
- **Type-safe**: Full Swift type safety with proper error handling
- **Cross-platform**: Works on iOS and macOS

## Architecture

The library consists of:

1. **Rust Core** (`src/lib.rs`): Wraps the original `hyperliquid-rust-sdk` with UniFFI-compatible types
2. **UniFFI Interface** (`src/hyperliquid.udl`): Defines the FFI interface between Rust and Swift
3. **Generated Bindings**: Automatically generated Swift code that provides the native Swift API
4. **Swift Package**: Complete Swift Package Manager integration

## Generated API

### Core Types

```swift
enum BaseUrl {
    case mainnet
    case testnet
}

struct OrderRequest {
    let asset: String
    let isBuy: Bool
    let size: Double
    let price: Double
    let reduceOnly: Bool
}

struct UserState {
    let address: String
    let marginSummaryEquity: Double
    let marginSummaryAccountValue: Double
    let marginSummaryTotalMarginUsed: Double
}
```

### Exchange Client

```swift
// Create exchange client (requires private key)
let exchangeClient = try createExchangeClient(
    privateKey: "your_private_key_here", 
    baseUrl: .testnet
)

// Place order
let orderRequest = OrderRequest(
    asset: "ETH",
    isBuy: true,
    size: 0.1,
    price: 3000.0,
    reduceOnly: false
)
let result = try exchangeClient.placeOrder(orderRequest)

// Cancel order
let cancelRequest = CancelRequest(asset: "ETH", oid: 12345)
let cancelResult = try exchangeClient.cancelOrder(cancelRequest)
```

### Info Client

```swift
// Create info client (no private key needed)
let infoClient = try createInfoClient(baseUrl: .testnet)

// Get user state
let userState = try infoClient.getUserState(address: "0x...")

// Get open orders
let openOrders = try infoClient.getOpenOrders(address: "0x...")

// Get all mid prices
let mids = try infoClient.getAllMids()
```

## Deployment

### Prerequisites

- iOS 13.0+ / macOS 10.15+
- Swift 5.7+
- Xcode (for iOS/macOS development)
- Rust (only for building from source)

### Option 1: Using Pre-built Library

If the library is already built (contains `libhyperliquid_swift.dylib`):

1. **Add as Swift Package dependency**:
   ```swift
   // Package.swift
   dependencies: [
       .package(path: "./hyperliquid-swift")
       // Or from git repository:
       // .package(url: "https://github.com/your-username/hyperliquid-swift", branch: "main")
   ]
   ```

2. **Import and use**:
   ```swift
   import HyperliquidSwift
   
   // Create clients
   let infoClient = try createInfoClient(baseUrl: .mainnet)
   let exchangeClient = try createExchangeClient(
       privateKey: "0x...", 
       baseUrl: .mainnet
   )
   
   // Place order
   let order = OrderRequest(
       asset: "ETH",
       isBuy: true,
       size: 0.1,
       price: 3000.0,
       reduceOnly: false
   )
   let result = try exchangeClient.placeOrder(order)
   ```

3. **Required files for deployment**:
   - `libhyperliquid_swift.dylib` (compiled Rust library)
   - `Sources/HyperliquidSwift/hyperliquid.swift` (Swift API)
   - `Sources/CHyperliquidSwift/` (C headers)
   - `Package.swift` (package configuration)

### Option 2: Building from Source

1. **Build the Rust library**:
```bash
cargo build --release
```

2. **Generate Swift bindings**:
```bash
cargo run --bin generate-bindings
```

3. **Test the Swift package** (optional):
```bash
swift test  # Note: Some tests require network access
```

### Files Generated

- `bindings/hyperliquid.swift` - Swift API implementation
- `bindings/hyperliquidFFI.h` - C header file
- `bindings/hyperliquidFFI.modulemap` - Module map for C interop
- `target/release/libhyperliquid_swift.dylib` - Native library

## Project Structure

```
hyperliquid-swift/
├── src/
│   ├── lib.rs                  # Rust wrapper implementation
│   └── hyperliquid.udl         # UniFFI interface definition
├── Sources/
│   ├── HyperliquidSwift/       # Swift package source
│   └── CHyperliquidSwift/      # C headers for system library
├── Tests/
│   └── HyperliquidSwiftTests/  # Swift tests
├── bindings/                   # Generated bindings
├── demo.swift                  # Usage example
├── Package.swift               # Swift Package Manager manifest
└── Cargo.toml                  # Rust project configuration
```

## Usage Example

```swift
import HyperliquidSwift

// Get market data
let infoClient = try createInfoClient(baseUrl: .testnet)
let mids = try infoClient.getAllMids()
print("ETH mid price: $\\(mids["ETH"] ?? "N/A")")

// Trading (requires valid private key)
let exchangeClient = try createExchangeClient(
    privateKey: "your_private_key", 
    baseUrl: .testnet
)

let order = OrderRequest(
    asset: "ETH",
    isBuy: true,
    size: 0.1,
    price: 3000.0,
    reduceOnly: false
)

let response = try exchangeClient.placeOrder(order)
print("Order response: \\(response)")
```

## Error Handling

The library provides typed error handling:

```swift
do {
    let client = try createExchangeClient(privateKey: "invalid", baseUrl: .testnet)
} catch HyperliquidError.invalidPrivateKey(let message) {
    print("Invalid private key: \\(message)")
} catch HyperliquidError.networkError(let message) {
    print("Network error: \\(message)")
} catch {
    print("Other error: \\(error)")
}
```

## Technical Details

### UniFFI Integration

This library uses Mozilla's UniFFI to generate language bindings:

- **Interface Definition**: `src/hyperliquid.udl` defines the public API
- **Type Mapping**: Complex Rust types are mapped to Swift-friendly equivalents
- **Memory Management**: Automatic memory management between Rust and Swift
- **Error Handling**: Rust `Result<T, E>` types become Swift throwing functions

### Threading Model

- **Async Operations**: Rust async functions are wrapped with tokio runtime
- **Thread Safety**: All operations are thread-safe through Arc<Mutex<_>> where needed
- **Blocking Calls**: Swift calls block until Rust futures complete

## Limitations

- **Network Dependency**: Requires active internet connection for API calls
- **Platform Support**: Currently supports macOS and iOS (can be extended to other platforms)
- **Private Keys**: Must be provided as hex strings with 0x prefix
- **Error Messages**: Some error messages may be technical (from underlying Rust library)

## Contributing

To extend this library:

1. Modify `src/lib.rs` to add new wrapper functions
2. Update `src/hyperliquid.udl` to expose the new functions to Swift
3. Rebuild with `cargo build --release`
4. Regenerate bindings with `cargo run --bin generate-bindings`
5. Test the updated Swift API

## License

This project wraps the `hyperliquid-rust-sdk` which is licensed under MIT. Please check the original project's license for details.