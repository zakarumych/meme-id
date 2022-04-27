use core::fmt;

use crate::{
    dict::{Adjective, Mapper, ObjectPronoun, Singular, Verb},
    Hyphenated,
};

use super::{string_to_words, Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Punk {
    pub verbs: [&'static str; 4],
    pub pronouns: [&'static str; 4],
    pub adjectives: [&'static str; 4],
}

impl Punk {
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

impl fmt::Display for Punk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {}{}, {}{} {}{}\n{}{} {}{}, {}{} {}{}\n{}{}, {}{}\n{}{}, {}{}",
            &self.verbs[0].chars().next().unwrap().to_uppercase(),
            &self.verbs[0][1..],
            &self.pronouns[0].chars().next().unwrap().to_uppercase(),
            &self.pronouns[0][1..],
            &self.verbs[1].chars().next().unwrap().to_uppercase(),
            &self.verbs[1][1..],
            &self.pronouns[1].chars().next().unwrap().to_uppercase(),
            &self.pronouns[1][1..],
            &self.verbs[2].chars().next().unwrap().to_uppercase(),
            &self.verbs[2][1..],
            &self.pronouns[2].chars().next().unwrap().to_uppercase(),
            &self.pronouns[2][1..],
            &self.verbs[3].chars().next().unwrap().to_uppercase(),
            &self.verbs[3][1..],
            &self.pronouns[3].chars().next().unwrap().to_uppercase(),
            &self.pronouns[3][1..],
            &self.adjectives[0].chars().next().unwrap().to_uppercase(),
            &self.adjectives[0][1..],
            &self.adjectives[1].chars().next().unwrap().to_uppercase(),
            &self.adjectives[1][1..],
            &self.adjectives[2].chars().next().unwrap().to_uppercase(),
            &self.adjectives[2][1..],
            &self.adjectives[3].chars().next().unwrap().to_uppercase(),
            &self.adjectives[3][1..],
        )
    }
}

impl fmt::Display for Hyphenated<Punk> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}",
            &self.0.verbs[0],
            &self.0.pronouns[0],
            &self.0.verbs[1],
            &self.0.pronouns[1],
            &self.0.verbs[2],
            &self.0.pronouns[2],
            &self.0.verbs[3],
            &self.0.pronouns[3],
            &self.0.adjectives[0],
            &self.0.adjectives[1],
            &self.0.adjectives[2],
            &self.0.adjectives[3],
        )
    }
}

/// Encodes bits into a punky phrase.
/// For 64-bit ids.
pub fn encode(bits: u64) -> Punk {
    let (verbs, bits) = Verb::<Singular>::encode_words(bits.into());
    let (pronouns, bits) = ObjectPronoun::encode_words(bits.into());
    let (adjectives, bits) = Adjective::encode_words(bits.into());

    debug_assert_eq!(bits, 0);

    Punk {
        verbs,
        pronouns,
        adjectives,
    }
}

/// Decodes a punky phrase.
/// For 64-bit ids.
pub fn decode(s: &str) -> Result<u64, Error> {
    let mut split = string_to_words(s);

    let verb1 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 0,
    })?;
    let pronoun1 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 1,
    })?;
    let verb2 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 2,
    })?;
    let pronoun2 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 3,
    })?;
    let verb3 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 4,
    })?;
    let pronoun3 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 5,
    })?;
    let verb4 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 6,
    })?;
    let pronoun4 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 7,
    })?;
    let adjective1 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 8,
    })?;
    let adjective2 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 9,
    })?;
    let adjective3 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 10,
    })?;
    let adjective4 = split.next().ok_or(Error::NotEnoughWords {
        expected: 12,
        actual: 11,
    })?;

    if split.next().is_some() {
        return Err(Error::TrailingWords);
    }

    let adjectives = [adjective4, adjective3, adjective2, adjective1];
    let pronouns = [pronoun4, pronoun3, pronoun2, pronoun1];
    let verbs = [verb4, verb3, verb2, verb1];

    let mut bits = 0;
    bits = Adjective::decode_words(adjectives, bits).map_err(|i| Error::Unrecognized {
        word: adjectives[i],
    })?;
    bits = ObjectPronoun::decode_words(pronouns, bits)
        .map_err(|i| Error::Unrecognized { word: pronouns[i] })?;
    bits = Verb::<Singular>::decode_words(verbs, bits)
        .map_err(|i| Error::Unrecognized { word: verbs[i] })?;

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
