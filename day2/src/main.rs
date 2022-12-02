use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

const LOOKUP: [[Outcome; 3]; 3] = [
    [Outcome::Draw, Outcome::Win, Outcome::Loss],
    [Outcome::Loss, Outcome::Draw, Outcome::Win],
    [Outcome::Win, Outcome::Loss, Outcome::Draw],
];

fn choice_value(choice: &char) -> usize {
    return match choice {
        'A' | 'X' => 0,
        'B' | 'Y' => 1,
        'C' | 'Z' => 2,
        _ => panic!("Invalid input"),
    };
}

fn extract_choices(line: String) -> (usize, usize) {
    let mut chars = line.chars();
    return (
        choice_value(&chars.nth(0).unwrap()),
        choice_value(&chars.nth(1).unwrap()),
    );
}

fn part_one(choices: &(usize, usize)) -> u32 {
    return LOOKUP[choices.0][choices.1] as u32 + choices.1 as u32 + 1;
}

fn part_two(choices: &(usize, usize)) -> u32 {
    let intended_outcome = match choices.1 {
        0 => Outcome::Loss,
        1 => Outcome::Draw,
        2 => Outcome::Win,
        _ => panic!("Invalid option"),
    };

    let option = LOOKUP[choices.0]
        .iter()
        .position(|outcome| outcome == &intended_outcome)
        .unwrap();

    return option as u32 + intended_outcome as u32 + 1;
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let all_lines: Vec<(usize, usize)> = lines.map(|l| extract_choices(l.unwrap())).collect();

        let results_one: u32 = all_lines.iter().map(|line| part_one(&line)).sum();
        println!("Part one: {}", results_one);

        let results_two: u32 = all_lines.iter().map(|line| part_two(&line)).sum();
        println!("Part two: {}", results_two);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
