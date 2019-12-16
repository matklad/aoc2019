use std::{iter::repeat, mem::swap};

use aoc::{read_stdin_to_string, Result};

fn main() -> Result<()> {
    let input = read_stdin_to_string()?;
    let input = input.trim();
    let res = solve(&input);
    println!("{}", res);
    Ok(())
}

fn solve(input: &str) -> String {
    let mut input = input.bytes().map(|it| it - b'0').collect::<Vec<u8>>();
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
        *o = (sum % 10).abs() as u8;
    }
}

#[test]
fn test_examples() {
    assert_eq!(solve("80871224585914546619083218645595"), "24176176");
    assert_eq!(solve("19617804207202209144916044189917"), "73745418");
    assert_eq!(solve("69317163492948606335995924319873"), "52432133");
}
