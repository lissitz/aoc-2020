fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/25/input.txt")?;
    let keys: Vec<_> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let N = 20201227;
    let subject = 7;
    let mut loop_size = None;
    'outer: for (j, p) in keys.iter().cloned().enumerate() {
        let mut value = 1;
        for i in 1..100000000 as usize {
            value = (value * subject) % N;
            if value == p {
                loop_size = Some((j, i));
                break 'outer;
            }
        }
    }
    let loop_size = loop_size.unwrap();
    let mut it = keys
        .iter()
        .cloned()
        .enumerate()
        .filter(|x| x.0 != loop_size.0);
    let subject = it.next().unwrap().1;
    let mut value = 1;
    for _ in 0..loop_size.1 {
        value = (value * subject) % N;
    }
    println!("{:?}", loop_size);
    println!("{:?}", value);
    Ok(())
}
