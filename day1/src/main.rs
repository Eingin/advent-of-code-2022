use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elfs = HashMap::new();

    if let Ok(lines) = read_lines("./input.txt") {
        let mut current_elf: u32 = 1;
        for line in lines {
            if let Ok(raw_line) = line {
                if raw_line.is_empty() {
                    current_elf += 1;
                } else if let Ok(cal) = raw_line.parse::<u32>() {
                    elfs.entry(current_elf)
                        .and_modify(|cals| *cals += cal)
                        .or_insert(cal);
                } else {
                    print!("Invalid line")
                }
            }
        }
    }

    let top = elfs.iter().max_by(|x, y| x.1.cmp(y.1)).expect("No entries");
    println!("Top elf: {} {}", top.0, top.1);

    let mut sorted: Vec<(&u32, &u32)> = elfs.iter().collect();
    sorted.sort_by(|x, y| y.1.cmp(x.1));
    let top_three: u32 = sorted.drain(0..3).map(|elf| elf.1).sum();
    println!("Top three elfs: {}", top_three);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
