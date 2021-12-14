use std::collections::HashMap;
use std::collections::HashSet;
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
    points: HashMap<Location, Point>,
    num_rows: usize,
    num_cols: usize,
}

type Location = (u32, u32);
type Point = (Location, u32);

pub struct BasinSearch {
    seen: HashSet<Point>,
    candidates: HashSet<Point>,
    points: HashSet<Point>,
}

impl BasinSearch {
    pub fn new(p: &Point) -> BasinSearch {
        let mut candidates = HashSet::new();
        let points = HashSet::new();
        let seen = HashSet::new();
        candidates.insert(*p);
        BasinSearch {
            seen: seen,
            candidates: candidates,
            points: points,
        }
    }

    pub fn search(&mut self, m: &Map) {
        while self.candidates.len() > 0 {
            let candidate = self.candidates.clone().into_iter().next().unwrap();
            let (i, j) = candidate.0;
            for (d_i, d_j) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next = m.get(i as i32 + d_i, j as i32 + d_j);
                match next {
                    None => continue,
                    Some(point) => {
                        if point.1 != 9 && !self.seen.contains(point) && point.1 > candidate.1 {
                            self.candidates.insert(*point);
                        }
                    }
                }
            }
            self.candidates.remove(&candidate);
            self.seen.insert(candidate);
            self.points.insert(candidate);
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}

impl Map {
    pub fn from_str(s: &str) -> Map {
        let mut points: HashMap<Location, Point> = HashMap::new();

        for (i, row) in s.lines().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                let loc = (i as u32, j as u32);
                let val = ch.to_digit(10).unwrap();
                let pt = (loc, val);
                points.insert(loc, pt);
            }
        }

        Map {
            points: points,
            num_rows: s.lines().count(),
            num_cols: s.lines().next().unwrap().len(),
        }
    }

    pub fn get(&self, i: i32, j: i32) -> Option<&Point> {
        if i < 0 || j < 0 {
            return None;
        }

        self.points.get(&(i as u32, j as u32))
    }

    pub fn is_low(&self, i: i32, j: i32) -> bool {
        let pt = self.get(i, j);
        if pt == None {
            return false;
        }

        let point = pt.unwrap();
        let mut candidates: HashSet<Option<&Point>> = HashSet::new();
        candidates.insert(self.get(i, j - 1));
        candidates.insert(self.get(i, j + 1));
        candidates.insert(self.get(i - 1, j));
        candidates.insert(self.get(i + 1, j));

        let lower_point = candidates
            .iter()
            .filter(|x| x != &&None && x.unwrap().1 <= point.1)
            .next();

        match lower_point {
            None => true,
            _ => false,
        }
    }
}

pub fn part1(input: &String) -> u32 {
    let m = Map::from_str(input);
    let mut sum = 0;
    for i in 0..m.num_rows {
        for j in 0..m.num_cols {
            if m.is_low(i as i32, j as i32) {
                sum += m.get(i as i32, j as i32).unwrap().1 + 1;
            }
        }
    }
    sum
}

pub fn part2(input: &String) -> u32 {
    let m = Map::from_str(input);
    let mut basins: Vec<BasinSearch> = Vec::new();

    // First find the low points
    for i in 0..m.num_rows {
        for j in 0..m.num_cols {
            if m.is_low(i as i32, j as i32) {
                let basin = BasinSearch::new(&m.get(i as i32, j as i32).unwrap());
                basins.push(basin);
            }
        }
    }

    // Create a search for each
    let mut searched = Vec::new();
    for mut basin in basins {
        basin.search(&m);
        searched.push(basin);
    }

    // Find the top 3
    searched.sort_by(|x: &BasinSearch, y: &BasinSearch| x.len().cmp(&y.len()));
    let mut sum = 1;
    for _ in 0..3 {
        sum *= searched.pop().unwrap().len() as u32
    }
    sum
}
