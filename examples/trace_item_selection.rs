/// Trace item selection in Rust

use pokemon_showdown::{Dex, PRNG, PRNGSeed};

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, 1])));

    let mut all_species: Vec<_> = dex.species.values().collect();
    all_species.sort_by_key(|s| &s.name);

    let mut all_items: Vec<_> = dex.items.keys().map(|id| id.as_str()).collect();
    all_items.sort();

    // Call 1: species
    prng.sample(&all_species);

    // Call 2: level
    prng.random_range(50, 101);

    // Call 3: item
    println!("All items length: {}", all_items.len());
    let item = prng.sample(&all_items).expect("No items");
    println!("Selected item: {}", item);
    println!("Index of selected item: {:?}", all_items.iter().position(|&x| x == *item));
}
