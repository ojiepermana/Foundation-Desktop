import Testing
import Interop

@Suite struct BindingContractTests {
    let greeter = CoreGreeter()

    @Test func v1AyuReturnsHelloAyu() {
        #expect(greeter.hello(name: "Ayu") == "Hello, Ayu!")
    }

    @Test func v2EmptyNameIsAccepted() {
        #expect(greeter.hello(name: "") == "Hello, !")
    }

    @Test func v6UnicodeStaysIntact() {
        let name = "\u{141}ukasz \u{674E} \u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}"
        #expect(greeter.hello(name: name) == "Hello, \(name)!")
        #expect(Array(greeter.hello(name: name).utf8) == Array("Hello, ".utf8) + Array(name.utf8) + Array("!".utf8))
    }

    @Test func v7NoNormalisation() {
        let out = greeter.hello(name: "Cafe\u{301}")
        #expect(Array(out.utf8) == Array("Hello, Cafe\u{301}!".utf8))
        #expect(Array(out.utf8) != Array("Hello, Caf\u{e9}!".utf8))
    }

    @Test func v10NulInTheMiddleIsKept() {
        let out = greeter.hello(name: "A\0B")
        #expect(Array(out.utf8) == Array("Hello, A\0B!".utf8))
        #expect(out.utf8.count == 11)
    }

    @Test func v11LongNameIsNotCut() {
        let name = String(repeating: "a", count: 1_048_576)
        let out = greeter.hello(name: name)
        #expect(out.utf8.count == 1_048_584)
        #expect(out.hasPrefix("Hello, "))
        #expect(out.hasSuffix("!"))
        #expect(out.utf8.dropFirst(7).dropLast(1).elementsEqual(name.utf8))
    }

    @Test func repeatedCallsFromManyThreads() async {
        await withTaskGroup(of: Bool.self) { group in
            for i in 0..<64 {
                group.addTask {
                    (0..<200).allSatisfy { j in
                        CoreGreeter().hello(name: "n\(i)-\(j)") == "Hello, n\(i)-\(j)!"
                    }
                }
            }
            for await ok in group { #expect(ok) }
        }
    }
}
