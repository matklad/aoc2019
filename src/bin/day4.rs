use std::time::Instant;

type Code = [u8; 6];

fn main() {
    let t = Instant::now();
    for _ in 0..1000 {
        match std::env::args().nth(1).unwrap().as_str() {
            "1" => brute_force(236491, 713787),
            "2" => faster([2, 3, 6, 6, 6, 6], [7; 6]),
            "3" => faster_manual_inline([2, 3, 6, 6, 6, 6], [7; 6]),
            "4" => faster_increment([2, 3, 6, 6, 6, 6], [7; 6]),
            _ => panic!(),
        };
    }
    eprintln!("{:?}", t.elapsed());
}

fn to_code(mut code: u32) -> Code {
    let mut res: Code = [0; 6];
    for digit in res.iter_mut().rev() {
        *digit = (code % 10) as u8;
        code /= 10;
    }
    assert!(code == 0);
    res
}

fn verify(code: Code) -> bool {
    verify_increasing(code) && verify_repeat(code)
}

fn verify_increasing(code: Code) -> bool {
    let mut pairs = code.iter().copied().zip(code[1..].iter().copied());
    pairs.all(|(d1, d2)| d1 <= d2)
}

fn verify_repeat(code: Code) -> bool {
    for i in 0..=4 {
        if code[i] == code[i + 1]
            && (i == 0 || code[i - 1] != code[i])
            && (i == 4 || code[i + 2] != code[i])
        {
            return true;
        }
    }
    false
}

fn brute_force(low: u32, hi: u32) -> u32 {
    (low..=hi).map(to_code).filter(|&it| verify(it)).count() as u32
}

fn faster(low: Code, hi: Code) -> u32 {
    let mut code = low;
    match go(hi, &mut code, 0) {
        Err(it) => return it,
        Ok(_) => unreachable!(),
    }

    #[inline(always)]
    fn go(hi: Code, code: &mut Code, i: usize) -> Result<u32, u32> {
        if i < 6 {
            let mut res = 0;
            loop {
                match go(hi, code, i + 1) {
                    Ok(it) => res += it,
                    Err(it) => return Err(res + it),
                };
                if code[i] == 9 {
                    return Ok(res);
                }
                let next = code[i] + 1;
                code[i..].iter_mut().for_each(|it| *it = next);
            }
        } else {
            if *code == hi {
                return Err(0);
            }
            return Ok(verify_repeat(*code) as u32);
        }
    }
}

fn faster_manual_inline(low: Code, hi: Code) -> u32 {
    macro_rules! go {
        (x $($x:ident)*; $i:expr) => {
            (|code: &mut Code| -> Result<u32, u32> {
                let mut res = 0;
                loop {
                    match go!($($x)*; $i + 1)(code) {
                        Ok(it) => res += it,
                        Err(it) => return Err(res + it),
                    };
                    if code[$i] == 9 {
                        return Ok(res);
                    }
                    let next = code[$i] + 1;
                    code[$i..].iter_mut().for_each(|it| *it = next);
                }
            })
        };
        (; $i:expr) => {
            (|code: &mut Code| -> Result<u32, u32> {
                if *code == hi {
                    return Err(0);
                }
                return Ok(verify_repeat(*code) as u32);
            })
        };
    }

    let mut code = low;
    match go!(x x x x x x; 0)(&mut code) {
        Err(it) => return it,
        Ok(_) => unreachable!(),
    }
}

fn increment(code: &mut Code) {
    for i in (0..6).rev() {
        let c = code[i];
        if c < 9 {
            code[i..].iter_mut().for_each(|it| *it = c + 1);
            break;
        }
    }
}

fn faster_increment(lo: Code, hi: Code) -> u32 {
    let mut code = lo;
    let mut res = 0;
    loop {
        if verify_repeat(code) {
            res += 1;
        }
        if code == hi {
            return res;
        }
        increment(&mut code);
    }
}

#[test]
fn test_verify() {
    assert!(verify(to_code(111111)));
    assert!(!verify(to_code(223450)));
    assert!(!verify(to_code(123789)));
}
