# password-generator-rs

This is my first rust project, the code might be weird.

## Usage

```bash
pw --help
pw -luns 32 # default options

pw -l # only lowercase letters
pw -u # only uppercase letters
pw -s # only symbols
pw -n # only numbers
pw 20 # 20 character password
```

## Installation

Required software: Git, Rust & Cargo

```bash
git clone https://github.com/melusc/password-generator-rs.git
cd password-generator-rs
cargo build --release
```

Adding the binary to PATH is left as an exercise to the reader.
The binary will be available as `pw`.

## License

This project is licensed under the MIT license. See `./license`.
