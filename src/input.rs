// == getting input from the logcart
use std::io::{self, BufRead};


pub fn read_lines() -> impl Iterator<Item = String> {
    let stdin = io::stdin();
    stdin.lock().lines().filter_map(Result::ok)
}
