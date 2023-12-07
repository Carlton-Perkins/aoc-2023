use std::{fs::read_to_string, path::Path};

fn main() {
    let file = read_to_string(Path::new("data/d1-1.input")).unwrap();

    let mut agg = 0;
    for line in file.lines() {
        let first = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

        agg += format!("{}{}", first, last).parse::<i32>().unwrap()
    }

    println!("{}", agg);
}
