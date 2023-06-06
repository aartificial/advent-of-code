
fn main() {
    let solution = include_str!("../res/input.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let ((f1, t1), (f2, t2)) = (
                first.split_once('-').unwrap(),
                second.split_once('-').unwrap()
            );
            (
                f1.parse::<u8>().unwrap(),
                t1.parse::<u8>().unwrap(),
                f2.parse::<u8>().unwrap(),
                t2.parse::<u8>().unwrap(),
            )
        })
        .filter(|(f1, t1, f2, t2)| (f1 >= f2 && t1 <= t2) || (f1 <= f2 && t1 >= t2))
        .count();

    println!("{}", solution)
}