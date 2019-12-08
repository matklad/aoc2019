use std::io::{stdin, Read};

fn main() -> aoc::Result<()> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;

    let image = buf.trim().as_bytes();
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let n_layers = image.len() / width * height;
    let mut chunks = image.chunks_exact(layer_size);
    let mut res = vec![2; layer_size];
    for chunk in chunks.by_ref().rev() {
        for (dst, &src) in res.iter_mut().zip(chunk.iter()) {
            if src != b'2' {
                *dst = src
            }
        }
    }
    assert!(chunks.remainder().is_empty());
    let mut buf = String::new();
    for row in res.chunks_exact(width) {
        buf.clear();
        for pixel in row.iter() {
            let c = match pixel {
                b'0' => '.',
                b'1' => 'X',
                c => panic!("uncovered pixel {}", c),
            };
            buf.push(c);
        }
        println!("{}", buf)
    }
    Ok(())
}

fn count(chunk: &[u8], digit: u8) -> usize {
    assert!(b'0' <= digit && digit <= b'9');
    chunk.iter().filter(|&&b| b == digit).count()
}
