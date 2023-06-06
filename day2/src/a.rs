fn main() {
    let data = include_bytes!("../res/input.txt")
        .split(|c| *c == b'\n')
        .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16))
        .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
        .sum::<i16>();
    println!("{}", data);
}



