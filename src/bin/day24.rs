use std::collections::HashSet;

use aoc::{Board, Direction, Point};

fn main() {
    let board = big_solve(
        "\
#.#.#
.#...
...#.
.###.
###.#",
    );

    eprintln!("board = {:?}", board);
}

fn solve(ex: &str) -> u64 {
    let mut board = Board::from_ascii(ex);
    let mut seen = HashSet::new();
    let dupe = loop {
        // println!();
        // board.print(|it| *it as char);
        if !seen.insert(board.clone()) {
            break board;
        }
        board = step(&board)
    };

    return biodiv(&dupe);
}

fn big_solve(ex: &str) -> usize {
    let seed_board = Board::from_ascii(ex);
    let mut boards = vec![Board::new(seed_board.dim(), b'.'); 400];
    let mid = boards.len() / 2;
    boards[mid] = seed_board;
    for _ in 0..200 {
        boards = big_step(&boards);
    }
    boards
        .iter()
        .flat_map(|it| it.iter())
        .filter(|(_, &it)| it == b'#')
        .count()
}

fn biodiv(b: &Board<u8>) -> u64 {
    let mut res = 0u64;
    let mut mul = 1;
    for (_, b) in b.iter() {
        if *b == b'#' {
            res += mul
        }
        mul *= 2;
    }
    res
}

fn step(board: &Board<u8>) -> Board<u8> {
    let mut res = Board::new(board.dim(), b'.');
    for (p, &b) in board.iter() {
        let adj_bugs = p
            .neighbors()
            .iter()
            .flat_map(|it| board.get(*it))
            .filter(|&&it| it == b'#')
            .count();
        let b = b == b'#';
        let has_bug = (b && adj_bugs == 1) || (!b && (adj_bugs == 1 || adj_bugs == 2));
        if has_bug {
            res[p] = b'#'
        }
    }
    res
}

fn big_step(boards: &[Board<u8>]) -> Vec<Board<u8>> {
    let dim = boards[0].dim();
    let mut res = vec![Board::new(dim, b'.'); boards.len()];
    let dim = Point(dim.0 as i64, dim.1 as i64);

    for (board_idx, board) in boards.iter().enumerate() {
        for (p, &bug) in board.iter() {
            if p == dim / 2 {
                continue;
            }
            let adj_bugs = neighbors(dim, board_idx as i64, p)
                .into_iter()
                .filter_map(|(idx, p)| {
                    if 0 <= idx && idx < boards.len() as i64 {
                        Some(boards[idx as usize][p])
                    } else {
                        None
                    }
                })
                .filter(|&it| it == b'#')
                .count();
            let b = bug == b'#';
            let has_bug = (b && adj_bugs == 1) || (!b && (adj_bugs == 1 || adj_bugs == 2));
            if has_bug {
                res[board_idx][p] = b'#'
            }
        }
    }

    res
}

fn neighbors(dim: Point, idx: i64, point: Point) -> Vec<(i64, Point)> {
    let mid = dim / 2;
    let mut res = Vec::new();
    for &d in Direction::ALL.iter() {
        let p = point + d.delta();
        if p == mid {
            #[rustfmt::skip]
            let side = match d {
                Direction::Up    => [Point(0, 4), Point(1, 4), Point(2, 4), Point(3, 4), Point(4, 4)],
                Direction::Down  => [Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0), Point(4, 0)],
                Direction::Right => [Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3), Point(0, 4)],
                Direction::Left  => [Point(4, 0), Point(4, 1), Point(4, 2), Point(4, 3), Point(4, 4)],
            };
            res.extend(side.iter().map(|&p| (idx + 1, p)));
        } else if p.0 < 0 {
            res.push((idx - 1, Point(1, 2)));
        } else if p.0 == dim.0 {
            res.push((idx - 1, Point(3, 2)));
        } else if p.1 < 0 {
            res.push((idx - 1, Point(2, 1)));
        } else if p.1 == dim.1 {
            res.push((idx - 1, Point(2, 3)));
        } else {
            assert!(0 <= p.0 && p.0 < dim.0 && 0 <= p.1 && p.1 < dim.1);
            res.push((idx, p));
            continue;
        }
    }
    res
}
