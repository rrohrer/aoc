fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    println!("part 1: {}", result);
    let result = part_2(input);
    println!("part 2: {}", result);
}

fn part_1(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|s| {
            let batteries = s
                .chars()
                .map(|c| c.to_digit(10).expect("non-digit") as u64)
                .collect::<Vec<u64>>();
            let first_slice = &batteries[..batteries.len() - 1];
            let max = first_slice.iter().max().unwrap();
            let first_index = first_slice.iter().position(|i| i == max).unwrap();
            let second_max = batteries[first_index + 1..].iter().max().unwrap();
            max * 10 + second_max
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|s| {
            let batteries = s
                .chars()
                .map(|c| c.to_digit(10).expect("non-digit") as u64)
                .collect::<Vec<u64>>();
            let mut remaining_digits = 12;
            let mut current_start = 0;
            let mut digits = Vec::new();
            while remaining_digits != 0 {
                let (digit, new_start) = get_next_digit(
                    &batteries[current_start..batteries.len() - (remaining_digits - 1)],
                );
                digits.push(digit);
                current_start += new_start + 1;
                remaining_digits -= 1;
            }
            digits
                .iter()
                .rev()
                .fold((0, 1), |(val, scale), i| (val + i * scale, scale * 10))
                .0
        })
        .sum()
}

fn get_next_digit(input: &[u64]) -> (u64, usize) {
    let max = input.iter().max().unwrap();
    let position = input.iter().position(|i| max == i).unwrap();
    (*max, position)
}
