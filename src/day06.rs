use crate::file_util::read_file_to_string_by_new_line;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_question_count(input: &String) -> usize {
    let question_list = input.split_whitespace();

    let mut person_count = 0;

    let mut map = HashMap::new();
    for question in question_list {
        question
            .chars().for_each(|c| {
                //The insert value is 0 for new key because the right assignment would increase 0 by 1, so the final value inserted for a new key is 1.
                *map.entry(c).or_insert(0) += 1;
            });
        person_count += 1;
    }

    return map.values().filter(|v| *v == &person_count).count();
}

pub fn solution() -> usize {
    //part 1 - 6457
//    let input = read_file_to_string_by_new_line("input/day06" , "");
//    return input
//        .iter()
//        .map(|s| s.chars().collect::<HashSet<_>>().len())
//        .sum();

    //part 2 - 3260
    let input = read_file_to_string_by_new_line("input/day06" , " ");
    return input
        .iter()
        .map(|s| get_question_count(&s))
        .sum();
}