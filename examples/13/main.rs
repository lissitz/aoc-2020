fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/13/input.txt")?;
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse::<u64>().unwrap());
    let minutes = buses.map(|x| (x - (timestamp % x), x)).min().unwrap();
    println!("{}", minutes.1 * minutes.0);

    // Part 2
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let buses: Vec<(usize, u64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .map(|(i, x)| (i, x.parse::<u64>().unwrap()))
        .collect();

    // All ids are primes so they are pairwise coprimes, so we can use the chinese remainder theorem
    // to solve the set of linear congruences
    let product: u64 = buses.iter().map(|(_, x)| x).product();
    let solution: u64 = buses
        .iter()
        .map(|(i, x)| {
            let rem = x - (*i as u64 % x);
            let m = product / x;
            // the second factor could be found using the extended Euclidean algorithm
            rem * ((1..).find(|y| (y * m) % x == 1)).unwrap() * m
        })
        .sum();
    println!("{}", solution % product);

    Ok(())
}
