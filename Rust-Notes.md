# Rust Notes

## Installation on Linux
The primary way that folks install Rust is through a tool called Rustup, which is a Rust installer and version management tool.
To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.

> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Update Rust
Rust updates very frequently. If you have installed Rustup some time ago, chances are your Rust version is out of date. Get the latest version of Rust by running:

> rustup update

## Cargo
Build tool and package manager. Cargo does lots of things:

> - cargo build   : build project
> - cargo run     : run project
> - cargo test    : test project
> - cargo doc     : build documentation
> - cargo publish : publish a library to crates.io (https://crates.io/)

To test installation:

> cargo --version
cargo 1.83.0 (5ffbef321 2024-10-29)

## Ignore files
.gitignore for executable crates
```
   # Generated files
   /target/
```

.gitignore for library crates
```
   # Generated files
   /target/

   # Well, this depends... See below!
   Cargo.lock
```

Reference:
https://stackoverflow.com/questions/43667176/what-files-in-a-cargo-project-should-be-in-my-gitignore
