use std::convert::TryInto;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct IntCode<'a> {
    mem: &'a mut [u64],
    ip: usize,
}

impl IntCode<'_> {
    pub fn new(mem: &mut [u64]) -> IntCode {
        IntCode { mem, ip: 0 }
    }
    pub fn run(mut self) {
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
