use std::env::args;

use meme_id::{adjective_noun, complex_phrase, phrase, punk, simple_phrase, Error};

fn main() {
    let mut args = args();
    let arg = match args.nth(1) {
        None => {
            eprintln!("Expected one numeric argument");
            std::process::exit(1);
        }
        Some(arg) => arg,
    };

    match adjective_noun::decode(&arg) {
        Ok(num) => {
            println!("{}", num);
            return;
        }
        Err(Error::TrailingWords) => {}
        Err(err) => {
            eprintln!("Failed: {}", err);
            std::process::exit(1);
        }
    }

    match simple_phrase::decode(&arg) {
        Ok(num) => {
            println!("{}", num);
            return;
        }
        Err(Error::TrailingWords) => {}
        Err(err) => {
            eprintln!("Failed: {}", err);
            std::process::exit(1);
        }
    }

    match phrase::decode(&arg) {
        Ok(num) => {
            println!("{}", num);
            return;
        }
        Err(Error::TrailingWords) => {}
        Err(err) => {
            eprintln!("Failed: {}", err);
            std::process::exit(1);
        }
    }

    match punk::decode(&arg) {
        Ok(num) => {
            println!("{}", num);
            return;
        }
        Err(Error::TrailingWords) => {}
        Err(err) => {
            eprintln!("Failed: {}", err);
            std::process::exit(1);
        }
    }

    match complex_phrase::decode(&arg) {
        Ok(num) => {
            println!("{}", num);
            return;
        }
        Err(err) => {
            eprintln!("Failed: {}", err);
            std::process::exit(1);
        }
    }
}
