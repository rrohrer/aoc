use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_digit(s: &str) -> char {
    match s {
        "1" => '1',
        "one" => '1',
        "2" => '2',
        "two" => '2',
        "3" => '3',
        "three" => '3',
        "4" => '4',
        "four" => '4',
        "5" => '5',
        "five" => '5',
        "6" => '6',
        "six" => '6',
        "7" => '7',
        "seven" => '7',
        "8" => '8',
        "eight" => '8',
        "9" => '9',
        "nine" => '9',
        _ => panic!("Invalid!"),
    }
}

fn calibration(line: String) -> i32 {
    let p = [
        "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven",
        "8", "eight", "9", "nine",
    ];
    let mut matches = Vec::new();
    for pattern in p.iter() {
        matches.extend(line.match_indices(pattern));
    }

    matches.sort_by(|a, b| {
        if a.0 < b.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let first = matches.first().unwrap();
    let last = matches.last().unwrap();
    let s = format!("{}{}", get_digit(first.1), get_digit(last.1));
    s.parse::<i32>().unwrap()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        sum += calibration(line.unwrap());
    }

    println!("sum: {}", sum);

    Ok(())
}
