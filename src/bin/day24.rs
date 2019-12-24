use std::collections::HashSet;

use aoc::Board;

fn main() {
    let board = Board::from_ascii(
        "\
#.#.#
.#...
...#.
.###.
###.#",
    );

    let board = solve(
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
