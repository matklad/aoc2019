use std::convert::TryInto;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct IntCode<'a> {
    mem: &'a mut [i64],
    ip: usize,
}

impl IntCode<'_> {
    pub fn new(mem: &mut [i64]) -> IntCode {
        IntCode { mem, ip: 0 }
    }
    pub fn run(mut self) -> Result<()> {
        while self.step()? {}
        Ok(())
    }
    fn step(&mut self) -> Result<bool> {
        let op = self.decode()?;
        match op {
            Op::Halt => return Ok(false),
            Op::Arith { op, lhs, rhs, dst } => {
                let lhs = self.load(lhs)?;
                let rhs = self.load(rhs)?;
                let res = op.eval(lhs, rhs);
                self.store(dst, res)?;
            }
        }
        self.ip += op.code_len();
        Ok(true)
    }
    fn load(&self, addr: i64) -> Result<i64> {
        if !(0 <= addr && addr <= self.mem.len() as i64) {
            Err(format!("invalid addr on load: {}", addr))?
        }
        Ok(self.mem[addr as usize])
    }
    fn store(&mut self, addr: i64, value: i64) -> Result<()> {
        if !(0 <= addr && addr <= self.mem.len() as i64) {
            Err(format!("invalid addr on store: {}", addr))?
        }
        self.mem[addr as usize] = value;
        Ok(())
    }

    fn decode(&self) -> Result<Op> {
        let op_code = self.mem[self.ip];
        let res = match op_code {
            1 | 2 => {
                let op = match op_code {
                    1 => ArithOp::Add,
                    2 => ArithOp::Mul,
                    _ => unreachable!(),
                };
                let [lhs, rhs, dst]: [i64; 3] = self.mem[self.ip + 1..self.ip + 4].try_into()?;
                Op::Arith { op, lhs, rhs, dst }
            }
            99 => Op::Halt,
            _ => Err(format!("invalide op code: {}", op_code))?,
        };
        Ok(res)
    }
}

#[derive(Clone, Copy)]
enum Op {
    Halt,
    Arith {
        op: ArithOp,
        lhs: i64,
        rhs: i64,
        dst: i64,
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
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            ArithOp::Add => lhs + rhs,
            ArithOp::Mul => lhs * rhs,
        }
    }
}
