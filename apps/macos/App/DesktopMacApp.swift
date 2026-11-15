import Presentation
import SwiftUI

@main
struct DesktopMacApp: App {
    /// The product name Xcode writes into the bundle at build time. Its
    /// source is `build.product_name` in the shared public configuration.
    private let productName =
        Bundle.main.object(forInfoDictionaryKey: "CFBundleDisplayName") as? String
        ?? ProcessInfo.processInfo.processName

    var body: some Scene {
        WindowGroup(productName) {
            GreetingView(model: GreetingViewModel())
        }
        .windowResizability(.contentSize)
    }
}
