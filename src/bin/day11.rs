use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

use aoc::{
    extend_memory, parse_memory, read_stdin_to_string, Direction, IntCode, Point, Result, SlotIo,
};

fn main() -> Result<()> {
    let input = read_stdin_to_string()?;
    let mut prog = parse_memory(&input)?;
    extend_memory(&mut prog);

    let painted = run(prog)?;
    let min_x = painted.keys().map(|p| p.1).min().unwrap();
    let min_y = painted.keys().map(|p| -p.0).min().unwrap();

    let mut hull = vec![vec![Color::Black; 80]; 10];
    for (p, color) in painted {
        let y = (-p.0 - min_y) as usize;
        let x = (p.1 - min_x) as usize;
        hull[y][x] = color;
    }
    for row in hull {
        let row = row
            .into_iter()
            .map(|c| match c {
                Color::Black => ' ',
                Color::White => '0',
            })
            .collect::<String>();
        println!("{}", row);
    }
    Ok(())
}

struct Robot {
    pos: Point,
    dir: Direction,
    painted: HashMap<Point, Color>,
}

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

impl From<Color> for i64 {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = ();
    fn try_from(value: i64) -> Result<Color, ()> {
        let res = match value {
            0 => Color::Black,
            1 => Color::White,
            _ => return Err(()),
        };
        Ok(res)
    }
}

fn run(mut prog: Vec<i64>) -> Result<HashMap<Point, Color>> {
    let slot = SlotIo::default();
    let io = &slot;
    let mut computer = IntCode::new(io, &mut prog);

    let mut robot = Robot {
        pos: Point(0, 0),
        dir: Direction::Up,
        painted: HashMap::new(),
    };
    robot.painted.insert(Point(0, 0), Color::White);

    'outer: loop {
        let color = robot
            .painted
            .get(&robot.pos)
            .copied()
            .unwrap_or(Color::Black);
        slot.set(color as i64);

        while !slot.clear_read() {
            if !computer.step()? {
                break 'outer;
            }
        }

        while !slot.clear_write() {
            if !computer.step()? {
                break 'outer;
            }
        }
        let color: Color = slot.get().try_into().map_err(|()| "bad color")?;
        let pos = robot.pos;
        robot.painted.insert(pos, color);

        while !slot.clear_write() {
            if !computer.step()? {
                break 'outer;
            }
        }
        robot.dir = match slot.get() {
            0 => robot.dir.turn_left(),
            1 => robot.dir.turn_right(),
            it => Err(format!("unknown direction {}", it))?,
        };
        robot.pos += robot.dir.delta();
    }
    Ok(robot.painted)
}
