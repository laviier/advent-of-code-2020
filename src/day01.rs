use crate::file_util::read_file_to_usize;

fn two_sum(list: Vec<usize>) -> usize {
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[i] + list[j] == 2020 {
                return list[i] * list[j];
            }
        }
    }
    return 0;
}

fn three_sum(list: Vec<usize>) -> usize {
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

pub fn solution() -> usize {
    let vec = read_file_to_usize("input/day01");
    return three_sum(vec);
}