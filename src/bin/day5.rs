use aoc::{parse_memory, IntCode, Result, StdIo};

fn main() -> Result<()> {
    let memory = std::env::args().nth(1).ok_or("no memory specified")?;
    let memory = if memory.ends_with(".in") {
        std::fs::read_to_string(memory)?
    } else {
        memory.to_string()
    };
    let mut memory = parse_memory(memory.as_str())?;
    let mut io = StdIo::new();
    let computer = IntCode::new(&mut io, &mut memory);
    computer.run()
}
