use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::for_gen(1).unwrap();
    
    let mut counted = 0;
    let mut skipped_no_tier = 0;
    let mut skipped_nonstandard = 0;
    let mut skipped_illegal = 0;
    let mut skipped_not_exists = 0;
    
    for (id, pkmn) in dex.species.iter() {
        if pkmn.num > 151 {
            continue; // Skip non-Gen 1 Pokemon
        }
        
        if !pkmn.exists {
            skipped_not_exists += 1;
            continue;
        }
        
        if pkmn.is_nonstandard.is_some() {
            skipped_nonstandard += 1;
            continue;
        }
        
        if pkmn.tier.as_deref() == Some("Illegal") {
            skipped_illegal += 1;
            continue;
        }
        
        if pkmn.is_cosmetic_forme {
            continue;
        }
        
        let base = pkmn.base_species.as_ref().unwrap_or(&pkmn.name);
        if &pkmn.name == base {
            counted += 1;
            if counted <= 5 || counted > 146 {
                println!("Species {}: {} (num: {}, tier: {:?})", counted, pkmn.name, pkmn.num, pkmn.tier);
            }
        }
    }
    
    println!("\nCounted: {}", counted);
    println!("Skipped (no exists): {}", skipped_not_exists);
    println!("Skipped (is_nonstandard): {}", skipped_nonstandard);
    println!("Skipped (tier Illegal): {}", skipped_illegal);
}
