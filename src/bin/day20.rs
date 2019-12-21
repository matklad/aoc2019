use std::collections::{
    hash_map::{Entry, HashMap},
    VecDeque,
};

use aoc::{read_stdin_to_string, Board, Direction, Point, Result};

fn main() -> Result<()> {
    let input = read_stdin_to_string()?;
    let res = solve(&input);
    eprintln!("res = {:?}", res);
    Ok(())
}

fn solve(maze: &str) -> u32 {
    let board = Board::from_ascii(maze);
    let labels = labels(&board);
    let start = labels[b"AA"];
    let finish = labels[b"ZZ"];

    let mut work = VecDeque::new();
    let mut dists: HashMap<(Point, u16), u32> = HashMap::new();

    work.push_back((start, 0));
    dists.insert((start, 0), 0);
    'outer: while let Some((u, level)) = work.pop_front() {
        let dist = dists[&(u, level)] + 1;

        let connection = label(&board, u)
            .and_then(|l| labels.get(&l).copied())
            .map(|p| p ^ u)
            .and_then(|p| {
                let level = if is_inner(&board, u) {
                    level + 1
                } else {
                    level.checked_sub(1)?
                };
                Some((p, level))
            });

        for (v, level) in u
            .neighbors()
            .iter()
            .copied()
            .map(|it| (it, level))
            .chain(connection)
        {
            if board.get(v).copied().unwrap_or(0) != b'.' {
                continue;
            }
            match dists.entry((v, level)) {
                Entry::Occupied(_) => continue,
                Entry::Vacant(entry) => {
                    entry.insert(dist);
                }
            }
            if v == finish && level == 0 {
                break 'outer;
            }
            work.push_back((v, level))
        }
    }

    dists[&(finish, 0)]
}

type Label = [u8; 2];

// Be unreasonably cute and store xor of points
fn labels(board: &Board<u8>) -> HashMap<Label, Point> {
    let mut res = HashMap::new();
    for p in board.points() {
        if let Some(l) = label(board, p) {
            *res.entry(l).or_default() ^= p
        }
    }
    res
}

fn is_inner(board: &Board<u8>, p: Point) -> bool {
    (4 <= p.0 && p.0 <= board.dim().0 as i64 - 4) && (4 <= p.1 && p.1 <= board.dim().1 as i64 - 4)
}

fn label(board: &Board<u8>, p: Point) -> Option<[u8; 2]> {
    if board.get(p) != Some(&b'.') {
        return None;
    }
    for &d in Direction::ALL.iter() {
        let get = |i: i64| board.get(p + d.delta() * i).copied().unwrap_or(0);

        let mut labels = [get(1), get(2)];
        if is_label(labels[0]) && is_label(labels[1]) {
            match d {
                Direction::Up | Direction::Left => labels.swap(0, 1),
                _ => (),
            }

            return Some(labels);
        }
    }
    return None;

    fn is_label(c: u8) -> bool {
        b'A' <= c && c <= b'Z'
    }
}

#[test]
fn test_example() {
    let maze = "
             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M
";

    assert_eq!(solve(maze), 396)
}
