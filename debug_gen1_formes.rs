use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::for_gen(1).unwrap();

    let mut formes = Vec::new();

    for (_id, pkmn) in dex.species.iter() {
        // Apply existence function
        if !pkmn.exists {
            continue;
        }
        if pkmn.is_nonstandard.is_some() {
            continue;
        }
        if pkmn.tier.as_deref() == Some("Illegal") {
            continue;
        }

        if pkmn.is_cosmetic_forme {
            continue;
        }

        let base = pkmn.base_species.as_ref().unwrap_or(&pkmn.name);
        if &pkmn.name != base {
            formes.push((
                pkmn.name.clone(),
                pkmn.num,
                pkmn.forme.clone(),
                pkmn.gen,
                pkmn.is_nonstandard.clone(),
                pkmn.tier.clone()
            ));
        }
    }

    println!("Found {} formes in Gen 1:", formes.len());
    for (name, num, forme, gen, nonstandard, tier) in formes.iter().take(10) {
        println!("  {} (num: {}, forme: {:?}, gen: {:?}, isNonstandard: {:?}, tier: {:?})",
                 name, num, forme, gen, nonstandard, tier);
    }
}
