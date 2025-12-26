use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::load_default().unwrap(); // Load without forGen
    
    let mut gen1_with_nonstandard = 0;
    
    for (id, pkmn) in dex.species.iter() {
        if pkmn.num >= 1 && pkmn.num <= 151 {
            if let Some(ref ns) = pkmn.is_nonstandard {
                gen1_with_nonstandard += 1;
                if gen1_with_nonstandard <= 10 {
                    println!("Gen 1 Pokemon {} (num: {}) has isNonstandard: {}", pkmn.name, pkmn.num, ns);
                }
            }
        }
    }
    
    println!("\nTotal Gen 1 Pokemon (num 1-151) with isNonstandard: {}", gen1_with_nonstandard);
}
