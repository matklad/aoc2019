use std::io::Read;

use aoc::{parse_memory, IntCode, Result, SlotIo};

fn main() -> Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let program = parse_memory(&buf)?;
    let res = maximize_thrust(&program);
    println!("{}", res);
    Ok(())
}

fn maximize_thrust(program: &[i64]) -> i64 {
    let mut phases = [5, 6, 7, 8, 9];
    let mut res = i64::min_value();
    permutations(&mut phases, |phases| res = res.max(run(program, phases)));
    res
}

fn run(program: &[i64], phases: &[i64]) -> i64 {
    let slot = &SlotIo::default();
    let mut amps = (0..phases.len())
        .map(|_| (program.to_vec(), slot))
        .collect::<Vec<_>>();
    let mut amps = phases
        .iter()
        .zip(&mut amps)
        .map(|(phase, (mem, io))| {
            let mut ic = IntCode::new(*io, mem.as_mut_slice());
            slot.set(*phase);
            while !slot.clear_read() {
                assert!(ic.step().unwrap());
            }
            ic
        })
        .collect::<Vec<_>>();

    slot.set(0);
    let mut thrust = 0;
    'outer: loop {
        for amp in amps.iter_mut() {
            while !slot.clear_read() {
                if !amp.step().unwrap() {
                    break 'outer;
                }
            }
            while !slot.clear_write() {
                if !amp.step().unwrap() {
                    break 'outer;
                }
            }
        }
        thrust = slot.get();
    }
    thrust
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
        maximize_thrust(&[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ]),
        139629729
    );
    assert_eq!(
        maximize_thrust(&[
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ]),
        18216
    );
}
