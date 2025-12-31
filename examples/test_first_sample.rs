/// Trace first species selection in Rust

use pokemon_showdown::{Dex, PRNG, PRNGSeed};

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, 1])));

    let mut all_species: Vec<_> = dex.species.values().collect();
    all_species.sort_by_key(|s| &s.name);

    println!("Total species: {}", all_species.len());
    println!("\n=== First PRNG call (for species selection) ===");

    let index = prng.random_range(0, all_species.len() as i32) as usize;
    println!("Calculated index: {}", index);
    println!("Species at that index: {}", all_species[index].name);

    println!("\nSpecies around that index:");
    let start = if index >= 2 { index - 2 } else { 0 };
    let end = (index + 3).min(all_species.len());
    for i in start..end {
        println!("  [{}]: {}{}", i, all_species[i].name, if i == index { " <-- selected" } else { "" });
    }
}
