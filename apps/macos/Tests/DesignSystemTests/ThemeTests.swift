import AppKit
import DesignSystem
import SwiftUI
import Testing

/// Returns the sRGB channels, 0 to 255, that `color` has under `appearance`.
private func resolved(_ color: NSColor, under appearance: NSAppearance.Name) -> [UInt8] {
    var channels: [UInt8] = []
    NSAppearance(named: appearance)?.performAsCurrentDrawingAppearance {
        guard let srgb = color.usingColorSpace(.sRGB) else { return }
        channels = [srgb.redComponent, srgb.greenComponent, srgb.blueComponent]
            .map { UInt8(($0 * 255).rounded()) }
    }
    return channels
}

@Suite struct ThemeTests {
    @Test func channelsSplitLightAndDark() {
        let color = ThemeColor(light: 0x112233, dark: 0xAABBCC)
        #expect(color.channels(dark: false) == [0x11, 0x22, 0x33])
        #expect(color.channels(dark: true) == [0xAA, 0xBB, 0xCC])
    }

    @Test func colorFollowsTheAppearance() {
        let color = ThemeColor(light: 0x112233, dark: 0xAABBCC).nsColor
        #expect(resolved(color, under: .aqua) == [0x11, 0x22, 0x33])
        #expect(resolved(color, under: .darkAqua) == [0xAA, 0xBB, 0xCC])
        #expect(resolved(color, under: .accessibilityHighContrastAqua) == [0x11, 0x22, 0x33])
        #expect(resolved(color, under: .accessibilityHighContrastDarkAqua) == [0xAA, 0xBB, 0xCC])
    }

    @Test func everyColorTokenResolvesInBothAppearances() {
        let tokens = [Tokens.Colors.accent, Tokens.Colors.background, Tokens.Colors.textPrimary]
        for token in tokens {
            #expect(resolved(token.nsColor, under: .aqua) == token.channels(dark: false))
            #expect(resolved(token.nsColor, under: .darkAqua) == token.channels(dark: true))
        }
    }

    @Test func textRoleUsesTheSystemTextStyle() {
        let role = TextRole(style: .body, weight: .semibold)
        #expect(role.font == Font.system(.body).weight(.semibold))
        #expect(role.font != Font.system(.body).weight(.regular))
        #expect(Tokens.Typography.body == TextRole(style: .body, weight: .regular))
    }

    @Test func spacingTokensAreWholePoints() {
        for value in [Tokens.Spacing.gapControl, Tokens.Spacing.insetWindow] {
            #expect(value >= 0)
            #expect(value == value.rounded())
        }
    }
}
