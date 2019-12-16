use std::{iter::repeat, mem::swap};

use aoc::{read_stdin_to_string, Result};

fn main() -> Result<()> {
    let input = read_stdin_to_string()?;
    let input = input.trim();
    eprintln!("{}", solve2(input));
    Ok(())
}

fn to_digits(input: &str) -> Vec<u8> {
    input.bytes().map(|it| it - b'0').collect::<Vec<u8>>()
}

fn solve(input: &str) -> String {
    let mut input = to_digits(input);
    let patterns = build_patterns(input.len());
    let mut output = vec![0; input.len()];
    for _ in 0..100 {
        round(&patterns, &input, &mut output);
        swap(&mut input, &mut output);
    }
    input
        .iter()
        .take(8)
        .map(|byte| char::from(byte + b'0'))
        .collect()
}

fn solve2(input: &str) -> String {
    let offset = input[..7].parse::<usize>().unwrap();
    assert!(offset > input.len() * 5000);

    let half = repeat_n(input, 5000).collect::<String>();

    let mut half = to_digits(&half);

    for _ in 0..100 {
        half_round(half.as_mut_slice());
    }
    half[offset - half.len()..][..8]
        .iter()
        .map(|byte| char::from(byte + b'0'))
        .collect()
}

fn repeat_n<T: Clone>(value: T, n: usize) -> impl Iterator<Item = T> + Clone {
    repeat(value).take(n)
}

fn build_patterns(len: usize) -> Vec<Vec<i8>> {
    (0..len)
        .map(|i| {
            let r = |value: i8| repeat_n(value, i + 1);
            r(0).chain(r(1))
                .chain(r(0))
                .chain(r(-1))
                .cycle()
                .skip(1)
                .take(len)
                .collect()
        })
        .collect()
}

fn round(patterns: &[Vec<i8>], input: &[u8], output: &mut [u8]) {
    assert!(input.len() == output.len() && output.len() == patterns.len());
    for (o, pattern) in output.iter_mut().zip(patterns.iter()) {
        let sum = input
            .iter()
            .copied()
            .zip(pattern.iter().copied())
            .map(|(a, b)| a as i64 * b as i64)
            .sum::<i64>();
        *o = last_digit(sum);
    }
}

fn last_digit(n: i64) -> u8 {
    (n % 10).abs() as u8
}

fn half_round(buf: &mut [u8]) {
    let mut total = buf.iter().map(|it| *it as i64).sum::<i64>();
    for digit in buf.iter_mut() {
        let d = *digit;
        *digit = last_digit(total);
        total -= d as i64
    }
}

#[test]
fn test_examples() {
    assert_eq!(solve("80871224585914546619083218645595"), "24176176");
    assert_eq!(solve("19617804207202209144916044189917"), "73745418");
    assert_eq!(solve("69317163492948606335995924319873"), "52432133");

    assert_eq!(solve2("03036732577212944063491565474664"), "84462026");
    assert_eq!(solve2("02935109699940807407585447034323"), "78725270");
    assert_eq!(solve2("03081770884921959731165446850517"), "53553731");
}
