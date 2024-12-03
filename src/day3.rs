use std::arch::x86_64::*;

unsafe fn find_pattern_simd(haystack: &[u8], start: usize, pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() || haystack.len() < start + pattern.len() {
        return None;
    }

    let pattern_len = pattern.len();
    let haystack_len = haystack.len();
    let first_byte = _mm_set1_epi8(pattern[0] as i8);

    let mut i = start;

    while i + 16 <= haystack_len {
        let chunk = _mm_loadu_si128(haystack.as_ptr().add(i) as *const __m128i);

        // Compare first byte
        let matches = _mm_cmpeq_epi8(chunk, first_byte);
        let mask = _mm_movemask_epi8(matches);

        // Process all matches in this chunk
        if mask != 0 {
            let mut match_mask = mask;
            while match_mask != 0 {
                let offset = match_mask.trailing_zeros() as usize;
                let pos = i + offset;

                // Check full pattern match
                if pos + pattern_len <= haystack_len
                    && haystack[pos..pos + pattern_len] == pattern[..]
                {
                    return Some(pos);
                }

                // Clear the matched bit
                match_mask &= match_mask - 1;
            }
        }

        i += 16;
    }

    // Process the tail using smaller SIMD chunks or scalar
    while i + pattern_len <= haystack_len {
        if &haystack[i..i + pattern_len] == pattern {
            return Some(i);
        }
        i += 1;
    }

    None
}

pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut i = 0;

    let mul_pattern = b"mul(";

    unsafe {
        while i < bytes.len() {
            // Find "mul(" using SIMD
            i = find_pattern_simd(bytes, i, mul_pattern).unwrap_or(bytes.len());

            if i == bytes.len() {
                break;
            }

            i += mul_pattern.len(); // Move past "mul("

            // Parse the first number
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

                // Parse the second number
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
        }
    }

    sum
}

pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut start = 0;

    let dont_pattern = b"don't()";
    let do_pattern = b"do()";

    unsafe {
        while start < bytes.len() {
            // Find "don't()" using SIMD
            let end = find_pattern_simd(bytes, start, dont_pattern).unwrap_or(bytes.len());

            // Sum the part before "don't()"
            sum += part1(std::str::from_utf8_unchecked(&bytes[start..end]));

            // If no more "don't()", we are done
            if end == bytes.len() {
                break;
            }

            // Find "do()" using SIMD
            start = find_pattern_simd(bytes, end + dont_pattern.len(), do_pattern)
                .map(|i| i + do_pattern.len())
                .unwrap_or(bytes.len());

            // If no more "do()", we are done
            if start >= bytes.len() {
                break;
            }
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
    fn test_find_pattern_simd_basic() {
        unsafe {
            let haystack = b"abcdefg don't() some text do() and more";
            assert_eq!(find_pattern_simd(haystack, 0, b"don't()"), Some(8));
            assert_eq!(find_pattern_simd(haystack, 0, b"do()"), Some(26));
        }
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
