use crate::file_util::read_file_to_string;
use lazy_static::lazy_static;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r"bags|bag").unwrap();
    static ref RULE_REGEX: Regex = Regex::new(r"(\D+) (\d+) (\D+)").unwrap();
}

fn get_bag_rule(input: &String) -> (String, HashMap<String, usize>) {
    let (mut parent_bag, mut child_bag) = (String::new(), HashMap::new());

    for (i, mut part)  in BAG_REGEX.split(input).enumerate() {
        part =  part.trim();
        if i == 0 {
            parent_bag = part.to_string();
            continue;
        }
        if part == "contain no other" || part == "."{
            break;
        }
        let results = RULE_REGEX.captures(part).unwrap();
        let bag_num = results.get(2).unwrap().as_str().parse().unwrap();
        let bag_color = results.get(3).unwrap().as_str().to_string();
        child_bag.insert(bag_color, bag_num);
    }

    return (parent_bag, child_bag);
}

fn find_valid_path(bag_rules: &HashMap<String, HashMap<String, usize>>, rule: &HashMap<String, usize>, to_find: &String) -> bool {
    if rule.is_empty() {
        return false;
    }
    if rule.contains_key(to_find) {
        return true;
    }

    for key in rule.keys() {
        let r = &bag_rules.get(key).unwrap();
        if find_valid_path(&bag_rules, r, to_find) {
            return true;
        }
    }
    return false;
}

fn get_bag_total(bag_rules: &HashMap<String, HashMap<String, usize>>, to_find: &String) -> usize {
    let rule = bag_rules.get(to_find).unwrap();
    let mut total = 0;

    if rule.is_empty() {
        return total;
    }
    for (k, v) in rule {
        let k_total = get_bag_total(&bag_rules, k);
        total += v + v * k_total;
    }

    return total;
}

pub fn solution() -> usize {
    let input = read_file_to_string("input/day07");

    let mut bag_rules: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for line in input {
        let result =  get_bag_rule(&line);
        bag_rules.insert(result.0, result.1);
    }

    // part 1 - 222
//    let mut count = 0;
//    for (_color, bag_rule) in &bag_rules {
//        if find_valid_path(&bag_rules, &bag_rule, &"shiny gold".to_string()) {
//            count += 1;
//        }
//    }
//    return count;

    //part 2 - 13264
    return get_bag_total(&bag_rules, &"shiny gold".to_string());
}