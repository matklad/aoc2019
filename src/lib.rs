use std::{
    cell::Cell,
    fmt,
    io::{self, Read, Write},
    iter, mem, ops,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = match self {
            Direction::Up => 'U',
            Direction::Right => 'R',
            Direction::Down => 'D',
            Direction::Left => 'L',
        };
        write!(f, "{}", l)
    }
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
            Direction::Up => Point(0, -1),
            Direction::Right => Point(1, 0),
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
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

    pub fn turn(self, delta: isize) -> Direction {
        let idx = (self as isize + delta) as usize;
        Direction::ALL[idx % 4]
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

impl ops::BitXor for Point {
    type Output = Point;
    fn bitxor(self, rhs: Point) -> Point {
        Point(self.0 ^ rhs.0, self.1 ^ rhs.1)
    }
}

impl ops::BitXorAssign for Point {
    fn bitxor_assign(&mut self, rhs: Point) {
        *self = *self ^ rhs
    }
}

impl Point {
    pub fn neighbors(self) -> [Point; 4] {
        let mut res = [self; 4];
        for i in 0..4 {
            res[i] = self + Direction::ALL[i].delta();
        }
        res
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

pub struct AsciiIo {
    stdio: StdIo,
    buf: Vec<u8>,
}

impl AsciiIo {
    pub fn new() -> AsciiIo {
        AsciiIo {
            stdio: StdIo::new(),
            buf: Vec::new(),
        }
    }
}

impl Io for AsciiIo {
    fn read(&mut self) -> Result<i64> {
        if self.buf.is_empty() {
            let buf = mem::take(&mut self.buf);
            let mut buf = String::from_utf8(buf).unwrap();
            self.stdio.stdin.read_line(&mut buf)?;
            self.buf = buf.into_bytes();
            self.buf.reverse();
        }
        Ok(self.buf.pop().unwrap().into())
    }
    fn write(&mut self, value: i64) -> Result<()> {
        if value <= 128 {
            write!(self.stdio.stdout, "{}", value as u8 as char)
        } else {
            writeln!(self.stdio.stdout, "non-ASCII: {}", value)
        }?;
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

pub struct IntCode<'a, IO> {
    pub io: IO,
    mem: &'a mut [i64],
    ip: i64,
    bp: i64,
}

impl<'a, IO: Io> IntCode<'a, IO> {
    pub fn new(io: IO, mem: &'a mut [i64]) -> IntCode<'a, IO> {
        IntCode {
            io,
            mem,
            ip: 0,
            bp: 0,
        }
    }
    pub fn run(&mut self) -> Result<()> {
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
    read_slot: Option<i64>,
    write_slot: Option<i64>,
}

impl Default for StepIo {
    fn default() -> Self {
        StepIo {
            read_slot: None,
            write_slot: Some(0),
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

impl Io for StepIo {
    fn read(&mut self) -> Result<i64> {
        let res = self.read_slot.take().ok_or(ReadFail)?;
        Ok(res)
    }
    fn write(&mut self, value: i64) -> Result<()> {
        if self.write_slot.is_some() {
            Err(WriteFail)?;
        }
        self.write_slot = Some(value);
        Ok(())
    }
}

impl<'a> IntCode<'a, StepIo> {
    pub fn new_step(mem: &'a mut [i64]) -> Self {
        IntCode::new(StepIo::default(), mem)
    }

    pub fn input(&mut self, value: i64) {
        loop {
            match self.step() {
                Ok(_) => (),
                Err(e) if e.downcast_ref::<ReadFail>().is_some() => break,
                Err(_) => panic!(),
            }
        }
        self.io.read_slot = Some(value);
        self.step().unwrap();
    }

    pub fn output(&mut self) -> i64 {
        loop {
            match self.step() {
                Ok(_) => (),
                Err(e) if e.downcast_ref::<WriteFail>().is_some() => break,
                Err(_) => panic!(),
            }
        }
        self.io.write_slot = None;
        self.step().unwrap();
        self.io.write_slot.unwrap()
    }
}

pub struct Board<T> {
    dim: (usize, usize),
    origin: Point,
    data: Vec<T>,
}

impl Board<u8> {
    pub fn from_ascii(text: &str) -> Board<u8> {
        let height = text.lines().count();
        let width = text.lines().map(|it| it.len()).max().unwrap();
        let mut buf = Vec::new();
        for line in text.lines() {
            buf.extend(line.bytes());
            buf.extend(iter::repeat(b' ').take(width - line.len()));
        }
        Board::from_elements((width, height), buf)
    }
}

impl<T> Board<T> {
    pub fn new(dim: (usize, usize), element: T) -> Board<T>
    where
        T: Clone,
    {
        Board {
            dim,
            origin: Point::default(),
            data: vec![element; dim.0 * dim.1],
        }
    }

    pub fn from_elements(dim: (usize, usize), elements: impl IntoIterator<Item = T>) -> Board<T> {
        let data: Vec<_> = elements.into_iter().collect();
        assert!(
            data.len() == dim.0 * dim.1,
            "dim: {:?}, len: {}",
            dim,
            data.len()
        );
        Board {
            dim,
            origin: Point::default(),
            data,
        }
    }

    pub fn move_origin_to_center(mut self) -> Self {
        self.origin = Point((self.dim.0 / 2) as i64, (self.dim.1 / 2) as i64);
        self
    }

    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }

    pub fn get(&self, idx: Point) -> Option<&T> {
        let idx = self.to_index(idx)?;
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, idx: Point) -> Option<&mut T> {
        let idx = self.to_index(idx)?;
        self.data.get_mut(idx)
    }

    pub fn print(&self, display: impl Fn(&T) -> char) {
        for row in self.data.chunks(self.dim.0) {
            let row = row.iter().map(&display).collect::<String>();
            println!("{}", row)
        }
    }

    pub fn find(&self, pred: impl Fn(&T) -> bool) -> Option<Point> {
        self.data
            .iter()
            .position(pred)
            .map(|idx| self.to_point(idx))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(move |(idx, val)| (self.to_point(idx), val))
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.iter().map(|(p, _)| p)
    }

    fn to_index(&self, p: Point) -> Option<usize> {
        fn abs(rel: i64, orig: i64, dim: usize) -> Option<usize> {
            let res = rel + orig;
            if !(0 <= res && res < dim as i64) {
                return None;
            }
            Some(res as usize)
        }

        let x = abs(p.0, self.origin.0, self.dim.0)?;
        let y = abs(p.1, self.origin.1, self.dim.1)?;
        Some(y * self.dim.0 + x)
    }

    fn to_point(&self, idx: usize) -> Point {
        fn rel(abs: usize, orig: i64) -> i64 {
            abs as i64 - orig
        }
        let x = idx % self.dim.0;
        let y = idx / self.dim.0;
        let res = Point(rel(x, self.origin.0), rel(y, self.origin.1));
        assert_eq!(self.to_index(res).unwrap(), idx);
        res
    }
}

impl<T> ops::Index<Point> for Board<T> {
    type Output = T;
    fn index(&self, index: Point) -> &T {
        self.get(index).unwrap()
    }
}

impl<T> ops::IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        self.get_mut(index).unwrap()
    }
}
