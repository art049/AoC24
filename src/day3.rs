use std::{arch::x86_64::*, sync::LazyLock};

fn build_lut(pattern: &[u8]) -> [u8; 256] {
    let mut lut = [0u8; 256];
    for &byte in pattern {
        lut[byte as usize] = 1;
    }
    lut
}

unsafe fn find_pattern_simd_with_lut(
    haystack: &[u8],
    start: usize,
    pattern: &[u8],
    lut: &[u8; 256],
) -> Option<usize> {
    if pattern.is_empty() || haystack.len() < start + pattern.len() {
        return None;
    }

    let pattern_len = pattern.len();
    let haystack_len = haystack.len();

    let mut i = start;

    while i + 16 <= haystack_len {
        let chunk = _mm_loadu_si128(haystack.as_ptr().add(i) as *const __m128i);

        // Check for bytes that exist in the pattern
        let mut mask = 0u16;
        for j in 0..16 {
            let byte = *haystack.get_unchecked(i + j);
            if lut[byte as usize] != 0 {
                mask |= 1 << j;
            }
        }

        // Check each possible match directly
        for offset in 0..16 {
            if (mask & (1 << offset)) != 0 {
                let pos = i + offset;
                if pos + pattern_len <= haystack_len
                    && haystack[pos..pos + pattern_len] == pattern[..]
                {
                    return Some(pos);
                }
            }
        }

        i += 16;
    }

    // Process the tail
    while i + pattern_len <= haystack_len {
        if &haystack[i..i + pattern_len] == pattern {
            return Some(i);
        }
        i += 1;
    }

    None
}

const MUL_PATTERN: &[u8] = b"mul(";
static MUL_LUT: LazyLock<[u8; 256]> = LazyLock::new(|| build_lut(MUL_PATTERN));

pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut i = 0;

    unsafe {
        while i < bytes.len() {
            // Find "mul(" using SIMD
            i = find_pattern_simd_with_lut(bytes, i, &MUL_PATTERN, &MUL_LUT).unwrap_or(bytes.len());

            if i == bytes.len() {
                break;
            }

            i += MUL_PATTERN.len(); // Move past "mul("

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

const DONT_PATTERN: &[u8] = b"don't()";
const DO_PATTERN: &[u8] = b"do()";
static DONT_LUT: LazyLock<[u8; 256]> = LazyLock::new(|| build_lut(DONT_PATTERN));
static DO_LUT: LazyLock<[u8; 256]> = LazyLock::new(|| build_lut(DO_PATTERN));

pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut start = 0;

    unsafe {
        while start < bytes.len() {
            // Find "don't()" using SIMD
            let end = find_pattern_simd_with_lut(bytes, start, DONT_PATTERN, &DONT_LUT)
                .unwrap_or(bytes.len());

            // Sum the part before "don't()"
            sum += part1(std::str::from_utf8_unchecked(&bytes[start..end]));

            // If no more "don't()", we are done
            if end == bytes.len() {
                break;
            }

            // Find "do()" using SIMD
            start =
                find_pattern_simd_with_lut(bytes, end + DONT_PATTERN.len(), DO_PATTERN, &DO_LUT)
                    .map(|i| i + DO_PATTERN.len())
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
            assert_eq!(
                find_pattern_simd_with_lut(haystack, 0, DONT_PATTERN, &DONT_LUT),
                Some(8)
            );
            assert_eq!(
                find_pattern_simd_with_lut(haystack, 0, DO_PATTERN, &DO_LUT),
                Some(26)
            );
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
