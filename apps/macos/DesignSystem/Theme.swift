import AppKit
import SwiftUI

/// A semantic color with one sRGB value for the light appearance and one for
/// the dark appearance, each written as `0xRRGGBB`.
public struct ThemeColor: Sendable, Equatable {
    /// The value in the light appearance.
    public let light: UInt32

    /// The value in the dark appearance.
    public let dark: UInt32

    public init(light: UInt32, dark: UInt32) {
        self.light = light
        self.dark = dark
    }

    /// The red, green, and blue channels, 0 to 255, for one appearance.
    public func channels(dark isDark: Bool) -> [UInt8] {
        let value = isDark ? dark : light
        return [UInt8((value >> 16) & 0xFF), UInt8((value >> 8) & 0xFF), UInt8(value & 0xFF)]
    }

    /// An AppKit color that follows the appearance of the view that draws it.
    public var nsColor: NSColor {
        NSColor(name: nil) { appearance in
            let isDark = appearance.bestMatch(from: [.aqua, .darkAqua]) == .darkAqua
            let channels = self.channels(dark: isDark).map { CGFloat($0) / 255 }
            return NSColor(srgbRed: channels[0], green: channels[1], blue: channels[2], alpha: 1)
        }
    }

    /// The SwiftUI color. It follows the light and dark setting of the system.
    public var color: Color {
        Color(nsColor: nsColor)
    }
}

/// A typography role: a native text style and a weight. The size comes from
/// the system text style, never from a fixed number.
public struct TextRole: Sendable, Equatable {
    /// The native text style that sets the size.
    public let style: Font.TextStyle

    /// The font weight.
    public let weight: Font.Weight

    public init(style: Font.TextStyle, weight: Font.Weight) {
        self.style = style
        self.weight = weight
    }

    /// The SwiftUI font for this role.
    public var font: Font {
        Font.system(style).weight(weight)
    }
}
