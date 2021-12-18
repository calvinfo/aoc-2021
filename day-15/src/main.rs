use std::collections::HashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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

type Point = (usize, usize);

pub struct Map {
    points: HashMap<Point, u32>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Path {
    cost: u32,
    point: Point,
}

impl Map {
    pub fn from_str(s: &str) -> Map {
        let mut points = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.char_indices() {
                points.insert((row, col), ch.to_digit(10).unwrap());
            }
        }
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        Map {
            points: points,
            width: width,
            height: height,
        }
    }

    pub fn from_str_v2(s: &str) -> Map {
        let mut points = HashMap::new();
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();

        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.char_indices() {
                for i in 0..5 {
                    for j in 0..5 {
                        let mut val = ch.to_digit(10).unwrap();
                        val += i;
                        if val > 9 {
                            val -= 9;
                        }
                        val += j;
                        if val > 9 {
                            val -= 9;
                        }
                        points.insert((row + (i as usize * height as usize), col + (j as usize * width as usize)), val);
                    }
                }
            }
        }
        Map {
            points: points,
            width: width * 5,
            height: height * 5,
        }
    }

    pub fn least_path(&self) -> u32 {
        // keep track of current paths
        // each path has a cost
        // next options have a new cost
        // then plus the value
        let mut considered_points: HashSet<Point> = HashSet::new();

        let mut paths = BinaryHeap::new();
        paths.push(Reverse(Path {
            point: (0, 0),
            cost: 0,
        }));

        while paths.len() > 0 {
            // always find the lowest cost path
            let rev = paths.pop().unwrap();
            let path = match rev {
                Reverse(path) => path,
                _ => panic!("error!"),
            };

            // if we've already seen this point in the meantime, ignore it
            // that must mean there's a lower cost path already found.
            if considered_points.contains(&path.point) {
                continue;
            }

            let (i, j) = path.point;
            let current_cost = path.cost;
            if i == self.height - 1 && j == self.width - 1 {
                // we're at the end!
                return current_cost;
            }

            considered_points.insert(path.point);
            for (d_x, d_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if (i as i32 + d_x < 0) || (j as i32 + d_y < 0) {
                    continue;
                }

                let x = (i as i32 + d_x) as usize;
                let y = (j as i32 + d_y) as usize;
                if considered_points.contains(&(x, y)) {
                    continue;
                }

                let val = self.points.get(&(x, y));
                match val {
                    None => {
                        continue;
                    }
                    Some(val) => {
                        paths.push(Reverse(Path {
                            point: (x, y),
                            cost: val + current_cost,
                        }))
                    }
                }
            }
        }
        0
    }
}


pub fn part1(input: &String) -> u32 {
    let map = Map::from_str(input);
    map.least_path()
}

pub fn part2(input: &String) -> u32 {
    let map = Map::from_str_v2(input);
    map.least_path()
}
