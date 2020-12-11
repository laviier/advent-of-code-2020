use crate::file_util::read_file_to_usize;
use std::collections::HashMap;

fn find_differences_count(input: &Vec<usize>) -> usize {
    let mut differences_map: HashMap<usize, usize> = HashMap::new();

    let mut key = input[0];
    for i in 0..input.len() {
        if i != 0 {
            key = input[i] - input[i - 1];
        }
        *differences_map.entry(key).or_insert(0) += 1;
    }

    return differences_map.get(&1).unwrap() * differences_map.get(&3).unwrap();
}

pub fn solution() -> usize {
    let mut input = read_file_to_usize("input/day10");
    input.sort();
    let built_in_adapter = input[input.len() - 1] + 3;
    input.push(built_in_adapter);
    //part 1 - 2414
    return find_differences_count(&input);
}