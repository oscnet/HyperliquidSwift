// swift-tools-version: 6.2
import PackageDescription

let package = Package(
    name: "HyperliquidSwift",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "HyperliquidSwift",
            targets: ["HyperliquidSwift"])
    ],
    dependencies: [
        .package(url: "https://github.com/swiftlang/swift-testing", branch: "main")
    ],
    targets: [
        .systemLibrary(
            name: "CHyperliquidSwift",
            path: "Sources/CHyperliquidSwift"
        ),
        .target(
            name: "HyperliquidSwift",
            dependencies: ["CHyperliquidSwift"],
            path: "Sources/HyperliquidSwift",
            sources: ["hyperliquid.swift"],
            linkerSettings: [
                .linkedLibrary("hyperliquid_swift"),
                .unsafeFlags(["-L", "./target/release"])
            ]
        ),
        .testTarget(
            name: "HyperliquidSwiftTests",
            dependencies: [
                "HyperliquidSwift",
                .product(name: "Testing", package: "swift-testing")
            ])
    ]
)