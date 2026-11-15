import Foundation

/// Keys of the shared catalog (`shared/localization/catalog.json`) that this app shows.
/// Spec `docs/specs/macos/0006-satu-contoh-lokalisasi/index.md`.
public enum TextKey: String, CaseIterable, Sendable {
    /// Label of the greeting button.
    case greetingSubmit = "greeting.submit"
}

/// Reads interface text from the generated `Localizable.strings` resources, as plain text.
public enum LocalizedText {
    /// The catalog's source language, used when no preferred language matches.
    public static let sourceLanguage = "en"

    /// The resource bundle that holds one `.lproj` folder per catalog language.
    public static var bundle: Bundle { .module }

    /// The catalog language Apple's negotiation picks for `preferences`, else the source language.
    /// The source language is always the last choice, so the result never depends on the host's languages.
    public static func language(for preferences: [String]) -> String {
        Bundle.preferredLocalizations(from: bundle.localizations, forPreferences: preferences + [sourceLanguage])
            .first ?? sourceLanguage
    }

    /// The text for `key` in `language`, as plain text.
    public static func string(_ key: TextKey, language: String) -> String {
        let folder = bundle.path(forResource: language, ofType: "lproj")
            ?? bundle.path(forResource: sourceLanguage, ofType: "lproj")
        let table = folder.flatMap(Bundle.init(path:)) ?? bundle
        return render(table.localizedString(forKey: key.rawValue, value: nil, table: nil))
    }

    /// The text for `key` in the language the user prefers in macOS settings.
    public static func string(_ key: TextKey) -> String {
        string(key, language: language(for: Locale.preferredLanguages))
    }

    /// Turns a resource text into the text people see. Only `%%` changes, into `%`.
    public static func render(_ template: String) -> String {
        String(format: template, arguments: [])
    }
}
