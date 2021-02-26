fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/5/input.txt")?;
    let seats: Vec<_> = input
        .lines()
        .map(|line| {
            u32::from_str_radix(
                &line
                    .chars()
                    .map(|ch| match ch {
                        'B' | 'R' => '1',
                        _ => '0',
                    })
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
        .collect();
    let max = seats.iter().max().unwrap();
    println!("{}", max);

    // part 2
    let missing = (*seats.iter().min().unwrap()..*max)
        .find(|x| !seats.contains(x))
        .unwrap();
    println!("{}", missing);
    Ok(())
}
