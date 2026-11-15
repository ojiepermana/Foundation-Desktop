import DesktopMacosFFI

/// Returns a greeting for a name. The app talks to the Rust Core through this.
public protocol Greeter: Sendable {
    func hello(name: String) -> String
}

/// The real greeter. It forwards to `hello` in the Rust Core through the UniFFI binding.
public struct CoreGreeter: Greeter {
    public init() {}

    public func hello(name: String) -> String {
        DesktopMacosFFI.hello(name: name)
    }
}
