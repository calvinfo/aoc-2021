use std::collections::HashMap;
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

pub struct Map {
    points: HashMap<Location, u32>,
    num_rows: usize,
    num_cols: usize,
}

type Location = (i32, i32);

impl Map {
    pub fn from_str(s: &str) -> Map {
        let mut points: HashMap<Location, u32> = HashMap::new();

        for (i, row) in s.lines().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                let loc = (i as i32, j as i32);
                let val = ch.to_digit(10).unwrap();
                points.insert(loc, val);
            }
        }

        Map {
            points: points,
            num_rows: s.lines().count(),
            num_cols: s.lines().next().unwrap().len(),
        }
    }

    pub fn next(&mut self) -> u32 {
        let mut pulsed = 0;
        let mut points = Vec::new();
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                points.push((i as i32, j as i32));
            }
        }
        for point in &points {
            let val = self.points.get_mut(&point).unwrap();
            *val += 1;
        }

        while points.len() > 0 {
            let point = points.pop().unwrap();
            let val = self.points.get_mut(&point).unwrap();
            if *val == 0 {
                continue;
            }

            if val > &mut 9 {
                pulsed += 1;
                *val = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        let new_x: i32 = point.0 as i32 + i as i32 - 1;
                        let new_y: i32 = point.1 as i32 + j as i32 - 1;

                        if (i == 1 && j == 1)
                            || (new_x < 0 || new_x >= self.num_rows as i32)
                            || (new_y < 0 || new_y >= self.num_cols as i32)
                        {
                            continue;
                        }
                        let neighbor_val = self.points.get_mut(&(new_x, new_y)).unwrap();
                        if *neighbor_val == 0 {
                            continue;
                        }

                        *neighbor_val += 1;
                        points.push((new_x, new_y));
                    }
                }
            }
        }
        pulsed
    }

    pub fn log(&self) {
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                print!("{}", self.points.get(&(i as i32, j as i32)).unwrap());
            }
            println!("");
        }
    }
}

pub fn part1(input: &String) -> u32 {
    let mut m = Map::from_str(input);
    let mut pulses = 0;
    for _ in 0..100 {
        pulses += m.next();
    }
    pulses
}

pub fn part2(input: &String) -> u32 {
    let mut m = Map::from_str(input);
    let mut steps = 0;
    loop {
        steps += 1;
        let pulsed = m.next();
        if pulsed == (m.num_cols * m.num_rows) as u32 {
            return steps;
        }
    }
}
