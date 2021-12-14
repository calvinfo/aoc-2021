use std::collections::VecDeque;
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

pub fn parse(s: &String) -> Vec<LanternFish> {
    s.split(",")
        .map(|i| LanternFish {
            days: i.parse::<u8>().unwrap(),
        })
        .collect()
}

pub struct LanternFish {
    days: u8,
}

impl LanternFish {
    pub fn next(&mut self) -> Option<LanternFish> {
        if self.days == 0 {
            self.days = 6;
            return Some(LanternFish { days: 8 });
        }
        self.days -= 1;
        return None;
    }
}

pub struct LanternFishPool {
    fish: [u64; 7],
    maturing: VecDeque<u64>,
    day: usize,
}

impl LanternFishPool {
    pub fn from_str(s: &str) -> LanternFishPool {
        let fish_days = s.split(",").map(|x| x.parse::<usize>().unwrap());
        let mut fish = [0; 7];
        for day in fish_days {
            fish[day % 7] += 1;
        }
        let maturing = VecDeque::from([0, 0]);
        return LanternFishPool {
            fish: fish,
            maturing: maturing,
            day: 0,
        };
    }

    pub fn next(&mut self) {
        let new_fish = self.maturing.pop_front().unwrap();
        let spawned = self.fish[self.day % 7];
        self.fish[self.day % 7] += new_fish;
        self.maturing.push_back(spawned);
        self.day += 1;
    }

    pub fn count(&self) -> u64 {
        let mut sum = 0;
        for i in 0..7 {
            sum += self.fish[i];
        }
        for maturing in &self.maturing {
            sum += maturing
        }
        sum
    }

    pub fn debug(&self) {
        println!("day: {}, {:?}, {:?}", self.day, self.fish, self.maturing);
    }
}

pub fn part1(input: &String) -> u32 {
    let mut fish = parse(input);
    for _ in 0..80 {
        let mut v = Vec::new();
        for mut f in fish.into_iter() {
            match f.next() {
                Some(x) => v.push(x),
                _ => {}
            }
            v.push(f);
        }
        fish = v
    }

    fish.len() as u32
}

pub fn part2(input: &String) -> u64 {
    let mut pool = LanternFishPool::from_str(&input);
    for i in 0..256 {
        pool.next();
    }
    pool.count()
}
