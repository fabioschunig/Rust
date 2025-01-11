# Rust - Dependencies


## Description
Simple project to test dependencies in Rust


## Adding dependencies

In our Cargo.toml file we’ll add this information (that we got from the crate page):

> [dependencies]
> ferris-says = "0.3.1"

We can also do this by running:

>  cargo add ferris-says

Now we can run:

> cargo build

...and Cargo will install our dependency for us.
