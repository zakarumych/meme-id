use core::fmt;

use crate::{
    dict::{Adjective, Adverb, Mapper, Noun, Plural, Preposition, Verb},
    Hyphenated,
};

use super::{skip_one_of, string_to_words, Error};

pub struct ComplexPhrase {
    adjective1: &'static str,
    noun1: &'static str,
    verb1: &'static str,
    adverb1: &'static str,
    preposition1: &'static str,
    adjective2: &'static str,
    noun2: &'static str,
    preposition2: &'static str,
    noun3: &'static str,
    verb2: &'static str,
    adverb2: &'static str,
    adjective3: &'static str,
    adjective4: &'static str,
    noun4: &'static str,
    verb3: &'static str,
    verb4: &'static str,
}

impl ComplexPhrase {
    /// Encodes bits into `adjective noun verb adverb` scheme
    #[inline]
    pub fn encode(bits: u128) -> Self {
        encode(bits)
    }

    /// Transform to hyphenated.
    #[inline]
    pub fn hyphenated(self) -> Hyphenated<Self> {
        Hyphenated(self)
    }
}

impl fmt::Display for ComplexPhrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The {} {} {} {} {} the {} {} {} the {} and {} {} that the {} {} {} {} and {}",
            self.adjective1,
            self.noun1,
            self.verb1,
            self.adverb1,
            self.preposition1,
            self.adjective2,
            self.noun2,
            self.preposition2,
            self.noun3,
            self.verb2,
            self.adverb2,
            self.adjective3,
            self.adjective4,
            self.noun4,
            self.verb3,
            self.verb4
        )
    }
}

impl fmt::Display for Hyphenated<ComplexPhrase> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}",
            self.0.adjective1,
            self.0.noun1,
            self.0.verb1,
            self.0.adverb1,
            self.0.preposition1,
            self.0.adjective2,
            self.0.noun2,
            self.0.preposition2,
            self.0.noun3,
            self.0.verb2,
            self.0.adverb2,
            self.0.adjective3,
            self.0.adjective4,
            self.0.noun4,
            self.0.verb3,
            self.0.verb4,
        )
    }
}

/// Encodes bits into a complex phrase.
/// For 128-bit ids.
pub fn encode(bits: u128) -> ComplexPhrase {
    let ([adjective1, adjective2, adjective3, adjective4], bits) = Adjective::encode_words(bits);
    let ([noun1, noun2, noun3, noun4], bits) = Noun::encode_words(bits);
    let ([verb1, verb2, verb3, verb4], bits) = Verb::<Plural>::encode_words(bits);
    let ([adverb1, adverb2], bits) = Adverb::encode_words(bits);
    let ([preposition1, preposition2], bits) = Preposition::encode_words(bits);
    debug_assert_eq!(bits, 0);

    ComplexPhrase {
        adjective1,
        noun1,
        verb1,
        adverb1,
        preposition1,
        adjective2,
        noun2,
        preposition2,
        noun3,
        verb2,
        adverb2,
        adjective3,
        adjective4,
        noun4,
        verb3,
        verb4,
    }
}

/// Decodes a complex phrase.
/// For 128-bit ids.
pub fn decode(s: &str) -> Result<u128, Error> {
    let mut iter = string_to_words(s);

    skip_one_of(&mut iter, &["a", "the"]);

    let adjective1 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 0,
    })?;
    let noun1 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 1,
    })?;
    let verb1 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 2,
    })?;
    let adverb1 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 3,
    })?;
    let preposition1 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 4,
    })?;

    skip_one_of(&mut iter, &["a", "the"]);
    let adjective2 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 5,
    })?;
    let noun2 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 6,
    })?;
    let preposition2 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 7,
    })?;

    skip_one_of(&mut iter, &["a", "the"]);
    let noun3 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 8,
    })?;

    skip_one_of(&mut iter, &["and"]);
    let verb2 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 9,
    })?;
    let adverb2 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 10,
    })?;

    skip_one_of(&mut iter, &["that"]);
    skip_one_of(&mut iter, &["a", "the"]);
    let adjective3 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 11,
    })?;
    let adjective4 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 12,
    })?;
    let noun4 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 13,
    })?;
    let verb3 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 14,
    })?;

    skip_one_of(&mut iter, &["and"]);

    let verb4 = iter.next().ok_or(Error::NotEnoughWords {
        expected: 16,
        actual: 15,
    })?;

    let mut bits = 0;

    let prepositions = [preposition2, preposition1];
    let adverbs = [adverb2, adverb1];
    let verbs = [verb4, verb3, verb2, verb1];
    let nouns = [noun4, noun3, noun2, noun1];
    let adjectives = [adjective4, adjective3, adjective2, adjective1];

    bits = Preposition::decode_words(prepositions, bits).map_err(|i| Error::Unrecognized {
        word: prepositions[i],
    })?;
    bits = Adverb::decode_words(adverbs, bits)
        .map_err(|i| Error::Unrecognized { word: adverbs[i] })?;
    bits = Verb::<Plural>::decode_words(verbs, bits)
        .map_err(|i| Error::Unrecognized { word: verbs[i] })?;
    bits = Noun::decode_words(nouns, bits).map_err(|i| Error::Unrecognized { word: nouns[i] })?;
    bits = Adjective::decode_words(adjectives, bits).map_err(|i| Error::Unrecognized {
        word: adjectives[i],
    })?;

    Ok(bits)
}

#[cfg(feature = "serde")]
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Copy + Into<u128>,
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
    u128: Into<T>,
    D: serde::de::Deserializer<'de>,
{
    use alloc::borrow::Cow;

    let s = <Cow<str> as serde::de::Deserialize>::deserialize(deserializer)?;
    match decode(&*s) {
        Err(err) => Err(serde::de::Error::custom(err)),
        Ok(id) => Ok(id.into()),
    }
}
