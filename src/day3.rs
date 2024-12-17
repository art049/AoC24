use regex::Regex;

pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for caps in Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
    {
        let a = caps[1].parse::<u64>().unwrap();
        let b = caps[2].parse::<u64>().unwrap();
        let result = a * b;
        sum += result;
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let mut start = 0;
    let mut end;
    loop {
        end = input[start..]
            .find("don't()")
            .map(|i| i + start)
            .unwrap_or(input.len());

        sum += part1(&input[start..end]);

        if end >= input.len() {
            break;
        }
        start = input[end..]
            .find("do()")
            .map(|i| i + end)
            .unwrap_or(input.len());

        if start >= input.len() {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = crate::utils::sample_input! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "};
        let expected = 161;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "};
        let expected = 48;
        assert_eq!(part2(&input), expected);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        println!("Part 1: {}", part1(&input));
    }
    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        println!("Part 2: {}", part2(&input));
    }
}
