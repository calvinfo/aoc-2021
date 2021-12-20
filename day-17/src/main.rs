fn main() {
    println!("Part 1 test: {}", part1((20, 30), (-10, -5)));
    println!("Part 1: {}", part1((269, 292), (-68, -44)));

    println!("Part 2 test: {}", part2((20, 30), (-10, -5)));
    println!("Part 2: {}", part2((269, 292), (-68, -44)));
}

pub fn in_x_range(mut x_vel: i64, x_min: i64, x_max: i64) -> bool {
    let mut pos = 0;
    while x_vel != 0 && (pos <= x_max) {
        pos += x_vel;
        if pos >= x_min && pos <= x_max {
            return true;
        }
        x_vel -= 1;
    }
    false
}

pub fn in_y_range(mut y_vel: i64, y_min: i64, y_max: i64) -> bool {
    let mut pos = 0;
    while pos >= y_min {
        pos += y_vel;
        if pos <= y_max && pos >= y_min {
            return true;
        }
        y_vel -= 1;
    }
    false
}

pub fn find_x_range(x_min: i64, x_max: i64) -> (i64, i64) {
    let v: Vec<i64> = (0..x_max + 1)
        .filter(|vel| in_x_range(*vel, x_min, x_max))
        .collect();
    (*v.iter().min().unwrap(), *v.iter().max().unwrap())
}

pub fn find_y_range(x_lim: i64, y_min: i64, y_max: i64) -> (i64, i64) {
    let v: Vec<i64> = (0..x_lim + 1)
        .filter(|vel| in_y_range(*vel, y_min, y_max))
        .collect();
    (*v.iter().min().unwrap(), *v.iter().max().unwrap())
}

pub fn test_x_vals(x_vel: i64, x_min: i64, x_max: i64) {
    println!(
        "x_v: {}, x_min: {}, x_max: {} = {}",
        x_vel,
        x_min,
        x_max,
        in_x_range(x_vel, x_min, x_max)
    )
}

pub fn test_y_vals(y_vel: i64, y_min: i64, y_max: i64) {
    println!(
        "x_v: {}, y_min: {}, y_max: {} = {}",
        y_vel,
        y_min,
        y_max,
        in_y_range(y_vel, y_min, y_max)
    )
}

pub fn part1(x_range: (i64, i64), y_range: (i64, i64)) -> i64 {
    let (x_min, x_max) = find_x_range(x_range.0, x_range.1);
    let (y_min, y_max) = find_y_range(x_range.1, y_range.0, y_range.1);
    (y_max * (y_max + 1)) / 2
}

pub fn inside(mut x_vel: i64, mut y_vel: i64, x_range: (i64, i64), y_range: (i64, i64)) -> bool {
    let mut x_pos = 0;
    let mut y_pos = 0;
    while x_pos <= x_range.1 && y_pos >= y_range.0 {
        x_pos += x_vel;
        y_pos += y_vel;
        if x_pos >= x_range.0 && x_pos <= x_range.1 && y_pos >= y_range.0 && y_pos <= y_range.1 {
            return true;
        }

        if x_vel > 0 {
            x_vel -= 1;
        }
        y_vel -= 1;
    }
    false
}

pub fn part2(x_range: (i64, i64), y_range: (i64, i64)) -> u64 {
    let (x_min, x_max) = find_x_range(x_range.0, x_range.1);
    let (_, y_max) = find_y_range(x_range.1, y_range.0, y_range.1);
    let mut inside_count = 0;
    for x in x_min..x_max + 1 {
        let mut y = y_range.0;
        while y <= y_max {
            if inside(x, y, x_range, y_range) {
                inside_count += 1;
            }
            y += 1;
        }
    }
    inside_count
}
