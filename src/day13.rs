pub fn part1(input: &str) -> i64 {
    let mut buttons = vec![];
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    while let Some(line_a) = lines.next() {
        let line_b = lines.next().unwrap();
        let line_prize = lines.next().unwrap();
        let a_values: Vec<_> = line_a
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|part| part.trim())
            .collect();
        let a_x: i64 = a_values[0][2..].parse().unwrap();
        let a_y: i64 = a_values[1][2..].parse().unwrap();
        let b_values: Vec<_> = line_b
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|part| part.trim())
            .collect();
        let b_x: i64 = b_values[0][2..].parse().unwrap();
        let b_y: i64 = b_values[1][2..].parse().unwrap();
        let prize_values: Vec<_> = line_prize
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|part| part.trim())
            .collect();
        let prize_x: i64 = prize_values[0][2..].parse().unwrap();
        let prize_y: i64 = prize_values[1][2..].parse().unwrap();

        buttons.push(((a_x, a_y), (b_x, b_y), (prize_x, prize_y)));
    }
    let mut spent = 0;
    for ((a_x, a_y), (b_x, b_y), (prize_x, prize_y)) in buttons {
        let det = (a_x * b_y) - (b_x * a_y);
        let n_a_n = prize_x * b_y - prize_y * b_x;
        let n_b_n = a_x * prize_y - a_y * prize_x;
        if det == 0 || n_a_n % det != 0 || n_b_n % det != 0 {
            continue;
        }
        let n_a = n_a_n / det;
        let n_b = n_b_n / det;
        if n_a > 100 || n_b > 100 {
            continue;
        }
        spent += 3 * n_a + n_b;
    }

    spent
}

pub fn part2(input: &str) -> i64 {
    let mut buttons = vec![];
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    while let Some(line_a) = lines.next() {
        let line_b = lines.next().unwrap();
        let line_prize = lines.next().unwrap();
        let a_values: Vec<_> = line_a
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|part| part.trim())
            .collect();
        let a_x: i64 = a_values[0][2..].parse().unwrap();
        let a_y: i64 = a_values[1][2..].parse().unwrap();
        let b_values: Vec<_> = line_b
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|part| part.trim())
            .collect();
        let b_x: i64 = b_values[0][2..].parse().unwrap();
        let b_y: i64 = b_values[1][2..].parse().unwrap();
        let prize_values: Vec<_> = line_prize
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|part| part.trim())
            .collect();
        const PRIZE_D: i64 = 10000000000000;
        let prize_x: i64 = prize_values[0][2..].parse::<i64>().unwrap() + PRIZE_D;
        let prize_y: i64 = prize_values[1][2..].parse::<i64>().unwrap() + PRIZE_D;

        buttons.push(((a_x, a_y), (b_x, b_y), (prize_x, prize_y)));
    }
    let mut spent = 0;
    for ((a_x, a_y), (b_x, b_y), (prize_x, prize_y)) in buttons {
        let det = (a_x * b_y) - (b_x * a_y);
        let n_a_n = prize_x * b_y - prize_y * b_x;
        let n_b_n = a_x * prize_y - a_y * prize_x;
        if det == 0 || n_a_n % det != 0 || n_b_n % det != 0 {
            continue;
        }
        let n_a = n_a_n / det;
        let n_b = n_b_n / det;
        spent += 3 * n_a + n_b;
    }

    spent
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = crate::utils::sample_input! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "};
        let expected = 200 + 280;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 30973);
    }

    #[test]
    fn test_part2_0() {
        let input = crate::utils::sample_input! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "};
        let expected = 875318608908;
        assert_eq!(part2(&input), expected);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 95688837203288);
    }
}
