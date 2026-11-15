import DesignSystem
import SwiftUI

/// The greeting window content: a name field, a button, and the result label.
public struct GreetingView: View {
    @State private var model: GreetingViewModel

    public init(model: GreetingViewModel) {
        _model = State(initialValue: model)
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: Tokens.Spacing.gapControl) {
            TextField("Name", text: $model.name)
            Button(LocalizedText.string(.greetingSubmit)) {
                model.greet()
            }
            .keyboardShortcut(.defaultAction)
            .buttonStyle(.borderedProminent)
            .tint(Tokens.Colors.accent.color)
            Text(verbatim: model.greeting ?? "")
                .font(Tokens.Typography.body.font)
                .foregroundStyle(Tokens.Colors.textPrimary.color)
        }
        .padding(Tokens.Spacing.insetWindow)
        .frame(minWidth: 320)
        .background(Tokens.Colors.background.color)
    }
}
