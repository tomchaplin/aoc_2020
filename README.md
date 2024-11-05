_#rr AOC 2023 - Rust

## Downloading Data

1. Put your AOC session token in a file called `.env` in the formac
```bash
AOC_SESSION="<your_session_cookie>"
```
2. Download the data for a given day with `./download_day.sh <day>`

## Running solutions

To run a solution simply run
```bash
cargo run <day>
```
In order to run, for example, just part `a` of day 4 run
```bash
cargo run 4a
```
To run all solutions for all days, run
```bash
cargo run
```
This will run until an error is reached (e.g. no input data)


## Credits

The system for running solutions is inspired by [this](https://www.reddit.com/r/adventofcode/comments/e5sa2d/comment/f9ltko3/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) comment from Reddit user `u/thaddeus_v`.
