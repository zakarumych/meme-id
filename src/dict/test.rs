

pub const WORD_ARRAY: [&'static str; 4] = [
    "bca",
    "abc",
    "qwe",
    "aaa",
];


pub const WORD_SEARCH: crate::mapper::WordMapper<'static> = crate::mapper::WordMapper::new(&WORD_ARRAY);
