use std::{cmp::Ordering, fs, io};

use aoc::{parse_memory, IntCode, Io, Result};

fn main() -> Result<()> {
    let mem = fs::read_to_string("./input/day13.in")?;
    let mut mem = parse_memory(&mem)?;
    mem[0] = 2;
    let mut io = ConsoleIo {
        buf: vec![],
        display: vec![vec![0; 40]; 21],
        score: 0,
        human: false,
        cnt: 0,
    };
    let mut computer = IntCode::new(io, &mut mem);
    computer.run()?;
    let mut io = computer.io;
    update_display(&mut io.score, &mut io.display, &io.buf);
    print_display(io.score, &io.display);
    Ok(())
}

struct ConsoleIo {
    buf: Vec<i64>,
    display: Vec<Vec<i64>>,
    score: i64,
    human: bool,
    cnt: u64,
}

impl Io for ConsoleIo {
    fn read(&mut self) -> Result<i64> {
        update_display(&mut self.score, &mut self.display, &self.buf);
        if self.cnt % 100 == 0 {
            print_display(self.score, &self.display);
        }
        self.cnt += 1;
        self.buf.clear();

        let res = if self.human {
            let mut buf = String::new();
            loop {
                buf.clear();
                io::stdin().read_line(&mut buf)?;
                break match buf.trim() {
                    "j" => -1,
                    "k" => 1,
                    "" => 0,
                    _ => continue,
                };
            }
        } else {
            let ball_pos = self
                .display
                .iter()
                .find_map(|row| row.iter().position(|&it| it == 4))
                .unwrap();

            let paddle_pos = self
                .display
                .iter()
                .find_map(|row| row.iter().position(|&it| it == 3))
                .unwrap();

            match ball_pos.cmp(&paddle_pos) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        };

        Ok(res)
    }
    fn write(&mut self, value: i64) -> Result<()> {
        self.buf.push(value);
        Ok(())
    }
}

fn update_display(score: &mut i64, display: &mut Vec<Vec<i64>>, buf: &[i64]) {
    let mut chunks = buf.chunks_exact(3);
    for tile in &mut chunks {
        let x = tile[0];
        let y = tile[1];
        let t = tile[2];
        if x == -1 {
            *score = t;
            continue;
        }
        display[y as usize][x as usize] = t;
    }
    assert!(chunks.remainder().is_empty());
}

fn print_display(score: i64, display: &Vec<Vec<i64>>) {
    println!("\nscore: {}", score);
    for row in display.iter() {
        let row = row.iter().copied().map(tile).collect::<String>();
        println!("{}", row);
    }

    fn tile(code: i64) -> char {
        match code {
            0 => ' ',
            1 => 'W',
            2 => 'b',
            3 => 'П',
            4 => '*',
            _ => panic!(),
        }
    }
}
