fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/10/input.txt")?;
    let mut v: Vec<_> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    v.push(0);
    v.sort();
    let mut ones = 0;
    let mut threes = 1;
    let diff = (1..v.len()).map(|i| v[i] - v[i - 1]);
    for x in diff {
        if x == 3 {
            threes += 1;
        } else if x == 1 {
            ones += 1;
        }
    }
    println!("{}", ones * threes);

    // Part 2
    // a_i element at position i
    // c_i = distinct ways in the sequence a_0 ... a_i
    // c_i = c_{i-1} + set of c_k so that a_i - a_k <= 3 , k < n
    let mut c: Vec<u64> = vec![0, 1, 1];
    for (i, a_i) in v.iter().enumerate().skip(2) {
        let j = i + 1;
        let mut c_j = c[j - 1];
        if a_i - v[i - 2] <= 3 {
            c_j += c[j - 2];
        }
        if i != 2 && a_i - v[i - 3] <= 3 {
            c_j += c[j - 3];
        }
        c.push(c_j);
    }
    println!("{}", c.last().unwrap());
    Ok(())
}
