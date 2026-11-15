// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "DesktopMacModules",
    defaultLocalization: "en",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "Presentation", targets: ["Presentation"]),
    ],
    dependencies: [
        .package(path: "../../bindings/swift"),
    ],
    targets: [
        .target(
            name: "Interop",
            dependencies: [.product(name: "DesktopMacosFFI", package: "swift")],
            path: "Interop"
        ),
        .target(name: "DesignSystem", path: "DesignSystem"),
        .target(name: "Presentation", dependencies: ["Interop", "DesignSystem"], path: "Presentation", resources: [.process("Resources")]),
        .testTarget(name: "InteropTests", dependencies: ["Interop"], path: "Tests/InteropTests"),
        .testTarget(name: "PresentationTests", dependencies: ["Presentation", "Interop"], path: "Tests/PresentationTests"),
        .testTarget(name: "DesignSystemTests", dependencies: ["DesignSystem"], path: "Tests/DesignSystemTests"),
    ]
)
