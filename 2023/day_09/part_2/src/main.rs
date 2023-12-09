fn calculate(v: &Vec<i64>) -> i64 {
    let mut table: Vec<Vec<i64>> = Vec::new();
    table.push(Vec::new());
    v.iter().for_each(|v| table[0].push(*v));

    let mut done = false;
    while !done {
        let mut row = Vec::new();
        let l = table.last().unwrap();
        l.iter()
            .zip(l.iter().skip(1))
            .for_each(|(a, b)| row.push(b - a));
        done = row.iter().all(|x| *x == 0);
        table.push(row);
    }
    table
        .iter()
        .rev()
        .map(|l| l.first().unwrap())
        .fold(0, |a, b| b - a)
}

fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<Vec<_>> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split(" ").filter_map(|s| s.parse::<i64>().ok()).collect())
        .collect();

    let result: i64 = data.iter().map(|v| calculate(v)).sum();
    println!("result: {}", result);
}
