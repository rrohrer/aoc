use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone)]
struct Mapping {
    dest: usize,
    src: usize,
    len: usize,
}

impl Mapping {
    fn new(dest: usize, src: usize, len: usize) -> Self {
        Self { dest, src, len }
    }
}

struct InputMap {
    seeds: Vec<usize>,
    data: Vec<Vec<Mapping>>,
}

impl InputMap {
    fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        let s = lines.next().unwrap().unwrap();
        let seeds = s[s.find(":").unwrap() + 1..]
            .trim()
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let mut builder = Vec::new();
        let mut data = Vec::new();
        _ = lines.next();
        _ = lines.next();
        for line in lines {
            let l = line.unwrap();
            if l.len() == 0 {
                data.push(builder.clone());
                builder.clear();
                continue;
            } else if l.contains(":") {
                continue;
            }

            let m = l
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            builder.push(Mapping::new(m[0], m[1], m[2]));
        }

        data.push(builder);

        data.iter_mut().for_each(|m| {
            m.sort_by(|a, b| {
                if a.src > b.src {
                    Ordering::Greater
                } else if a.src < b.src {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
        });

        Self { seeds, data }
    }

    fn map_seed(&self, seed: usize) -> usize {
        let mut current = seed;
        for m in &self.data {
            for r in m {
                if r.src <= current && r.src + r.len > current {
                    let delta = current - r.src;
                    current = r.dest + delta;
                    break;
                } else {
                    continue;
                }
            }
        }
        current
    }
}

fn main() {
    let mapping = InputMap::new("input.txt");

    let mut min = usize::MAX;

    for i in (0..mapping.seeds.len()).step_by(2) {
        let start = mapping.seeds[i];
        let len = mapping.seeds[i + 1];

        for j in start..start + len {
            min = usize::min(mapping.map_seed(j), min);
        }
    }

    println!("{}", min);
}
