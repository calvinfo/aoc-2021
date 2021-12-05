use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


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

pub fn parse(input: &String) -> Vec<Line> {
    input.lines().map(|s| Line::from_str(s)).collect()
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Point {
    fn from_str(s: &str) -> Point {
        let mut it = s.split(",");
        let x = it.next().unwrap().parse::<i32>().unwrap();
        let y = it.next().unwrap().parse::<i32>().unwrap();
        return Point { x: x, y: y };
    }
}

impl Line {
    fn from_str(s: &str) -> Line {
        let mut it = s.split(" -> ");
        let start_str = it.next().unwrap();
        let end_str = it.next().unwrap();

        let a = Point::from_str(start_str);
        let b = Point::from_str(end_str);
        let (start, end) = match a < b {
            true => (a, b),
            _ => (b, a),
        };

        return Line {
            start: start,
            end: end,
        };
    }

    fn is_horizontal(&self) -> bool {
        self.end.y == self.start.y
    }

    fn is_vertical(&self) -> bool {
        self.end.x == self.start.x
    }

    fn points(&self) -> Vec<Point> {
        let mut v = Vec::new();
        if self.is_vertical() {
            for i in 0..(self.end.y - self.start.y + 1) {
                v.push(Point{ x: self.start.x, y: self.start.y + i });
            }
            return v;
        }

        if self.is_horizontal() {
            for i in 0..(self.end.x - self.start.x + 1) {
                v.push(Point{ x: self.start.x + i, y: self.start.y });
            }
            return v;
        }

        let delta_x = delta(self.start.x, self.end.x);
        let delta_y = delta(self.start.y, self.end.y);

        for i in 0..(self.end.x - self.start.x + 1) {
            v.push(Point{ x: self.start.x + delta_x * i, y: self.start.y + delta_y * i})
        }

        return v;
    }
}

fn delta(start: i32, end: i32) -> i32 {
    if end > start {
        return 1
    }
    return -1;
}

pub fn part1(input: &String) -> u32 {
    let lines = parse(input);
    let straight_lines = lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical());

    let mut map: HashMap<Point, u32> = HashMap::new();
    for line in straight_lines {
        for point in line.points() {
            let entry = map.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    map.values().filter(|x| *x > &1).count() as u32
}

pub fn part2(input: &String) -> u32 {
    let lines = parse(input);
    let mut map: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        for point in line.points() {
            let entry = map.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    map.values().filter(|x| *x > &1).count() as u32
}
