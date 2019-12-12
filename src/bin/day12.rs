use std::{cmp::Ordering, mem, ops};

use aoc::gcd;

fn main() {
    // let mut system = System::new(vec![
    //     Body::new([-13, -13, -13]),
    //     Body::new([5, -8, 3]),
    //     Body::new([-6, -10, -3]),
    //     Body::new([0, 5, -5]),
    // ]);

    let systems = vec![
        System2::new(vec![-13, 5, -6, 0]),
        System2::new(vec![-13, -8, -10, 5]),
        System2::new(vec![-13, 3, -3, -5]),
    ];
    let mut res = 1;
    for s in systems {
        let c = s.cycle_len() as i64;
        res = res * c / gcd(res, c);
    }
    println!("{}", res);
}

type Vec3 = [i64; 3];

#[derive(PartialEq, Eq, Clone, Copy)]
struct BodyId(usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Body {
    pos: Vec3,
    vel: Vec3,
}

impl Body {
    fn new(pos: Vec3) -> Body {
        Body {
            pos,
            vel: Vec3::default(),
        }
    }

    fn energy(&self) -> u64 {
        fn abs_sum(v: Vec3) -> u64 {
            (v[0].abs() + v[1].abs() + v[2].abs()) as u64
        }
        abs_sum(self.pos) * abs_sum(self.vel)
    }
}

struct System {
    bodies: Vec<Body>,
    buf: Vec<Body>,
}

impl System {
    fn new(bodies: Vec<Body>) -> System {
        let buf = Vec::with_capacity(bodies.len());
        System { bodies, buf }
    }
    fn energy(&self) -> u64 {
        self.all().map(|id| self[id].energy()).sum()
    }
    fn step(&mut self) {
        self.buf = {
            let mut buf = mem::replace(&mut self.buf, Vec::default());
            buf.extend(self.all().map(|idx| self.step_body(idx)));
            buf
        };
        mem::swap(&mut self.buf, &mut self.bodies);
        self.buf.clear();
    }
    fn step_body(&self, id: BodyId) -> Body {
        let mut res = self[id];

        for neighbor in self.all().filter(|it| *it != id) {
            for axis in 0..3 {
                let dv = match self[id].pos[axis].cmp(&self[neighbor].pos[axis]) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
                res.vel[axis] += dv;
            }
        }

        for axis in 0..3 {
            res.pos[axis] += res.vel[axis];
        }

        res
    }
    fn all(&self) -> impl Iterator<Item = BodyId> {
        (0..self.bodies.len()).map(BodyId)
    }
}

impl ops::Index<BodyId> for System {
    type Output = Body;
    fn index(&self, index: BodyId) -> &Body {
        &self.bodies[index.0]
    }
}

#[derive(Clone)]
struct System2 {
    pos: Vec<i64>,
    vel: Vec<i64>,
}

impl System2 {
    fn new(pos: Vec<i64>) -> System2 {
        let vel = vec![0; pos.len()];
        System2 { pos, vel }
    }

    fn step(&mut self) {
        let n = self.pos.len();
        for i in 0..n {
            let dv: i64 = (0..n)
                .filter(|&it| it != i)
                .map(|j| match self.pos[i].cmp(&self.pos[j]) {
                    Ordering::Less => 1i64,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                })
                .sum();
            self.vel[i] += dv;
        }
        for (p, v) in self.pos.iter_mut().zip(self.vel.iter()) {
            *p += *v;
        }
    }
    fn cycle_len(&self) -> u64 {
        let mut sim = self.clone();
        for idx in 1u64.. {
            sim.step();
            if sim.pos == self.pos && sim.vel == self.vel {
                return idx;
            }
        }
        panic!()
    }
}

#[test]
#[rustfmt::skip]
fn test_example() {
    let mut system = System::new(vec![
        Body::new([-1,   0,  2]),
        Body::new([ 2, -10, -7]),
        Body::new([ 4,  -8,  8]),
        Body::new([ 3,   5, -1]),
    ]);
    assert_eq!(&system.bodies, &vec![
        Body { pos: [-1,   0,  2], vel: [ 0,  0,  0] },
        Body { pos: [ 2, -10, -7], vel: [ 0,  0,  0] },
        Body { pos: [ 4,  -8,  8], vel: [ 0,  0,  0] },
        Body { pos: [ 3,   5, -1], vel: [ 0,  0,  0] },
    ]);

    system.step();
    assert_eq!(&system.bodies, &vec![
        Body { pos: [ 2,  -1,  1], vel: [ 3, -1, -1] },
        Body { pos: [ 3,  -7, -4], vel: [ 1,  3,  3] },
        Body { pos: [ 1,  -7,  5], vel: [-3,  1, -3] },
        Body { pos: [ 2,   2,  0], vel: [-1, -3,  1] },
    ]);

    system.step();
    assert_eq!(&system.bodies, &vec![
        Body { pos: [ 5,  -3, -1], vel: [ 3, -2, -2] },
        Body { pos: [ 1,  -2,  2], vel: [-2,  5,  6] },
        Body { pos: [ 1,  -4, -1], vel: [ 0,  3, -6] },
        Body { pos: [ 1,  -4,  2], vel: [-1, -6,  2] },
    ]);
}

#[test]
#[rustfmt::skip]
fn test_example_energy() {
    let mut system = System::new(vec![
        Body::new([-1,   0,  2]),
        Body::new([ 2, -10, -7]),
        Body::new([ 4,  -8,  8]),
        Body::new([ 3,   5, -1]),
    ]);
    for _ in 0..10 {
        system.step();
    }
    assert_eq!(system.energy(), 179)
}
