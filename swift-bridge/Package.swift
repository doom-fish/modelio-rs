// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "ModelIOBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "ModelIOBridge",
            type: .static,
            targets: ["ModelIOBridge"])
    ],
    targets: [
        .target(
            name: "ModelIOBridge",
            path: "Sources/ModelIOBridge")
    ]
)
