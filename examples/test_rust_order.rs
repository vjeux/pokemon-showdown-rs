/// Check the order of species/moves/items/natures in Rust

use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");

    // Get all species
    let mut all_species: Vec<_> = dex.species.values().collect();
    all_species.sort_by_key(|s| &s.name);
    println!("=== Rust HashMap/Vec Orders ===\n");
    println!("Total species: {}", all_species.len());
    print!("First 10 species: ");
    for (i, species) in all_species.iter().take(10).enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", species.name);
    }
    println!("\n");

    // Get all moves
    let mut all_moves: Vec<_> = dex.moves.keys().map(|id| id.as_str()).collect();
    all_moves.sort();
    println!("Total moves: {}", all_moves.len());
    print!("First 10 moves: ");
    for (i, move_id) in all_moves.iter().take(10).enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", move_id);
    }
    println!("\n");

    // Get all items
    let mut all_items: Vec<_> = dex.items.keys().map(|id| id.as_str()).collect();
    all_items.sort();
    println!("Total items: {}", all_items.len());
    print!("First 10 items: ");
    for (i, item_id) in all_items.iter().take(10).enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", item_id);
    }
    println!("\n");

    // Get all natures
    let mut all_natures: Vec<_> = dex.natures.keys().map(|id| id.as_str()).collect();
    all_natures.sort();
    println!("Total natures: {}", all_natures.len());
    print!("All natures: ");
    for (i, nature_id) in all_natures.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", nature_id);
    }
    println!();
}
