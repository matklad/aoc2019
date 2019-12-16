use std::{fs, ops};

use aoc::{parse_memory, Direction, Point, Result, StepCode, StepIo};

fn main() -> Result<()> {
    let prog = fs::read_to_string("./input/day15.in")?;
    let mut prog = parse_memory(&prog)?;

    let mut io = &StepIo::default();
    let cpu = StepCode::new(&mut io, &mut prog);
    let mut ctx = Ctx::new(cpu, (50, 50));
    ctx.dfs();
    ctx.print();

    let p = ctx.board.find(|it| it == &Cell::Target).unwrap();
    let dists = bfs(&ctx.board, p);
    println!(
        "{}",
        dists.data.iter().map(|it| it.unwrap_or(0)).max().unwrap()
    );
    Ok(())
}

fn bfs(board: &Board<Cell>, start: Point) -> Board<Option<usize>> {
    let mut work = Vec::new();
    let mut dists = Board::new((50, 50), None);
    dists[start] = Some(0);
    work.push(start);
    let mut idx = 0;
    while idx < work.len() {
        let curr = work[idx];
        idx += 1;
        for &d in Direction::ALL.iter() {
            let next = curr + d.delta();
            if board[next] == Cell::Wall || dists[next].is_some() {
                continue;
            }
            dists[next] = Some(dists[curr].unwrap() + 1);
            work.push(next)
        }
    }
    dists
}

struct Ctx<'a> {
    cpu: StepCode<'a>,
    board: Board<Cell>,
    pos: Point,
}

impl Ctx<'_> {
    fn new(cpu: StepCode, dim: (usize, usize)) -> Ctx {
        let mut board = Board::new(dim, Cell::Fog);
        let pos = Point::default();
        board[pos] = Cell::Empty;
        Ctx { cpu, board, pos }
    }

    fn dfs(&mut self) {
        for &dir in Direction::ALL.iter() {
            let p = self.pos + dir.delta();
            if self.board[p] == Cell::Fog {
                let cell = self.step(dir);
                self.board[p] = cell;
                match cell {
                    Cell::Fog => unreachable!(),
                    Cell::Empty | Cell::Target => {
                        self.pos += dir.delta();
                        self.dfs();
                        self.step(dir.rev());
                        self.pos += dir.rev().delta();
                    }
                    Cell::Wall => (),
                }
            }
        }
    }

    fn step(&mut self, d: Direction) -> Cell {
        self.cpu.input(match d {
            Direction::Up => 1,
            Direction::Right => 4,
            Direction::Down => 2,
            Direction::Left => 3,
        });
        match self.cpu.output() {
            0 => Cell::Wall,
            1 => Cell::Empty,
            2 => Cell::Target,
            _ => panic!(),
        }
    }

    fn print(&self) {
        self.board.print(&|cell| match cell {
            Cell::Empty => '.',
            Cell::Fog => ' ',
            Cell::Wall => 'X',
            Cell::Target => 'O',
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Fog,
    Empty,
    Wall,
    Target,
}

struct Board<T> {
    dim: (usize, usize),
    data: Vec<T>,
}

impl<T> Board<T> {
    fn new(dim: (usize, usize), element: T) -> Board<T>
    where
        T: Clone,
    {
        Board {
            dim,
            data: vec![element; dim.0 * dim.1],
        }
    }

    fn to_index(&self, p: Point) -> usize {
        fn abs(rel: i64, dim: usize) -> usize {
            let res = rel + dim as i64 / 2;
            assert!(0 <= res && res < dim as i64);
            res as usize
        }

        let x = abs(p.0, self.dim.0);
        let y = abs(p.1, self.dim.1);
        y * self.dim.0 + x
    }

    fn to_point(&self, idx: usize) -> Point {
        fn rel(abs: usize, dim: usize) -> i64 {
            abs as i64 - (dim as i64) / 2
        }
        let x = idx % self.dim.0;
        let y = idx / self.dim.0;
        let res = Point(rel(x, self.dim.0), rel(y, self.dim.1));
        assert_eq!(self.to_index(res), idx);
        res
    }

    fn print(&self, display: &dyn Fn(&T) -> char) {
        for row in self.data.chunks(self.dim.0) {
            let row = row.iter().map(display).collect::<String>();
            println!("{}", row)
        }
    }

    fn find(&self, pred: impl Fn(&T) -> bool) -> Option<Point> {
        self.data
            .iter()
            .position(pred)
            .map(|idx| self.to_point(idx))
    }
}

impl<T> ops::Index<Point> for Board<T> {
    type Output = T;
    fn index(&self, index: Point) -> &T {
        let idx = self.to_index(index);
        &self.data[idx]
    }
}

impl<T> ops::IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        let idx = self.to_index(index);
        &mut self.data[idx]
    }
}
