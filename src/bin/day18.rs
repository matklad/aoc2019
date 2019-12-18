use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

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
    let initial_positions = map
        .iter()
        .filter(|(_, cell)| **cell == b'@')
        .map(|(p, _)| p)
        .collect::<Vec<Point>>();
    assert_eq!(initial_positions.len(), 4);
    let initial_positions = [
        initial_positions[0],
        initial_positions[1],
        initial_positions[2],
        initial_positions[3],
    ];

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
    let all_keys = (1 << (max_key + 1)) - 1;

    let mut work: BinaryHeap<(Reverse<u32>, [Point; 4], u32)> = BinaryHeap::new();
    let mut done: HashSet<([Point; 4], u32)> = HashSet::new();
    let mut dists: HashMap<([Point; 4], u32), u32> = HashMap::new();
    work.push((Reverse(0), initial_positions, 0));
    dists.insert((initial_positions, 0), 0);

    'outer: while let Some((dist, positions, keymask)) = work.pop() {
        let dist = dist.0;
        if !done.insert((positions, keymask)) {
            continue;
        }
        assert_eq!(dists[&(positions, keymask)], dist);
        if keymask == all_keys {
            continue;
        }

        for (delta, positions, keymask) in next_positions(&map, positions, keymask) {
            let dist = dist + delta;
            if done.contains(&(positions, keymask)) {
                assert!(dists[&(positions, keymask)] <= dist);
                continue;
            }
            let prev = dists
                .entry((positions, keymask))
                .or_insert(u32::max_value());

            if *prev > dist {
                *prev = dist;
                work.push((Reverse(dist), positions, keymask))
            }
        }
    }

    dists
        .into_iter()
        .filter(|((_, mask), _)| *mask == all_keys)
        .map(|(_, d)| d)
        .min()
        .unwrap()
}

fn next_positions(
    board: &Board<Cell>,
    positions: [Point; 4],
    mask: u32,
) -> Vec<(u32, [Point; 4], u32)> {
    let mut res = Vec::new();
    for i in 0..4 {
        for (dist, pos, keyset) in next_bot_positions(board, positions[i], mask) {
            let mut positions = positions;
            positions[i] = pos;
            res.push((dist, positions, keyset))
        }
    }
    res
}

fn next_bot_positions(board: &Board<Cell>, pos: Point, mask: u32) -> Vec<(u32, Point, u32)> {
    let mut res = Vec::new();

    let mut work = VecDeque::new();
    let mut visited = HashSet::new();
    work.push_back((pos, 0u32));
    visited.insert(pos);

    while let Some((pos, dist)) = work.pop_front() {
        for &next_pos in pos.neighbors().iter() {
            match board.get(next_pos).copied().unwrap_or(Cell::Wall) {
                Cell::Wall => continue,
                Cell::Empty => (),
                Cell::Pass(pass) => {
                    if mask & pass.bit() != pass.bit() {
                        if !pass.is_door {
                            res.push((dist + 1, next_pos, mask | pass.bit()));
                        }
                        continue;
                    }
                }
            }
            if visited.insert(next_pos) {
                work.push_back((next_pos, dist + 1))
            }
        }
    }

    res
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

#[test]
fn test_examples2() {
    assert_eq!(
        solve(
            "
#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######",
        ),
        8
    );

    assert_eq!(
        solve(
            "
#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############",
        ),
        32
    );

    assert_eq!(
        solve(
            "
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
        ),
        72
    );
}
