use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

use aoc::{Result, Point};

fn main() -> Result<()> {
    let map = Map::read(&mut io::stdin().lock())?;
    eprintln!("{}", map.winning_bet());
    Ok(())
}

struct Map {
    #[allow(unused)]
    dim: Point,
    asteroids: Vec<Point>,
}

impl Map {
    fn read(r: &mut impl BufRead) -> io::Result<Map> {
        let mut dim = Point::default();
        let mut asteroids = Vec::new();

        for line in r.lines() {
            let line = line?;
            if line.trim().is_empty() {
                break;
            }
            if !(dim.0 == 0 || dim.0 == line.bytes().len() as i64) {
                Err(io::Error::new(io::ErrorKind::InvalidData, "invalid map"))?
            };
            dim.0 = line.bytes().len() as i64;
            dim.1 += 1;

            let y = dim.1 - 1;
            for (x, b) in line.bytes().enumerate() {
                let x = x as i64;
                match b {
                    b'#' => asteroids.push(Point(x, y)),
                    b'.' => (),
                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, "invalid map"))?,
                }
            }
        }

        Ok(Map { dim, asteroids })
    }

    fn visible_from(&self, origin: Point) -> usize {
        let mut classified: HashSet<(Point, bool)> = HashSet::new();

        for &p in self.asteroids.iter() {
            if p == origin {
                continue;
            }
            let p = p - origin;
            let (slope, dist) = classify(p);
            classified.insert((slope, dist > 0));
        }

        classified.len()
    }

    fn destruction_order(&self, origin: Point) -> Vec<Point> {
        let mut classified: HashMap<Point, Vec<Point>> = HashMap::new();

        for &p in self.asteroids.iter() {
            if p == origin {
                continue;
            }
            let mut p = p - origin;
            p.1 *= -1;
            let (mut slope, dist) = classify(p);
            if dist < 0 {
                slope = -slope;
            }
            classified.entry(slope).or_default().push(p);
        }

        let mut keyed = classified
            .into_iter()
            .flat_map(|(slope, points)| {
                points
                    .into_iter()
                    .enumerate()
                    .map(move |(idx, p)| (idx, slope, p))
            })
            .collect::<Vec<_>>();
        keyed.sort_by(|(idx1, slope1, _), (idx2, slope2, _)| {
            idx1.cmp(idx2).then_with(|| cmp(*slope1, *slope2))
        });

        keyed
            .into_iter()
            .map(|(_, _, mut p)| {
                p.1 *= -1;
                p + origin
            })
            .collect()
    }

    fn winning_bet(&self) -> i64 {
        let (p, _) = self.best();
        let a = self.destruction_order(p)[199];
        a.0 * 100 + a.1
    }

    fn best(&self) -> (Point, usize) {
        self.asteroids
            .iter()
            .map(|&p| (p, self.visible_from(p)))
            .max_by_key(|(_p, key)| *key)
            .unwrap()
    }
}

fn cmp(slope1: Point, slope2: Point) -> Ordering {
    (slope1.0 < 0)
        .cmp(&(slope2.0 < 0))
        .then_with(|| (slope1.1 * slope2.0).cmp(&(slope2.1 * slope1.0)).reverse())
}

fn classify(p: Point) -> (Point, i64) // (slope, dist)
{
    let gcd = gcd(p.0, p.1);
    let slope = Point(p.0 / gcd, p.1 / gcd);
    (slope, gcd)
}

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[test]
fn test_classify() {
    let (slope1, dist1) = classify(Point(1, 2));
    let (slope2, dist2) = classify(Point(2, 4));
    assert_eq!(slope1, slope2);
    assert!(dist2.abs() > dist1.abs());

    let (slope3, dist3) = classify(Point(-1, -2));
    let (slope4, dist4) = classify(Point(-2, -4));
    assert_eq!(slope3, slope4);
    assert!(dist4.abs() > dist3.abs());

    assert_ne!(dist1 > 0, dist3 > 0);
}

#[test]
fn test_examples() {
    fn solve(map: &str) -> usize {
        Map::read(&mut map.as_bytes()).unwrap().best().1
    }

    let map = "\
.#..#
.....
#####
....#
...##";
    assert_eq!(solve(map), 8);

    let map = "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
    assert_eq!(solve(map), 33);

    let map = "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
    assert_eq!(solve(map), 35);

    let map = "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
    assert_eq!(solve(map), 41);

    let map = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    assert_eq!(solve(map), 210);
}

#[test]
fn test_sort_by_angle() {
    let points = vec![
        Point(0, 1),
        Point(1, 1),
        Point(1, 0),
        Point(1, -1),
        Point(0, -1),
        Point(-1, -1),
        Point(-1, 0),
        Point(-1, 1),
    ];
    let mut temp = points.clone();
    temp.sort_by(|p1, p2| cmp(*p1, *p2));
    assert_eq!(temp, points)
}

#[test]
fn test_example2() {
    let map = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    let map = Map::read(&mut map.as_bytes()).unwrap();
    assert_eq!(map.winning_bet(), 802);
}
