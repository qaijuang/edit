// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Everything related to Unicode lives here.

mod measurement;
mod tables;

pub use measurement::*;

#[inline]
pub(crate) fn is_non_control_warning(ch: char) -> bool {
    // Rust exposes White_Space but not Bidi_Control. The explicit values are
    // Bidi_Control, ZERO WIDTH SPACE, WORD JOINER, and ZERO WIDTH NO-BREAK SPACE.
    !ch.is_ascii()
        && !ch.is_control()
        && (ch.is_whitespace()
            || matches!(
                ch,
                '\u{061c}'
                    | '\u{200b}'
                    | '\u{200e}'..='\u{200f}'
                    | '\u{202a}'..='\u{202e}'
                    | '\u{2060}'
                    | '\u{2066}'..='\u{2069}'
                    | '\u{feff}'
            ))
}

#[cfg(test)]
mod tests {
    use super::is_non_control_warning;

    #[test]
    fn non_control_warning_matches_expected_characters() {
        for ch in char::MIN..=char::MAX {
            let value = ch as u32;
            // Keep this oracle pinned to Unicode 16. Rust may use a newer Unicode version.
            let expected = matches!(
                value,
                // Bidi_Control
                0x061C
                | 0x200E..=0x200F
                | 0x202A..=0x202E
                | 0x2066..=0x2069
                // Non-control, non-ASCII White_Space
                | 0x00A0
                | 0x1680
                | 0x2000..=0x200A
                | 0x2028..=0x2029
                | 0x202F
                | 0x205F
                | 0x3000
                // Other format characters
                | 0x200B
                | 0x2060
                | 0xFEFF
            );
            let actual = is_non_control_warning(ch);
            assert_eq!(actual, expected, "U+{value:04X}");
        }
    }
}
