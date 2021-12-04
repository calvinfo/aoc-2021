use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::fmt;


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

pub fn parse(input: &String) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut boards = Vec::new();

    let mut it = input.split("\n\n");
    let numbers = parse_numbers(String::from(it.next().unwrap()));
    for board in it {
        boards.push(BingoBoard::from_str(String::from(board)))
    }

    return (numbers, boards);
}

pub fn parse_numbers(input: String) -> Vec<u32> {
    let numbers = input.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    return numbers;
}


pub struct BingoBoard {
    numbers: [[u32; 5]; 5]
}

impl BingoBoard {
    pub fn from_str(s: String) -> BingoBoard {
        let mut numbers = [[0; 5]; 5];
        let lines = s.lines();
        let mut i = 0;
        for line in lines {
            let mut j = 0;
            for num in line.split_ascii_whitespace() {
                numbers[i][j] = num.parse::<u32>().unwrap();
                j += 1;
            }
            i += 1;
        }
        return BingoBoard { numbers: numbers }
    }

    pub fn check(&self, set: &HashSet<u32>) -> bool {
        for i in 0..5 {
            if self.check_row(i, set) || self.check_col(i, set) {
                return true;
            }
        }
        return self.check_x(set);
    }

    fn check_row(&self, row: usize, set: &HashSet<u32>) -> bool {
        for i in 0..5 {
            if !set.contains(&self.numbers[row][i]) {
                return false
            }
        }
        return true;
    }

    fn check_col(&self, col: usize, set: &HashSet<u32>) -> bool {
        for i in 0..5 {
            if !set.contains(&self.numbers[i][col]) {
                return false
            }
        }
        return true;
    }

    fn check_x(&self, set: &HashSet<u32>) -> bool {
        for i in 0..5 {
            if !set.contains(&self.numbers[i][i]) {
                break
            }
            if i == 5 {
                return true;
            }
        }

        for i in 0..5 {
            if !set.contains(&self.numbers[4-i][i]) {
                break
            }
            if i == 5 {
                return true;
            }
        }

        return false;
    }

    pub fn unmarked_sum(&self, set: &HashSet<u32>) -> u32 {
        let mut result = 0;
        for i in 0..5 {
            for j  in 0..5 {
                if !set.contains(&self.numbers[i][j]) {
                    result += self.numbers[i][j];
                }
            }
        }
        return result
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..5 {
            for j in 0..5 {
                write!(f, "{:>2} ", self.numbers[i][j]);
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}


pub fn part1(input: &String) -> u32 {
    let (numbers, boards) = parse(input);
    let mut set = HashSet::new();

    for num in numbers {
        set.insert(num);
        for board in &boards {
            if board.check(&set) {
                return board.unmarked_sum(&set) * num;
            }
        }
    }
    return 0;
}

pub fn part2(input: &String) -> u32 {
    let (numbers, boards) = parse(input);
    let mut set = HashSet::new();
    let mut found_board = false;
    let mut board: &BingoBoard = &BingoBoard{ numbers: [[0; 5]; 5]};
    let mut num_it = numbers.into_iter();

    // find our board
    while !found_board {
        let num = num_it.next().unwrap();
        set.insert(num);

        let remaining = boards.iter().filter(|board| !board.check(&set)).count();
        if remaining == 1 {
            board = boards.iter().filter(|board| !board.check(&set)).next().unwrap();
            found_board = true;
        }
    }

    // continue checking
    while !board.check(&set) {
        let num = num_it.next().unwrap();
        set.insert(num);

        if board.check(&set) {
            return board.unmarked_sum(&set) * num;
        }
    }

    return 0;
}
