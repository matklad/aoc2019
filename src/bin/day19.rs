use std::fs;

use aoc::{parse_memory, IntCode, MemIo, Point, Result};

fn main() -> Result<()> {
    let prog = fs::read_to_string("./input/day19.in")?;
    let prog = parse_memory(&prog)?;
    let ctx = Ctx { prog };

    let mut lo = 10;
    let mut hi = 10000;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if ctx.fit_at_y(mid).is_some() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let p = ctx.fit_at_y(lo).unwrap();

    println!("{}", p.0 * 10000 + p.1);
    Ok(())
}

struct Ctx {
    prog: Vec<i64>,
}

impl Ctx {
    fn is_covered(&self, point: Point) -> bool {
        let mut prog = self.prog.clone();
        let mut cpu = IntCode::new(MemIo::new(vec![point.0, point.1]), &mut prog);
        cpu.run().unwrap();
        let output = cpu.io.into_output();
        assert!(output.len() == 1);
        output[0] == 1
    }

    fn covers_ship(&self, point: Point) -> bool {
        let size = 100 - 1;
        let points = [
            point,
            point + Point(size, 0),
            point + Point(0, size),
            point + Point(size, size),
        ];
        points.iter().all(|&p| self.is_covered(p))
    }

    fn point_on_beam_at_y(&self, y: i64) -> Point {
        assert!(y >= 10);
        let x = (y as f64 * 1.22) as i64;
        let p = Point(x, y);
        assert!(self.is_covered(p));
        p
    }

    fn range_at_y(&self, y: i64) -> (Point, Point) {
        let p = self.point_on_beam_at_y(y);
        let mut low = p;
        while self.is_covered(low - Point(1, 0)) {
            low.0 -= 1;
        }
        let mut hi = p;
        while self.is_covered(hi + Point(1, 0)) {
            hi.0 += 1;
        }
        (low, hi)
    }

    fn fit_at_y(&self, y: i64) -> Option<Point> {
        let (min_p, max_p) = self.range_at_y(y);
        for x in min_p.0..=max_p.0 {
            let p = Point(x, min_p.1);
            if self.covers_ship(p) {
                return Some(p);
            }
        }
        None
    }
}
