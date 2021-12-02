use itertools::multizip;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn main() {
    let v = load(String::from("./input"));
    let sol1 = part1(v);
    println!("Part 1: {}", sol1);

    let v2 = load(String::from("./input"));
    let sol2 = part2(v2);
    println!("Part 2: {}", sol2);
}

pub fn load(filename: String) -> Vec<Instruction> {
    let mut input = match File::open(filename) {
        Ok(f) => f,
        Err(err) => panic!("Error: {}", err),
    };

    let mut content = String::new();
    match input.read_to_string(&mut content) {
        Err(err) => panic!("Error: {}", err),
        Ok(_) => {}
    }

    content
        .lines()
        .map(|line| Instruction::from_str(String::from(line)))
        .collect()
}

pub enum Instruction {
    Forward { x: i32 },
    Down { x: i32 },
    Up { x: i32 },
}

impl Instruction {
    fn from_str(line: String) -> Instruction {
        let mut split = line.split(" ");
        let dir = split.next().unwrap();
        let amount = split.next().unwrap().parse::<i32>().unwrap();

        return match dir {
            "forward" => Instruction::Forward { x: amount },
            "down" => Instruction::Down { x: amount },
            "up" => Instruction::Up { x: amount },
            _ => panic!("Unmatched instruction: {}", dir)
        };
    }
}

pub struct Submarine {
    depth: i32,
    horizontal: i32
}

impl Submarine {
    fn next(&mut self, inst: Instruction) {
        match inst {
            Instruction::Forward {x: amount} => self.horizontal += amount,
            Instruction::Down {x: amount} => self.depth += amount,
            Instruction::Up {x: amount} => self.depth -= amount,
        }
    }
}

pub struct SubmarineV2 {
    depth: i32,
    horizontal: i32,
    aim: i32
}

impl SubmarineV2 {
    fn next(&mut self, inst: Instruction) {
        match inst {
            Instruction::Forward {x: amount} => { 
                self.horizontal += amount; 
                self.depth += self.aim * amount;
            },
            Instruction::Down {x: amount} => self.aim += amount,
            Instruction::Up {x: amount} => self.aim -= amount,
        }
    }
}

pub fn part1(input: Vec<Instruction>) -> i32 {
    let mut sub = Submarine{ depth: 0, horizontal: 0};
    for inst in input {
        sub.next(inst);
    }
    return sub.depth * sub.horizontal;
}

pub fn part2(input: Vec<Instruction>) -> i32 {
    let mut sub = SubmarineV2{ depth: 0, horizontal: 0, aim: 0};
    for inst in input {
        sub.next(inst);
    }
    return sub.depth * sub.horizontal;
}
