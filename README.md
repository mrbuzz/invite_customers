# Technical Choices 
- Command line app
- Language of choice is Rust
  as it is particularly suited for building command line apps and it is usually my language of choice for such projects. 

# Assumptions
- Earth radius in kilometers: `6371.0`
- floats are rounded with the `ceiling` function
# Installation
You just need to install the Rust development environment. It is cross-paltform so it can be installed pretty much everywere:
- If you’re running macOS, Linux, or another Unix-like OS you can just run the following:
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- If you are using Windows see [other installation methods](https://forge.rust-lang.org/infra/other-installation-methods.html)

More here: https://www.rust-lang.org/learn/get-started
# Building
- `cargo build` to build a non optimized version with debuginfo 
- `cargo build --release` to build a release version

# Running Tests 
- `cargo test`  runs the entire test suite 

# How to run
The app can be executed through `cargo` or manually calling the compiled executable from the `target` folder

- `cargo run -- /path/to/imput/file.txt`
- `target/release/invite_customers /path/to/imput/file.txt`

An optional output file can be specified,if nothing is specified it defaults to `stdout`
- `cargo run -- /path/to/imput/file.txt -o output.txt`
- `cargo run -- /path/to/imput/file.txt --output output.txt`

An optional km range can be specified, default is 100 Km
- `cargo run -- /path/to/imput/file.txt -r 25`
- `cargo run -- /path/to/imput/file.txt --range 25`
```bash

> cargo run --help
invite_customers 0.1.0
Giorgio G.
Computes a list of customers within a given distance from the Dublin office

USAGE:
    invite_customers [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>    Sets an optional output file, if not specified defaults to stdout.
    -r, --range <KM>       Sets the exclusion range in kilometers.

ARGS:
    <INPUT>    Sets the input file to use
```

