/// Check unsorted items order in Rust

use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");

    // Get items WITHOUT sorting
    let all_items: Vec<_> = dex.items.keys().map(|id| id.as_str()).collect();

    println!("Total items: {}", all_items.len());
    println!("First 20 items (unsorted): {:?}", &all_items[0..20.min(all_items.len())]);
    println!();

    // Find specific items
    println!("Index of firiumz: {:?}", all_items.iter().position(|&x| x == "firiumz"));
    println!("Index of fightiniumz: {:?}", all_items.iter().position(|&x| x == "fightiniumz"));
}
