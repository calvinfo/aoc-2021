use std::collections::HashMap;
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

pub fn parse(s: &str) -> (Polymer, InsertionRules) {
    let mut it = s.split("\n\n");
    let polymer = Polymer::from_str(it.next().unwrap());
    let rules = InsertionRules::from_str(it.next().unwrap());
    (polymer, rules)
}

pub struct Polymer {
    template: HashMap<(char, char), u64>,
    first: char,
    last: char,
}

pub struct InsertionRules {
    rules: HashMap<(char, char), char>,
}

impl InsertionRules {
    pub fn get(&self, seq: &(char, char)) -> Option<&char> {
        self.rules.get(seq)
    }
}

impl Polymer {
    pub fn from_str(s: &str) -> Polymer {
        let first = s.chars();
        let second = s.chars().skip(1);
        let mut map = HashMap::new();

        for pair in first.zip(second) {
            match map.get_mut(&pair) {
                Some(x) => {
                    *x += 1;
                }
                None => {
                    map.insert(pair, 1);
                }
            }
        }

        return Polymer {
            first: s.chars().next().unwrap(),
            last: s.chars().last().unwrap(),
            template: map,
        };
    }

    pub fn apply(&self, rules: &InsertionRules) -> Polymer {
        let mut map: HashMap<(char, char), u64> = HashMap::new();
        for (key, val) in self.template.iter() {
            match rules.get(key) {
                Some(c) => {
                    let first = (key.0, *c);
                    if map.contains_key(&first) {
                        *map.get_mut(&first).unwrap() += val;
                    } else {
                        map.insert(first, *val); // add the amount to the map
                    }

                    let second = (*c, key.1);
                    if map.contains_key(&second) {
                        *map.get_mut(&second).unwrap() += val;
                    } else {
                        map.insert(second, *val); // add the amount to the map
                    }
                }
                None => {}
            }
        }

        Polymer {
            template: map,
            first: self.first,
            last: self.last,
        }
    }

    pub fn counts(&self) -> HashMap<char, u64> {
        let mut counts = self.template.iter().fold(HashMap::new(), |mut acc, item| {
            let (a, b) = item.0;
            let count = item.1;
            match acc.get_mut(a) {
                Some(val) => {
                    *val += *count;
                }
                None => {
                    acc.insert(*a, *count);
                }
            };
            match acc.get_mut(b) {
                Some(val) => {
                    *val += *count;
                }
                None => {
                    acc.insert(*b, *count);
                }
            };
            acc
        });

        *counts.get_mut(&self.first).unwrap() += 1;
        *counts.get_mut(&self.last).unwrap() += 1;

        for (_, val) in counts.iter_mut() {
            *val /= 2;
        }
        counts
    }
}

impl InsertionRules {
    pub fn from_str(s: &str) -> InsertionRules {
        let mut map = HashMap::new();
        for line in s.lines() {
            let mut it = line.split(" -> ");
            let seq = it.next().unwrap();
            let res = it.next().unwrap();
            let a = seq.chars().next().unwrap();
            let b = seq.chars().last().unwrap();
            map.insert((a, b), res.chars().next().unwrap());
        }

        InsertionRules { rules: map }
    }
}

pub fn part1(input: &String) -> u64 {
    let (start, rules) = parse(input);
    let mut polymer = start;
    for _ in 0..10 {
        polymer = polymer.apply(&rules);
    }

    let counts = &polymer.counts();
    let min = counts.into_iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    let max = counts.into_iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    max.1 - min.1
}

pub fn part2(input: &String) -> u64 {
    let (start, rules) = parse(input);
    let mut polymer = start;
    for _ in 0..40 {
        polymer = polymer.apply(&rules);
    }

    let counts = &polymer.counts();
    let min = counts.into_iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    let max = counts.into_iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    max.1 - min.1
}
