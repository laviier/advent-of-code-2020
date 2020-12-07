extern crate lazy_static;
extern crate regex;

mod day06;
mod file_util;

use day06::solution;

fn main() {
    println!("{:?}", solution());
}
