pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes.get(i..i + 4) == Some(b"mul(") {
            i += 4;
            let mut a = 0;
            while let Some(&b) = bytes.get(i) {
                if b.is_ascii_digit() {
                    a = a * 10 + (b - b'0') as u64;
                    i += 1;
                } else {
                    break;
                }
            }
            if bytes.get(i) == Some(&b',') {
                i += 1;
                let mut b = 0;
                while let Some(&c) = bytes.get(i) {
                    if c.is_ascii_digit() {
                        b = b * 10 + (c - b'0') as u64;
                        i += 1;
                    } else {
                        break;
                    }
                }
                if bytes.get(i) == Some(&b')') {
                    i += 1;
                    sum += a * b;
                }
            }
        } else {
            i += 1;
        }
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut start = 0;

    loop {
        // Find the next occurrence of "don't()"
        let mut end = bytes.len();
        for i in start..bytes.len() - 6 {
            if &bytes[i..i + 7] == b"don't()" {
                end = i;
                break;
            }
        }

        // Sum the part before "don't()"
        sum += part1(unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) });

        // If no more "don't()", we are done
        if end == bytes.len() {
            break;
        }

        // Find the next occurrence of "do()"
        start = bytes.len();
        for i in end..bytes.len() - 3 {
            if &bytes[i..i + 4] == b"do()" {
                start = i + 4; // Move past "do()"
                break;
            }
        }

        // If no more "do()", we are done
        if start >= bytes.len() {
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
