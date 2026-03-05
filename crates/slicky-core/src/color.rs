//! Color definitions and named presets for Slicky lights.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::{Result, SlickyError};

/// An RGB color with each channel 0–255.
///
/// # Examples
///
/// ```
/// use slicky_core::Color;
///
/// let red = Color::new(255, 0, 0);
/// assert_eq!(red.to_hex(), "#FF0000");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// The "off" color (all channels zero).
    pub const fn off() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    /// Parse a hex color string.
    ///
    /// Accepts `"#RRGGBB"`, `"RRGGBB"`, `"#RGB"`, or `"RGB"` (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use slicky_core::Color;
    ///
    /// let red = Color::from_hex("#FF0000").unwrap();
    /// assert_eq!(red, Color::new(255, 0, 0));
    ///
    /// let short = Color::from_hex("f00").unwrap();
    /// assert_eq!(short, Color::new(255, 0, 0));
    /// ```
    pub fn from_hex(s: &str) -> Result<Self> {
        let hex = s.strip_prefix('#').unwrap_or(s);
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| SlickyError::InvalidHexColor(s.to_string()))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| SlickyError::InvalidHexColor(s.to_string()))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| SlickyError::InvalidHexColor(s.to_string()))?;
                Ok(Self { r, g, b })
            }
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16)
                    .map_err(|_| SlickyError::InvalidHexColor(s.to_string()))?;
                let g = u8::from_str_radix(&hex[1..2], 16)
                    .map_err(|_| SlickyError::InvalidHexColor(s.to_string()))?;
                let b = u8::from_str_radix(&hex[2..3], 16)
                    .map_err(|_| SlickyError::InvalidHexColor(s.to_string()))?;
                Ok(Self {
                    r: r * 17,
                    g: g * 17,
                    b: b * 17,
                })
            }
            _ => Err(SlickyError::InvalidHexColor(s.to_string())),
        }
    }

    /// Format as a `#RRGGBB` hex string.
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Returns `true` if all channels are zero.
    pub fn is_off(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Named color presets for common status light colors.
///
/// # Examples
///
/// ```
/// use slicky_core::Preset;
///
/// let p = Preset::from_name("in-meeting").unwrap();
/// assert_eq!(p, Preset::InMeeting);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Preset {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Orange,
    Purple,
    Available,
    Busy,
    Away,
    InMeeting,
}

/// All preset variants in declaration order.
const ALL_PRESETS: &[Preset] = &[
    Preset::Red,
    Preset::Green,
    Preset::Blue,
    Preset::Yellow,
    Preset::Cyan,
    Preset::Magenta,
    Preset::White,
    Preset::Orange,
    Preset::Purple,
    Preset::Available,
    Preset::Busy,
    Preset::Away,
    Preset::InMeeting,
];

impl Preset {
    /// The RGB color for this preset.
    pub fn color(&self) -> Color {
        match self {
            Self::Red => Color::new(255, 0, 0),
            Self::Green => Color::new(0, 255, 0),
            Self::Blue => Color::new(0, 0, 255),
            Self::Yellow => Color::new(255, 255, 0),
            Self::Cyan => Color::new(0, 255, 255),
            Self::Magenta => Color::new(255, 0, 255),
            Self::White => Color::new(255, 255, 255),
            Self::Orange => Color::new(255, 165, 0),
            Self::Purple => Color::new(128, 0, 128),
            Self::Available => Color::new(0, 255, 0),
            Self::Busy => Color::new(255, 0, 0),
            Self::Away => Color::new(255, 255, 0),
            Self::InMeeting => Color::new(255, 69, 0),
        }
    }

    /// All available preset variants.
    pub fn all() -> &'static [Preset] {
        ALL_PRESETS
    }

    /// Look up a preset by name (case-insensitive, allows hyphens).
    ///
    /// # Examples
    ///
    /// ```
    /// use slicky_core::Preset;
    ///
    /// assert_eq!(Preset::from_name("RED").unwrap(), Preset::Red);
    /// assert_eq!(Preset::from_name("in-meeting").unwrap(), Preset::InMeeting);
    /// assert_eq!(Preset::from_name("InMeeting").unwrap(), Preset::InMeeting);
    /// ```
    pub fn from_name(s: &str) -> Result<Self> {
        let normalized: String = s.to_lowercase().replace('-', "");
        match normalized.as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            "yellow" => Ok(Self::Yellow),
            "cyan" => Ok(Self::Cyan),
            "magenta" => Ok(Self::Magenta),
            "white" => Ok(Self::White),
            "orange" => Ok(Self::Orange),
            "purple" => Ok(Self::Purple),
            "available" => Ok(Self::Available),
            "busy" => Ok(Self::Busy),
            "away" => Ok(Self::Away),
            "inmeeting" => Ok(Self::InMeeting),
            _ => Err(SlickyError::UnknownPreset(s.to_string())),
        }
    }

    /// The lowercase display name for this preset.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Red => "red",
            Self::Green => "green",
            Self::Blue => "blue",
            Self::Yellow => "yellow",
            Self::Cyan => "cyan",
            Self::Magenta => "magenta",
            Self::White => "white",
            Self::Orange => "orange",
            Self::Purple => "purple",
            Self::Available => "available",
            Self::Busy => "busy",
            Self::Away => "away",
            Self::InMeeting => "in-meeting",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Color::from_hex ---

    #[test]
    fn from_hex_6_char_with_hash() {
        let c = Color::from_hex("#FF0000").unwrap();
        assert_eq!(c, Color::new(255, 0, 0), "should parse #FF0000");
    }

    #[test]
    fn from_hex_6_char_without_hash() {
        let c = Color::from_hex("00FF00").unwrap();
        assert_eq!(c, Color::new(0, 255, 0), "should parse 00FF00");
    }

    #[test]
    fn from_hex_lowercase() {
        let c = Color::from_hex("ff8000").unwrap();
        assert_eq!(c, Color::new(255, 128, 0), "should parse lowercase hex");
    }

    #[test]
    fn from_hex_3_char_with_hash() {
        let c = Color::from_hex("#f00").unwrap();
        assert_eq!(
            c,
            Color::new(255, 0, 0),
            "should expand 3-char #f00 to #FF0000"
        );
    }

    #[test]
    fn from_hex_3_char_without_hash() {
        let c = Color::from_hex("0f0").unwrap();
        assert_eq!(c, Color::new(0, 255, 0), "should expand 3-char 0f0");
    }

    #[test]
    fn from_hex_all_zeros() {
        let c = Color::from_hex("000000").unwrap();
        assert_eq!(c, Color::off(), "000000 should equal Color::off()");
    }

    #[test]
    fn from_hex_all_ff() {
        let c = Color::from_hex("FFFFFF").unwrap();
        assert_eq!(c, Color::new(255, 255, 255), "FFFFFF should be white");
    }

    #[test]
    fn from_hex_invalid_chars() {
        assert!(
            Color::from_hex("ZZZZZZ").is_err(),
            "should reject invalid hex chars"
        );
    }

    #[test]
    fn from_hex_wrong_length() {
        assert!(Color::from_hex("FF00").is_err(), "should reject 4-char hex");
        assert!(
            Color::from_hex("FF00000").is_err(),
            "should reject 7-char hex"
        );
    }

    #[test]
    fn from_hex_empty() {
        assert!(Color::from_hex("").is_err(), "should reject empty string");
    }

    #[test]
    fn from_hex_hash_only() {
        assert!(Color::from_hex("#").is_err(), "should reject bare #");
    }

    // --- Color::to_hex ---

    #[test]
    fn to_hex_round_trip() {
        let c = Color::new(255, 128, 0);
        assert_eq!(c.to_hex(), "#FF8000");
    }

    #[test]
    fn to_hex_black() {
        assert_eq!(Color::off().to_hex(), "#000000");
    }

    // --- Color::is_off ---

    #[test]
    fn is_off_true() {
        assert!(Color::off().is_off());
    }

    #[test]
    fn is_off_false() {
        assert!(!Color::new(1, 0, 0).is_off());
    }

    // --- Color::Display ---

    #[test]
    fn display_matches_to_hex() {
        let c = Color::new(0, 128, 255);
        assert_eq!(format!("{c}"), c.to_hex());
    }

    // --- Preset::from_name ---

    #[test]
    fn preset_from_name_lowercase() {
        assert_eq!(Preset::from_name("red").unwrap(), Preset::Red);
    }

    #[test]
    fn preset_from_name_uppercase() {
        assert_eq!(
            Preset::from_name("RED").unwrap(),
            Preset::Red,
            "should be case-insensitive"
        );
    }

    #[test]
    fn preset_from_name_mixed_case() {
        assert_eq!(Preset::from_name("Green").unwrap(), Preset::Green);
    }

    #[test]
    fn preset_from_name_hyphenated() {
        assert_eq!(
            Preset::from_name("in-meeting").unwrap(),
            Preset::InMeeting,
            "should accept hyphenated names"
        );
    }

    #[test]
    fn preset_from_name_pascal_case() {
        assert_eq!(
            Preset::from_name("InMeeting").unwrap(),
            Preset::InMeeting,
            "should accept PascalCase"
        );
    }

    #[test]
    fn preset_from_name_unknown() {
        assert!(
            Preset::from_name("foobar").is_err(),
            "should reject unknown preset names"
        );
    }

    #[test]
    fn preset_all_variants() {
        assert_eq!(Preset::all().len(), 13, "should have 13 presets");
    }

    #[test]
    fn preset_name_round_trip() {
        for p in Preset::all() {
            let name = p.name();
            let resolved = Preset::from_name(name).unwrap();
            assert_eq!(*p, resolved, "round-trip for preset {name}");
        }
    }

    #[test]
    fn preset_color_values() {
        assert_eq!(Preset::Red.color(), Color::new(255, 0, 0));
        assert_eq!(Preset::Green.color(), Color::new(0, 255, 0));
        assert_eq!(Preset::Blue.color(), Color::new(0, 0, 255));
        assert_eq!(Preset::InMeeting.color(), Color::new(255, 69, 0));
        assert_eq!(Preset::Orange.color(), Color::new(255, 165, 0));
        assert_eq!(Preset::Purple.color(), Color::new(128, 0, 128));
    }
}
