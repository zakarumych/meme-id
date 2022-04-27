use core::fmt;

use crate::{
    dict::{Adjective, Mapper, Noun},
    Hyphenated,
};

use super::{skip_one_of, string_to_words, Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdjectiveNoun {
    pub adjective: &'static str,
    pub noun: &'static str,
}

impl AdjectiveNoun {
    /// Encodes bits into `adjective noun verb adverb` scheme
    #[inline]
    pub fn encode(bits: u16) -> Self {
        encode(bits)
    }

    /// Transform to hyphenated.
    #[inline]
    pub fn hyphenated(self) -> Hyphenated<Self> {
        Hyphenated(self)
    }
}

impl fmt::Display for AdjectiveNoun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The {} {}", self.adjective, self.noun)
    }
}

impl fmt::Display for Hyphenated<AdjectiveNoun> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0.adjective, self.0.noun)
    }
}

/// Encodes bits into `adjective-noun` scheme
/// For 16-bit ids.
pub fn encode(bits: u16) -> AdjectiveNoun {
    let (adjective, bits) = Adjective::encode_word(bits.into());
    let (noun, bits) = Noun::encode_word(bits);

    debug_assert_eq!(bits, 0);

    AdjectiveNoun { adjective, noun }
}

/// Decodes `adjective-noun` scheme
/// For 16-bit ids.
pub fn decode(s: &str) -> Result<u16, Error> {
    let mut iter = string_to_words(s);

    skip_one_of(&mut iter, &["a", "the"]);

    let adjective = iter.next().ok_or(Error::NotEnoughWords {
        expected: 2,
        actual: 0,
    })?;
    let noun = iter.next().ok_or(Error::NotEnoughWords {
        expected: 2,
        actual: 1,
    })?;

    if iter.next().is_some() {
        return Err(Error::TrailingWords);
    }

    let mut bits = 0;
    bits = Noun::decode_word(noun, bits).ok_or(Error::Unrecognized { word: noun })?;
    bits =
        Adjective::decode_word(adjective, bits).ok_or(Error::Unrecognized { word: adjective })?;
    Ok(bits as u16)
}

#[cfg(feature = "serde")]
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Copy + Into<u16>,
    S: serde::ser::Serializer,
{
    use alloc::string::ToString;
    use serde::Serialize;

    let an = encode((*value).into());
    an.to_string().serialize(serializer)
}

#[cfg(feature = "serde")]
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    u16: Into<T>,
    D: serde::de::Deserializer<'de>,
{
    use alloc::borrow::Cow;

    let s = <Cow<str> as serde::de::Deserialize>::deserialize(deserializer)?;
    match decode(&*s) {
        Err(err) => Err(serde::de::Error::custom(err)),
        Ok(id) => Ok(id.into()),
    }
}
