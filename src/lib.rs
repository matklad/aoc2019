use std::{
    convert::TryFrom,
    io::{self, Write},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn parse_memory(text: &str) -> Result<Vec<i64>> {
    let res = text
        .trim()
        .split(',')
        .map(|it| it.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(res)
}

pub trait Io {
    fn read(&mut self) -> Result<i64>;
    fn write(&mut self, value: i64) -> Result<()>;
}

pub struct StdIo {
    stdout: io::Stdout,
    stdin: io::Stdin,
}

impl StdIo {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            stdin: io::stdin(),
        }
    }
}

impl Io for StdIo {
    fn read(&mut self) -> Result<i64> {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf)?;
        let res = buf.trim().parse()?;
        Ok(res)
    }
    fn write(&mut self, value: i64) -> Result<()> {
        writeln!(self.stdout, "{}", value)?;
        Ok(())
    }
}

pub struct IntCode<'a> {
    io: &'a mut dyn Io,
    mem: &'a mut [i64],
    ip: usize,
}

impl<'a> IntCode<'a> {
    pub fn new(io: &'a mut dyn Io, mem: &'a mut [i64]) -> IntCode<'a> {
        IntCode { io, mem, ip: 0 }
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
            Op::Input { dst } => {
                let res = self.io.read()?;
                self.store(dst, res)?;
            }
            Op::Output { src } => {
                let res = self.load(src)?;
                self.io.write(res)?;
            }
        }
        self.ip += op.code_len();
        Ok(true)
    }
    fn load(&self, value: Value) -> Result<i64> {
        match value {
            Value::Immediate(it) => Ok(it),
            Value::Addr(addr) => {
                if !(0 <= addr && addr <= self.mem.len() as i64) {
                    Err(format!("invalid addr on load: {}", addr))?
                }
                Ok(self.mem[addr as usize])
            }
        }
    }
    fn store(&mut self, addr: i64, value: i64) -> Result<()> {
        if !(0 <= addr && addr <= self.mem.len() as i64) {
            Err(format!("invalid addr on store: {}", addr))?
        }
        self.mem[addr as usize] = value;
        Ok(())
    }

    fn decode(&self) -> Result<Op> {
        macro_rules! args {
            ($n:expr) => {{
                let bytes = self
                    .mem
                    .get(self.ip + 1..self.ip + 1 + $n)
                    .ok_or("invalid op args")?;
                <[i64; $n]>::try_from(bytes).unwrap()
            }};
        }

        fn to_value(modes: &mut i64, value: i64) -> Result<Value> {
            let res = match *modes % 10 {
                0 => Value::Addr(value),
                1 => Value::Immediate(value),
                _ => Err("invalid addr mode")?,
            };
            *modes /= 10;
            Ok(res)
        }

        let op_code = self.mem[self.ip];
        let (mut modes, op_code) = (op_code / 100, op_code % 100);
        let res = match op_code {
            1 | 2 => {
                let op = match op_code {
                    1 => ArithOp::Add,
                    2 => ArithOp::Mul,
                    _ => unreachable!(),
                };
                let [lhs, rhs, dst] = args!(3);
                let lhs = to_value(&mut modes, lhs)?;
                let rhs = to_value(&mut modes, rhs)?;
                Op::Arith { op, lhs, rhs, dst }
            }
            3 => {
                let [dst] = args!(1);
                Op::Input { dst }
            }
            4 => {
                let [src] = args!(1);
                let src = to_value(&mut modes, src)?;
                Op::Output { src }
            }
            99 => Op::Halt,
            _ => Err(format!("invalid op code: {}", op_code))?,
        };
        if modes != 0 {
            Err("invalid add mode")?
        }
        Ok(res)
    }
}

#[derive(Clone, Copy)]
enum Value {
    Immediate(i64),
    Addr(i64),
}

#[derive(Clone, Copy)]
enum Op {
    Halt,
    Arith {
        op: ArithOp,
        lhs: Value,
        rhs: Value,
        dst: i64,
    },
    Input {
        dst: i64,
    },
    Output {
        src: Value,
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
            Op::Input { .. } | Op::Output { .. } => 2,
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
