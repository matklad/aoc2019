use std::fs;

use aoc::{parse_memory, Board, Direction, IntCode, Point, Result, StepIo};

fn main() -> Result<()> {
    let prog = fs::read_to_string("./input/day15.in")?;
    let mut prog = parse_memory(&prog)?;

    let cpu = IntCode::new_step(&mut prog);
    let mut ctx = Ctx::new(cpu, (50, 50));
    ctx.dfs();
    ctx.print();

    let p = ctx.board.find(|it| it == &Cell::Target).unwrap();
    let dists = bfs(&ctx.board, p);
    println!(
        "{}",
        dists.iter().map(|(_, it)| it.unwrap_or(0)).max().unwrap()
    );
    Ok(())
}

fn bfs(board: &Board<Cell>, start: Point) -> Board<Option<usize>> {
    let mut work = Vec::new();
    let mut dists = Board::new((50, 50), None).move_origin_to_center();
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
    cpu: IntCode<'a, StepIo>,
    board: Board<Cell>,
    pos: Point,
}

impl Ctx<'_> {
    fn new(cpu: IntCode<StepIo>, dim: (usize, usize)) -> Ctx {
        let mut board = Board::new(dim, Cell::Fog).move_origin_to_center();
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
        self.board.print(|cell| match cell {
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
