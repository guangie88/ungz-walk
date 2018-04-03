# `unwalk`

Simple program to walk directory recursively and find `.gz` files to perform
un-gzip process.

## How to Build

Use [https://www.rustup.rs/](`rustup`) and get the commands `cargo` and
`rustc`.

Run `cargo build --release` to get the program at `./target/release/unwalk`.

## How to Run

Assuming there are `.gz` files located within `./data`:

```bash
./target/release/unwalk -f ./data
```

The above command will walk all sub-directories recursively from `./data`,
un-gzipping all found `.gz` files and deleting the original `.gz` files after
successful un-gzipping.

Add flags `-x` to disable the deletion of original `.gz` files, and
`-l ./config/log_config.yml` to use `log4rs` for more complex logging. 

For more information, run:

```bash
./target/release/unwalk --help
```
