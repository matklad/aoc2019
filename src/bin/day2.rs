use std::{convert::TryInto, io::stdin};

use aoc::Result;

fn main() -> Result<()> {
    let line = {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        buf
    };
    let memory = line
        .trim()
        .split(',')
        .map(|it| it.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            let memory = run(memory);
            if memory[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                return Ok(());
            }
        }
    }
    panic!("no answer")
}

fn run(mut mem: Vec<u64>) -> Vec<u64> {
    Computer::new(&mut mem).run();
    mem
}

struct Computer<'a> {
    mem: &'a mut [u64],
    ip: usize,
}

impl Computer<'_> {
    fn new(mem: &mut [u64]) -> Computer {
        Computer { mem, ip: 0 }
    }
    fn run(mut self) {
        while self.step() {}
    }
    fn step(&mut self) -> bool {
        let op = self.decode();
        match op {
            Op::Halt => return false,
            Op::Arith { op, lhs, rhs, dst } => {
                let lhs = self.mem[lhs as usize];
                let rhs = self.mem[rhs as usize];
                self.mem[dst as usize] = op.eval(lhs, rhs);
            }
        }
        self.ip += op.code_len();
        true
    }
    fn decode(&self) -> Op {
        let op_code = self.mem[self.ip];
        match op_code {
            1 | 2 => {
                let op = match op_code {
                    1 => ArithOp::Add,
                    2 => ArithOp::Mul,
                    _ => unreachable!(),
                };
                let [lhs, rhs, dst]: [u64; 3] = self.mem[self.ip + 1..self.ip + 4]
                    .try_into()
                    .expect("invalid arith op");
                Op::Arith { op, lhs, rhs, dst }
            }
            99 => Op::Halt,
            _ => panic!("unknown op: {}", op_code),
        }
    }
}

#[derive(Clone, Copy)]
enum Op {
    Halt,
    Arith {
        op: ArithOp,
        lhs: u64,
        rhs: u64,
        dst: u64,
    },
}

#[derive(Clone, Copy)]
enum ArithOp {
    Add,
    Mul,
}

impl Op {
    fn code_len(&self) -> usize {
        match self {
            Op::Halt => 1,
            Op::Arith { .. } => 4,
        }
    }
}

impl ArithOp {
    fn eval(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            ArithOp::Add => lhs + rhs,
            ArithOp::Mul => lhs * rhs,
        }
    }
}

#[test]
fn smoke() {
    assert_eq!(
        run(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    assert_eq!(run(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(run(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(run(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
}
