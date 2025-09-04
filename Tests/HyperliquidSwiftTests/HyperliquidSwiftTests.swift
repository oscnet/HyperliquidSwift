import Testing
@testable import HyperliquidSwift

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