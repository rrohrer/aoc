use std::collections::HashMap;

fn main() {
    let raw_input = include_str!("../input.txt");
    let mut input = raw_input.split("\n\n");
    let route = input.next().unwrap();
    let graph: HashMap<_, _> = input
        .next()
        .unwrap()
        .split("\n")
        .filter_map(|s| {
            if s.len() != 0 {
                Some((&s[0..3], (&s[7..10], &s[12..15])))
            } else {
                None
            }
        })
        .collect();

    let mut result = 0;
    let mut current = "AAA";

    let mut r = route.chars().cycle();

    while current != "ZZZ" {
        let c = r.next().unwrap();
        let next = graph.get(current).unwrap();
        current = match c {
            'L' => next.0,
            'R' => next.1,
            _ => panic!("uh oh"),
        };
        result += 1;
    }

    println!("count: {}", result);
}
