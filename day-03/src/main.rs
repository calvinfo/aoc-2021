use itertools::multizip;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn main() {
    let v = load(String::from("./input"));
    let sol1 = part1(v);
    println!("Part 1: {}", sol1);

    let v2 = load(String::from("./input-test"));
    let sol2 = part2(v2);
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

pub struct Matrix {
    rows: Vec<String>,
}

impl Matrix {
    fn from_str(s: String) -> Matrix {
        let v = s.lines().map(|x| String::from(x)).collect();
        return Matrix { rows: v };
    }

    fn gamma(&self) -> u32 {
        let mut result = 0;
        for i in 0..self.size() {
            result *= 2;
            result += self.gamma_bit(i);
        }
        return result;
    }

    fn size(&self) -> usize {
        return self.rows[0].len();
    }

    fn gamma_bit(&self, n: usize) -> u32 {
        let mut ones = 0;
        let mut zeros = 0;

        self
            .rows
            .iter()
            .for_each(|x| {
                match x.chars().nth(n) {
                    Some('1') => ones += 1,
                    Some('0') => zeros += 1,
                    _ => panic!("error"),
                };
            });

        if zeros > ones {
            return 0;
        }
        return 1;
    }

    fn epsilon(&self) -> u32 {
        let mut result = 0;
        for i in 0..self.size() {
            result *= 2;
            if self.gamma_bit(i) == 0 {
                result += 1
            }
        }
        return result;
    }
}

pub fn part1(input: String) -> u32 {
    let m = Matrix::from_str(input);
    return m.gamma() * m.epsilon();
}

pub fn part2(input: String) -> i32 {
    return 0;
}
