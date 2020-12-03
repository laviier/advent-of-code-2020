use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_to_vec() -> Vec<i32> {
    let filename = "input/day01";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        //println!("{}", line);
        vec.push(line.parse::<i32>().unwrap());
    }

    return vec;
}
