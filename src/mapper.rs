use core::{
    cmp::{Ord, Ordering},
    ops::ControlFlow,
};

pub struct WordMapper<'a> {
    array: &'a [&'a str],
}

impl<'a> WordMapper<'a> {
    /// Returns new world mapper instance from particularly ordered array of words.
    pub(crate) const fn new(array: &'a [&'a str]) -> Self {
        assert!(array.len().is_power_of_two());
        WordMapper { array }
    }

    /// Returns number of bits that can be encoded by word with this mapper.
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.array.len().trailing_zeros()
    }

    /// Returns bit mask for bits.
    #[inline]
    pub const fn bit_mask(&self) -> usize {
        self.array.len() - 1
    }

    /// Returns bits for the specified word.
    #[inline]
    pub fn decode_word(&self, word: &str, bits: u128) -> Option<u128> {
        let idx = eytzinger_search(self.array, word)?;
        Some(bits << self.bits() | idx as u128)
    }

    /// Returns bits for the specified word.
    #[inline]
    pub fn decode_words<const N: usize>(
        &self,
        words: [&str; N],
        mut bits: u128,
    ) -> Result<u128, usize> {
        let mask = self.bit_mask();
        let shift = self.bits();

        for (i, word) in words.iter().enumerate() {
            let idx = eytzinger_search(self.array, word).ok_or(i)?;
            bits = (bits << shift) | (idx & mask) as u128;
        }

        Ok(bits)
    }

    /// Returns bits for the specified word.
    #[inline]
    pub fn decode_words_norepeat<const N: usize>(
        &self,
        words: [&str; N],
        mut bits: u128,
    ) -> Result<u128, usize> {
        let less_bits_each = N.next_power_of_two().trailing_zeros();
        let mask = self.bit_mask() >> less_bits_each;
        let shift = self.bits() - less_bits_each;

        for (i, word) in words.iter().enumerate() {
            let idx = eytzinger_search(self.array, word).ok_or(i)?;
            bits = (bits << shift) | (idx & mask) as u128;
        }

        Ok(bits)
    }

    /// Return word for the specified bits.
    #[inline]
    pub fn encode_word(&self, mut bits: u128) -> (&'a str, u128) {
        let word = self.array[(bits as usize) & self.bit_mask()];
        bits >>= self.bits();
        (word, bits)
    }

    /// Return word for the specified bits.
    #[inline]
    pub fn encode_words<const N: usize>(&self, mut bits: u128) -> ([&'a str; N], u128) {
        let mask = self.bit_mask();
        let shift = self.bits();

        let mut i = 0;
        let words = [(); N].map(|()| {
            let word = self.array[(bits as usize) & mask];
            i += 1;
            bits >>= shift;
            word
        });

        (words, bits)
    }

    /// Return word for the specified bits.
    #[inline]
    pub fn encode_words_norepeat<const N: usize>(&self, mut bits: u128) -> ([&'a str; N], u128) {
        let less_bits_each = N.next_power_of_two().trailing_zeros();
        let mask = self.bit_mask() >> less_bits_each;
        let shift = self.bits() - less_bits_each;

        let mut i = 0;
        let words = [(); N].map(|()| {
            let word = self.array[((bits as usize) & mask) + (i << shift)];
            i += 1;
            bits >>= shift;
            word
        });

        (words, bits)
    }
}

#[inline]
fn eytzinger_search(array: &[&str], s: &str) -> Option<usize> {
    let mut i = 0;
    while i < array.len() {
        let v = array[i]; // this range check is optimized out :D
        i = match cmp_ignore_case_ascii(v, s) {
            Ordering::Greater | Ordering::Equal => 2 * i + 1,
            Ordering::Less => 2 * i + 2,
        };
    }

    // magic from the paper to fix up the (incomplete) final tree layer
    // (only difference is that we recheck f() because this is exact search)
    let p = i + 1;
    let j = p >> (1 + (!p).trailing_zeros());
    if j != 0 && (str::eq_ignore_ascii_case(array[j - 1], s)) {
        Some(j - 1)
    } else {
        None
    }
}

#[inline]
fn cmp_ignore_case_ascii(a: &str, b: &str) -> Ordering {
    let cf = a.bytes().zip(b.bytes()).try_for_each(|(a, b)| {
        match Ord::cmp(&a.to_ascii_lowercase(), &b.to_ascii_lowercase()) {
            Ordering::Equal => ControlFlow::Continue(()),
            ord => ControlFlow::Break(ord),
        }
    });

    match cf {
        ControlFlow::Break(ord) => ord,
        _ => Ord::cmp(&a.len(), &b.len()),
    }
}
