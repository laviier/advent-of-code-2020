use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file_to_vec() -> Vec<i32> {
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

fn two_sum(list: Vec<i32>) -> i32 {
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[i] + list[j] == 2020 {
                return list[i] * list[j];
            }
        }
    }
    return 0;
}

fn three_sum(list: Vec<i32>) -> i32 {
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            for k in j + 1..list.len() {
                if list[i] + list[j] + list[k] == 2020 {
                    return list[i] * list[j] * list[k];
                }
            }
        }
    }
    return 0;
}

pub fn solution() -> i32 {
    let vec = read_file_to_vec();
    return three_sum(vec);
}