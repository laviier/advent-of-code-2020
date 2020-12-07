use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_to_usize(filename: &str) -> Vec<usize> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        //println!("{}", line);
        vec.push(line.parse::<usize>().unwrap());
    }

    return vec;
}


pub fn read_file_to_string(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        vec.push(line.unwrap());
    }

    return vec;
}

pub fn read_file_to_string_by_new_line(filename: &str, separator: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let (mut vec, mut doc) = (Vec::new(), String::new());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();

        if line.trim().is_empty() && !doc.trim().is_empty() {
            vec.push(doc);
            doc = String::new();
        } else if !line.trim().is_empty() {
            doc += &(separator.to_string().to_owned() + &line);
        }
    }

    if !doc.trim().is_empty() {
        vec.push(doc.trim().to_string());
    }

    return vec;
}