import Interop
import Observation

/// State for the greeting window. It sends the name as typed and keeps the
/// result exactly as the greeter returned it.
@MainActor
@Observable
public final class GreetingViewModel {
    /// The text in the name field, unchanged.
    public var name: String = ""

    /// The last greeting returned by the greeter. `nil` until the first press.
    public private(set) var greeting: String?

    private let greeter: any Greeter

    public init(greeter: any Greeter = CoreGreeter()) {
        self.greeter = greeter
    }

    /// Sends `name` to the greeter and stores what comes back.
    public func greet() {
        greeting = greeter.hello(name: name)
    }
}
