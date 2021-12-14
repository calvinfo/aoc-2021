use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str::Chars;

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

pub fn parse(s: &String) -> Vec<Line> {
    s.lines().map(|s| Line::from_str(s)).collect()
}

pub struct Line {
    digits: Vec<Digit>,
    output: Vec<Digit>,
}

impl Line {
    pub fn from_str(s: &str) -> Line {
        let mut split = s.split(" | ");
        let digits = split
            .next()
            .unwrap()
            .split(" ")
            .map(|s| Digit {
                string: String::from(s),
            })
            .collect();

        let output = split
            .next()
            .unwrap()
            .split(" ")
            .map(|s| Digit {
                string: String::from(s),
            })
            .collect();

        //

        return Line {
            digits: digits,
            output: output,
        };
    }
}

pub struct Digit {
    string: String,
}

impl Digit {
    pub fn naive_value(&self) -> Option<u32> {
        match self.string.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    /**
    * Digit map
    * 
    *  000
    * 1   2
    *  333
    * 4   5
    *  666
    */
    pub fn value(&self, char_map: &[char; 7]) -> u32 {
        let mut digit_map = [0; 7];
        for c in self.chars() {
            let index = char_index(char_map, c);
            digit_map[index] = 1;
        }

        match digit_map {
            [1, 1, 1, 0, 1, 1, 1] => 0,
            [0, 0, 1, 0, 0, 1, 0] => 1,
            [1, 0, 1, 1, 1, 0, 1] => 2,
            [1, 0, 1, 1, 0, 1, 1] => 3,
            [0, 1, 1, 1, 0, 1, 0] => 4,
            [1, 1, 0, 1, 0, 1, 1] => 5,
            [1, 1, 0, 1, 1, 1, 1] => 6,
            [1, 0, 1, 0, 0, 1, 0] => 7,
            [1, 1, 1, 1, 1, 1, 1] => 8,
            [1, 1, 1, 1, 0, 1, 1] => 9,
            _ => panic!("received a bad matrix: {:?}", digit_map)
        }
    }

    pub fn chars(&self) -> Chars<'_> {
        self.string.chars()
    }

    pub fn contains_all(&self, v: &Vec<char>) -> bool {
        for c in v {
            if self.missing(&c) {
                return false;
            }
        }
        true
    }

    pub fn missing(&self, c: &char) -> bool {
        for ch in self.chars() {
            if ch == *c {
                return false;
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        return self.string.len();
    }
}

pub fn char_to_digit(c: char) -> u32 {
    let start = 'a'.to_digit(17).unwrap();
    return c.to_digit(17).unwrap() - start;
}

pub fn char_index(char_map: &[char; 7], c: char) -> usize {
    for i in 0..7 {
        if char_map[i] == c {
            return i
        }
    }
    8
}

/**
 * Takes in a set of digits, and maps them via the following scheme in the
 * array mapped
 *  000
 * 1   2
 *  333
 * 4   5
 *  666
 *
 * 1. mapped[2] = digit 1 w/ count == 8
 * 2. mapped[5] = digit 1 w/ count == 9
 * 3. mapped[0] = digit 7 unmapped
 * 4. mapped[3] = digit 4 unmapped w/ count == 7
 * 5. mapped[1] = digit 4 unmapped w/ count == 6
 * 6. mapped[6] = digit 5 unmapped
 * 7. mapped[4] = remaining
 *
 */
pub fn map_digits(digits: &Vec<Digit>) -> [char; 7] {
    let mut unmapped = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    let mut mapped = ['x'; 7];
    let mut total_counts = [0; 7];
    for digit in digits {
        digit
            .chars()
            .for_each(|c| total_counts[char_to_digit(c) as usize] += 1);
    }

    // Steps 1 & 2.
    let one = digits.iter().find(|x| x.naive_value() == Some(1)).unwrap();
    for c in one.chars() {
        match total_counts[char_to_digit(c) as usize] {
            8 => mapped[2] = c,
            9 => mapped[5] = c,
            _ => panic!("an error occured!"),
        };
        unmapped.remove(&c);
    }

    // Step 3.
    let seven = digits.iter().find(|x| x.naive_value() == Some(7)).unwrap();
    for c in seven.chars() {
        if unmapped.contains(&c) {
            mapped[0] = c;
            unmapped.remove(&c);
        }
    }

    // Step 4 & 5.
    let four = digits.iter().find(|x| x.naive_value() == Some(4)).unwrap();
    for c in four.chars() {
        if !unmapped.contains(&c) {
            continue;
        }

        match total_counts[char_to_digit(c) as usize] {
            6 => mapped[1] = c,
            7 => mapped[3] = c,
            _ => panic!("an error occured!"),
        };
        unmapped.remove(&c);
    }

    // Step 6.
    let five_digits = vec![mapped[0], mapped[1], mapped[3], mapped[5]];
    let five = digits
        .iter()
        .find(|x| x.contains_all(&five_digits) && x.len() == 5 && x.missing(&mapped[2]))
        .unwrap();

    for c in five.chars() {
        if !unmapped.contains(&c) {
            continue;
        }

        mapped[6] = c;
        unmapped.remove(&c);
    }

    // Step 7.
    mapped[4] = *unmapped.iter().next().unwrap();
    return mapped;
}


pub fn part1(input: &String) -> u32 {
    let lines = parse(input);
    let mut sum = 0;
    for line in lines {
        for digit in line.output {
            match digit.naive_value() {
                None => {}
                Some(_) => sum += 1,
            }
        }
    }
    sum
}

pub fn part2(input: &String) -> u32 {
    let lines = parse(input);
    let mut sum = 0;
    for line in lines {
        let digit_map = map_digits(&line.digits);
        let mut output_sum = 0;
        for digit in line.output {
            output_sum *= 10;
            output_sum += digit.value(&digit_map);
        }
        sum += output_sum;
    }
    sum
}
