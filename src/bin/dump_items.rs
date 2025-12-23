use pokemon_showdown::data::items::ITEMS;

fn main() {
    let mut item_ids: Vec<String> = ITEMS.keys()
        .map(|id| id.to_string())
        .collect();
    
    item_ids.sort();
    
    for id in &item_ids {
        println!("{}", id);
    }
}
