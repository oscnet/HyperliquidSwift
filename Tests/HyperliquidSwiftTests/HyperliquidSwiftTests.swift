import Testing
@testable import HyperliquidSwift
import Foundation

@Test("Create info client for testnet")
func testCreateInfoClient() throws {
    // Test creating an info client for testnet
    _ = try createInfoClient(baseUrl: .testnet)
    // Just verify it doesn't crash, the client is not optional
}

@Test("Create exchange client with test private key")
func testCreateExchangeClient() throws {
    // Test creating an exchange client with a test private key
    // This appears to succeed even with a test key, so just verify it doesn't crash
    let testPrivateKey = "0x0000000000000000000000000000000000000000000000000000000000000001"
    
    _ = try createExchangeClient(privateKey: testPrivateKey, baseUrl: .testnet)
    // Test passes if it doesn't crash
}

@Test("OrderRequest properties")
func testOrderRequest() {
    let orderRequest = OrderRequest(
        asset: "ETH",
        isBuy: true,
        size: 0.1,
        price: 3000.0,
        reduceOnly: false
    )
    
    #expect(orderRequest.asset == "ETH")
    #expect(orderRequest.isBuy == true)
    #expect(orderRequest.size == 0.1)
    #expect(orderRequest.price == 3000.0)
    #expect(orderRequest.reduceOnly == false)
}

@Test("CancelRequest properties")
func testCancelRequest() {
    let cancelRequest = CancelRequest(
        asset: "ETH",
        oid: 12345
    )
    
    #expect(cancelRequest.asset == "ETH")
    #expect(cancelRequest.oid == 12345)
}

@Test("SOL Trading on Mainnet - Place and Cancel Order")
func testSOLTradingMainnet() throws {
    // Load private key from environment
    guard let privateKey = ProcessInfo.processInfo.environment["privateKey"] ?? getPrivateKeyFromEnvFile() else {
        print("⚠️ Private key not found in environment or .env file - skipping test")
        return
    }
    
    print("🔗 Connecting to Hyperliquid Mainnet...")
    
    // Create clients
    let infoClient = try createInfoClient(baseUrl: .mainnet)
    let exchangeClient = try createExchangeClient(privateKey: privateKey, baseUrl: .mainnet)
    
    print("✅ Connected successfully")
    print("📍 Wallet Address: \(exchangeClient.getWalletAddress())")
    
    // Get current SOL price
    print("💰 Fetching current SOL price...")
    let allMids = try infoClient.getAllMids()
    
    guard let solPriceStr = allMids["SOL"],
          let currentSolPrice = Double(solPriceStr) else {
        print("❌ Could not get SOL price from market data")
        throw HyperliquidError.ApiError(message: "SOL price not available")
    }
    
    print("📈 Current SOL price: $\(currentSolPrice)")
    
    // Calculate order price (current price - 10) with proper tick size
    // For SOL, use 0.01 tick size (2 decimal places) to be safer
    let rawOrderPrice = currentSolPrice - 10.0
    let orderPrice = floor(rawOrderPrice * 100) / 100  // Floor to 2 decimal places
    print("🎯 Order price: $\(orderPrice)")
    
    // Create order request
    let orderRequest = OrderRequest(
        asset: "SOL",
        isBuy: true,
        size: 0.1,
        price: orderPrice,
        reduceOnly: false
    )
    
    // Place the order
    print("📤 Placing SOL buy order: 0.1 SOL at $\(orderPrice)...")
    let orderResponse = try exchangeClient.placeOrder(order: orderRequest)
    print("✅ Order placed: \(orderResponse)")
    
    // Wait a moment for order to be processed
    Thread.sleep(forTimeInterval: 2.0)
    
    // Get open orders to find the order ID
    print("🔍 Fetching open orders...")
    let openOrders = try infoClient.getOpenOrders(address: exchangeClient.getWalletAddress())
    
    // Find our SOL order
    if let solOrder = openOrders.first(where: { $0.asset == "SOL" && $0.isBuy == true }) {
        print("📋 Found SOL order - ID: \(solOrder.oid), Price: $\(solOrder.price), Size: \(solOrder.size)")
        
        // Cancel the order
        let cancelRequest = CancelRequest(asset: "SOL", oid: solOrder.oid)
        print("❌ Cancelling order...")
        let cancelResponse = try exchangeClient.cancelOrder(cancel: cancelRequest)
        print("✅ Order cancelled: \(cancelResponse)")
        
    } else {
        print("⚠️ No SOL buy order found in open orders")
    }
    
    print("🏁 Test completed successfully")
}

// Helper function to read private key from .env file
private func getPrivateKeyFromEnvFile() -> String? {
    guard let projectPath = ProcessInfo.processInfo.environment["PWD"] else { return nil }
    let envPath = "\(projectPath)/.env"
    
    guard let envContent = try? String(contentsOfFile: envPath) else { return nil }
    
    for line in envContent.components(separatedBy: .newlines) {
        let trimmed = line.trimmingCharacters(in: .whitespacesAndNewlines)
        if trimmed.hasPrefix("privateKey=") {
            let keyPart = trimmed.replacingOccurrences(of: "privateKey=", with: "")
            return keyPart.trimmingCharacters(in: CharacterSet(charactersIn: "\""))
        }
    }
    
    return nil
}