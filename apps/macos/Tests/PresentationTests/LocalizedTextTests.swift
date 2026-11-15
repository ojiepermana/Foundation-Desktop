import Foundation
import Testing
import Presentation

/// Spec macos/0006: text per language, language choice, missing keys, plain text, and `hello`.
@MainActor
@Suite struct LocalizedTextTests {
    @Test func bundleListsTheCatalogLanguages() {
        #expect(LocalizedText.bundle.localizations.sorted() == ["en", "id"])
        #expect(LocalizedText.bundle.developmentLocalization == "en")
    }

    @Test func picksTheCatalogLanguage() {
        #expect(LocalizedText.language(for: ["en"]) == "en")
        #expect(LocalizedText.language(for: ["id"]) == "id")
        #expect(LocalizedText.language(for: ["id-ID"]) == "id")
        #expect(LocalizedText.language(for: ["fr"]) == "en")
        #expect(LocalizedText.language(for: []) == "en")
    }

    @Test func showsTheTextForEachLanguage() {
        #expect(LocalizedText.string(.greetingSubmit, language: "en") == "Greet")
        #expect(LocalizedText.string(.greetingSubmit, language: "id") == "Sapa")
        #expect(LocalizedText.string(.greetingSubmit, language: LocalizedText.language(for: ["id-ID"])) == "Sapa")
        #expect(LocalizedText.string(.greetingSubmit, language: LocalizedText.language(for: ["fr"])) == "Greet")
        #expect(LocalizedText.string(.greetingSubmit, language: "xx") == "Greet")
    }

    @Test func everyKeyHasTextInEveryLanguage() {
        for language in LocalizedText.bundle.localizations {
            for key in TextKey.allCases {
                let text = LocalizedText.string(key, language: language)
                #expect(text != key.rawValue, "\(key.rawValue) has no text for \(language)")
                #expect(!text.isEmpty, "\(key.rawValue) is empty for \(language)")
                #expect(!text.contains("(null)"), "\(key.rawValue) needs placeholder values")
            }
        }
    }

    @Test func percentSignsStayPlain() {
        #expect(LocalizedText.render("100%% *done* <b> & _x_") == "100% *done* <b> & _x_")
    }

    @Test func greetingStaysEnglish() {
        #expect(LocalizedText.string(.greetingSubmit, language: "id") == "Sapa")
        let model = GreetingViewModel()
        model.name = "Ayu"
        model.greet()
        #expect(model.greeting == "Hello, Ayu!")
    }
}
