extern "C" {
    pub fn part2_fast() -> i32;
}

fn main() {
    let a = unsafe { part2_fast() };
    println!("{a}");
}
