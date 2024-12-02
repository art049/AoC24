fn is_safe(levels: &[i32]) -> bool {
    let mut iter = levels.iter();
    let mut next = iter.next().unwrap();
    let mut current;
    let mut sig = 0;
    for level in iter {
        current = next;
        next = level;
        let d = next - current;
        if d.abs() > 3 || d.abs() == 0 {
            return false;
        }
        if sig == 0 {
            sig = d.signum();
        } else if sig != d.signum() {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>();
            is_safe(&levels)
        })
        .count()
}

fn is_safe_with_skip(levels: &[i32], skip_index: usize) -> bool {
    let filtered_levels: Vec<i32> = levels
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != skip_index)
        .map(|(_, &level)| level)
        .collect();

    if filtered_levels.len() < 2 {
        return true; // A report with less than two levels is considered safe
    }

    is_safe(&filtered_levels)
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (0..levels.len()).any(|skip_index| is_safe_with_skip(&levels, skip_index))
        })
        .count()
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::get_day_input;

    #[test]
    fn test_part1() {
        let input = crate::utils::sample_input! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "};
        let expected = 2;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "};
        let expected = 4;
        assert_eq!(part2(&input), expected);
    }

    #[test]
    fn run_part1() {
        let input = get_day_input!();
        println!("Part 1: {}", part1(&input));
    }
    #[test]
    fn run_part2() {
        let input = get_day_input!();
        println!("Part 2: {}", part2(&input));
    }
}
