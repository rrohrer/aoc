use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

struct Hand {
    red: i32,
    green: i32,
    blue: i32,
}

impl Hand {
    fn new() -> Self {
        Hand {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn from_counts(red: i32, green: i32, blue: i32) -> Self {
        Hand { red, green, blue }
    }

    fn from_game(s: String) -> Vec<Self> {
        // chop off "Game X:" header
        let game = &s[s.find(':').unwrap() + 1..];
        let hands = game.split(';');
        let mut result = Vec::new();

        for hand in hands {
            let items = hand.split(',');
            let mut h = Hand::new();
            for item in items {
                let parts: Vec<_> = item.trim().split(' ').collect();
                let value = parts[0].trim().parse::<i32>().unwrap();
                match parts[1].trim() {
                    "red" => h.red = value,
                    "green" => h.green = value,
                    "blue" => h.blue = value,
                    _ => (),
                }
            }

            result.push(h);
        }

        result
    }

    fn is_possible(&self, hands: &Vec<Hand>) -> bool {
        for h in hands {
            if h.red > self.red {
                return false;
            }
            if h.green > self.green {
                return false;
            }
            if h.blue > self.blue {
                return false;
            }
        }

        return true;
    }

    fn find_power(hands: &Vec<Hand>) -> i32 {
        let mut minimum = Hand::new();
        for h in hands {
            if h.red > minimum.red {
                minimum.red = h.red;
            }
            if h.green > minimum.green {
                minimum.green = h.green;
            }
            if h.blue > minimum.blue {
                minimum.blue = h.blue;
            }
        }

        return minimum.red * minimum.blue * minimum.green;
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let base_hand = Hand::from_counts(12, 13, 14);

    let mut counter = 1;
    let mut sum = 0;
    let mut sum_power = 0;
    for line in reader.lines() {
        let hands = Hand::from_game(line.unwrap());
        if base_hand.is_possible(&hands) {
            sum += counter;
        }
        counter += 1;

        sum_power += Hand::find_power(&hands);
    }

    println!("{}", sum);
    println!("{}", sum_power);

    Ok(())
}
