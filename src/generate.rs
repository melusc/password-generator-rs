extern crate rand;

use super::args::Config;
use rand::{
    rngs::ThreadRng,
    seq::{IteratorRandom, SliceRandom},
};

const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER: &'static str = "0123456789";
const SPECIAL: &'static str = "~!@#$%^&*()_-+=:;<,>.?/";

fn random_str(str: &str, rng: &mut ThreadRng) -> char {
    // We know all inputs will be non-empty
    str.chars().choose(rng).unwrap()
}

pub fn generate_password(config: Config) -> String {
    // Allocate for max length of all combined charsets 26 + 26 + 10 + 23
    let mut all_chars = String::with_capacity(85);
    // Allocate for default password length 32
    let mut password = Vec::with_capacity(32);

    let mut rng = rand::rng();

    if config.special {
        all_chars.push_str(SPECIAL);
        password.push(random_str(SPECIAL, &mut rng))
    }

    if config.number {
        all_chars.push_str(NUMBER);
        password.push(random_str(NUMBER, &mut rng))
    }

    if config.upper {
        all_chars.push_str(UPPERCASE);
        password.push(random_str(UPPERCASE, &mut rng))
    }

    if config.lower {
        all_chars.push_str(LOWERCASE);
        password.push(random_str(LOWERCASE, &mut rng))
    }

    let length = usize::from(config.length);
    while password.len() < length {
        password.push(random_str(&all_chars, &mut rng));
    }

    password.shuffle(&mut rng);

    String::from_iter(password)
}
