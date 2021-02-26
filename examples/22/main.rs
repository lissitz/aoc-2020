use std::collections::VecDeque;
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/22/input.txt")?;
    let initial_decks: Vec<VecDeque<_>> = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|line| line.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let mut decks = initial_decks.clone();
    let total_cards: usize = decks.iter().map(|x| x.len()).sum();
    while !decks.iter().any(|x| x.len() == total_cards) {
        let mut max = (0, 0);
        let mut cards = VecDeque::new();
        for (i, deck) in decks.iter_mut().enumerate() {
            let n = deck.pop_front().unwrap();
            cards.push_back(n);
            if n > max.1 {
                max = (i, n);
            }
        }
        let i = cards.iter().position(|x| x == &max.1).unwrap();
        cards.swap(0, i);
        decks[max.0].append(&mut cards);
    }
    let score = |decks: &Vec<Deck>| {
        decks
            .iter()
            .find(|x| !x.is_empty())
            .unwrap()
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + (i as u64 + 1) * x)
    };
    println!("{:?}", score(&decks));

    // Part 2
    let (_, decks) = game_init(initial_decks);
    println!("{:?}", score(&decks));
    Ok(())
}

type Deck = VecDeque<u64>;
type Round = Vec<Deck>;
type Winner = usize;

fn game_init(decks: Round) -> (Winner, Round) {
    game(decks, &mut Vec::new())
}

fn game(decks: Round, prev_rounds: &mut Vec<Round>) -> (Winner, Round) {
    let mut decks = decks;
    let total_cards: usize = decks.iter().map(|x| x.len()).sum();
    let mut winner = 0;
    while !decks.iter().any(|x| x.len() == total_cards) {
        if prev_rounds.iter().any(|x| deep_equal(x, &decks)) {
            winner = 0;
            break;
        }
        prev_rounds.push(decks.clone());
        let (winner_round, next_decks) = round(decks);
        decks = next_decks;
        winner = winner_round;
    }
    (winner, decks)
}

// This should use a hash function and a HashSet
fn deep_equal(a: &Round, b: &Round) -> bool {
    a.iter()
        .zip(b)
        .all(|(deck_a, deck_b)| deck_a.iter().zip(deck_b).all(|(x, y)| x == y))
}

fn round(decks: Vec<Deck>) -> (Winner, Round) {
    let mut decks = decks.clone();
    let winner_round = {
        if decks.iter().all(|deck| deck.len() > deck[0] as usize) {
            let (winner, _) = game(
                decks
                    .iter()
                    .map(|x| x.iter().skip(1).take(x[0] as usize).cloned().collect())
                    .collect(),
                &mut Vec::new(),
            );
            winner
        } else {
            decks.iter().enumerate().max_by_key(|x| x.1).unwrap().0
        }
    };
    let mut cards = Deck::new();
    decks[winner_round].rotate_left(1);
    for (i, deck) in decks.iter_mut().enumerate() {
        if i != winner_round {
            cards.push_back(deck.pop_front().unwrap());
        }
    }
    decks[winner_round].append(&mut cards);
    (winner_round, decks)
}
