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
    let (chunk, _n_zeros) = chunks
        .by_ref()
        .map(|it| (it, count(it, b'0')))
        .min_by_key(|(_, n_zeros)| *n_zeros)
        .unwrap();
    assert!(chunks.remainder().is_empty());

    let res = count(chunk, b'1') * count(chunk, b'2');
    println!("{}", res);
    Ok(())
}

fn count(chunk: &[u8], digit: u8) -> usize {
    assert!(b'0' <= digit && digit <= b'9');
    chunk.iter().filter(|&&b| b == digit).count()
}
