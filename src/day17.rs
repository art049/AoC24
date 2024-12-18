use core::str;
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
};

use memchr::memchr;

type Num = u32;

#[inline(always)]
unsafe fn parse_int(slice: &[u8], mod_: Num) -> Num {
    let mut value = 0 as Num;
    let negative = *slice.get_unchecked(0) == b'-';
    let start = if negative { 1 } else { 0 };
    for &byte in slice.get_unchecked(start..slice.len()) {
        value = value
            .unchecked_mul(10)
            .unchecked_add(byte.unchecked_sub(b'0') as Num);
    }
    if negative {
        mod_ - value
    } else {
        value
    }
}

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

pub fn part1(input: &str) -> String {
    unsafe { part1_inner(input) }
}

unsafe fn part1_inner(input: &str) -> String {
    let input_bytes = input.as_bytes();
    let input_len = input.len();
    let mut i = 0;
    i += "Register A: ".len();
    let a_end = i + memchr(b'\n', &input_bytes[i..]).unwrap_unchecked();
    let mut reg_a: u32 = unsafe { parse_int(&input_bytes[i..a_end], 0) };
    i = a_end + 1; // skip '\n'
    i += "Register B: ".len();
    let b_end = i + memchr(b'\n', &input_bytes[i..]).unwrap_unchecked();
    let mut reg_b = unsafe { parse_int(&input_bytes[i..b_end], 0) };
    i = b_end + 1; // skip '\n'
    i += "Register C: ".len();
    let c_end = i + memchr(b'\n', &input_bytes[i..]).unwrap_unchecked();
    let mut reg_c = unsafe { parse_int(&input_bytes[i..c_end], 0) };
    i = c_end + 2; // skip '\n\n'
    i += "Program: ".len();

    let program_start = i;
    let mut out = Vec::<u8>::with_capacity(256);
    while i < input_len {
        let opcode = *input_bytes.get_unchecked(i) - b'0';
        let operand = *input_bytes.get_unchecked(i + 2) - b'0';
        let combo_operand = match operand {
            0..=3 => operand as Num,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => {
                panic!("Invalid operand value")
            }
        };
        i += 4;
        dbg!(reg_a, reg_b, reg_c);
        dbg!(opcode, operand, combo_operand);

        match opcode {
            ADV => reg_a = reg_a / (2 as Num).pow(combo_operand),
            BXL => reg_b = reg_b ^ operand as Num,
            BST => reg_b = combo_operand % 8,
            JNZ => {
                if reg_a != 0 {
                    i = (program_start + operand as usize * 4) as usize;
                }
            }
            BXC => reg_b = reg_b ^ reg_c,
            OUT => {
                if !out.is_empty() {
                    out.push(b',')
                }
                out.push(b'0' + (combo_operand % 8) as u8);
            }
            BDV => reg_b = reg_a / (2 as Num).pow(combo_operand),
            CDV => reg_c = reg_a / (2 as Num).pow(combo_operand),
            _ => panic!("Invalid opcode"),
        };
    }
    out.as_ascii_unchecked().as_str().to_owned()
}

pub fn part2(input: &str) -> Num {
    unsafe { part2_inner(input) }
}

unsafe fn part2_inner(input: &str) -> Num {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = crate::utils::sample_input! {"
          Register A: 729
          Register B: 0
          Register C: 0

          Program: 0,1,5,4,3,0
        "};
        let expected = "4,6,3,5,6,3,5,2,1,0";
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, "7,1,5,2,4,0,7,6,1");
    }

    #[test]
    fn test_part2_0() {
        let input = crate::utils::sample_input! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
        "};
        let expected = 117440;
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
