use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::for_gen(1).unwrap();
    
    let mut species_count = 0;
    let mut formes_count = 0;
    let mut total_checked = 0;
    
    for (id, pkmn) in dex.species.iter() {
        if !(pkmn.exists && pkmn.is_nonstandard.is_none() && pkmn.tier.as_deref() != Some("Illegal")) {
            continue;
        }
        
        total_checked += 1;
        
        if pkmn.is_cosmetic_forme {
            continue;
        }
        
        let base = pkmn.base_species.as_ref().unwrap_or(&pkmn.name);
        if &pkmn.name != base {
            formes_count += 1;
            if formes_count <= 5 {
                println!("Forme {}: {} (base: {})", formes_count, pkmn.name, base);
            }
        } else {
            species_count += 1;
            if species_count <= 5 || species_count > 146 {
                println!("Species {}: {} (num: {}, tier: {:?})", species_count, pkmn.name, pkmn.num, pkmn.tier);
            }
        }
    }
    
    println!("\nTotal checked: {}", total_checked);
    println!("Species: {}", species_count);
    println!("Formes: {}", formes_count);
}
