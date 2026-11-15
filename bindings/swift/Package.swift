// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "DesktopMacosFFI",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "DesktopMacosFFI", targets: ["DesktopMacosFFI"]),
    ],
    targets: [
        .systemLibrary(name: "desktop_macos_ffiFFI"),
        .target(
            name: "DesktopMacosFFI",
            dependencies: ["desktop_macos_ffiFFI"],
            linkerSettings: [.linkedLibrary("desktop_macos_ffi")]
        ),
    ]
)
