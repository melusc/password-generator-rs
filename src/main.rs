use std::process;

use pw::args::parse_args;
use pw::generate::generate_password;

fn main() {
    let config = match parse_args() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    let password = generate_password(config);
    println!("{password}");
}
