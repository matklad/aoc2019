use std::{
    cell::Cell,
    fmt,
    io::{self, Read, Write},
    ops,
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub fn delta(self) -> Point {
        match self {
            Direction::Up => Point(1, 0),
            Direction::Right => Point(0, 1),
            Direction::Down => Point(-1, 0),
            Direction::Left => Point(0, -1),
        }
    }

    pub fn turn_left(self) -> Direction {
        self.turn(-1)
    }

    pub fn turn_right(self) -> Direction {
        self.turn(1)
    }

    pub fn rev(self) -> Direction {
        self.turn(2)
    }

    fn turn(self, delta: isize) -> Direction {
        let idx = (self as isize + delta) as usize;
        Direction::ALL[idx % 4]
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point(pub i64, pub i64);

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point(-self.0, -self.1)
    }
}

impl ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Point {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

pub fn read_stdin_to_string() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    buf.truncate(buf.trim_end().len());
    Ok(buf)
}

pub fn parse_memory(text: &str) -> Result<Vec<i64>> {
    let mut res = text
        .trim()
        .split(',')
        .map(|it| it.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    extend_memory(&mut res);
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

pub struct MemIo {
    input: Vec<i64>,
    output: Vec<i64>,
}

impl MemIo {
    pub fn new(mut input: Vec<i64>) -> Self {
        input.reverse();
        Self {
            input,
            output: Vec::new(),
        }
    }

    pub fn into_output(self) -> Vec<i64> {
        self.output
    }
}

impl Io for MemIo {
    fn read(&mut self) -> Result<i64> {
        let res = self.input.pop().ok_or("EOF")?;
        Ok(res)
    }
    fn write(&mut self, value: i64) -> Result<()> {
        self.output.push(value);
        Ok(())
    }
}

#[derive(Default)]
pub struct SlotIo {
    slot: Cell<i64>,
    read: Cell<bool>,
    write: Cell<bool>,
}

impl Io for &SlotIo {
    fn read(&mut self) -> Result<i64> {
        let res = self.get();
        self.read.set(true);
        Ok(res)
    }
    fn write(&mut self, value: i64) -> Result<()> {
        self.set(value);
        self.write.set(true);
        Ok(())
    }
}

impl SlotIo {
    pub fn get(&self) -> i64 {
        self.slot.get()
    }
    pub fn set(&self, value: i64) {
        self.slot.set(value)
    }
    pub fn clear_read(&self) -> bool {
        let res = self.read.get();
        self.read.set(false);
        res
    }
    pub fn clear_write(&self) -> bool {
        let res = self.write.get();
        self.write.set(false);
        res
    }
}

pub struct IntCode<'a> {
    io: &'a mut dyn Io,
    mem: &'a mut [i64],
    ip: i64,
    bp: i64,
}

impl<'a> IntCode<'a> {
    pub fn new(io: &'a mut dyn Io, mem: &'a mut [i64]) -> IntCode<'a> {
        IntCode {
            io,
            mem,
            ip: 0,
            bp: 0,
        }
    }
    pub fn run(mut self) -> Result<()> {
        while self.step()? {}
        Ok(())
    }
    pub fn step(&mut self) -> Result<bool> {
        let (size, op) = self.decode()?;
        match op {
            Op::Halt => return Ok(false),
            Op::Arith { op, lhs, rhs, dst } => {
                let lhs = self.decode_value(lhs)?;
                let rhs = self.decode_value(rhs)?;
                let res = op.eval(lhs, rhs);
                self.store(dst, res)?;
            }
            Op::Jump { op, cond, tgt } => {
                let cond = self.decode_value(cond)?;
                let jump = match op {
                    JumpOp::IfTrue => cond != 0,
                    JumpOp::IfFalse => cond == 0,
                };
                if jump {
                    self.ip = self.decode_value(tgt)?;
                    return Ok(true);
                }
            }
            Op::Input { dst } => {
                let res = self.io.read()?;
                self.store(dst, res)?;
            }
            Op::Output { src } => {
                let res = self.decode_value(src)?;
                self.io.write(res)?;
            }
            Op::AdjBp { adj } => {
                let adj = self.decode_value(adj)?;
                self.bp += adj;
            }
        }
        self.ip += size;
        Ok(true)
    }

    fn decode_value(&self, value: Value) -> Result<i64> {
        match value {
            Value::Immediate(it) => Ok(it),
            Value::Addr(addr) => self.load(addr),
        }
    }
    fn decode_addr(&self, addr: Addr) -> Result<usize> {
        let addr = if addr.rel {
            self.bp + addr.value
        } else {
            addr.value
        };
        if !(0 <= addr && addr <= self.mem.len() as i64) {
            Err(format!("invalid addr: {}", addr))?
        }
        Ok(addr as usize)
    }
    fn load(&self, addr: Addr) -> Result<i64> {
        let addr = self.decode_addr(addr)?;
        Ok(self.mem[addr])
    }
    fn load_instr(&self, addr: i64) -> Result<i64> {
        self.load(Addr {
            value: addr,
            rel: false,
        })
    }
    fn store(&mut self, addr: Addr, value: i64) -> Result<()> {
        let addr = self.decode_addr(addr)?;
        self.mem[addr] = value;
        Ok(())
    }

    fn decode(&self) -> Result<(i64, Op)> {
        fn to_value(modes: &mut i64, value: i64) -> Result<Value> {
            let res = match *modes % 10 {
                0 => Value::Addr(Addr { value, rel: false }),
                1 => Value::Immediate(value),
                2 => Value::Addr(Addr { value, rel: true }),
                _ => Err("invalid addr mode")?,
            };
            *modes /= 10;
            Ok(res)
        }

        let op_code = self.load_instr(self.ip)?;
        let (mut modes, op_code) = (op_code / 100, op_code % 100);
        let mut size = 0;
        macro_rules! args {
            ($($m:ident)*) => {{
                let res = ($(args!(@ $m),)*);
                if modes != 0 {
                    Err(format!("leftover modes: {}", modes))?
                }
                size += 1;
                res
            }};
            (@ v) => {{
                size += 1;
                let val = self.load_instr(self.ip + size)?;
                to_value(&mut modes, val)?
            }};
            (@ a) => {{
                size += 1;
                let val = self.load_instr(self.ip + size)?;
                match to_value(&mut modes, val)? {
                    Value::Addr(it) => it,
                    Value::Immediate(_) => Err("Immediate address")?,
                }
            }};
        }

        let res = match op_code {
            1 | 2 | 7 | 8 => {
                let op = match op_code {
                    1 => ArithOp::Add,
                    2 => ArithOp::Mul,
                    7 => ArithOp::LessThan,
                    8 => ArithOp::Equals,
                    _ => unreachable!(),
                };
                let (lhs, rhs, dst) = args!(v v a);
                Op::Arith { op, lhs, rhs, dst }
            }
            3 => {
                let (dst,) = args!(a);
                Op::Input { dst }
            }
            4 => {
                let (src,) = args!(v);
                Op::Output { src }
            }
            5 | 6 => {
                let op = match op_code {
                    5 => JumpOp::IfTrue,
                    6 => JumpOp::IfFalse,
                    _ => unreachable!(),
                };
                let (cond, tgt) = args!(v v);
                Op::Jump { op, cond, tgt }
            }
            9 => {
                let (adj,) = args!(v);
                Op::AdjBp { adj }
            }
            99 => Op::Halt,
            _ => Err(format!("invalid op code: {}", op_code))?,
        };
        Ok((size, res))
    }
}

#[derive(Debug, Clone, Copy)]
struct Addr {
    value: i64,
    rel: bool,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Immediate(i64),
    Addr(Addr),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Halt,
    Arith {
        op: ArithOp,
        lhs: Value,
        rhs: Value,
        dst: Addr,
    },
    Input {
        dst: Addr,
    },
    Output {
        src: Value,
    },
    Jump {
        op: JumpOp,
        cond: Value,
        tgt: Value,
    },
    AdjBp {
        adj: Value,
    },
}

#[derive(Debug, Clone, Copy)]
enum ArithOp {
    Add,
    Mul,
    LessThan,
    Equals,
}

impl ArithOp {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            ArithOp::Add => lhs + rhs,
            ArithOp::Mul => lhs * rhs,
            ArithOp::LessThan => (lhs < rhs) as i64,
            ArithOp::Equals => (lhs == rhs) as i64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum JumpOp {
    IfTrue,
    IfFalse,
}

pub fn extend_memory(mem: &mut Vec<i64>) {
    let limit = 64 * 1024;
    if mem.len() < limit {
        mem.resize(limit, 0);
    }
}

pub struct StepIo {
    read_slot: Cell<Option<i64>>,
    write_slot: Cell<Option<i64>>,
}

impl Default for StepIo {
    fn default() -> Self {
        StepIo {
            read_slot: Cell::new(None),
            write_slot: Cell::new(Some(0)),
        }
    }
}

#[derive(Debug)]
struct ReadFail;
impl std::error::Error for ReadFail {}
impl fmt::Display for ReadFail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReadFail")
    }
}

#[derive(Debug)]
struct WriteFail;
impl std::error::Error for WriteFail {}
impl fmt::Display for WriteFail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WriteFail")
    }
}

impl Io for &StepIo {
    fn read(&mut self) -> Result<i64> {
        let res = self.read_slot.take().ok_or(ReadFail)?;
        Ok(res)
    }
    fn write(&mut self, value: i64) -> Result<()> {
        if self.write_slot.get().is_some() {
            Err(WriteFail)?;
        }
        self.write_slot.set(Some(value));
        Ok(())
    }
}

pub struct StepCode<'a> {
    io: &'a StepIo,
    cpu: IntCode<'a>,
}

impl<'a> StepCode<'a> {
    pub fn new(io: &'a mut &StepIo, mem: &'a mut [i64]) -> StepCode<'a> {
        let io_copy = &*io;
        StepCode {
            io: io_copy,
            cpu: IntCode::new(io, mem),
        }
    }

    pub fn input(&mut self, value: i64) {
        loop {
            match self.cpu.step() {
                Ok(_) => (),
                Err(e) if e.downcast_ref::<ReadFail>().is_some() => break,
                Err(_) => panic!(),
            }
        }
        self.io.read_slot.set(Some(value));
        self.cpu.step().unwrap();
    }

    pub fn output(&mut self) -> i64 {
        loop {
            match self.cpu.step() {
                Ok(_) => (),
                Err(e) if e.downcast_ref::<WriteFail>().is_some() => break,
                Err(_) => panic!(),
            }
        }
        self.io.write_slot.set(None);
        self.cpu.step().unwrap();
        self.io.write_slot.get().unwrap()
    }
}
