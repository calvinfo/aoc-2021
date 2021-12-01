use itertools::multizip;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn main() {
    let v = load(String::from("./input"));
    let sol1 = part1(v);
    println!("Part 1: {}", sol1);

    let v2 = load(String::from("./input"));
    let sol2 = part2(&v2);
    println!("Part 2: {}", sol2);
}

pub fn load(filename: String) -> Vec<i32> {
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
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn part1(input: Vec<i32>) -> i32 {
    let mut it = input.into_iter();
    let mut current = it.next().unwrap();
    let mut increases = 0;

    for i in it {
        if current < i {
            increases += 1;
        }
        current = i;
    }
    return increases;
}

pub fn part2(input: &Vec<i32>) -> i32 {
    let a = input.into_iter();
    let b = input.into_iter().skip(1);
    let c = input.into_iter().skip(2);
    let mut current = -1;
    let mut increases = 0;

    for (i, j, k) in multizip((a, b, c)) {
        if current == -1 {
            current = i + j + k;
        }
        if i + j + k > current {
            increases += 1;
        }
        current = i + j + k;
    }
    return increases;
}
