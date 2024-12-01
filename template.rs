pub fn part1(input: &str) -> u64 {
    todo!()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_day_input;

    #[test]
    fn test_part1() {
        let input = crate::utils::sample_input! {"
        "};
        let expected = 11;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
        "};
        let expected = 31;
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
