use std::u128;

enum Direction {
    Left,
    Right,
}

struct Command {
    direction: Direction,
    distance: u32,
}

impl Command {
    fn from_str(s: &str) -> Self {
        let direction = match s.chars().nth(0).expect("Bad Input") {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Bad Input"),
        };
        let distance = s[1..].parse::<u32>().expect("Bad Input");
        Self {
            direction,
            distance,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let commands: Vec<Command> = input
        .split_whitespace()
        .map(|s| Command::from_str(s))
        .collect();
    let result = count_zeros(50, &commands);
    let result2 = count_zeros_ext(50, &commands);
    println!("result: {} {}", result, result2);
    benchmark(50, &commands);
    benchmark_complete();
}

fn count_zeros(start: u32, commands: &Vec<Command>) -> u32 {
    let mut count = 0;
    _ = commands.iter().fold(start, |position, c| {
        let new_position = match c.direction {
            Direction::Left => {
                let truncated = c.distance % 100;
                if truncated > position {
                    let sub = truncated - position;
                    100 - sub
                } else {
                    position - truncated
                }
            }
            Direction::Right => (position + c.distance) % 100,
        };
        if new_position == 0 {
            count += 1
        }
        new_position
    });

    count
}

fn count_zeros_ext(start: u32, commands: &Vec<Command>) -> u32 {
    let mut count = 0;
    let mut wrapped = 0;
    _ = commands.iter().fold(start, |position, c| {
        wrapped += c.distance / 100;
        let truncated = c.distance % 100;
        let new_position = match c.direction {
            Direction::Left => {
                if truncated > position {
                    if position != 0 {
                        wrapped += 1;
                    }
                    let sub = truncated - position;
                    100 - sub
                } else {
                    position - truncated
                }
            }
            Direction::Right => {
                if truncated + position > 100 {
                    wrapped += 1;
                }
                (position + c.distance) % 100
            }
        };
        if new_position == 0 {
            count += 1
        }
        new_position
    });

    count + wrapped
}

fn benchmark(start: u32, commands: &Vec<Command>) {
    let tries = 100000;
    let mut min = u128::MAX;
    for _ in 0..tries {
        let time = std::time::Instant::now();
        std::hint::black_box(count_zeros(start, commands));
        let result = time.elapsed().as_micros();
        if result < min {
            min = result;
        }
    }
    println!("part 1 avg: {}us", min);

    let mut min = u128::MAX;
    for _ in 0..tries {
        let time = std::time::Instant::now();
        std::hint::black_box(count_zeros_ext(start, commands));
        let result = time.elapsed().as_micros();
        if result < min {
            min = result;
        }
    }
    println!("part 2 avg: {}us", min);
}

fn benchmark_complete() {
    let tries = 100000;
    let mut min = u128::MAX;
    for _ in 0..tries {
        let time = std::time::Instant::now();
        let input = include_str!("../input.txt");
        let commands: Vec<Command> = input
            .split_whitespace()
            .map(|s| Command::from_str(s))
            .collect();
        std::hint::black_box(count_zeros(50, &commands));
        let result = time.elapsed().as_micros();
        if result < min {
            min = result;
        }
    }
    println!("part 1 avg: {}us", min);

    let mut min = u128::MAX;
    for _ in 0..tries {
        let time = std::time::Instant::now();
        let input = include_str!("../input.txt");
        let commands: Vec<Command> = input
            .split_whitespace()
            .map(|s| Command::from_str(s))
            .collect();
        std::hint::black_box(count_zeros_ext(50, &commands));
        let result = time.elapsed().as_micros();
        if result < min {
            min = result;
        }
    }
    println!("part 2 avg: {}us", min);
}
