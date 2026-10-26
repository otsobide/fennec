//! Caseless, composed Unicode text.

use unicode_normalization::UnicodeNormalization;

/// Returns `text` lowercased with the Unicode default case mapping and put
/// in Normalization Form C, so canonically equivalent spellings in any case
/// compare equal as plain strings.
///
/// Lowercasing comes first and NFC last, because a lowercase mapping can
/// produce a decomposed sequence (`İ` lowercases to `i` followed by a
/// combining dot above).
///
/// # Example
///
/// ```rust
/// use shared_valueobject::domain::unicode::lowercase_nfc::lowercase_nfc;
///
/// assert_eq!(lowercase_nfc("JOSE\u{301}"), "jos\u{e9}");
/// ```
pub fn lowercase_nfc(text: &str) -> String {
    text.to_lowercase().nfc().collect()
}
