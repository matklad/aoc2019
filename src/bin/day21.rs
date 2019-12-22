use aoc::{parse_memory, AsciiIo, IntCode, Result};

fn main() -> Result<()> {
    let prog = std::fs::read_to_string("./input/day21.in")?;
    let mut prog = parse_memory(&prog)?;
    let mut io = AsciiIo::new();
    let cpu = IntCode::new(&mut io, &mut prog);
    cpu.run()?;
    Ok(())
}
