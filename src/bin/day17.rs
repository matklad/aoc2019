use std::{collections::HashSet, convert::TryFrom, fs, hash::Hash};

use aoc::{parse_memory, read_stdin_to_string, Board, Direction, IntCode, MemIo, Point, Result};

fn main() -> Result<()> {
    let res = b"A,A,B,C,C,A,C,B,C,B\nL,4,L,4,L,6,R,10,L,6\nL,12,L,6,R,10,L,6\nR,8,R,10,L,6\nn\n";

    let input = fs::read_to_string("input/day17.in")?;
    let mut mem = parse_memory(&input)?;
    mem[0] = 2;
    let mut io = MemIo::new(res.iter().map(|it| *it as i64).collect());
    let mut cpu = IntCode::new(&mut io, &mut mem);
    cpu.run();
    let out = io.into_output();
    eprintln!("out = {:?}", out);
    let s = String::from_utf8(out.into_iter().map(|it| it as u8).collect()).unwrap();
    eprintln!("{}", s);


    // let (board, init_pos) = read_board()?;
    // let final_pos = Point(12, 30);
    // board.print(|c| match c {
    //     Cell::Scaffold => '#',
    //     Cell::Space => '.',
    // });

    // let path = heuristic_path(&board, init_pos, Direction::Up);
    // eprintln!("path = {:?}", path);
    // let (segs, path) = triangulate(&path).unwrap();

    // let path = path
    //     .iter()
    //     .map(|it| ["A", "B", "C"][*it])
    //     .collect::<Vec<_>>()
    //     .join(",");
    // println!("{}", path);
    // for &seg in segs.iter() {
    //     println!("{}", encode(seg))
    // }

    // let mut cnt = 0;
    // enumerate_paths(
    //     &mut |path| {
    //         if let Some(it) = triangulate(path) {
    //             eprintln!("it = {:?}", it);
    //         }
    //         cnt += 1;
    //         if cnt % 100 == 0 {
    //             eprintln!("{:?}", cnt);
    //         }
    //     },
    //     &board,
    //     &mut 0,
    //     &mut vec![L],
    //     &mut Board::new(board.dim(), false),
    //     init_pos,
    //     Direction::Left,
    // );
    Ok(())
}

fn encode(path: &[Step]) -> String {
    rle(path)
        .into_iter()
        .map(|(s, r)| match s {
            Step::L => "L".to_string(),
            Step::R => "R".to_string(),
            Step::F => r.to_string(),
        })
        .collect::<Vec<String>>()
        .join(",")
}

fn heuristic_path(board: &Board<Cell>, init_pos: Point, init_dir: Direction) -> Vec<Step> {
    let mut curr = init_pos;
    let mut steps = vec![];
    let mut dir = init_dir;
    loop {
        let next = [(dir, F), (dir.turn_left(), L), (dir.turn_right(), R)]
            .iter()
            .copied()
            .map(|(d, s)| (d, curr + d.delta(), s))
            .find(|(_d, next, _s)| is_scaffold(board, *next));
        match next {
            None => break,
            Some((d, next, step)) => {
                if dir == d {
                    curr = next;
                }
                dir = d;
                steps.push(step);
            }
        }
    }
    steps
}

const FINAL_POS: Point = Point(12, 30);
const LEN: u32 = 288;

fn enumerate_paths(
    cb: &mut dyn FnMut(&[Step]),
    board: &Board<Cell>,
    len: &mut u32,
    steps: &mut Vec<Step>,
    on_path: &mut Board<bool>,
    pos: Point,
    dir: Direction,
) {
    if pos == FINAL_POS && *len == LEN {
        cb(steps.as_slice());
        return;
    }
    if *len >= LEN {
        return;
    }
    if is_scaffold(board, pos + dir.delta()) {
        let pos = pos + dir.delta();
        *len += 1;
        steps.push(F);
        on_path[pos] = true;
        enumerate_paths(cb, board, len, steps, on_path, pos, dir);
        *len -= 1;
        steps.pop();
        on_path[pos] = false;
    }
    for &step in [L, R].iter() {
        let dir = if step == L {
            dir.turn_left()
        } else {
            dir.turn_right()
        };
        let pos = pos + dir.delta();
        if is_scaffold(board, pos) && !on_path[pos] {
            *len += 1;
            steps.push(step);
            on_path[pos] = true;
            enumerate_paths(cb, board, len, steps, on_path, pos, dir);
            *len -= 1;
            steps.pop();
            on_path[pos] = false;
        }
    }
}

fn is_scaffold(board: &Board<Cell>, pos: Point) -> bool {
    board.get(pos).copied().unwrap_or(Cell::Space) == Cell::Scaffold
}

fn read_board() -> Result<(Board<Cell>, Point)> {
    let input = fs::read_to_string("input/day17.in")?;
    let mut mem = parse_memory(&input)?;
    let mut io = MemIo::new(vec![]);
    let mut cpu = IntCode::new(&mut io, &mut mem);
    cpu.run();

    let output = io.into_output();
    //     let output = "#######...#####
    // #.....#...#...#
    // #.....#...#...#
    // ......#...#...#
    // ......#...###.#
    // ......#.....#.#
    // ^########...#.#
    // ......#.#...#.#
    // ......#########
    // ........#...#..
    // ....#########..
    // ....#...#......
    // ....#...#......
    // ....#...#......
    // ....#####......

    // "
    //     .to_string()
    //     .into_bytes();

    let width = output
        .iter()
        .position(|&it| it as i64 == b'\n' as i64)
        .unwrap();
    let height = output.len() / (width + 1);
    assert_eq!((width + 1) * height + 1, output.len());
    let board = Board::from_elements(
        (width, height),
        output
            .iter()
            .map(|&it| u8::try_from(it).unwrap())
            .filter(|&it| it != b'\n')
            .map(|it| match it {
                b'.' | b'^' => Cell::Space,
                b'#' => Cell::Scaffold,
                _ => panic!(),
            }),
    );
    let robot_x = output
        .iter()
        .filter(|&&it| it as i64 != b'\n' as i64)
        .position(|&it| it as i64 == b'^' as i64)
        .unwrap();
    Ok((
        board,
        Point((robot_x % width) as i64, (robot_x / width) as i64),
    ))
}

fn triangulate<T>(xs: &[T]) -> Option<([&[T]; 3], Vec<usize>)>
where
    T: Eq + Hash,
{
    let segments = interesting_segments(xs);
    let firsts = segments
        .iter()
        .copied()
        .filter(|seg| xs.starts_with(seg))
        .collect::<Vec<_>>();
    let lasts = segments
        .iter()
        .copied()
        .filter(|seg| xs.ends_with(seg))
        .collect::<Vec<_>>();

    for &first_seg in firsts.iter() {
        for &second_seg in lasts.iter() {
            for &third_seg in segments.iter() {
                if first_seg == second_seg || first_seg == third_seg || second_seg == third_seg {
                    continue;
                }
                let segs = [first_seg, second_seg, third_seg];
                if let Some(idxes) = try_triangulate(xs, segs) {
                    return Some((segs, idxes));
                }
            }
        }
    }
    None
}

fn try_triangulate<'a, T>(mut xs: &'a [T], segs: [&'a [T]; 3]) -> Option<Vec<usize>>
where
    T: Eq,
{
    let mut res = Vec::new();
    'outer: while !xs.is_empty() {
        for i in 0..3 {
            if xs.starts_with(segs[i]) {
                xs = &xs[segs[i].len()..];
                res.push(i);
                continue 'outer;
            }
        }
        return None;
    }
    Some(res)
}

fn interesting_segments<T>(xs: &[T]) -> HashSet<&[T]>
where
    T: Eq + Hash,
{
    let min_len = 3;
    let min_repeat = 2;

    (0..xs.len())
        .flat_map(move |i| (i + min_len..xs.len()).map(move |j| (i, j)))
        .map(move |(i, j)| &xs[i..j])
        .filter(|seg| count(xs, seg) >= min_repeat)
        .collect()
}

fn count<T: Eq>(haystack: &[T], needle: &[T]) -> usize {
    (0..haystack.len() - needle.len())
        .filter(|&idx| &haystack[idx..][..needle.len()] == needle)
        .count()
}

fn rle<T: Clone + Eq>(sequence: &[T]) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    let mut iter = sequence.iter();
    match iter.next() {
        Some(it) => res.push((it.clone(), 1)),
        None => return res,
    };

    for element in iter {
        let (prev, cnt) = res.last_mut().unwrap();
        if prev == element {
            *cnt += 1
        } else {
            res.push((element.clone(), 1))
        }
    }

    res
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Space,
    Scaffold,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Step {
    L,
    R,
    F,
}
use Step::*;
