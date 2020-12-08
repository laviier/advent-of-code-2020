extern crate lazy_static;
extern crate regex;

mod day08;
mod file_util;

use day08::solution;

fn main() {
    println!("{:?}", solution());
}
