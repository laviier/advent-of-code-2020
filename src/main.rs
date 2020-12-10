extern crate lazy_static;
extern crate regex;

mod day09;
mod file_util;

use day09::solution;

fn main() {
    println!("{:?}", solution());
}
