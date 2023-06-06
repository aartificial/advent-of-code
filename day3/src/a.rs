fn priority(c: &u8) -> i16 {
    if *c > b'a' {
        (*c - b'a') as i16 + 1
    } else {
        (*c - b'A') as i16 + 27
    }
}

fn main() {
    let solution = include_bytes!("../res/input.txt")
        .split(|x| *x == b'\n')
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a,b)| a
                .iter()
                .filter(|a| b.contains(a))
                .map(|b| priority(b))
                .next()
                .unwrap())
        .sum::<i16>();

    println!("{}", solution);
}