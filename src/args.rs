use std::{process, u16};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub lower: bool,
    pub upper: bool,
    pub special: bool,
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
        -h, --help  Display help-text

    Notes:

        Default is `-luns 32`.
        `-luns 1` implies a password with length 4.

    Examples:

        pw
        pw -ul 18
        pw -s 25
        pw -ul

    MIT (c) Luca Schnellmann, 2025
"
    );

    process::exit(0);
}

pub fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut lower = false;
    let mut upper = false;
    let mut number = false;
    let mut special = false;

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
                        's' => special = true,
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
    if !special && !lower && !upper && !number {
        (special, lower, upper, number) = (true, true, true, true);
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
                special: true,
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
                special: true,
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
                special: false,
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
                special: false,
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
                special: true,
            }
        )
    }
}
