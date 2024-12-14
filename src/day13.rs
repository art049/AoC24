pub fn part1(input: &str) -> i64 {
    let mut buttons = Vec::new();
    let input_bytes = input.as_bytes();
    let input_len = input.len();
    let mut i = 0;

    fn parse_int(slice: &[u8]) -> i64 {
        let mut value = 0i64;
        let mut negative = false;
        let mut start = 0;
        if slice[0] == b'-' {
            negative = true;
            start = 1;
        }
        for &byte in &slice[start..] {
            value = value * 10 + (byte - b'0') as i64;
        }
        if negative {
            -value
        } else {
            value
        }
    }

    while i < input_len {
        let (a_x, a_y) = {
            i += "Button A: X+".len();
            let a_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
            let a_x = parse_int(&input_bytes[i..a_x_end]);
            i = a_x_end + ", Y+".len();
            let a_y_end = i + input_bytes[i..].iter().position(|&b| b == b'\n').unwrap();
            let a_y = parse_int(&input_bytes[i..a_y_end]);
            i = a_y_end + 1;
            (a_x, a_y)
        };

        let (b_x, b_y) = {
            i += "Button B: X+".len();
            let b_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
            let b_x = parse_int(&input_bytes[i..b_x_end]);
            i = b_x_end + ", Y+".len();
            let b_y_end = i + input_bytes[i..].iter().position(|&b| b == b'\n').unwrap();
            let b_y = parse_int(&input_bytes[i..b_y_end]);
            i = b_y_end + 1;
            (b_x, b_y)
        };

        let (prize_x, prize_y) = {
            i += "Prize: X=".len();
            let prize_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
            let prize_x = parse_int(&input_bytes[i..prize_x_end]);
            i = prize_x_end + ", Y=".len();
            let prize_y_end = i + input_bytes[i..]
                .iter()
                .position(|&b| b == b'\n')
                .unwrap_or(input_len - i);
            let prize_y = parse_int(&input_bytes[i..prize_y_end]);
            i = prize_y_end + 1;
            (prize_x, prize_y)
        };
        buttons.push(((a_x, a_y), (b_x, b_y), (prize_x, prize_y)));
        i += 1;
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
    let mut buttons = Vec::new();
    let input_bytes = input.as_bytes();
    let input_len = input.len();
    let mut i = 0;

    fn parse_int(slice: &[u8]) -> i64 {
        let mut value = 0i64;
        let mut negative = false;
        let mut start = 0;
        if slice[0] == b'-' {
            negative = true;
            start = 1;
        }
        for &byte in &slice[start..] {
            value = value * 10 + (byte - b'0') as i64;
        }
        if negative {
            -value
        } else {
            value
        }
    }

    while i < input_len {
        let (a_x, a_y) = {
            i += "Button A: X+".len();
            let a_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
            let a_x = parse_int(&input_bytes[i..a_x_end]);
            i = a_x_end + ", Y+".len();
            let a_y_end = i + input_bytes[i..].iter().position(|&b| b == b'\n').unwrap();
            let a_y = parse_int(&input_bytes[i..a_y_end]);
            i = a_y_end + 1;
            (a_x, a_y)
        };

        let (b_x, b_y) = {
            i += "Button B: X+".len();
            let b_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
            let b_x = parse_int(&input_bytes[i..b_x_end]);
            i = b_x_end + ", Y+".len();
            let b_y_end = i + input_bytes[i..].iter().position(|&b| b == b'\n').unwrap();
            let b_y = parse_int(&input_bytes[i..b_y_end]);
            i = b_y_end + 1;
            (b_x, b_y)
        };

        let (prize_x, prize_y) = {
            i += "Prize: X=".len();
            let prize_x_end = i + input_bytes[i..].iter().position(|&b| b == b',').unwrap();
            let prize_x = parse_int(&input_bytes[i..prize_x_end]);
            i = prize_x_end + ", Y=".len();
            let prize_y_end = i + input_bytes[i..]
                .iter()
                .position(|&b| b == b'\n')
                .unwrap_or(input_len - i);
            let prize_y = parse_int(&input_bytes[i..prize_y_end]);
            i = prize_y_end + 1;
            (prize_x, prize_y)
        };

        const PRIZE_D: i64 = 10000000000000;
        let adjusted_prize_x = prize_x + PRIZE_D;
        let adjusted_prize_y = prize_y + PRIZE_D;

        buttons.push(((a_x, a_y), (b_x, b_y), (adjusted_prize_x, adjusted_prize_y)));
        i += 1;
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
