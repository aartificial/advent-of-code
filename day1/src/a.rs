fn main() {
    let solution = include_str!("../res/input.txt")
        .split("\n\n")
        .map(|x| x.lines()
            .map(|y| y.parse::<u32>().unwrap())
            .sum::<u32>())
        .max()
        .unwrap();
    println!("{}", solution)
}
