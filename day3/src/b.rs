fn priority(c: &u8) -> i16 {
    if *c > b'a' {
        (*c - b'a') as i16 + 1
    } else {
        (*c - b'A') as i16 + 27
    }
}

fn main() {
    let solution = include_bytes!("../res/input.txt")
        .split(|c| *c == b'\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|rucksacks|
            rucksacks[0]
                .iter()
                .find(|c| rucksacks[1].contains(c) && rucksacks[2].contains(c))
                .unwrap())
        .map(|c| priority(c))
        .sum::<i16>();

    println!("{}", solution)
}