use std::process;

use pw::{args, generate};

fn main() {
    let config = match args::parse_args() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    let password = generate::generate_password(config);
    println!("{password}");
}
