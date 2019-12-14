use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use aoc::Result;

fn main() -> Result<()> {
    let scheme = parse(&mut stdin().lock())?;
    let res = solve(&scheme, "FUEL");
    println!("{}", res);
    Ok(())
}

struct Reaction {
    reagents: Vec<(u64, String)>,
    amount: u64,
}

type Scheme = HashMap<String, Reaction>;

fn parse(rdr: &mut impl BufRead) -> Result<Scheme> {
    let mut res = HashMap::new();
    for line in rdr.lines() {
        let line = line?;

        let (reagents, target) = split_at(&line, "=>").ok_or("invalid rule")?;
        let (amount, tgt_name) = parse_ingredient(target)?;
        let reagents = reagents
            .split(", ")
            .map(parse_ingredient)
            .collect::<Result<Vec<_>>>()?;

        let prev = res.insert(tgt_name, Reaction { reagents, amount });
        assert!(prev.is_none());
    }
    Ok(res)
}

fn parse_ingredient(s: &str) -> Result<(u64, String)> {
    let res = _parse_ingredient(s).ok_or("invalid ingredient")?;
    Ok(res)
}
fn _parse_ingredient(s: &str) -> Option<(u64, String)> {
    let mut words = s.split_ascii_whitespace();
    let amount = words.next()?.parse::<u64>().ok()?;
    let name = words.next()?.to_string();
    if words.next().is_some() {
        return None;
    }
    Some((amount, name))
}

fn split_at<'a>(s: &'a str, sep: &str) -> Option<(&'a str, &'a str)> {
    let idx = s.find(sep)?;
    Some((&s[..idx], &s[idx + sep.len()..]))
}

fn solve<'a>(scheme: &'a Scheme, target: &'a str) -> u64 {
    let mut work: HashMap<&'a str, i64> = HashMap::new();
    work.insert(target, -1);

    loop {
        let (chemical, amount) = match work.iter().find(|(k, v)| **v < 0 && **k != "ORE") {
            Some((k, v)) => (*k, -v as u64),
            _ => return -work["ORE"] as u64,
        };
        let reaction = &scheme[chemical];
        let multiplier = (amount + reaction.amount - 1) / reaction.amount;
        for (amount, reagent) in reaction.reagents.iter() {
            *work.entry(reagent.as_str()).or_default() -= (amount * multiplier) as i64
        }
        *work.get_mut(chemical).unwrap() += (multiplier * reaction.amount) as i64;
        assert!(work[chemical] >= 0)
    }
}
