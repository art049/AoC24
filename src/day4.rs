use std::simd::{cmp::SimdPartialEq, u32x8};

pub fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let grid_height = lines.len();
    let grid_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Map characters to integers: '.' -> 0, 'X' -> 1, 'M' -> 2, 'A' -> 3, 'S' -> 4
    let mut grid = vec![vec![0u8; grid_width + 3]; grid_height + 3]; // Pad grid to avoid bounds checking
    let char_to_int = |c| match c {
        'X' => 1,
        'M' => 2,
        'A' => 3,
        'S' => 4,
        _ => 0,
    };

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = char_to_int(c);
        }
    }

    let mut xmas_count = 0;
    let pattern = (1u32 << 24) | (2u32 << 16) | (3u32 << 8) | 4u32; // 'XMAS'
    let rev_pattern = (4u32 << 24) | (3u32 << 16) | (2u32 << 8) | 1u32; // 'SAMX'

    // Right Direction
    for y in 0..grid_height {
        let row = &grid[y];
        for x in 0..=grid_width - 4 {
            let vals = (row[x] as u32) << 24
                | (row[x + 1] as u32) << 16
                | (row[x + 2] as u32) << 8
                | (row[x + 3] as u32);
            if vals == pattern || vals == rev_pattern {
                xmas_count += 1;
            }
        }
    }

    // Down Direction
    for x in 0..grid_width {
        for y in 0..=grid_height - 4 {
            let vals = (grid[y][x] as u32) << 24
                | (grid[y + 1][x] as u32) << 16
                | (grid[y + 2][x] as u32) << 8
                | (grid[y + 3][x] as u32);
            if vals == pattern || vals == rev_pattern {
                xmas_count += 1;
            }
        }
    }

    // Down-Right Direction
    for y in 0..=grid_height - 4 {
        for x in 0..=grid_width - 4 {
            let vals = (grid[y][x] as u32) << 24
                | (grid[y + 1][x + 1] as u32) << 16
                | (grid[y + 2][x + 2] as u32) << 8
                | (grid[y + 3][x + 3] as u32);
            if vals == pattern || vals == rev_pattern {
                xmas_count += 1;
            }
        }
    }

    // Down-Left Direction
    for y in 0..=grid_height - 4 {
        for x in 3..grid_width {
            let vals = (grid[y][x] as u32) << 24
                | (grid[y + 1][x - 1] as u32) << 16
                | (grid[y + 2][x - 2] as u32) << 8
                | (grid[y + 3][x - 3] as u32);
            if vals == pattern || vals == rev_pattern {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

pub fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let grid_height = lines.len();
    let grid_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut grid = vec![vec!['.'; grid_width]; grid_height];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'M' || c == 'A' || c == 'S' {
                grid[y][x] = c;
            }
        }
    }

    let mut xmas_count = 0;
    let patterns = [
        [('M', -1, -1), ('M', 1, -1), ('S', -1, 1), ('S', 1, 1)],
        [('S', -1, -1), ('S', 1, -1), ('M', -1, 1), ('M', 1, 1)],
        [('S', -1, -1), ('M', 1, -1), ('S', -1, 1), ('M', 1, 1)],
        [('M', -1, -1), ('S', 1, -1), ('M', -1, 1), ('S', 1, 1)],
    ];

    for y in 1..grid_height - 1 {
        for x in 1..grid_width - 1 {
            if grid[y][x] == 'A' {
                for pattern in patterns.iter() {
                    let mut matched = true;
                    for &(expected_c, dx, dy) in pattern.iter() {
                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;
                        if grid[ny][nx] != expected_c {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        xmas_count += 1;
                        break;
                    }
                }
            }
        }
    }

    xmas_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = crate::utils::sample_input! {"
        ..X...
        .SAMX.
        .A..A.
        XMAS.S
        .X....
        "};
        let expected = 4;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part1_1() {
        let input = crate::utils::sample_input! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "};
        let expected = 18;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        assert_eq!(output, 2591);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
          M.S
          .A.
          M.S
        "};
        let expected = 1;
        assert_eq!(part2(&input), expected);
    }

    #[test]
    fn test_part2_1() {
        let input = crate::utils::sample_input! {"
          .M.S......
          ..A..MSMS.
          .M.S.MAA..
          ..A.ASMSM.
          .M.S.M....
          ..........
          S.S.S.S.S.
          .A.A.A.A..
          M.M.M.M.M.
          ..........
        "};
        let expected = 9;
        assert_eq!(part2(&input), expected);
    }
    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        assert_eq!(output, 1880);
    }
}
