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

pub struct Cave {
    links: HashMap<String, Vec<String>>,
}

impl Cave {
    pub fn from_str(s: &str) -> Cave {
        let mut links = HashMap::new();
        for line in s.lines() {
            let mut it = line.split("-");
            let a = String::from(it.next().unwrap());
            let b = String::from(it.next().unwrap());

            if !links.contains_key(&a) {
                links.insert(a.clone(), Vec::new());
            }

            if !links.contains_key(&b) {
                links.insert(b.clone(), Vec::new());
            }
            // Goofy, but you have to create individual scope for each of these.
            {
                let mut a_paths = links.get_mut(&a).unwrap();
                a_paths.push(b.clone());
            }
            {
                let mut b_paths = links.get_mut(&b).unwrap();
                b_paths.push(a.clone());
            }
        }
        Cave { links: links }
    }
}

pub fn copy(v: &Vec<String>) -> Vec<String> {
    v.iter().map(|x| String::from(x)).collect()
}

pub fn is_small(s: &String) -> bool {
    return &s.to_lowercase() == s;
}

pub fn search(
    links: &HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
    current: String,
) -> u32 {
    let mut candidates = copy(links.get(&current).unwrap());
    let mut val = 0;
    while candidates.len() > 0 {
        let next = candidates.pop().unwrap();
        if next == "end" {
            val += 1;
            continue;
        }
        if path.contains(&next) && is_small(&next) {
            continue;
        }

        path.push(current.clone());
        val += search(links, path, next.clone());
        path.pop();
    }
    val
}

pub fn search_v2(
    links: &HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
    current: String,
    threshold: usize,
) -> u32 {
    let mut candidates = copy(links.get(&current).unwrap());
    let mut val = 0;
    while candidates.len() > 0 {
        let mut new_threshold = threshold;
        let next = candidates.pop().unwrap();
        if next == "end" {
            val += 1;
            continue;
        }

        if (is_small(&next) && path.iter().filter(|x| x == &&next).count() > threshold) || next == "start" {
            continue;
        }

        if path.contains(&next) && is_small(&next) {
            new_threshold = 0;
        }

        path.push(current.clone());
        val += search_v2(links, path, next.clone(), new_threshold);
        path.pop();
    }
    val
}

pub fn part1(input: &String) -> u32 {
    let cave = Cave::from_str(input);

    // DFS
    let path = &mut Vec::new();
    search(&cave.links, path, String::from("start"))
}

pub fn part2(input: &String) -> u32 {
    let cave = Cave::from_str(input);

    // DFS
    let path = &mut Vec::new();
    search_v2(&cave.links, path, String::from("start"), 1)
}
