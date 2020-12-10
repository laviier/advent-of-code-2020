use crate::file_util::read_file_to_usize;

fn is_valid(previous_numbers: &[usize], target: usize) -> bool {
    for i in  0..previous_numbers.len() {
        for j in i..previous_numbers.len() {
            if previous_numbers[i] + previous_numbers[j] == target {
                return true;
            }
        }
    }
    return false;
}

fn find_invalid_number(input: &Vec<usize>) -> usize {
    for index in 25..input.len() {
        let target = input[index];
        let previous_numbers = &input[index-25..index];
        if !is_valid(&previous_numbers, target) {
            return target;
        }
    }
    panic!("can't find the invalid number.")
}

fn find_sum(mut input: Vec<usize>, target: usize) -> usize {
    for i in 0..input.len() {
        let mut sum = input[i];
        for j in i+1..input.len() {
            sum += input[j];
            if sum == target {
                let subset = &mut input[i..j+1];
                subset.sort();
                return subset[0] + subset[subset.len() - 1];
            } else if sum < target {
                continue;
            } else {
                break;
            }
        }
    }
    panic!("not found");
}

pub fn solution() -> usize {
    let input = read_file_to_usize("input/day09");
    //part 1 - 144381670
    //return find_invalid_number(&input);

    //part 2 - 20532569
    return find_sum(input, 144381670);
}