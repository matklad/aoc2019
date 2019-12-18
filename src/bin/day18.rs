use std::collections::{HashSet, VecDeque};

use aoc::{read_stdin_to_string, Board, Point, Result};

fn main() -> Result<()> {
    let map = read_stdin_to_string()?;
    println!("{}", solve(&map));
    Ok(())
}

fn solve(map: &str) -> u32 {
    let map = map.trim();
    let width = map.lines().next().unwrap().len();
    let height = map.len() / width;

    let map = Board::from_elements(
        (width, height),
        map.bytes().filter(|&it| it != b'\n' && it != b' '),
    );
    let (init_pos, _) = map.iter().find(|(_, cell)| **cell == b'@').unwrap();
    let mut max_key = 0;
    let map = Board::from_elements(
        map.dim(),
        map.iter().map(|(_p, cell)| match cell {
            b'.' | b'@' => Cell::Empty,
            b'#' => Cell::Wall,
            c => {
                let is_door = c.is_ascii_uppercase();
                let idx = c.to_ascii_lowercase() as u32 - 'a' as u32;
                max_key = max_key.max(idx);
                assert!(idx < 26);
                Cell::Pass(Pass {
                    idx: idx as u8,
                    is_door,
                })
            }
        }),
    );

    let mut work: VecDeque<(Point, u32, u32)> = VecDeque::new();
    let mut visited: HashSet<(Point, u32)> = HashSet::new();
    work.push_back((init_pos, 0, 0));
    visited.insert((init_pos, 0));

    let all_keys = (1u32 << (max_key + 1)) - 1;
    let mut res = None;
    'outer: while let Some((pos, keys, dist)) = work.pop_front() {
        for &next_pos in pos.neighbors().iter() {
            let mut keys = keys;
            match map.get(next_pos).copied().unwrap_or(Cell::Wall) {
                Cell::Wall => continue,
                Cell::Empty => (),
                Cell::Pass(pass) => {
                    if pass.is_door {
                        if keys & pass.bit() != pass.bit() {
                            continue;
                        }
                    } else {
                        keys |= pass.bit();
                    }
                }
            }
            if keys == all_keys {
                res = Some(dist + 1);
                break 'outer;
            }
            if !visited.insert((next_pos, keys)) {
                continue;
            }
            work.push_back((next_pos, keys, dist + 1))
        }
    }

    res.unwrap()
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Pass(Pass),
}

#[derive(Clone, Copy)]
struct Pass {
    idx: u8,
    is_door: bool,
}

impl Pass {
    fn bit(&self) -> u32 {
        1u32 << self.idx
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        solve(
            "
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
        ),
        132
    );

    assert_eq!(
        solve(
            "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
        ),
        136
    );

    assert_eq!(
        solve(
            "
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
        ),
        81
    );
}
