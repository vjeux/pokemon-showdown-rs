/// Check filtered species count

use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");

    // All species
    let all_count = dex.species.len();

    // Filtered (non-cosmetic)
    let mut filtered: Vec<_> = dex.species.values()
        .filter(|s| !s.is_cosmetic_forme)
        .collect();
    filtered.sort_by_key(|s| &s.name);

    println!("Total species in dex: {}", all_count);
    println!("Non-cosmetic species: {}", filtered.len());
    println!("Cosmetic formes: {}", all_count - filtered.len());

    println!("\nFirst 10 non-cosmetic species:");
    for (i, species) in filtered.iter().take(10).enumerate() {
        println!("  {}: {}", i, species.name);
    }
}
