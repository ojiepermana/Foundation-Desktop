import SwiftUI

/// The greeting window content: a name field, a button, and the result label.
public struct GreetingView: View {
    @State private var model: GreetingViewModel

    public init(model: GreetingViewModel) {
        _model = State(initialValue: model)
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            TextField("Name", text: $model.name)
            Button("Greet") {
                model.greet()
            }
            .keyboardShortcut(.defaultAction)
            Text(verbatim: model.greeting ?? "")
        }
        .padding(20)
        .frame(minWidth: 320)
    }
}
