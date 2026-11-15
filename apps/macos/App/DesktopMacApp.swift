import Presentation
import SwiftUI

@main
struct DesktopMacApp: App {
    var body: some Scene {
        WindowGroup("Desktop Foundation") {
            GreetingView(model: GreetingViewModel())
        }
        .windowResizability(.contentSize)
    }
}
