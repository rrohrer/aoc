fn calculate(v: &Vec<i32>) -> i32 {
    let mut table: Vec<Vec<i32>> = Vec::new();
    table.push(Vec::new());
    v.iter().for_each(|v| table[0].push(*v));

    let mut done = false;
    while !done {
        let mut row = Vec::new();
        let l = table.last().unwrap();
        l.iter()
            .zip(l.iter().skip(1))
            .for_each(|(a, b)| row.push(b - a));
        done = row.iter().sum::<i32>() == 0;
        table.push(row);
    }
    table.iter().rev().filter_map(|l| l.last()).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<Vec<_>> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split(" ").filter_map(|s| s.parse::<i32>().ok()).collect())
        .collect();

    let result: i32 = data.iter().map(|v| calculate(v)).sum();
    println!("result: {}", result);
}
