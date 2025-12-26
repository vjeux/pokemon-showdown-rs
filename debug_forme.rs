use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::for_gen(1).unwrap();
    
    // Check specific Alola formes
    let sandslash_alola = dex.get_species("Sandslash-Alola").unwrap();
    println!("Sandslash-Alola: forme={:?}, tier={:?}, isNonstandard={:?}", 
             sandslash_alola.forme, sandslash_alola.tier, sandslash_alola.is_nonstandard);
    
    let meowth_alola = dex.get_species("Meowth-Alola").unwrap();
    println!("Meowth-Alola: forme={:?}, tier={:?}, isNonstandard={:?}",
             meowth_alola.forme, meowth_alola.tier, meowth_alola.is_nonstandard);
}
