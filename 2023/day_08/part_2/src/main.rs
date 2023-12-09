use num::integer;
use std::collections::HashMap;

fn calculate_route(graph: &HashMap<&str, (&str, &str)>, start: &str, route: &str) -> u64 {
    let mut result = 0;
    let mut current = start;

    let mut r = route.chars().cycle();

    while !current.ends_with("Z") {
        let c = r.next().unwrap();
        let next = graph.get(current).unwrap();
        current = match c {
            'L' => next.0,
            'R' => next.1,
            _ => panic!("uh oh"),
        };
        result += 1;
    }
    result
}

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

    let result = graph
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| calculate_route(&graph, s, route))
        .fold(1, |a, x| integer::lcm(a, x));

    println!("count: {}", result);
}
