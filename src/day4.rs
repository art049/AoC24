pub fn part1(input: &str) -> u64 {
    const GRID_SIZE: usize = 140;
    const DIRECTIONS: &[(isize, isize)] = &[
        (1, 0),  // Right
        (0, 1),  // Down
        (1, 1),  // Down-Right
        (-1, 1), // Down-Left
    ];
    const PATTERN: [u8; 4] = [b'X', b'M', b'A', b'S'];
    const PATTERN_REVERSED: [u8; 4] = [b'S', b'A', b'M', b'X'];

    // Fixed-size grid
    let mut grid = [b'.'; GRID_SIZE * GRID_SIZE];
    let mut grid_width = 0;
    let mut grid_height = 0;

    // Populate grid (assumes input lines are less than GRID_SIZE)
    for (y, line) in input.lines().enumerate() {
        grid_height = y + 1;
        for (x, &byte) in line.as_bytes().iter().enumerate() {
            grid[y * GRID_SIZE + x] = byte;
            grid_width = grid_width.max(x + 1);
        }
    }

    let mut xmas_count = 0;

    // Main processing loop
    for y in 0..grid_height {
        for x in 0..grid_width {
            let idx = y * GRID_SIZE + x;
            let cell = grid[idx];
            if cell == b'X' || cell == b'S' {
                let is_reversed = cell == b'S';
                let pattern = if is_reversed {
                    PATTERN_REVERSED
                } else {
                    PATTERN
                };

                for &(dx, dy) in DIRECTIONS.iter() {
                    let mut matched = true;

                    for i in 0..4 {
                        let nx = x as isize + dx * i;
                        let ny = y as isize + dy * i;
                        if nx < 0
                            || ny < 0
                            || nx >= grid_width as isize
                            || ny >= grid_height as isize
                        {
                            matched = false;
                            break;
                        }
                        let nidx = (ny as usize) * GRID_SIZE + nx as usize;
                        if grid[nidx] != pattern[i as usize] {
                            matched = false;
                            break;
                        }
                    }

                    if matched {
                        xmas_count += 1;
                    }
                }
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
