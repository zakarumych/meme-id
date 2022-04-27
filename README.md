# meme-id

[![crates](https://img.shields.io/crates/v/meme-id.svg?style=for-the-badge&label=meme-id)](https://crates.io/crates/meme-id)
[![docs](https://img.shields.io/badge/docs.rs-meme--id-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white)](https://docs.rs/meme-id)
[![actions](https://img.shields.io/github/workflow/status/zakarumych/meme-id/badge/master?style=for-the-badge)](https://github.com/zakarumych/meme-id/actions?query=workflow%3ARust)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](COPYING)
![loc](https://img.shields.io/tokei/lines/github/zakarumych/meme-id?style=for-the-badge)

Provides functions to transform IDs to memorable phrases and back.
Supports parsing IDs from strings with arbitrary casing and delimiters.

16, 32, 64 and 128 bit IDs are supported.

`serde` feature enables usage of serde attribute `#[serde(with = "meme_id::<scheme-name>")]`
to serialize and deserialize IDs into phrases.

## License

Licensed under either of

* Apache License, Version 2.0, ([license/APACHE](license/APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([license/MIT](license/MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
