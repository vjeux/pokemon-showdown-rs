/// Check where extra species come from in Rust

use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");

    println!("Total species in Rust dex: {}", dex.species.len());

    // Get all species names
    let mut all_names: Vec<_> = dex.species.values().map(|s| s.name.as_str()).collect();
    all_names.sort();

    // Count duplicates
    let unique_names: std::collections::HashSet<_> = all_names.iter().collect();
    println!("Unique species names: {}", unique_names.len());

    if all_names.len() != unique_names.len() {
        println!("\nFound duplicates!");
        let mut seen = std::collections::HashSet::new();
        for name in &all_names {
            if !seen.insert(name) {
                println!("  Duplicate: {}", name);
            }
        }
    }

    // Check species by ID vs name
    println!("\nFirst 20 species:");
    let mut species: Vec<_> = dex.species.iter().collect();
    species.sort_by_key(|(id, _)| id.as_str());

    for (i, (id, spec)) in species.iter().take(20).enumerate() {
        println!("  {}: id='{}', name='{}'", i, id.as_str(), spec.name);
    }
}
