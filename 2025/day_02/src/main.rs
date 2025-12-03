fn main() {
    let input = include_str!("../input.txt");
    let sum = run_with_validator(input, &is_valid);
    println!("part1: {}", sum);
    let sum = run_with_validator(input, &is_valid2);
    println!("part2: {}", sum);
}

fn run_with_validator(input: &str, valid: &dyn Fn(u64) -> bool) -> u64 {
    let ranges: Vec<(u64, u64)> = input
        .split(",")
        .map(|s| {
            let range: Vec<u64> = s
                .split("-")
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();
            (range[0], range[1])
        })
        .collect();

    ranges
        .iter()
        .map(|(start, end)| {
            (*start..=*end)
                .map(|i| if !valid(i) { Some(i) } else { None })
                .flatten()
                .into_iter()
        })
        .flatten()
        .sum::<u64>()
}

fn is_valid(input: u64) -> bool {
    let s = input.to_string();
    if s.len() % 2 == 1 {
        return true;
    }

    !s.chars()
        .zip(s[s.len() / 2..].chars())
        .fold(true, |result, (a, b)| if result { a == b } else { result })
}

fn is_valid2(input: u64) -> bool {
    let s = input.to_string();
    let mid = s.len() / 2;

    !(1..=mid).rev().any(|i| {
        if s.len() % i != 0 {
            false
        } else {
            let v = s.chars().collect::<Vec<char>>();
            let mut c = v.chunks(i);
            let b = c.next().unwrap();
            c.all(|c| b == c)
        }
    })
}
