use std::{process, u16};

#[derive(Debug, PartialEq)]
pub enum SpecialType {
    None,
    Basic,
    Full,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub lower: bool,
    pub upper: bool,
    pub special: SpecialType,
    pub number: bool,
    pub length: u16,
}

fn print_help() {
    println!(
        "    pw [options] [length]

    Options:
        -u          Include _U_ppercase characters.
        -l          Include _L_owercase characters.
        -n          Include _N_umeric characters.
        -s          Include _S_pecial characters.
        -b          Only use basic special characters.
                    Implies -s
        -h, --help  Display help-text

    Notes:

        Default is `-luns 32`.
        `-luns 1` implies a password with length 4.

    Examples:

        pw
        pw -ul 18
        pw -s 25
        pw -ul
        pw -buln

    MIT (c) Luca Schnellmann, 2025
"
    );

    process::exit(0);
}

pub fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut lower = false;
    let mut upper = false;
    let mut number = false;
    let mut special: SpecialType = SpecialType::None;

    let mut length: u16 = 32;

    for argument in args {
        match argument.as_str() {
            "--help" => print_help(),
            flags if flags.starts_with('-') => {
                for flag in argument.chars().skip(1) {
                    match flag {
                        'l' => lower = true,
                        'u' => upper = true,
                        'n' => number = true,
                        's' => {
                            if special == SpecialType::None {
                                special = SpecialType::Full;
                            }
                        }
                        'b' => {
                            special = SpecialType::Basic;
                        }
                        'h' => print_help(),
                        _ => return Err(format!("Invalid flag -{flag}")),
                    }
                }
            }
            length_str => {
                length = length_str.parse::<u16>().map_err(|_| {
                    return format!("Invalid length {argument} (max {})", u16::MAX);
                })?;
            }
        }
    }

    // No charset is enabled, so -ln would only enable lower and number
    // If none are requested though, default to enabling all
    if special == SpecialType::None && !lower && !upper && !number {
        (special, lower, upper, number) = (SpecialType::Full, true, true, true);
    }

    Ok(Config {
        special,
        number,
        lower,
        upper,
        length,
    })
}

#[cfg(test)]
mod test_parse_args {
    use std::iter;

    use super::*;

    #[test]
    fn empty() {
        let config = parse_args(iter::empty::<String>()).unwrap();
        assert_eq!(
            config,
            Config {
                length: 32,
                lower: true,
                upper: true,
                number: true,
                special: SpecialType::Full,
            }
        )
    }

    #[test]
    fn override_length() {
        let config = parse_args(vec![String::from("30")].into_iter()).unwrap();
        assert_eq!(
            config,
            Config {
                length: 30,
                lower: true,
                upper: true,
                number: true,
                special: SpecialType::Full,
            }
        )
    }

    #[test]
    fn charsets_split_flags() {
        let config = parse_args(vec![String::from("-l"), String::from("-u")].into_iter()).unwrap();

        assert_eq!(
            config,
            Config {
                length: 32,
                lower: true,
                upper: true,
                number: false,
                special: SpecialType::None,
            }
        )
    }

    #[test]
    fn charsets_joined_flags() {
        let config = parse_args(vec![String::from("-ln")].into_iter()).unwrap();
        assert_eq!(
            config,
            Config {
                length: 32,
                lower: true,
                upper: false,
                number: true,
                special: SpecialType::None,
            }
        )
    }

    #[test]
    fn charsets_and_length() {
        let config = parse_args(
            vec![String::from("-lu"), String::from("44"), String::from("-s")].into_iter(),
        )
        .unwrap();
        assert_eq!(
            config,
            Config {
                length: 44,
                lower: true,
                upper: true,
                number: false,
                special: SpecialType::Full,
            }
        )
    }

    #[test]
    fn special_basic() {
        let config = parse_args(vec![String::from("-bu")].into_iter()).unwrap();
        assert_eq!(
            config,
            Config {
                length: 32,
                upper: true,
                lower: false,
                number: false,
                special: SpecialType::Basic,
            }
        )
    }
}
