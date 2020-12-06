use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

static ROW_RANGE: RangeInclusive<usize> = 0..=127;
static COL_RANGE: RangeInclusive<usize> = 0..=7;

fn read_file_to_vec() -> Vec<String> {
    let filename = "input/day05";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        vec.push(line.unwrap());
    }

    return vec;
}

fn calculate_seat_id(seat_input: &String) -> usize{
    let row_input = seat_input[..7].to_string();
    let col_input = seat_input[7..10].to_string();

    let row_id = get_row_id(&row_input);
    let col_id = get_col_id(&col_input);

    return row_id * 8 + col_id;
}

fn get_row_id(row_input: &String) -> usize{

    let mut row_range = ROW_RANGE.clone();

    let mut i = 0;

    while i < row_input.len() {
        let flag = row_input.chars().nth(i).unwrap();
        row_range = get_new_range(row_range, flag);
        i += 1;
    }

    return *row_range.end();
}

fn get_col_id(col_input: &String) -> usize{

    let mut col_range = COL_RANGE.clone();

    let mut i = 0;

    while i < col_input.len() {
        let flag = col_input.chars().nth(i).unwrap();
        col_range = get_new_range(col_range, flag);
        i += 1;
    }

    return *col_range.end();
}

fn get_new_range(cur_range: RangeInclusive<usize>, flag: char) -> RangeInclusive<usize> {
    let (lower_bound, upper_bound) = cur_range.clone().into_inner();
    let count = (upper_bound - lower_bound + 1) / 2;

    if flag == 'F' || flag == 'L' {
        return lower_bound..=(lower_bound + count - 1);
    } else {
        return (lower_bound + count)..=upper_bound;
    }
}

fn find_missing_seat(num_list: Vec<usize>) -> usize {
    let (mut i, mut seat) = (0, num_list[0]);

    while i < num_list.len() {

        if num_list[i] != seat {
            return seat;
        }

        i += 1;
        seat += 1;
    }

    panic!("No valid seat found.");
}

pub fn solution() -> usize {
    let input = read_file_to_vec();

    //part 1 - 959
    //let max = input.iter().map(|s| calculate_seat_id(&s)).max();
    //return max.unwrap();

    //part 1 - 527
    let mut seat_id_list: Vec<usize> = input.iter().map(|s| calculate_seat_id(&s)).collect();
    seat_id_list.sort();
    return find_missing_seat(seat_id_list);
}