use std::io::Read;

use aoc::{parse_memory, IntCode, MemIo, Result};

fn main() -> Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let program = parse_memory(&buf)?;
    let res = maximize_thrust(&program);
    println!("{}", res);
    Ok(())
}

fn maximize_thrust(program: &[i64]) -> i64 {
    let mut phases = [0, 1, 2, 3, 4];
    let mut res = i64::min_value();
    let mut mem = vec![0; program.len()];
    permutations(&mut phases, |phases| {
        res = res.max(run(program, &mut mem, phases))
    });
    res
}

fn run(program: &[i64], mem: &mut [i64], phases: &[i64]) -> i64 {
    let mut res = 0;
    for phase in phases.iter().copied() {
        mem.copy_from_slice(program);
        let mut io = MemIo::new(vec![phase, res]);
        IntCode::new(&mut io, mem).run().unwrap();
        let output = io.into_output();
        assert!(output.len() == 1);
        res = output[0];
    }
    res
}

fn permutations<T, F>(xs: &mut [T], mut f: F)
where
    F: FnMut(&[T]),
    T: std::fmt::Debug,
{
    go(xs, &mut f, 0);
}

fn go<T, F>(xs: &mut [T], f: &mut F, i: usize)
where
    F: FnMut(&[T]),
    T: std::fmt::Debug,
{
    if i == xs.len() {
        f(xs);
        return;
    }
    for j in i..xs.len() {
        xs.swap(i, j);
        go(xs, f, i + 1);
    }
    xs[i..].rotate_left(1);
}

#[test]
fn test_examples() {
    assert_eq!(
        maximize_thrust(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
        43210
    );
    assert_eq!(
        maximize_thrust(&[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ]),
        54321
    );
    assert_eq!(
        maximize_thrust(&[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ]),
        65210
    );
}
