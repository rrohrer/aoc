fn main() {
    cc::Build::new()
        .file("../part2_fast.c")
        .flag("-O3")
        .compile("part2");
}
