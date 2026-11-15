import Testing
import Interop
import Presentation

private final class RecordingGreeter: Greeter, @unchecked Sendable {
    var received: [String] = []
    func hello(name: String) -> String {
        received.append(name)
        return "<<from greeter>>"
    }
}

@MainActor
@Suite struct GreetingViewModelTests {
    @Test func startsWithoutGreeting() {
        let model = GreetingViewModel()
        #expect(model.greeting == nil)
        #expect(model.name == "")
    }

    @Test func showsWhatCoreReturns() {
        let model = GreetingViewModel()
        model.name = "Ayu"
        model.greet()
        #expect(model.greeting == "Hello, Ayu!")
    }

    @Test func sendsEmptyNameAsIs() {
        let model = GreetingViewModel()
        model.greet()
        #expect(model.greeting == "Hello, !")
    }

    @Test func sendsNameUnchangedAndShowsResultUnchanged() {
        let greeter = RecordingGreeter()
        let model = GreetingViewModel(greeter: greeter)
        model.name = "  Ayu  "
        model.greet()
        #expect(greeter.received == ["  Ayu  "])
        #expect(model.greeting == "<<from greeter>>")
    }
}
