use aoc::{parse_memory, read_stdin_to_string, IntCode, Result, StdIo};

fn main() -> Result<()> {
    let line = read_stdin_to_string()?;

    let memory = parse_memory(line.as_str())?;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            let memory = run(memory);
            if memory[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                return Ok(());
            }
        }
    }
    panic!("no answer")
}

fn run(mut mem: Vec<i64>) -> Vec<i64> {
    let mut io = StdIo::new();
    IntCode::new(&mut io, &mut mem).run().unwrap();
    mem
}

#[test]
fn smoke() {
    assert_eq!(
        run(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    assert_eq!(run(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(run(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(run(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
}
