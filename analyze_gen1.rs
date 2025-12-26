use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::for_gen(1).unwrap();
    
    let mut base_species = Vec::new();
    let mut formes = Vec::new();
    let mut filtered_out = Vec::new();
    
    for (_id, pkmn) in dex.species.iter() {
        // Only look at Gen 1 Pokemon (num 1-151)
        if pkmn.num < 1 || pkmn.num > 151 {
            continue;
        }
        
        // Apply the existence function
        let passes_filter = pkmn.exists && pkmn.is_nonstandard.is_none() && pkmn.tier.as_deref() != Some("Illegal");
        
        if !passes_filter {
            filtered_out.push((pkmn.name.clone(), pkmn.num, pkmn.tier.clone(), pkmn.is_nonstandard.clone()));
            continue;
        }
        
        if pkmn.is_cosmetic_forme {
            continue;
        }
        
        let base = pkmn.base_species.as_ref().unwrap_or(&pkmn.name);
        if &pkmn.name != base {
            formes.push((pkmn.name.clone(), pkmn.num));
        } else {
            base_species.push((pkmn.name.clone(), pkmn.num));
        }
    }
    
    println!("Base species count: {}", base_species.len());
    println!("Formes count: {}", formes.len());
    println!("Filtered out count: {}", filtered_out.len());
    
    println!("\nFirst 10 filtered out:");
    for (name, num, tier, nonstandard) in filtered_out.iter().take(10) {
        println!("  {} (num: {}, tier: {:?}, isNonstandard: {:?})", name, num, tier, nonstandard);
    }
    
    if base_species.len() < 151 {
        println!("\nMissing {} base species!", 151 - base_species.len());
        println!("\nChecking which nums 1-151 are missing as base species:");
        for num in 1..=151 {
            let found = base_species.iter().any(|(_, n)| *n == num);
            if !found {
                println!("  Missing num {}", num);
                // Check if it exists as a forme or filtered out
                let as_forme = formes.iter().find(|(_, n)| *n == num);
                let as_filtered = filtered_out.iter().find(|(_, n, _, _)| *n == num);
                if let Some((name, _)) = as_forme {
                    println!("    -> Found as forme: {}", name);
                }
                if let Some((name, _, tier, ns)) = as_filtered {
                    println!("    -> Filtered out: {} (tier: {:?}, isNonstandard: {:?})", name, tier, ns);
                }
            }
        }
    }
}
