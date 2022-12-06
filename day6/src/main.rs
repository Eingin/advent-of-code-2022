use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(mut lines) = read_lines("./input.txt") {
        let data = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let result_one = data
            .windows(4)
            .position(|window| {
                window[0] != window[1]
                    && window[0] != window[2]
                    && window[0] != window[3]
                    && window[1] != window[2]
                    && window[1] != window[3]
                    && window[2] != window[3]
            })
            .unwrap();

        let result_two = data
            .windows(14)
            .position(|window| {
                let mut seen = HashSet::new();

                for c in window {
                    if seen.contains(&c) {
                        return false;
                    } else {
                        seen.insert(c);
                    }
                }
                return true;
            })
            .unwrap();

        println!("Part one: {}", result_one + 4);
        println!("Part two: {}", result_two + 14);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
