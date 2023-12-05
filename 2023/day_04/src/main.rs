use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    values: Vec<i32>,
    winners: HashSet<i32>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(card: &str) -> Result<Self, Self::Err> {
        let start_index = card.find(":").ok_or("could not find :")?;
        let mut card_parts = card[start_index + 1..].trim().split("|");
        let winners: HashSet<i32> = card_parts
            .next()
            .ok_or("Passwords not found")?
            .trim()
            .split(" ")
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        let values: Vec<i32> = card_parts
            .next()
            .ok_or("values not found")?
            .trim()
            .split(" ")
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        Ok(Self { values, winners })
    }
}

impl Card {
    fn score(&self) -> i32 {
        self.values.iter().fold(0, |a, x| {
            if self.winners.contains(x) {
                if a == 0 {
                    1
                } else {
                    a * 2
                }
            } else {
                a
            }
        })
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let points = reader
        .lines()
        .filter_map(|l| Card::from_str(&l.unwrap()).ok())
        .fold(0, |a, c| a + c.score());

    println!("points: {}", points);

    Ok(())
}
