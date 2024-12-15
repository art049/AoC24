use std::cmp::Ordering;

use arrayvec::ArrayVec;

type Num = i32;
const N_ROBOT: usize = 500;

#[inline(always)]
unsafe fn parse_int(slice: &[u8]) -> Num {
    let mut value = 0 as Num;
    let mut negative = false;
    let mut start = 0;
    if slice[0] == b'-' {
        negative = true;
        start = 1;
    }
    for &byte in &slice[start..] {
        value = value
            .unchecked_mul(10)
            .unchecked_add(byte.unchecked_sub(b'0') as Num);
    }
    if negative {
        -value
    } else {
        value
    }
}

pub fn part1(input: &str) -> Num {
    const WIDTH: Num = 101;
    const HEIGHT: Num = 103;
    unsafe { part1_inner(input, WIDTH, HEIGHT) }
}

#[inline(always)]
unsafe fn part1_inner(input: &str, width: Num, height: Num) -> Num {
    let input_bytes = input.as_bytes();
    let input_len = input.len();
    let mut robot_by_quadrant = [0 as Num; 4];
    let mut i = 0;
    while i < input_len {
        i += "p=".len();
        let p_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
        let p_x = parse_int(&input_bytes[i..p_x_end]);
        i = p_x_end + ",".len();
        let p_y_end = i + input_bytes[i..].iter().position(|&b| b == b' ').unwrap();
        let p_y = parse_int(&input_bytes[i..p_y_end]);
        i = p_y_end + 1;

        i += "v=".len();
        let v_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
        let v_x = parse_int(&input_bytes[i..v_x_end]);
        i = v_x_end + ",".len();
        let v_y_end = i + input_bytes[i..].iter().position(|&b| b == b'\n').unwrap();
        let v_y = parse_int(&input_bytes[i..v_y_end]);
        i = v_y_end + 1;
        const STEPS: Num = 100;
        let final_pos_x = (p_x + v_x * STEPS).wrapping_rem_euclid(width);
        let final_pos_y = (p_y + v_y * STEPS).wrapping_rem_euclid(height);
        match (
            final_pos_x.cmp(&(width / 2)),
            final_pos_y.cmp(&(height / 2)),
        ) {
            (Ordering::Less, Ordering::Less) => robot_by_quadrant[0] += 1,
            (Ordering::Less, Ordering::Greater) => robot_by_quadrant[1] += 1,
            (Ordering::Greater, Ordering::Less) => robot_by_quadrant[2] += 1,
            (Ordering::Greater, Ordering::Greater) => robot_by_quadrant[3] += 1,
            _ => {}
        };
    }
    robot_by_quadrant.into_iter().reduce(|a, b| a * b).unwrap()
}

struct Robot {
    x: Num,
    y: Num,
    v_x: Num,
    v_y: Num,
}

fn print_robots(robots: &[Robot], width: Num, height: Num) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }
    for row in grid {
        println!("{}", row.into_iter().collect::<String>());
    }
}

pub fn part2(input: &str) -> Num {
    unsafe { part2_inner(input) }
}

#[inline(always)]
unsafe fn part2_inner(input: &str) -> Num {
    const WIDTH: Num = 101;
    const HEIGHT: Num = 103;
    const INV_MOD: Num = 51; // mod_inverse(WIDTH, HEIGHT);
    let input_bytes = input.as_bytes();
    let input_len = input.len();
    let mut i = 0;
    let mut robots = ArrayVec::<_, N_ROBOT>::new();
    while i < input_len {
        i += "p=".len();
        let p_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
        let p_x = parse_int(&input_bytes[i..p_x_end]);
        i = p_x_end + ",".len();
        let p_y_end = i + input_bytes[i..].iter().position(|&b| b == b' ').unwrap();
        let p_y = parse_int(&input_bytes[i..p_y_end]);
        i = p_y_end + 1;

        i += "v=".len();
        let v_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
        let v_x = parse_int(&input_bytes[i..v_x_end]);
        i = v_x_end + ",".len();
        let v_y_end = i + input_bytes[i..].iter().position(|&b| b == b'\n').unwrap();
        let v_y = parse_int(&input_bytes[i..v_y_end]);
        i = v_y_end + 1;
        robots.push_unchecked(Robot {
            x: p_x,
            y: p_y,
            v_x,
            v_y,
        });
    }
    // Compute X clustering cycle
    let mut biggest_cluster = 0;
    let mut most_clustered_x_iteration = 0;
    let mut positions = ArrayVec::<_, N_ROBOT>::new();
    for robot in &robots {
        positions.push_unchecked(robot.x);
    }
    let mut i = 0;
    loop {
        let mut counts = vec![0; WIDTH as usize];
        for &pos in &positions {
            counts[pos as usize] += 1;
        }
        let max_count = counts.iter().max().unwrap_unchecked();
        if max_count > &biggest_cluster {
            biggest_cluster = *max_count;
            most_clustered_x_iteration = i;
        }
        i += 1;
        if i >= WIDTH {
            break;
        }
        for j in 0..positions.len() {
            positions[j] = positions[j]
                .wrapping_add(robots[j].v_x)
                .wrapping_rem_euclid(WIDTH);
        }
    }
    // Compute Y clustering cycle
    let mut positions = ArrayVec::<_, N_ROBOT>::new();
    for robot in &robots {
        positions.push_unchecked(robot.y);
    }
    let mut biggest_cluster = 0;
    let mut most_clustered_y_iteration = 0;
    let mut i = 0;
    loop {
        let mut counts = vec![0; HEIGHT as usize];
        for &pos in &positions {
            counts[pos as usize] += 1;
        }
        let max_count = counts.iter().max().unwrap_unchecked();
        if max_count > &biggest_cluster {
            biggest_cluster = *max_count;
            most_clustered_y_iteration = i;
        }
        i += 1;
        if i >= HEIGHT {
            break;
        }
        for j in 0..positions.len() {
            positions[j] = positions[j]
                .wrapping_add(robots[j].v_y)
                .wrapping_rem_euclid(HEIGHT);
        }
    }
    most_clustered_x_iteration.unchecked_add(
        ((INV_MOD
            .unchecked_mul(most_clustered_y_iteration.unchecked_sub(most_clustered_x_iteration)))
        .wrapping_rem_euclid(HEIGHT))
        .unchecked_mul(WIDTH),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = crate::utils::sample_input! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        let expected = 12;
        assert_eq!(unsafe { part1_inner(input, 11, 7) }, expected);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 230900224);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 6532);
    }
}
