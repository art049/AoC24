struct Pattern {
    points: [(isize, isize); 4],
}

const PATTERNS: [Pattern; 4] = [
    // XMAS
    Pattern {
        // XMAS horizontal
        points: [(0, 0), (0, 1), (0, 2), (0, 3)],
    },
    Pattern {
        // XMAS vertical
        points: [(0, 0), (1, 0), (2, 0), (3, 0)],
    },
    Pattern {
        // XMAS diagonal to bottom right
        points: [(0, 0), (1, 1), (2, 2), (3, 3)],
    },
    Pattern {
        // XMAS diagonal to bottom up
        points: [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
    },
];
const GRID_SIZE: usize = 140;
pub fn part1(input: &str) -> u64 {
    let mut grid = ['.'; GRID_SIZE * GRID_SIZE];
    let mut start_pos = [None; GRID_SIZE * GRID_SIZE];
    let mut start_pos_iter = start_pos.iter_mut();

    input.lines().enumerate().for_each(|(y, line)| {
        let line_offset = y * GRID_SIZE;
        line.chars().enumerate().for_each(|(x, c)| {
            let i = line_offset + x;
            if c == 'X' || c == 'M' || c == 'A' || c == 'S' {
                grid[i] = c;
            }
            if c == 'X' || c == 'S' {
                start_pos_iter.next().unwrap().replace((x, y, c));
            }
        });
    });
    let mut start_pos_iter = start_pos.iter();
    let mut xmas_count = 0;
    while let Some(Some((x, y, c))) = start_pos_iter.next() {
        let is_reversed = *c == 'S';
        for pattern in PATTERNS.iter() {
            let mut found = true;
            for (i, (dx, dy)) in pattern.points.iter().enumerate() {
                let x = x.checked_add_signed(*dx);
                let y = y.checked_add_signed(*dy);
                let (Some(x), Some(y)) = (x, y) else {
                    found = false;
                    break;
                };
                if x >= GRID_SIZE || y >= GRID_SIZE {
                    found = false;
                    break;
                }
                let expected_letter = ['X', 'M', 'A', 'S'][if is_reversed { 3 - i } else { i }];
                if grid[y * GRID_SIZE + x] != expected_letter {
                    found = false;
                    break;
                }
            }
            if found {
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

macro_rules! check {
    ($grid:ident, $x:expr, $y:expr, $c:expr) => {
        $grid[$y * GRID_SIZE + $x] == $c
    };
}
pub fn part2(input: &str) -> u64 {
    let mut grid = ['.'; GRID_SIZE * GRID_SIZE];
    let mut start_pos = [None; GRID_SIZE * GRID_SIZE];
    let mut start_pos_iter = start_pos.iter_mut();

    input.lines().enumerate().for_each(|(y, line)| {
        let line_offset = y * GRID_SIZE;
        line.chars().enumerate().for_each(|(x, c)| {
            let i = line_offset + x;
            if c == 'M' || c == 'A' || c == 'S' {
                grid[i] = c;
            }
            if c == 'A' && x > 0 && y > 0 && x < GRID_SIZE - 1 && y < GRID_SIZE - 1 {
                start_pos_iter.next().unwrap().replace((x, y, c));
            }
        });
    });
    let mut start_pos_iter = start_pos.iter();
    let mut xmas_count = 0;
    while let Some(Some((x, y, c))) = start_pos_iter.next() {
        if (check!(grid, x - 1, y - 1, 'M')
            && check!(grid, x + 1, y - 1, 'M')
            && check!(grid, x - 1, y + 1, 'S')
            && check!(grid, x + 1, y + 1, 'S'))
            || (check!(grid, x - 1, y - 1, 'S')
                && check!(grid, x + 1, y - 1, 'S')
                && check!(grid, x - 1, y + 1, 'M')
                && check!(grid, x + 1, y + 1, 'M'))
            || (check!(grid, x - 1, y - 1, 'S')
                && check!(grid, x + 1, y - 1, 'M')
                && check!(grid, x - 1, y + 1, 'S')
                && check!(grid, x + 1, y + 1, 'M'))
            || (check!(grid, x - 1, y - 1, 'M')
                && check!(grid, x + 1, y - 1, 'S')
                && check!(grid, x - 1, y + 1, 'M')
                && check!(grid, x + 1, y + 1, 'S'))
        {
            xmas_count += 1;
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
