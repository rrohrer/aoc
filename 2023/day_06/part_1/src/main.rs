fn main() {
    let races = vec![(42, 308), (89, 1170), (91, 1291), (89, 1467)];
    let result: usize = races
        .iter()
        .map(|r| {
            (1..=r.0)
                .filter_map(|x| if x * (r.0 - x) > r.1 { Some(x) } else { None })
                .count()
        })
        .product();
    println!("result: {}", result);
}
