/// Export filtered species list from Rust

use pokemon_showdown::Dex;
use std::fs;

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");

    let mut species: Vec<_> = dex.species.values()
        .filter(|s| !s.is_cosmetic_forme)
        .map(|s| s.name.as_str())
        .collect();
    species.sort();

    let output = species.join("\n");
    fs::write("rust-species-list.txt", output).expect("Failed to write file");

    println!("Wrote {} species to rust-species-list.txt", species.len());
}
