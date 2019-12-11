use std::io::{stdin, Read};

use aoc::{parse_memory, IntCode, Result, MemIo, extend_memory};

fn main() -> Result<()> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;

    let mut mem = parse_memory(&buf)?;
    extend_memory(&mut mem);

    let mut io = MemIo::new(vec![2]);
    let computer = IntCode::new(&mut io, &mut mem);
    computer.run()?;
    println!("{:?}", io.into_output());

    Ok(())
}

#[test]
fn test_examples() {
    fn run(mut prog: Vec<i64>) -> Vec<i64> {
        extend_memory(&mut prog);
        let mut io = aio::MemIo::new(vec![]);
        let computer = IntCode::new(&mut io, &mut prog);
        computer.run().unwrap();
        io.into_output()
    }

    assert_eq!(
        run(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]),
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,]
    );
    assert_eq!(
        run(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]),
        vec![1219070632396864],
    );
    assert_eq!(run(vec![104, 1125899906842624, 99]), vec![1125899906842624],)
}
