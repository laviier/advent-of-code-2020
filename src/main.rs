extern crate lazy_static;
extern crate regex;

mod day01;
mod file_util;

use day01::solution;

fn main() {
    println!("{:?}", solution());
}
