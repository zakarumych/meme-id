use std::env::args;

use meme_id::{adjective_noun, complex_phrase, phrase, simple_phrase};

fn main() {
    let mut args = args();
    let arg = match args.nth(1) {
        None => {
            eprintln!("Expected one numeric argument");
            std::process::exit(1);
        }
        Some(arg) => arg,
    };

    let num: u128 = match arg.parse() {
        Err(err) => {
            eprintln!("Expected one numeric argument. {:#}", err);
            std::process::exit(1);
        }
        Ok(num) => num,
    };

    if num <= u16::MAX as u128 {
        println!("{}", adjective_noun::encode(num as u16).to_string());
    } else if num <= u32::MAX as u128 {
        println!("{}", simple_phrase::encode(num as u32).to_string());
    } else if num <= u64::MAX as u128 {
        println!("{}", phrase::encode(num as u64).to_string());
        // println!("{}", punk::encode(num as u64).to_string());
    } else {
        println!("{}", complex_phrase::encode(num).to_string());
    }
}
