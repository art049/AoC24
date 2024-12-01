use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    let mut col_one = Vec::with_capacity(1000);
    let mut col_two = Vec::with_capacity(1000);
    input.lines().for_each(|l| {
        let mut split = l.split_ascii_whitespace();
        col_one.push(split.next().unwrap().parse::<u64>().unwrap());
        col_two.push(split.next().unwrap().parse::<u64>().unwrap());
    });
    col_one.sort();
    col_two.sort();
    let mut sum = 0;
    for i in 0..col_one.len() {
        sum += col_one[i].abs_diff(col_two[i]);
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut col_one = Vec::with_capacity(1000);
    let mut count = HashMap::with_capacity(1000);
    input.lines().for_each(|l| {
        let mut split = l.split_ascii_whitespace();
        col_one.push(split.next().unwrap().parse::<u64>().unwrap());
        count
            .entry(split.next().unwrap().parse::<u64>().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });
    col_one.sort();
    let mut sum = 0;
    for i in 0..col_one.len() {
        sum += col_one[i] * count.get(&col_one[i]).unwrap_or(&0);
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_day_input;

    #[test]
    fn test_part1() {
        let input = crate::utils::sample_input! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        let expected = 11;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
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
