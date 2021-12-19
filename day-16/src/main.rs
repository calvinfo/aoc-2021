use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str::Chars;

static mut SUM: u32 = 0;

fn main() {
    let v = load(String::from("./input"));
    let sol1 = part1(&v);
    println!("Part 1: {}", sol1);

    // let v2 = load(String::from("./input"));
    // let sol2 = part2(&v2);
    // println!("Part 2: {}", sol2);
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

trait Packet {
    fn version(&self) -> u8;
    fn typ(&self) -> u8;
    fn from_it(it: &mut Chars, version: u8, typ: u8) -> Self;
}


struct LiteralPacket {
}

struct OperatorPacket {
}

impl LiteralPacket {
    pub fn from_it(it: &mut Chars, version: u8) -> ParseResult {
        let mut val_digits = String::from("");
        let mut finished = false;
        let mut bits = 0;

        while !finished {
            match it.next() {
                Some(i) if i == '1' => {}
                Some(i) if i == '0' => finished = true,
                None => finished = true,
                _ => {
                    panic!("error")
                }
            }
            for _ in 0..4 {
                val_digits.push(it.next().unwrap());
            }
            bits += 5;
        }

        ParseResult{
            packets: 1,
            bits: bits,
        }
    }
}

impl OperatorPacket {
    pub fn from_it(it: &mut Chars, version: u8, typ: u8) -> ParseResult {
        let mut bits = 0;
        let mut packets = 0;
        let len_id = it.next().unwrap();
        if len_id == '0' {
            let mut val_digits = String::from("");
            for _ in 0..15 {
                val_digits.push(it.next().unwrap());
            }
            let mut bit_count = bin_to_num(&val_digits);
            while bit_count > 0 {
                let result = PacketParser::parse(it);
                if bit_count < result.bits {
                    break;
                }
                bit_count -= result.bits;
                bits += result.bits;
                packets += result.packets;
            }
        }

        if len_id == '1' {
            let mut val_digits = String::from("");
            for _ in 0..11 {
                val_digits.push(it.next().unwrap());
            }
            let mut packet_count = bin_to_num(&val_digits);
            while packet_count > 0 {
                let result = PacketParser::parse(it);
                packet_count -= 1;
                bits += result.bits;
                packets += result.packets;
            }
        }

        ParseResult{
            packets: packets,
            bits: bits,
        }
    }
}

struct PacketParser {}

struct ParseResult {
    packets: u32,
    bits: u32,
    // versions: Vec<u8>
}

impl PacketParser {
    pub fn parse(it: &mut Chars) -> ParseResult {
        // Parse header to figure out what's next..
        let mut version_str = String::from("");
        for _ in 0..3 {
            version_str.push(it.next().unwrap());
        }

        let mut typ_str = String::from("");
        for _ in 0..3 {
            typ_str.push(it.next().unwrap());
        }

        let version = bin_to_num(&version_str);
        let typ = bin_to_num(&typ_str);
        println!("{}", version);

        let res = match typ {
            4 => {
                LiteralPacket::from_it(it, version as u8)
            } // literal
            _ => {
                OperatorPacket::from_it(it, version as u8, typ as u8)
            }
        };
        ParseResult{packets: res.packets + 1, bits: res.bits + 6}
    }
}

pub fn part1(input: &String) -> u32 {
    let bin_str = hex_to_bin(input);
    let mut chars = bin_str.chars();
    PacketParser::parse(&mut chars);

    // let packet = LiteralPacket::from_str(&"D2FE28");
    // println!("{}", packet.version);
    // println!("{}", packet.typ);
    // println!("{}", packet.value);
    0
}

pub fn part2(input: &String) -> u32 {
    0
}
