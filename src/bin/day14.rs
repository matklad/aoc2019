fn main() {
    let text = &aoc::read_stdin_to_string().unwrap();
    let formulas = parse(text);
    println!("{}", max_fuel(&formulas, 1000000000000));
}

fn min_ore(formulas: &[Vec<i64>]) -> i64 {
    binary_search(|ore| {
        let mut amount = vec![0; formulas[0].len()];
        amount[0] = ore;
        *amount.last_mut().unwrap() = -1;
        is_feasible(amount, formulas)
    })
}

fn max_fuel(formulas: &[Vec<i64>], ore: i64) -> i64 {
    binary_search(|fuel| {
        let mut amount = vec![0; formulas[0].len()];
        amount[0] = ore;
        *amount.last_mut().unwrap() = -fuel;
        !is_feasible(amount, formulas)
    }) - 1
}

fn binary_search(f: impl Fn(i64) -> bool) -> i64 {
    let mut hi = 1;
    while !f(hi) {
        hi *= 2;
    }
    let mut lo = 0;
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        if f(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    hi
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let mut table = Vec::new();
    for line in input.lines() {
        let sep = "=>";
        let idx = line.find(sep).unwrap();
        let mut row = vec![parse_reagent(&line[idx + sep.len()..])];
        for reagent in line[..idx].split(", ") {
            let (amount, chemical) = parse_reagent(reagent);
            row.push((-amount, chemical));
        }
        table.push(row);
    }

    let chemicals = {
        let mut all = table.iter().map(|it| it[0].1).collect::<Vec<_>>();
        all.push("ORE");
        let mut top_sorted = Vec::new();
        for &chem in all.iter() {
            top_sort(&mut top_sorted, &table, chem);
        }
        top_sorted
    };
    assert_eq!(chemicals[0], "ORE");
    assert_eq!(chemicals.last().unwrap(), &"FUEL");

    table.sort_by_key(|row| chemicals.iter().position(|&it| it == row[0].1));

    let mut res = Vec::new();
    for row in table {
        let mut r = vec![0; chemicals.len()];
        for (amount, chemical) in row {
            let idx = chemicals.iter().position(|&it| it == chemical).unwrap();
            r[idx] = amount;
        }
        res.push(r)
    }
    return res;

    fn top_sort<'a>(sorted: &mut Vec<&'a str>, table: &[Vec<(i64, &'a str)>], chem: &'a str) {
        if sorted.contains(&chem) {
            return;
        }

        for row in table {
            if row[0].1 == chem {
                for (_, dep) in row[1..].iter() {
                    top_sort(sorted, table, *dep);
                }
            }
        }

        sorted.push(chem);
    }
}

fn parse_reagent(input: &str) -> (i64, &str) {
    let mut words = input.split_ascii_whitespace();
    let amount = words.next().unwrap().parse::<i64>().unwrap();
    let chemical = words.next().unwrap();
    assert!(words.next().is_none());
    (amount, chemical)
}

fn is_feasible(mut amount: Vec<i64>, formulas: &[Vec<i64>]) -> bool {
    assert_eq!(amount.len(), formulas[0].len());
    for i in (1..amount.len()).rev() {
        if amount[i] < 0 {
            let multiplier = div_round_up(-amount[i], formulas[i - 1][i]);
            assert!(multiplier >= 0);
            for (dst, src) in amount.iter_mut().zip(formulas[i - 1].iter()) {
                *dst += src * multiplier;
            }
        }
        assert!(amount[i] >= 0);
    }
    amount[0] >= 0
}

fn div_round_up(divident: i64, divisor: i64) -> i64 {
    (divident + divisor - 1) / divisor
}

#[test]
fn test_is_feasible() {
    let formulas = parse(
        "10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL",
    );

    assert_eq!(is_feasible(vec![29, 0, 0, 0, 0, 0, -1], &formulas), false);
    assert_eq!(is_feasible(vec![30, 0, 0, 0, 0, 0, -1], &formulas), false);
    assert_eq!(is_feasible(vec![31, 0, 0, 0, 0, 0, -1], &formulas), true);
    assert_eq!(is_feasible(vec![32, 0, 0, 0, 0, 0, -1], &formulas), true);
}

#[test]
fn test_examples() {
    fn check(input: &str, amount: i64) {
        let formulas = parse(input);
        let res = min_ore(&formulas);
        assert_eq!(res, amount)
    };

    check(
        "10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL",
        31,
    );
    check(
        "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL",
        165,
    );
    check(
        "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        13312,
    );
    check(
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF",
        180697,
    );
    check(
        "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX",
        2210736,
    );
}

#[test]
fn test_examples_part_2() {
    fn check(input: &str, amount: i64) {
        let formulas = parse(input);
        let res = max_fuel(&formulas, 1_000_000_000_000);
        assert_eq!(res, amount)
    };

    check(
        "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        82892753,
    );
    check(
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF",
        5586022,
    );
    check(
        "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX",
        460664,
    );
}
