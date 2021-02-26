use std::collections::HashSet;
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/6/input.txt")?;
    let s = input
        .split("\n\n")
        .map(|x| x.lines().flat_map(|x| x.chars()).collect::<HashSet<_>>());
    println!("{}", s.map(|x| x.len()).sum::<usize>());

    // Part 2
    let s = input.split("\n\n").map(|x| {
        x.lines()
            .map(|x| x.chars().collect::<HashSet<_>>())
            ._fold_first(|x, y| &x & &y)
            .unwrap()
    });
    println!("{}", s.map(|x| x.len()).sum::<usize>());
    Ok(())
}

trait FoldFirst: Iterator {
    fn _fold_first<F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item;
}

impl<I: Iterator> FoldFirst for I {
    fn _fold_first<F>(mut self, f: F) -> Option<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let initial = self.next()?;
        Some(self.fold(initial, f))
    }
}
