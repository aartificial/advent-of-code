fn main() {
    let mut inventories = include_str!("../res/input.txt")
        .split("\n\n")
        .map(|x| x.lines()
            .map(|y| y.parse::<u32>().unwrap())
            .sum::<u32>())
        .collect::<Vec<u32>>();

    inventories.sort_unstable_by(|a, b| b.cmp(a));

    let solution = inventories
        .into_iter()
        .take(3)
        .sum::<u32>();

    println!("{}", solution)
}