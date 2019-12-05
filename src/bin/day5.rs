use aoc::{parse_memory, IntCode, Io, Result, StdIo};

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

#[test]
fn test_examples() {
    struct MemIo {
        input: Vec<i64>,
        output: Vec<i64>,
    }

    impl MemIo {
        fn new(mut input: Vec<i64>) -> Self {
            input.reverse();
            Self {
                input,
                output: Vec::new(),
            }
        }
    }

    impl Io for MemIo {
        fn read(&mut self) -> Result<i64> {
            let res = self.input.pop().ok_or("EOF")?;
            Ok(res)
        }
        fn write(&mut self, value: i64) -> Result<()> {
            self.output.push(value);
            Ok(())
        }
    }

    fn check(memory: Vec<i64>, tests: Vec<(i64, i64)>) {
        for (i, o) in tests {
            let mut mem = memory.clone();
            let mut io = MemIo::new(vec![i]);
            let computer = IntCode::new(&mut io, &mut mem);
            computer.run().unwrap();
            assert_eq!(
                io.output,
                vec![o],
                "\nmemory: {:?}\noutput: {:?},\ni: {}\no: {}\n",
                memory,
                io.output,
                i,
                o
            )
        }
    }

    // check(
    //     vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
    //     vec![(0, 0), (7, 0), (8, 1), (92, 0)],
    // );
    // check(
    //     vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
    //     vec![(0, 1), (7, 1), (8, 0), (92, 0)],
    // );
    // check(
    //     vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
    //     vec![(0, 0), (7, 0), (8, 1), (92, 0)],
    // );
    // check(
    //     vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
    //     vec![(0, 1), (7, 1), (8, 0), (92, 0)],
    // );

    check(
        vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        vec![(-1, 1), (0, 0), (1, 1), (92, 1)],
    );
    check(
        vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        vec![(-1, 1), (0, 0), (1, 1), (92, 1)],
    );

    check(
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ],
        vec![(0, 999), (7, 999), (8, 1000), (92, 1001)],
    );
}
