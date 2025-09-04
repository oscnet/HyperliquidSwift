import XCTest
@testable import HyperliquidSwift

final class HyperliquidSwiftTests: XCTestCase {
    func testCreateInfoClient() throws {
        // Test creating an info client for testnet
        do {
            let infoClient = try createInfoClient(baseUrl: .testnet)
            XCTAssertNotNil(infoClient)
        } catch {
            XCTFail("Failed to create info client: \(error)")
        }
    }
    
    func testCreateExchangeClient() throws {
        // Test creating an exchange client (requires valid private key)
        let testPrivateKey = "0x0000000000000000000000000000000000000000000000000000000000000001" // Invalid key for testing
        
        do {
            let exchangeClient = try createExchangeClient(privateKey: testPrivateKey, baseUrl: .testnet)
            XCTFail("Should have failed with invalid private key")
        } catch HyperliquidError.InvalidPrivateKey {
            // Expected error
            XCTAssertTrue(true)
        } catch {
            XCTFail("Unexpected error: \(error)")
        }
    }
    
    func testOrderRequest() {
        let orderRequest = OrderRequest(
            asset: "ETH",
            isBuy: true,
            size: 0.1,
            price: 3000.0,
            reduceOnly: false
        )
        
        XCTAssertEqual(orderRequest.asset, "ETH")
        XCTAssertTrue(orderRequest.isBuy)
        XCTAssertEqual(orderRequest.size, 0.1)
        XCTAssertEqual(orderRequest.price, 3000.0)
        XCTAssertFalse(orderRequest.reduceOnly)
    }
    
    func testCancelRequest() {
        let cancelRequest = CancelRequest(
            asset: "ETH",
            oid: 12345
        )
        
        XCTAssertEqual(cancelRequest.asset, "ETH")
        XCTAssertEqual(cancelRequest.oid, 12345)
    }
}