use std::{env, process, u16};

pub struct Config {
    pub lower: bool,
    pub upper: bool,
    pub special: bool,
    pub number: bool,
    pub length: u16,
}

fn print_help() {
    println!(
        "
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
        pw -u -l 18
        pw -s 25
        pw -ul
"
    );

    process::exit(0);
}

pub fn parse_args() -> Result<Config, String> {
    let mut lower = false;
    let mut upper = false;
    let mut number = false;
    let mut special = false;

    let mut length: u16 = 32;

    let args = env::args();

    for argument in args.skip(1) {
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
