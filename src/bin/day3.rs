use aoc::{Direction, Error, Point, Result};

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut path1 = String::new();
    stdin.read_line(&mut path1)?;

    let mut path2 = String::new();
    stdin.read_line(&mut path2)?;

    let res = solve_steps(&path1, &path2);
    println!("{}", res);
    Ok(())
}

fn _solve_dist(path1: &str, path2: &str) -> i64 {
    let path1 = parse_path(path1).unwrap();
    let path2 = parse_path(path2).unwrap();

    let path1_points: HashSet<Point> = trace_path(&path1).into_iter().collect();
    let common_points = trace_path(&path2)
        .into_iter()
        .skip(1)
        .filter(|it| path1_points.contains(&it));

    let min_dist = common_points.map(|p| p.0.abs() + p.1.abs()).min().unwrap();
    min_dist
}

fn solve_steps(path1: &str, path2: &str) -> usize {
    let path1 = parse_path(path1).unwrap();
    let path2 = parse_path(path2).unwrap();

    let path1_points: HashMap<Point, usize> = trace_path(&path1)
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .rev()
        .collect();

    let common_points = trace_path(&path2)
        .into_iter()
        .enumerate()
        .skip(1)
        .filter_map(|(d2, p)| {
            let d1 = *path1_points.get(&p)?;
            Some(d1 + d2)
        });

    common_points.min().unwrap()
}

fn parse_path(text: &str) -> Result<Vec<Segment>> {
    text.trim().split(",").map(|it| it.parse()).collect()
}

struct Segment {
    dir: Direction,
    len: u64,
}

impl FromStr for Segment {
    type Err = Error;
    fn from_str(s: &str) -> Result<Segment> {
        let dir = match s.chars().next() {
            Some('U') => Direction::Up,
            Some('R') => Direction::Right,
            Some('D') => Direction::Down,
            Some('L') => Direction::Left,
            _ => Err("invalid direction")?,
        };
        let len: u64 = s[1..].parse()?;
        Ok(Segment { dir, len })
    }
}

fn trace_path(segments: &[Segment]) -> Vec<Point> {
    let mut res = vec![Point(0, 0)];
    for seg in segments.iter() {
        let last = *res.last().unwrap();
        let delta = seg.dir.delta();
        res.extend((1..=(seg.len as i64)).map(|d| last + delta * d));
    }
    res
}

#[test]
fn test_examples() {
    assert_eq!(
        _solve_dist(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ),
        159,
    );
    assert_eq!(
        _solve_dist(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ),
        135,
    );

    assert_eq!(
        solve_steps(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83"
        ),
        610,
    );
    assert_eq!(
        solve_steps(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ),
        410,
    );
}
