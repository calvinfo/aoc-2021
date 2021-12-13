use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn main() {
    let v = load(String::from("./input-test"));
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
    points: HashSet<Location>,
    folds: VecDeque<Fold>,
}

type Location = (i32, i32);

enum Direction {
    X,
    Y,
}

pub struct Fold {
    direction: Direction,
    amount: i32,
}

impl Map {
    pub fn from_str(s: &str) -> Map {
        let mut points: HashSet<Location> = HashSet::new();
        let mut folds = VecDeque::new();
        let mut sections = s.split("\n\n");
        let coords = sections.next().unwrap();
        let fold_lines = sections.next().unwrap();

        for line in coords.lines() {
            let mut it = line.split(',');
            let x = it.next().unwrap().parse::<i32>().unwrap();
            let y = it.next().unwrap().parse::<i32>().unwrap();
            points.insert((x, y));
        }

        for line in fold_lines.lines() {
            let mut it = line.split(' ');
            it.next();
            it.next();
            let val = it.next().unwrap();
            let mut val_it = val.split('=');
            let dir = match val_it.next() {
                Some("x") => Direction::X,
                Some("y") => Direction::Y,
                _ => panic!("error"),
            };
            let amount = val_it.next().unwrap().parse::<i32>().unwrap();
            folds.push_back(Fold {
                direction: dir,
                amount: amount,
            })
        }

        Map {
            folds: folds,
            points: points,
        }
    }

    pub fn fold(&mut self) {
        let fold = self.folds.pop_front().unwrap();
        let (delta_x, delta_y) = match fold.direction {
            Direction::X => (fold.amount, 0),
            Direction::Y => (0, fold.amount),
        };

        let mut updated = HashSet::new();
        for point in &self.points {
            let mut new_x = point.0;
            let mut new_y = point.1;

            if new_x < delta_x || new_y < delta_y {
                updated.insert(*point);
                continue;
            }

            if (delta_x > 0 && new_x == delta_x) || (delta_y > 0 && new_y == delta_y) {
                // skip the fold
                continue;
            }

            if delta_x > 0 && new_x > delta_x {
                new_x = delta_x - (new_x - delta_x);
            }

            if delta_y > 0 && new_y > delta_y {
                new_y = delta_y - (new_y - delta_y);
            }

            updated.insert((new_x, new_y));
        }

        self.points = updated;
    }

    pub fn log(&self) {
        let mut x_max = 0;
        let mut y_max = 0;
        for point in &self.points {
            if point.1 > y_max {
                y_max = point.1;
            }
            if point.0 > x_max {
                x_max = point.0;
            }
        }
        for j in 0..y_max+1  {
            for i in 0..x_max+1  {
                if self.points.contains(&(i, j)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }
}

pub fn part1(input: &String) -> u32 {
    let mut m = Map::from_str(input);
    m.fold();
    m.points.len() as u32
}

pub fn part2(input: &String) -> u32 {
    let mut m = Map::from_str(input);
    while m.folds.len() > 0 {
        m.fold();
    }
    m.log();
    m.points.len() as u32
}
