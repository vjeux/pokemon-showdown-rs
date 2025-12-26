use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::for_gen(4).unwrap();
    let mut formes = Vec::new();

    for (_id, pkmn) in dex.species.iter() {
        if !pkmn.exists { continue; }
        if pkmn.is_nonstandard.is_some() { continue; }
        if pkmn.tier.as_deref() == Some("Illegal") { continue; }
        if pkmn.is_cosmetic_forme { continue; }

        let base = pkmn.base_species.as_ref().unwrap_or(&pkmn.name);
        if &pkmn.name != base {
            formes.push((pkmn.name.clone(), pkmn.forme.clone(), pkmn.num, pkmn.gen));
        }
    }

    formes.sort();
    println!("Found {} formes in Gen 4:", formes.len());
    for (name, forme, num, gen) in formes {
        println!("  {} (forme: {:?}, num: {}, gen: {:?})", name, forme, num, gen);
    }
}
