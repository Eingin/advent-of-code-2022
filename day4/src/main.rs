use std::{
    fs::File,
    io::{self, BufRead},
    ops::Range,
    path::Path,
};

fn parse_range(raw: &str) -> Range<u32> {
    let mut parts = raw.split("-");
    let start: u32 = parts.next().unwrap().parse().unwrap();
    let end: u32 = parts.next().unwrap().parse().unwrap();
    return start..end;
}
fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let uw: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let ranges: Vec<(Range<u32>, Range<u32>)> = uw
            .iter()
            .map(|line| {
                let mut parts = line.split(",");
                let first: Range<u32> = parse_range(parts.next().unwrap());
                let second: Range<u32> = parse_range(parts.next().unwrap());
                (first, second)
            })
            .collect();

        let part_one = ranges
            .iter()
            .map(|(first, second)| {
                first.start >= second.start && first.end <= second.end
                    || second.start >= first.start && second.end <= first.end
            })
            .filter(|intersects| *intersects)
            .count();

        let part_two = ranges
            .iter()
            .map(|(first, second)| first.end >= second.start && first.start <= second.end)
            .filter(|intersects| *intersects)
            .count();

        println!("{}", part_one);
        println!("{}", part_two);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
