use std::fs::File;
use std::io::Read;

fn main() {
    let v = load(String::from("./input"));
    let sol1 = part1(&v);
    println!("Part 1: {}", sol1);

    let v2 = load(String::from("./input"));
    let sol2 = part2(&v2);
    println!("Part 2: {}", sol2);
}

pub fn load(filename: String) -> String {
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
}

pub fn parse(s: &String) -> Vec<i32> {
    let mut res: Vec<i32> = s.split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    res.sort();

    res
}

pub fn part1(input: &String) -> i32 {
    let crabs = &parse(input);
    let median = crabs.get(crabs.len() / 2).unwrap();

    let mut sum = 0;
    for crab in crabs {
        sum += (*median - crab).abs();
    }
    sum
}

pub fn cost(crabs: &Vec<i32>, n: i32) -> i32 {
    let mut cost = 0;
    for crab in crabs {
        let delta = (crab - n).abs();
        cost += (delta * (delta + 1)) / 2;
    }
    cost
}

pub fn part2(input: &String) -> i32 {
    let crabs = &parse(input);
    let min = crabs.first().unwrap();
    let max = crabs.last().unwrap();
    let mut res = i32::MAX;

    for i in *min..*max {
        let computed_cost = cost(&crabs, i);
        if computed_cost < res {
            res = computed_cost;
        }
    }
    res
}
