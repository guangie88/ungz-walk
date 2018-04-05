# `unwalk`

[![Build Status](https://travis-ci.org/guangie88/unwalk.svg?branch=master)](https://travis-ci.org/guangie88/unwalk)
[![codecov](https://codecov.io/gh/guangie88/unwalk/branch/master/graph/badge.svg)](https://codecov.io/gh/guangie88/unwalk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Simple program to walk directory recursively and find `.gz` files to perform
un-gzip process, which might be useful when copying files over from S3 / HDFS
with compression turned on.

This program may be possible to perform other modular actions in the future.

## How to install

Use [https://www.rustup.rs/](`rustup`) and get the commands `cargo` and
`rustc`.

Run `cargo install unwalk` for the installation.

## How to run

Assuming there are `.gz` files located within `data/`:

```bash
unwalk data/
```

The above command will walk all sub-directories recursively from `data/`,
un-gzipping all found `.gz` files, but not deleting any of the original `.gz`
files after successful un-gzipping.

To delete the files, add `-d` as such:

```bash
unwalk -d data/
```

For more information on the available CLI arguments, run:

```bash
unwalk -h
```

## How to build

Run `cargo build --all --release`. This builds all the associated libraries
and the executable. The executable will be built in `target/release/unwalk`.
