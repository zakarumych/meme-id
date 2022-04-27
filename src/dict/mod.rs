mod adjective;
mod adverb;
mod noun;
mod object_pronoun;
mod preposition;
mod verb_plural;
mod verb_singular;

pub enum Singular {}

pub enum Plural {}

pub trait Mapper {
    fn encode_word(bits: u128) -> (&'static str, u128);
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128);
    fn decode_word(word: &str, bits: u128) -> Option<u128>;
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize>;
}

pub enum Adjective {}

impl Mapper for Adjective {
    fn encode_word(bits: u128) -> (&'static str, u128) {
        adjective::WORD_MAPPER.encode_word(bits)
    }
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128) {
        adjective::WORD_MAPPER.encode_words_norepeat(bits)
    }
    fn decode_word(word: &str, bits: u128) -> Option<u128> {
        adjective::WORD_MAPPER.decode_word(word, bits)
    }
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize> {
        adjective::WORD_MAPPER.decode_words_norepeat(words, bits)
    }
}

pub enum Noun {}

impl Mapper for Noun {
    fn encode_word(bits: u128) -> (&'static str, u128) {
        noun::WORD_MAPPER.encode_word(bits)
    }
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128) {
        noun::WORD_MAPPER.encode_words_norepeat(bits)
    }
    fn decode_word(word: &str, bits: u128) -> Option<u128> {
        noun::WORD_MAPPER.decode_word(word, bits)
    }
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize> {
        noun::WORD_MAPPER.decode_words_norepeat(words, bits)
    }
}

pub enum Verb<T> {
    _Unused(T),
}

impl Mapper for Verb<Singular> {
    fn encode_word(bits: u128) -> (&'static str, u128) {
        verb_singular::WORD_MAPPER.encode_word(bits)
    }
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128) {
        verb_singular::WORD_MAPPER.encode_words_norepeat(bits)
    }
    fn decode_word(word: &str, bits: u128) -> Option<u128> {
        verb_singular::WORD_MAPPER.decode_word(word, bits)
    }
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize> {
        verb_singular::WORD_MAPPER.decode_words_norepeat(words, bits)
    }
}

impl Mapper for Verb<Plural> {
    fn encode_word(bits: u128) -> (&'static str, u128) {
        verb_plural::WORD_MAPPER.encode_word(bits)
    }
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128) {
        verb_plural::WORD_MAPPER.encode_words_norepeat(bits)
    }
    fn decode_word(word: &str, bits: u128) -> Option<u128> {
        verb_plural::WORD_MAPPER.decode_word(word, bits)
    }
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize> {
        verb_plural::WORD_MAPPER.decode_words_norepeat(words, bits)
    }
}

pub enum Adverb {}

impl Mapper for Adverb {
    fn encode_word(bits: u128) -> (&'static str, u128) {
        adverb::WORD_MAPPER.encode_word(bits)
    }
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128) {
        adverb::WORD_MAPPER.encode_words_norepeat(bits)
    }
    fn decode_word(word: &str, bits: u128) -> Option<u128> {
        adverb::WORD_MAPPER.decode_word(word, bits)
    }
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize> {
        adverb::WORD_MAPPER.decode_words_norepeat(words, bits)
    }
}

pub enum Preposition {}

impl Mapper for Preposition {
    fn encode_word(bits: u128) -> (&'static str, u128) {
        preposition::WORD_MAPPER.encode_word(bits)
    }
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128) {
        preposition::WORD_MAPPER.encode_words_norepeat(bits)
    }
    fn decode_word(word: &str, bits: u128) -> Option<u128> {
        preposition::WORD_MAPPER.decode_word(word, bits)
    }
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize> {
        preposition::WORD_MAPPER.decode_words_norepeat(words, bits)
    }
}

pub enum ObjectPronoun {}

impl Mapper for ObjectPronoun {
    fn encode_word(bits: u128) -> (&'static str, u128) {
        object_pronoun::WORD_MAPPER.encode_word(bits)
    }
    fn encode_words<const N: usize>(bits: u128) -> ([&'static str; N], u128) {
        object_pronoun::WORD_MAPPER.encode_words(bits)
    }
    fn decode_word(word: &str, bits: u128) -> Option<u128> {
        object_pronoun::WORD_MAPPER.decode_word(word, bits)
    }
    fn decode_words<const N: usize>(words: [&str; N], bits: u128) -> Result<u128, usize> {
        object_pronoun::WORD_MAPPER.decode_words(words, bits)
    }
}
