use std::{env, process};

use pw::{args, generate};

fn main() {
    let cli_args = env::args().skip(1);
    let config = match args::parse_args(cli_args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    let password = generate::generate_password(&config);

    match password {
        Ok(password) => println!("{password}"),
        Err(error_message) => {
            eprintln!("{error_message}");
            process::exit(1);
        }
    }
}
