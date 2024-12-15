use std::cmp::Ordering;

use arrayvec::ArrayVec;
use memchr::memchr;

type Num = u32;
const N_ROBOT: usize = 500;

#[inline(always)]
unsafe fn parse_int(slice: &[u8], mod_: Num) -> Num {
    let mut value = 0 as Num;
    let negative = *slice.get_unchecked(0) == b'-';
    let start = if negative { 1 } else { 0 };
    for &byte in slice.get_unchecked(start..slice.len()) {
        value = value
            .unchecked_mul(10)
            .unchecked_add(byte.unchecked_sub(b'0') as Num);
    }
    if negative {
        mod_ - value
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
        i += 2; // skip "p="
        let p_x_end = i + memchr(b',', &input_bytes[i..]).unwrap_unchecked();
        let p_x = parse_int(&input_bytes[i..p_x_end], width);
        i = p_x_end + 1; // skip ','
        let p_y_end = i + memchr(b' ', &input_bytes[i..]).unwrap_unchecked();
        let p_y = parse_int(&input_bytes[i..p_y_end], height);
        i = p_y_end + 1; // skip ' '

        i += 2; // skip "v="
        let v_x_end = i + memchr(b',', &input_bytes[i..]).unwrap_unchecked();
        let v_x = parse_int(&input_bytes[i..v_x_end], width);
        i = v_x_end + 1; // skip ','
        let v_y_end = i + memchr(b'\n', &input_bytes[i..]).unwrap_unchecked();
        let v_y = parse_int(&input_bytes[i..v_y_end], height);
        i = v_y_end + 1; // skip '\n'
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
        i += 2; // skip "p="
        let p_x_end = i + memchr(b',', &input_bytes[i..]).unwrap_unchecked();
        let p_x = parse_int(&input_bytes[i..p_x_end], WIDTH);
        i = p_x_end + 1; // skip ','
        let p_y_end = i + memchr(b' ', &input_bytes[i..]).unwrap_unchecked();
        let p_y = parse_int(&input_bytes[i..p_y_end], HEIGHT);
        i = p_y_end + 1; // skip ' '

        i += 2; // skip "v="
        let v_x_end = i + memchr(b',', &input_bytes[i..]).unwrap_unchecked();
        let v_x = parse_int(&input_bytes[i..v_x_end], WIDTH);
        i = v_x_end + 1; // skip ','
        let v_y_end = i + memchr(b'\n', &input_bytes[i..]).unwrap_unchecked();
        let v_y = parse_int(&input_bytes[i..v_y_end], HEIGHT);
        i = v_y_end + 1; // skip '\n'

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
        let mut counts = [0; WIDTH as usize];
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
        let mut counts = [0; HEIGHT as usize];
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
        (((INV_MOD as i32).unchecked_mul(
            (most_clustered_y_iteration as i32).unchecked_sub(most_clustered_x_iteration as i32),
        ))
        .wrapping_rem_euclid(HEIGHT as i32) as Num)
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
