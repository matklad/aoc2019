use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    ops,
};

use aoc::Result;

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
            if !(dim.x == 0 || dim.x == line.bytes().len() as i32) {
                Err(io::Error::new(io::ErrorKind::InvalidData, "invalid map"))?
            };
            dim.x = line.bytes().len() as i32;
            dim.y += 1;

            let y = dim.y - 1;
            for (x, b) in line.bytes().enumerate() {
                let x = x as i32;
                match b {
                    b'#' => asteroids.push(Point { x, y }),
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
            p.y *= -1;
            let (mut slope, dist) = classify(p);
            if dist < 0 {
                slope.x *= -1;
                slope.y *= -1;
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
                p.y *= -1;
                p + origin
            })
            .collect()
    }

    fn winning_bet(&self) -> i32 {
        let (p, _) = self.best();
        let a = self.destruction_order(p)[199];
        a.x * 100 + a.y
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
    (slope1.x < 0)
        .cmp(&(slope2.x < 0))
        .then_with(|| (slope1.y * slope2.x).cmp(&(slope2.y * slope1.x)).reverse())
}

fn classify(p: Point) -> (Point, i32) // (slope, dist)
{
    let gcd = gcd(p.x, p.y);
    let slope = Point {
        x: p.x / gcd,
        y: p.y / gcd,
    };
    (slope, gcd)
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn test_classify() {
    fn point(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    let (slope1, dist1) = classify(point(1, 2));
    let (slope2, dist2) = classify(point(2, 4));
    assert_eq!(slope1, slope2);
    assert!(dist2.abs() > dist1.abs());

    let (slope3, dist3) = classify(point(-1, -2));
    let (slope4, dist4) = classify(point(-2, -4));
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
    fn p(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    let points = vec![
        p(0, 1),
        p(1, 1),
        p(1, 0),
        p(1, -1),
        p(0, -1),
        p(-1, -1),
        p(-1, 0),
        p(-1, 1),
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
