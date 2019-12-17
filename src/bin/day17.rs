use std::convert::TryFrom;

use aoc::{parse_memory, read_stdin_to_string, Board, Direction, IntCode, MemIo, Point, Result};

fn main() -> Result<()> {
    let input = read_stdin_to_string()?;
    let mut mem = parse_memory(&input)?;
    let mut io = MemIo::new(vec![]);
    let mut cpu = IntCode::new(&mut io, &mut mem);
    cpu.run();

    let output = io.into_output();
    let width = output.iter().position(|&it| it == b'\n' as i64).unwrap();
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

    let res = board
        .iter()
        .filter(|(p, cell)| **cell == Cell::Scaffold)
        .filter(|(p, _)| {
            Direction::ALL
                .iter()
                .map(|it| *p + it.delta())
                .filter_map(|it| board.get(it))
                .all(|&it| it == Cell::Scaffold)
        })
        .map(|(p, _)| p)
        .map(|p| p.0 * p.1)
        .sum::<i64>();

    let robot_x = output.iter().position(|&it| it == b'^' as i64).unwrap();

    let mut prev = Point(0, 0);
    let mut curr = Point(robot_x as i64, 0);
    let mut path = Vec::new();
    let mut dir = Direction::Left;
    loop {
        let next = [dir, dir.turn(1), dir.turn(2), dir.turn(3)]
            .iter()
            .copied()
            .map(|d| (d, curr + d.delta()))
            .find(|(_d, next)| *next != prev && board.get(*next) == Some(&Cell::Scaffold));
        match next {
            None => break,
            Some((d, next)) => {
                dir = d;
                path.push(dir);
                prev = curr;
                curr = next;
            }
        }
    }

    eprintln!("path.len() = {:?}", path.len());

    let mut segments = vec![(Direction::Left, 0)];
    let mut cur_dir = Direction::Left;
    for &dir in path.iter() {
        if dir == cur_dir {
            segments.last_mut().unwrap().1 += 1
        } else {
            if cur_dir.turn_left() == dir {
                cur_dir = dir;
                segments.push((Direction::Left, 1))
            } else if cur_dir.turn_right() == dir {
                cur_dir = dir;
                segments.push((Direction::Right, 1))
            } else {
                panic!()
            }
        }
    }

    for s in segments {
        eprintln!("{:?}", s);
    }

    let turns = path
        .iter()
        .zip(&path[1..])
        .filter(|(x, y)| x != y)
        .map(|(&x, &y)| {
            if x.turn_left() == y {
                0
            } else if x.turn_right() == y {
                1
            } else {
                panic!()
            }
        })
        .collect::<Vec<_>>();

    eprintln!("turns = {:?}", turns);

    // let path = rle(&path);
    all_segments(&path).map(|seg| (seg, count(&path, seg)));

    Ok(())
}

fn all_segments<T>(xs: &[T]) -> impl Iterator<Item = &[T]> {
    (0..xs.len())
        .flat_map(move |i| (i + 2..xs.len()).map(move |j| (i, j)))
        .map(move |(i, j)| &xs[i..j])
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
