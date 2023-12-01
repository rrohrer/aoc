use aho_corasick::{AhoCorasick, MatchKind};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_calibration(line: String) -> i32 {
    let first = *line
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>()
        .first()
        .unwrap();
    let second = *line
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>()
        .last()
        .unwrap();
    let s = format!("{}{}", first, second);
    s.parse::<i32>().unwrap()
}

fn cleanup_string(line: String, ac: &AhoCorasick) -> String {
    let r = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    ac.replace_all(&line, r)
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let p = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(p)
        .unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        sum += get_calibration(cleanup_string(line.unwrap(), &ac));
    }

    println!("sum: {}", sum);

    Ok(())
}
