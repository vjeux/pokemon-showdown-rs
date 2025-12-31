/// Check Grubbin gender ratio in Rust

use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");

    let mut all_species: Vec<_> = dex.species.values().collect();
    all_species.sort_by_key(|s| &s.name);

    let grubbin = all_species.iter().find(|s| s.name == "Grubbin").expect("Grubbin not found");

    println!("Grubbin data:");
    println!("- Name: {}", grubbin.name);
    println!("- Gender ratio: {:?}", grubbin.gender_ratio);

    if let Some(ref ratio) = grubbin.gender_ratio {
        println!("  - M: {}", ratio.m);
        println!("  - F: {}", ratio.f);
        println!("- Has both genders? {}", ratio.m > 0.0 && ratio.f > 0.0);
    } else {
        println!("- No gender ratio (genderless)");
    }
}
