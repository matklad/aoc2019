use std::{collections::HashSet, convert::TryFrom, hash::Hash};

use aoc::{parse_memory, read_stdin_to_string, Board, Direction, IntCode, MemIo, Point, Result};

fn main() -> Result<()> {
    // let input = read_stdin_to_string()?;
    // let mut mem = parse_memory(&input)?;
    // let mut io = MemIo::new(vec![]);
    // let mut cpu = IntCode::new(&mut io, &mut mem);
    // cpu.run();

    // let output = io.into_output();
    // let width = output.iter().position(|&it| it == b'\n' as i64).unwrap();
    // let height = output.len() / (width + 1);
    // assert_eq!((width + 1) * height + 1, output.len());
    // let board = Board::from_elements(
    //     (width, height),
    //     output
    //         .iter()
    //         .map(|&it| u8::try_from(it).unwrap())
    //         .filter(|&it| it != b'\n')
    //         .map(|it| match it {
    //             b'.' | b'^' => Cell::Space,
    //             b'#' => Cell::Scaffold,
    //             _ => panic!(),
    //         }),
    // );

    // let robot_x = output.iter().position(|&it| it == b'^' as i64).unwrap();

    // let mut prev = Point(0, 0);
    // let mut curr = Point(robot_x as i64, 0);
    // let mut path = Vec::new();
    // let mut dir = Direction::Left;
    // loop {
    //     let next = [dir, dir.turn(1), dir.turn(2), dir.turn(3)]
    //         .iter()
    //         .copied()
    //         .map(|d| (d, curr + d.delta()))
    //         .find(|(_d, next)| *next != prev && board.get(*next) == Some(&Cell::Scaffold));
    //     match next {
    //         None => break,
    //         Some((d, next)) => {
    //             dir = d;
    //             path.push(dir);
    //             prev = curr;
    //             curr = next;
    //         }
    //     }
    // }

    let path = b"RFFFFFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFLFFFFFFLFFRFFFFRFFFFRFFFFFFFFRFFFFFFFFRFFFFFFFFLFFFFFFLFF";
    let res = triangulate(path);
    eprintln!("res = {:#?}", res);

    Ok(())
}

fn triangulate<T>(xs: &[T]) -> Option<([&[T]; 3], Vec<usize>)>
where
    T: Eq + Hash,
{
    let segments = interesting_segments(xs);
    for first_len in 0..xs.len() {
        let first_seg = &xs[..first_len];
        if !segments.contains(first_seg) {
            continue;
        }
        for &second_seg in segments.iter() {
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
    let min_repeat = 3;

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
