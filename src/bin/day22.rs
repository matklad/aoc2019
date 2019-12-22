use std::{collections::HashSet, fs, mem, ops, str::FromStr};

use aoc::{Error, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day22.in")?;
    let ops = parse(&input)?;

    let deck = run_deck(10007, &ops);
    eprintln!("{:?}", deck.iter().position(|it| *it == 2019));
    // 2322

    let pos = run_card(2019, 10007, &ops);
    eprintln!("pos = {:?}", pos);

    let lf = run_func(10007, &ops);
    eprintln!("{:?}", lf.apply(2019));

    let m = 119315717514047;
    let mut round = LinFunc { m, coefs: [0, 1] };
    for op in ops {
        round = op.as_lf(m).apply_to_lf(round)
    }

    let f = bin_exp(round, 101741582076661);
    let res = ((2020 - f.coefs[0]) * modulo_inverse(f.coefs[1], m)) % m;
    eprintln!("res = {:?}", res);
    Ok(())
}

fn bin_exp(lf: LinFunc, pow: u64) -> LinFunc {
    if pow == 0 {
        return lf.with([0, 1]);
    }
    let mut res = bin_exp(lf.apply_to_lf(lf), pow >> 1);
    if pow & 1 == 1 {
        res = res.apply_to_lf(lf)
    }
    res
}

fn modulo_inverse(x: i128, m: i128) -> i128 {
    let (gcd, [a, _b]) = egcd(x, m);
    assert!(gcd == 1);
    a
}

fn egcd(x: i128, y: i128) -> (i128, [i128; 2]) {
    if y == 0 {
        return (x, [1, 0]);
    }
    let (gcd, coefs) = egcd(y, x % y);
    (gcd, [coefs[1], coefs[0] - (x / y) * coefs[1]])
}

#[test]
fn test_egcd() {
    let (gcd, [a, b]) = egcd(639, 119315717514047);
    eprintln!("gcd = {:?}", gcd);
    eprintln!("{}", 639 * a + 119315717514047 * b)
}

fn parse(s: &str) -> Result<Vec<Op>> {
    s.trim().lines().map(Op::from_str).collect()
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Cut(isize),
    Deal(usize),
    Rev,
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Op> {
        if s == "deal into new stack" {
            return Ok(Op::Rev);
        }
        if let Some(i) = cut_prefix(s, "deal with increment ") {
            return Ok(Op::Deal(i.parse()?));
        }
        if let Some(i) = cut_prefix(s, "cut ") {
            return Ok(Op::Cut(i.parse()?));
        }
        Err("invalid op")?
    }
}

fn cut_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let cut = s.trim_start_matches(prefix);
    Some(cut).filter(|it| it.len() < s.len())
}

impl Op {
    fn apply_to_deck(self, stack: &[u16], buf: &mut [u16]) {
        match self {
            Op::Cut(n) => {
                let first = if n >= 0 {
                    n as usize
                } else {
                    stack.len() - (-n) as usize
                };
                let second = stack.len() - first;
                buf[..second].copy_from_slice(&stack[first..]);
                buf[second..].copy_from_slice(&stack[..first]);
            }
            Op::Deal(step) => {
                let mut dst = 0;
                for &x in stack.iter() {
                    buf[dst] = x;
                    dst += step;
                    dst %= buf.len();
                }
            }
            Op::Rev => {
                buf.copy_from_slice(stack);
                buf.reverse();
            }
        }
    }

    fn apply_to_card(self, card: u64, deck: u64) -> u64 {
        match self {
            Op::Cut(n) => {
                let cut = if n > 0 { n as u64 } else { deck - (-n) as u64 };
                if card < cut {
                    card + (deck - cut)
                } else {
                    card - cut
                }
            }
            Op::Deal(step) => card * (step as u64) % deck,
            Op::Rev => deck - card - 1,
        }
    }

    fn as_lf(self, m: i128) -> LinFunc {
        LinFunc {
            m,
            coefs: match self {
                Op::Cut(n) => [-(n as i128), 1],
                Op::Deal(step) => [0, step as i128],
                Op::Rev => [-1, -1],
            },
        }
    }

    fn apply_to_func(self, func: LinFunc) -> LinFunc {
        self.as_lf(func.m).apply_to_lf(func)
    }
}

#[derive(Clone, Copy)]
struct LinFunc {
    m: i128,
    coefs: [i128; 2],
}

impl LinFunc {
    fn with(self, coefs: [i128; 2]) -> LinFunc {
        fn n(x: i128, m: i128) -> i128 {
            ((x % m) + m) % m
        }

        LinFunc {
            m: self.m,
            coefs: [n(coefs[0], self.m), n(coefs[1], self.m)],
        }
    }

    fn apply(self, card: i128) -> i128 {
        (self.coefs[0] + self.coefs[1] * card) % self.m
    }

    fn apply_to_lf(self, lf: LinFunc) -> LinFunc {
        lf.with([
            lf.coefs[0] * self.coefs[1] + self.coefs[0],
            lf.coefs[1] * self.coefs[1],
        ])
    }
}

impl ops::Add<i128> for LinFunc {
    type Output = LinFunc;
    fn add(self, rhs: i128) -> LinFunc {
        self.with([self.coefs[0] + rhs, self.coefs[1]])
    }
}

impl ops::Mul<i128> for LinFunc {
    type Output = LinFunc;
    fn mul(self, rhs: i128) -> LinFunc {
        self.with([self.coefs[0] * rhs, self.coefs[1] * rhs])
    }
}

fn run_deck(n_cards: u16, ops: &[Op]) -> Vec<u16> {
    let mut deck = (0u16..n_cards).collect::<Vec<u16>>();
    let mut buf = vec![0u16; deck.len()];
    for &op in ops {
        op.apply_to_deck(&deck, &mut buf);
        mem::swap(&mut deck, &mut buf);
    }
    deck
}

fn run_card(mut card: u64, deck: u64, ops: &[Op]) -> u64 {
    for &op in ops {
        card = op.apply_to_card(card, deck)
    }
    card
}

fn run_func(deck: u64, ops: &[Op]) -> LinFunc {
    let mut lf = LinFunc {
        m: deck as i128,
        coefs: [0, 1],
    };
    for &op in ops {
        lf = op.apply_to_func(lf)
    }
    lf
}

#[test]
fn test_example() {
    let ops = parse(
        "\
deal with increment 7
deal into new stack
deal into new stack",
    )
    .unwrap();
    assert_eq!(run_deck(10, &ops), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

    let ops = parse(
        "\
cut 6
deal with increment 7
deal into new stack",
    )
    .unwrap();
    assert_eq!(run_deck(10, &ops), vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
}
