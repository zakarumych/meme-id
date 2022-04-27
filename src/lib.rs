//!
//! meme-id crate.
//!

#![no_std]

#[cfg(feature = "serde")]
extern crate alloc;

mod dict;
mod mapper;
mod schemes;

pub use self::schemes::*;
