use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");

    let mut item_ids: Vec<String> = dex.items.keys()
        .map(|id| id.to_string())
        .collect();

    item_ids.sort();

    for id in &item_ids {
        println!("{}", id);
    }
}
