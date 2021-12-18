use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
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

pub fn bin_to_num(s: &str) -> u32 {
    let mut res = 0;
    for ch in s.chars() {
        res *= 2;
        match ch {
            '1' => res += 1,
            '0' => {}
            _ => panic!("unknown character: {}", ch),
        }
    }
    res
}

pub fn hex_to_bin(s: &str) -> String {
    let mut res = String::from("");
    for ch in s.chars() {
        let digit = match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("unknown character, {}", ch),
        };
        res.push_str(&digit);
    }
    res
}

struct LiteralPacket {
    version: u8,
    typ: u8,
    value: u64,
}

impl LiteralPacket {
    pub fn from_str(s: &str) -> LiteralPacket {
        let bin_str = hex_to_bin(s);
        println!("{}", bin_str);

        let version = &bin_str[0..3];
        let typ = &bin_str[3..6];
        let i = 6;

        let mut it = bin_str.chars().skip(i);
        let mut val_digits = String::from("");
        let mut finished = false;
        while !finished {
            match it.next() {
                Some(i) if i == '1' => {},
                Some(i) if i == '0' => { finished = true },
                None => { finished = true},
                _ => { panic!("error") },
            }

            for _ in 0..4 {
                val_digits.push(it.next().unwrap());
            }
        }


        LiteralPacket {
            version: bin_to_num(&version) as u8,
            typ: bin_to_num(&typ) as u8,
            value: bin_to_num(&val_digits) as u64,
        }
    }
}

struct OperatorPacket {}

pub fn part1(input: &String) -> u32 {
    let packet = LiteralPacket::from_str(&"D2FE28");
    println!("{}", packet.version);
    println!("{}", packet.typ);
    println!("{}", packet.value);
    0
}

pub fn part2(input: &String) -> u32 {
    0
}
