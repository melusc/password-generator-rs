/*!
  This file is part of password-generator-rs, a tool to quickly
  generate passwords following password requirements.
  Copyright (C) 2026, Luca Schnellmann <oss@lusc.ch>

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
        Ok(password) => print!("{password}"),
        Err(error_message) => {
            eprintln!("{error_message}");
            process::exit(1);
        }
    }
}
