use std::collections::HashMap;
fn main() {
    for m in &[2020, 30000000] {
        let mut input: Vec<usize> = vec![0, 1, 5, 10, 3, 12, 19];
        input.reserve(*m);
        let n = input.len();
        let mut map: HashMap<usize, usize> = HashMap::new();
        for (i, x) in input.iter().enumerate() {
            map.insert(*x, i);
        }
        for i in (n - 1)..(m - 1) {
            let entry = map.entry(input[i]).or_insert(i);
            let v = *entry;
            *entry = i;
            let next = i - v;
            input.push(next);
        }
        println!("{:?}", input.last().unwrap());
    }
}
