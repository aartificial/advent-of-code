use std::str::from_utf8;
use regex::Regex;

fn main() {
    let data = include_bytes!("../res/input.txt");
    let (stage, instructions) = data.split_at(data.windows(2).position(|b| b == b"\n\n").unwrap() + 2);

    let stage = from_utf8(stage).unwrap();
    let mut stage: Vec<Vec<char>> = stage
        .trim()
        .lines()
        .map(|line| line
            .chars()
            .filter(|&c| c != ' ')
            .collect())
        .collect();

    let instructions = from_utf8(instructions).unwrap();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let instructions = instructions
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            (captures[1].parse::<usize>().unwrap(),
             captures[2].parse::<usize>().unwrap(),
             captures[3].parse::<usize>().unwrap())

        })
        .collect::<Vec<(usize, usize, usize)>>();

    instructions.iter()
        .for_each(|(n,a,b)| {
            for _ in 0..*n {
                let value = stage[*a - 1].pop().unwrap();
                stage[*b - 1].push(value);
            }
        });

    stage
        .iter()
        .for_each(|c|
            print!("{}",
               *c
                   .last()
                   .unwrap()))
}
