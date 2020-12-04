use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file_to_vec() -> Vec<String> {
    let filename = "input/day03";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        vec.push(line.unwrap());
    }

    return vec;
}

fn get_tree_count(tree_input: &Vec<String>, x_offset: &usize, y_offset: &usize) -> usize {
    let (mut counter, mut x_index, mut y_index) = (0, 0, 0);

    while y_index < tree_input.len() {

        let mut path_position = (x_index * x_offset + 1) % 31;
        if path_position == 0 {
            path_position = 31;
        }

        let path_value = tree_input[y_index].chars().nth(path_position - 1).unwrap();

        if path_value == '#' {
            counter += 1;
        }

        x_index += 1;
        y_index += y_offset;
    }
    return counter;
}

pub fn solution() -> usize {
    let tree_input = read_file_to_vec();

    // part 1 - 294
    // return get_tree_count(&tree_input, &3, &1);

    // part 2 - 5774564250
    return [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |result, (x, y)| result * get_tree_count(&tree_input, x, y));
}