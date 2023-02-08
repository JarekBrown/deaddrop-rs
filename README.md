# deaddrop-rs

A deaddrop utility written in Rust. Put files in a database behind a password to be retrieved at a later date.

This is a part of the University of Wyoming's Secure Software Design Course (Spring 2023). This is the base repository to be forked and updated for various assignments. Alternative language versions are available in:
- [Javascript](https://github.com/andey-robins/deaddrop-js)
- [Go](https://github.com/andey-robins/deaddrop-go)

## Versioning

`deaddrop-rs` is built with:
- cargo 1.66.0 (d65d197ad 2022-11-15)
- rust edition 2021

## Usage

`cargo run -- --help` for instructions

Then run `cargo run -- --new --user <username here>` and you will be prompted to create the initial password.

## Database

Data gets stored into the local database file dd.db. This file will not be synched to git repos. Delete this file if you don't set up a user properly on the first go

## Logging

Logging is done using [log4rs](https://crates.io/crates/log4rs). It is initialized using [this file](log4rs.yaml), and writes to a `logs.txt` file. The code that runs the logger can be found in [logging.rs](src/logging.rs).

### Categories

The following are possible categories you may see in your log file:

- INFO
  - used when logging generic information (nothing to really worry about)
    - Example: a new user was created
- WARN
  - used when there may be a problem you should check out
    - Example: if there are successive warn statements about failed authentication, a user may be trying to access someone else's messages
- ERROR
  - used when there was a problem encountered
    - Example: the specified user does not exist
- DEBUG
  - used when the `log_event` function is called incorrectly

**Note:** the log file is not synched to git repos.
