

pub const WORD_ARRAY: [&'static str; 4] = [
    "us",
    "me",
    "you",
    "it",
];


pub const WORD_MAPPER: crate::mapper::WordMapper<'static> = crate::mapper::WordMapper::new(&WORD_ARRAY);
