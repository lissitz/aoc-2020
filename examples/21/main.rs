use std::collections::{HashSet, VecDeque};
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/21/input.txt")?;
    let mut lists: Vec<IngredientList> = input
        .lines()
        .map(|line| {
            let mut it = line.split("(contains").map(|x| x.trim());
            let ingredients = it
                .next()
                .unwrap()
                .split(' ')
                .map(|x| x.to_string())
                .collect();
            let allergens = it
                .next()
                .unwrap()
                .trim_end_matches(')')
                .split(',')
                .map(|x| x.trim().to_string())
                .collect();
            IngredientList {
                ingredients,
                allergens,
            }
        })
        .collect();

    let allergens: HashSet<String> = lists
        .iter()
        .flat_map(|list| list.allergens.clone())
        .collect();

    let ingredients_may_contain_allergens = allergens
        .iter()
        .flat_map(|allergen| {
            let mut it = lists
                .iter()
                .filter(|list| list.allergens.contains(allergen))
                .map(|x| &x.ingredients);
            let mut intersection = it.next().unwrap().clone();
            for x in it {
                intersection = intersection.intersection(x).cloned().collect();
            }
            intersection
        })
        .collect();
    let n: usize = lists
        .iter()
        .map(|x| {
            x.ingredients
                .difference(&ingredients_may_contain_allergens)
                .count()
        })
        .sum();
    println!("{}", n);

    // Part 2
    let mut entries = Vec::new();
    let mut allergens_v: VecDeque<_> = allergens.iter().collect();
    while let Some(allergen) = allergens_v.pop_back() {
        let mut it = lists
            .iter()
            .filter(|list| list.allergens.contains(allergen))
            .map(|x| &x.ingredients);
        let mut intersection = it.next().unwrap().clone();
        for x in it {
            intersection = intersection.intersection(x).cloned().collect();
        }
        if intersection.len() == 1 {
            let ingredient = intersection.iter().cloned().next().unwrap();
            entries.push((allergen, ingredient.clone()));
            for list in &mut lists {
                list.allergens.remove(allergen);
                list.ingredients.remove(&ingredient);
            }
        } else {
            allergens_v.push_front(allergen);
        }
    }
    entries.sort();
    let solution = entries
        .into_iter()
        .map(|x| x.1)
        .collect::<Vec<_>>()
        .join(",");
    println!("{:?}", solution);
    Ok(())
}

#[derive(Clone, Debug)]
struct IngredientList {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}
