pub mod adjective_noun;
pub mod complex_phrase;
pub mod phrase;
pub mod punk;
pub mod simple_phrase;

use core::{fmt, iter::Peekable};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error<'a> {
    NotEnoughWords { expected: usize, actual: usize },
    TrailingWords,
    Unrecognized { word: &'a str },
}
impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotEnoughWords { expected, actual } => {
                write!(
                    f,
                    "Not enough words. Expected {}, actual {}",
                    expected, actual
                )
            }
            Error::TrailingWords => {
                write!(f, "Words left after parsing")
            }
            Error::Unrecognized { word } => {
                write!(f, "Word '{}' unrecognized", word)
            }
        }
    }
}

fn skip_one_of<'a>(iter: &mut Peekable<impl Iterator<Item = &'a str>>, skip: &[&str]) {
    iter.next_if(|word| {
        skip.iter()
            .any(|skip| str::eq_ignore_ascii_case(word, skip))
    });
}

fn string_to_words<'a>(s: &'a str) -> Peekable<impl Iterator<Item = &'a str>> {
    s.split(|ch: char| !ch.is_ascii_alphabetic())
        .filter(|s| !s.is_empty() && !s.contains(|ch: char| !ch.is_ascii_alphabetic()))
        .peekable()
}

/// Wrapper that changes `Display` behavior of the scheme.
/// Making it emit all words in one line with hyphen between them.
/// Without auxiliary words.
pub struct Hyphenated<T>(pub T);
