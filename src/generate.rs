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

pub fn generate_password(config: &Config) -> Result<String, &str> {
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

    if password.is_empty() {
        return Err("No charsets selected!");
    }

    let length = usize::from(config.length);
    while password.len() < length {
        password.push(random_str(&all_chars, &mut rng));
    }

    password.shuffle(&mut rng);

    Ok(String::from_iter(password))
}

#[cfg(test)]
mod test_generate_password {
    use std::collections::HashSet;

    use super::*;

    fn test_config(config: &Config, length: Option<u16>) {
        let length = length.unwrap_or(config.length);
        let length = usize::try_from(length).unwrap();

        let pw = generate_password(config).unwrap();
        assert_eq!(pw.len(), length);
        let pw_set: HashSet<char> = pw.chars().collect();
        assert_eq!(
            LOWERCASE.chars().any(|c| pw_set.contains(&c)),
            config.lower,
            "Expected lowercase = {}",
            config.lower
        );
        assert_eq!(
            UPPERCASE.chars().any(|c| pw_set.contains(&c)),
            config.upper,
            "Expected uppercase = {}",
            config.upper
        );
        assert_eq!(
            NUMBER.chars().any(|c| pw_set.contains(&c)),
            config.number,
            "Expected number = {}",
            config.number
        );
        assert_eq!(
            SPECIAL.chars().any(|c| pw_set.contains(&c)),
            config.special,
            "Expected special = {}",
            config.special
        );
    }

    #[test]
    fn normal_config() {
        let config = Config {
            length: 32,
            lower: true,
            upper: true,
            number: true,
            special: true,
        };
        test_config(&config, None);
    }

    #[test]
    fn shortest_password() {
        let config = Config {
            length: 0,
            lower: true,
            upper: true,
            number: true,
            special: true,
        };
        test_config(&config, Some(4));
    }

    #[test]
    fn lower_only() {
        let config = Config {
            length: 10,
            lower: true,
            upper: false,
            number: false,
            special: false,
        };
        test_config(&config, None);
    }

    #[test]
    fn upper_only() {
        let config = Config {
            length: 10,
            lower: false,
            upper: true,
            number: false,
            special: false,
        };
        test_config(&config, None);
    }

    #[test]
    fn number_only() {
        let config = Config {
            length: 10,
            lower: false,
            upper: false,
            number: true,
            special: false,
        };
        test_config(&config, None);
    }

    #[test]
    fn special_only() {
        let config = Config {
            length: 10,
            lower: false,
            upper: false,
            number: false,
            special: true,
        };
        test_config(&config, None);
    }

    #[test]
    fn no_charsets_selected() {
        let config = Config {
            length: 32,
            lower: false,
            upper: false,
            number: false,
            special: false,
        };
        let pw_result = generate_password(&config);
        pw_result.expect_err("generate_password did not error");
    }
}
