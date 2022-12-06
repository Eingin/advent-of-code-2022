use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
struct CargoHold {
    stacks: Vec<VecDeque<char>>,
}

impl CargoHold {
    fn new(stack_count: usize) -> CargoHold {
        let mut cargo = CargoHold {
            stacks: Vec::with_capacity(stack_count),
        };

        for _ in 0..stack_count {
            cargo.stacks.push(VecDeque::new());
        }

        return cargo;
    }

    fn from_input(raw: &[String]) -> CargoHold {
        let (stack_row, rows) = raw.split_last().unwrap();
        let stack_count = stack_row.split("   ").count();
        let mut cargo = CargoHold::new(stack_count);

        rows.iter()
            .rev()
            .map(|row| {
                let els = row
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|chunk| chunk.iter().collect())
                    .collect::<Vec<String>>();

                return els
                    .iter()
                    .map(|cell| cell.replace(&['[', ']', ' '], "").chars().nth(0))
                    .collect::<Vec<Option<char>>>();
            })
            .for_each(|row| {
                println!("{:?}", row);
                row.iter().enumerate().for_each(|(index, item)| match item {
                    Some(value) => cargo.stacks[index].push_front(value.clone()),
                    None => (),
                })
            });

        cargo
    }
}

trait Crane {
    fn do_move_crate_mover_9000(&mut self, count: usize, from: usize, to: usize);
    fn do_move_crate_mover_9001(&mut self, count: usize, from: usize, to: usize);
}

impl Crane for CargoHold {
    fn do_move_crate_mover_9000(&mut self, count: usize, from: usize, to: usize) {
        assert!(
            self.stacks.len() >= from && self.stacks.len() >= to,
            "Stacks indcies are aout of range"
        );
        assert!(
            self.stacks[from].len() >= count,
            "Not enought elements in from stack to take"
        );

        for _ in 0..count {
            if let Some(x) = self.stacks[from].pop_front() {
                self.stacks[to].push_front(x);
            }
        }
    }

    fn do_move_crate_mover_9001(&mut self, count: usize, from: usize, to: usize) {
        assert!(
            self.stacks.len() >= from && self.stacks.len() >= to,
            "Stacks indcies are aout of range"
        );
        assert!(
            self.stacks[from].len() >= count,
            "Not enought elements in from stack to take"
        );

        let mut buffer: Vec<char> = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(x) = self.stacks[from].pop_front() {
                buffer.push(x);
            }
        }
        // buffer.reverse();

        for _ in 0..count {
            self.stacks[to].push_front(buffer.pop().unwrap());
        }
    }
}

fn pase_movement(line: &String) -> (usize, usize, usize) {
    let words: Vec<&str> = line.split(" ").collect();
    (
        words[1].parse::<usize>().unwrap(),
        words[3].parse::<usize>().unwrap() - 1,
        words[5].parse::<usize>().unwrap() - 1,
    )
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let uw: Vec<String> = lines.map(|line| line.unwrap()).collect();

        let (stacks, instrustions) = uw.split_at(
            uw.iter()
                .position(|line| line.is_empty())
                .expect("Unable to find split point"),
        );

        let mut cargo_part_one = CargoHold::from_input(stacks);
        let mut cargo_part_two = CargoHold::from_input(stacks);

        let movements: Vec<(usize, usize, usize)> = instrustions
            .iter()
            .filter(|line| !line.is_empty())
            .map(pase_movement)
            .collect();

        movements.iter().for_each(|movement| {
            cargo_part_one.do_move_crate_mover_9000(movement.0, movement.1, movement.2)
        });
        movements.iter().for_each(|movement| {
            cargo_part_two.do_move_crate_mover_9001(movement.0, movement.1, movement.2)
        });

        let result_one = String::from_iter(
            cargo_part_one
                .stacks
                .iter()
                .map(|stack| stack.front().unwrap().clone()),
        );

        let result_two = String::from_iter(
            cargo_part_two
                .stacks
                .iter()
                .map(|stack| stack.front().unwrap().clone()),
        );

        println!("Part one: {}", result_one);
        println!("Part two: {}", result_two);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
