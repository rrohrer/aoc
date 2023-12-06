fn main() {
    let races = vec![(42899189usize, 308117012911467usize)];
    let result: usize = races
        .iter()
        .map(|r| {
            (1..=r.0)
                .filter_map(|x| if x * (r.0 - x) > r.1 { Some(x) } else { None })
                .count()
        })
        .last()
        .unwrap();
    println!("result: {}", result);
}
