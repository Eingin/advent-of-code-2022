#![feature(iter_collect_into)]
#![feature(exclusive_range_pattern)]

use std::{
    borrow::Borrow,
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn calc_value(item: &char) -> u32 {
    return item.clone() as u32
        - match item {
            'a'..='z' => 96,
            'A'..='Z' => 38,
            _ => panic!(),
        };
}

fn part_one(line: &String) -> u32 {
    let length = line.chars().count();
    let compartments = line.split_at(length / 2);

    let s1: HashSet<char> = HashSet::from_iter(compartments.0.chars());
    let s2 = HashSet::from_iter(compartments.1.chars());

    return calc_value(s1.intersection(&s2).nth(0).expect("No common value"));
}

fn part_two(group: &[String]) -> u32 {
    let mut sets: VecDeque<HashSet<char>> = group
        .iter()
        .map(|sack| HashSet::from_iter(sack.chars()))
        .collect();

    let mut root_set = sets.pop_front().expect("Empty sets");
    sets.iter()
        .for_each(|set| root_set.retain(|el| set.contains(el)));

    return calc_value(root_set.iter().nth(0).expect("No common value"));
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let uw: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let part_one_result = uw.iter().map(part_one).sum::<u32>();
        let part_two_result = uw.chunks(3).map(part_two).sum::<u32>();
        println!("Part one: {}", part_one_result);
        println!("Part two: {}", part_two_result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
