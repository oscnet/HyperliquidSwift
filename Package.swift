// swift-tools-version: 5.7
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
    dependencies: [],
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
            dependencies: ["HyperliquidSwift"])
    ]
)