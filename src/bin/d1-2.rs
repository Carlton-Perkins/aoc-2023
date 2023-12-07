use std::{
    collections::{hash_map::RandomState, HashMap},
    fs::read_to_string,
    path::Path,
};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let file = read_to_string(Path::new("data/d1-1.input")).unwrap();
    let digit_names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits: HashMap<String, i32, RandomState> = HashMap::from_iter(
        digit_names
            .iter()
            .enumerate()
            .map(|(i, s)| (s.to_string(), (i + 1) as i32)),
    );

    let regex = Regex::new(&format!("{}|[1-9]", digit_names.join("|"))).unwrap();
    let rev_regex = Regex::new(&format!(
        "{}|[1-9]",
        digit_names
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .join("|")
    ))
    .unwrap();

    let mut agg = 0;
    for line in file.lines() {
        let matches = regex.find_iter(&line).map(|m| m.as_str()).collect_vec();
        let rev_line = line.chars().rev().collect::<String>();
        let rev_matches = rev_regex
            .find_iter(&rev_line)
            .map(|m| m.as_str())
            .collect_vec();

        let parse = |s| match s {
            s if digit_names.contains(s) => digits[&String::from(*s)],
            s if s.parse::<i32>().is_ok() => s.parse::<i32>().unwrap(),
            _ => panic!("Invalid input"),
        };

        let first = parse(matches.first().unwrap());
        let last = parse(
            &rev_matches
                .first()
                .unwrap()
                .chars()
                .rev()
                .collect::<String>()
                .as_str(),
        );

        println!("{line} - {first}{last}");
        agg += format!("{}{}", first, last).parse::<i32>().unwrap()
    }

    println!("{}", agg);
}
