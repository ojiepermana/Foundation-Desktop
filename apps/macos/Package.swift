// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "DesktopMacModules",
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
        .target(name: "Presentation", dependencies: ["Interop"], path: "Presentation"),
        .testTarget(name: "InteropTests", dependencies: ["Interop"], path: "Tests/InteropTests"),
        .testTarget(name: "PresentationTests", dependencies: ["Presentation", "Interop"], path: "Tests/PresentationTests"),
    ]
)
