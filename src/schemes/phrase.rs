use core::fmt;

use crate::{
    dict::{Adjective, Adverb, Mapper, Noun, Plural, Preposition, Verb},
    Hyphenated,
};

use super::{skip_one_of, string_to_words, Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Phrase {
    pub adjective1: &'static str,
    pub adjective2: &'static str,
    pub noun1: &'static str,
    pub verb: &'static str,
    pub adverb: &'static str,
    pub preposition: &'static str,
    pub adjective3: &'static str,
    pub noun2: &'static str,
}

impl Phrase {
    /// Encodes bits into `adjective noun verb adverb` scheme
    #[inline]
    pub fn encode(bits: u64) -> Self {
        encode(bits)
    }

    /// Transform to hyphenated.
    #[inline]
    pub fn hyphenated(self) -> Hyphenated<Self> {
        Hyphenated(self)
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The {} {} {} {} {} {} the {} {}",
            self.adjective1,
            self.adjective2,
            self.noun1,
            self.verb,
            self.adverb,
            self.preposition,
            self.adjective3,
            self.noun2
        )
    }
}

impl fmt::Display for Hyphenated<Phrase> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}-{}-{}-{}-{}",
            self.0.adjective1,
            self.0.adjective2,
            self.0.noun1,
            self.0.verb,
            self.0.adverb,
            self.0.preposition,
            self.0.adjective3,
            self.0.noun2
        )
    }
}

/// Encodes bits into a phrase.
/// For 64-bit ids.
pub fn encode(bits: u64) -> Phrase {
    let ([adjective1, adjective2, adjective3], bits) = Adjective::encode_words(bits.into());
    let ([noun1, noun2], bits) = Noun::encode_words(bits);
    let (verb, bits) = Verb::<Plural>::encode_word(bits);
    let (adverb, bits) = Adverb::encode_word(bits);
    let (preposition, bits) = Preposition::encode_word(bits);

    debug_assert_eq!(bits, 0);

    Phrase {
        adjective1,
        adjective2,
        noun1,
        verb,
        adverb,
        preposition,
        adjective3,
        noun2,
    }
}

/// Decodes a phrase.
/// For 64-bit ids.
pub fn decode(s: &str) -> Result<u64, Error> {
    let mut iter = string_to_words(s);

    skip_one_of(&mut iter, &["a", "the"]);

    let adjective1 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 0,
    })?;
    let adjective2 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 1,
    })?;
    let noun1 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 2,
    })?;
    let verb = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 3,
    })?;
    let adverb = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 4,
    })?;
    let preposition = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 5,
    })?;

    skip_one_of(&mut iter, &["a", "the"]);
    let adjective3 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 6,
    })?;
    let noun2 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 8,
        actual: 7,
    })?;

    if iter.next().is_some() {
        return Err(Error::TrailingWords);
    }

    let nouns = [noun2, noun1];
    let adjectives = [adjective3, adjective2, adjective1];

    let mut bits = 0;
    bits = Preposition::decode_word(preposition, bits)
        .ok_or(Error::Unrecognized { word: preposition })?;
    bits = Adverb::decode_word(adverb, bits).ok_or(Error::Unrecognized { word: adverb })?;
    bits = Verb::<Plural>::decode_word(verb, bits).ok_or(Error::Unrecognized { word: verb })?;
    bits = Noun::decode_words(nouns, bits).map_err(|i| Error::Unrecognized { word: nouns[i] })?;
    bits = Adjective::decode_words(adjectives, bits).map_err(|i| Error::Unrecognized {
        word: adjectives[i],
    })?;

    Ok(bits as u64)
}

#[cfg(feature = "serde")]
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Copy + Into<u64>,
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
    u64: Into<T>,
    D: serde::de::Deserializer<'de>,
{
    use alloc::borrow::Cow;

    let s = <Cow<str> as serde::de::Deserialize>::deserialize(deserializer)?;
    match decode(&*s) {
        Err(err) => Err(serde::de::Error::custom(err)),
        Ok(id) => Ok(id.into()),
    }
}
