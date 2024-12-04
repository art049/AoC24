pub fn part1(input: &str) -> u64 {
    todo!()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = crate::utils::sample_input! {"
        "};
        let expected = 0;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_part2_0() {
        let input = crate::utils::sample_input! {"
        "};
        let expected = 0;
        assert_eq!(part2(&input), expected);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 0);
    }
}
