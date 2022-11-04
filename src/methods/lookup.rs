use std::fmt::Display;

use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};

/// Searches for a word or phrase in the dictionary and returns an automatically
/// generated dictionary entry.
#[derive(Debug, Serialize, Args)]
pub struct LookupRequest {
    /// Translation direction. For example, "en-ru" specifies to translate from
    /// English to Russian.
    pub lang: String,
    /// The word or phrase to find in the dictionary.
    #[clap(value_parser = validate_word)]
    pub text: String,
    /// The language of the user's interface for displaying names of parts of
    /// speech in the dictionary entry.
    pub ui: Option<String>,
    /// Search options (bitmask of flags).
    pub flags: Option<Flags>,
}

#[derive(Debug, Copy, Clone, Serialize, ValueEnum)]
pub enum Flags {
    /// Apply the family search filter
    Family = 0x0001,
    ///  Enable searching by word form.
    Morpho = 0x0004,
    /// Enable a filter that requires matching parts of
    /// speech for the search word and translation.
    PosFilter = 0x0008,
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", *self as u16)
    }
}

impl LookupRequest {
    pub fn new(lang: &str, text: &str, ui: Option<&str>, flags: Option<Flags>) -> Self {
        Self {
            lang: lang.to_owned(),
            text: text.to_owned(),
            ui: ui.map(str::to_string),
            flags,
        }
    }

    pub fn en_ru(text: &str) -> Self {
        Self::new("en-ru", text, None, None)
    }

    pub fn ru_en(text: &str) -> Self {
        Self::new("ru-en", text, None, None)
    }
}

#[derive(Debug, Deserialize)]
pub struct LookupResult {
    /// Result header (not used).
    #[serde(skip)]
    pub head: Option<()>,
    /// Dictionary entries. A transcription of the search word may be
    /// provided in the ts attribute.
    pub def: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(flatten)]
    pub attributes: Attributes,
    /// Translations
    pub tr: Option<Vec<Entry>>,
    /// Synonyms
    pub syn: Option<Vec<Entry>>,
    /// Meanings
    pub mean: Option<Vec<Entry>>,
    /// Examples
    pub ex: Option<Vec<Entry>>,
}

/// Attributes used in def, tr, syn, mean, and ex
#[derive(Debug, Deserialize)]
pub struct Attributes {
    /// Text of the entry, translation, or synonym (mandatory).
    pub text: String,
    /// Part of speech (may be omitted).
    pub pos: Option<String>,
    /// Aspect (if applicable)
    pub asp: Option<String>,
}

fn validate_word(s: &str) -> Result<String, String> {
    let s = s.trim();
    if s.chars().all(|c| c.is_alphabetic() || c == '-') {
        Ok(s.to_string())
    } else {
        Err("Text contains non-alphabetic characters".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation() {
        validate_word("hello").unwrap();
        validate_word("привет").unwrap();
        validate_word("   hello  ").unwrap();
        validate_word("fixed-price").unwrap();
        validate_word("myYpa$$word!").unwrap_err();
        validate_word("12345").unwrap_err();
        validate_word("hel lo").unwrap_err();
    }
}
