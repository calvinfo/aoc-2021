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

struct Line {
    text: String,
}

fn matching_char(ch: char) -> char {
    match ch {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        ']' => '[',
        ')' => '(',
        '}' => '{',
        '>' => '<',
        _ => panic!("err, bad char: {}", ch),
    }
}

impl Line {
    pub fn from_str(s: &str) -> Line {
        return Line {
            text: String::from(s),
        };
    }

    pub fn is_valid(&self) -> (bool, char) {
        let mut stack = Vec::new();
        for ch in self.text.chars() {
            match ch {
                '[' | '(' | '{' | '<' => stack.push(ch),
                ']' | ')' | '}' | '>' => {
                    let last = stack.pop();
                    if last == None || last.unwrap() != matching_char(ch) {
                        return (false, ch);
                    }
                }
                _ => panic!("unexpected character: {}", ch),
            }
        }

        (true, ' ')
    }

    pub fn complete(&self) -> Vec<char> {
        let mut stack = Vec::new();
        for ch in self.text.chars() {
            match ch {
                '[' | '(' | '{' | '<' => { stack.push(ch); },
                ']' | ')' | '}' | '>' => { stack.pop(); },
                _ => panic!("unexpected character: {}", ch),
            };
        }
        stack
    }

    pub fn complete_score(&self) -> u64 {
        let mut v = self.complete();
        let mut score = 0;
        while v.len() > 0 {
            let add = match matching_char(v.pop().unwrap()) {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("bad character"),
            };

            score *= 5;
            score += add;
        }
        score
    }
}

pub fn part1(input: &String) -> u64 {
    input
        .lines()
        .map(|line| Line::from_str(line))
        .map(|line| {
            let (valid, ch) = line.is_valid();
            match (valid, ch) {
                (false, ')') => 3,
                (false, ']') => 57,
                (false, '}') => 1197,
                (false, '>') => 25137,
                (false, _) => 0,
                (true, _) => 0,
            }
        })
        .sum()
}

pub fn part2(input: &String) -> u64 {
    let mut scores: Vec<u64> = input
        .lines()
        .map(|line| Line::from_str(line))
        .filter(|line| {
            let (valid, _) = line.is_valid();
            valid
        })
        .map(|line| line.complete_score())
        .collect();

    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}
