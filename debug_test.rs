use pokemon_showdown::Dex;

fn main() {
    let dex = Dex::for_gen(1).unwrap();
    
    // Check a few Gen 1 Pokemon
    let pikachu = dex.get_species("Pikachu").unwrap();
    println!("Pikachu: tier={:?}, isNonstandard={:?}, exists={}", 
             pikachu.tier, pikachu.is_nonstandard, pikachu.exists);
    
    let bulbasaur = dex.get_species("Bulbasaur").unwrap();
    println!("Bulbasaur: tier={:?}, isNonstandard={:?}, exists={}",
             bulbasaur.tier, bulbasaur.is_nonstandard, bulbasaur.exists);
    
    let chikorita = dex.get_species("Chikorita").unwrap();
    println!("Chikorita: tier={:?}, isNonstandard={:?}, exists={}",
             chikorita.tier, chikorita.is_nonstandard, chikorita.exists);
    
    // Count Pokemon with no tier
    let no_tier = dex.species.values().filter(|s| s.tier.is_none()).count();
    println!("Pokemon with no tier: {}", no_tier);
    
    let illegal_tier = dex.species.values().filter(|s| s.tier.as_deref() == Some("Illegal")).count();
    println!("Pokemon with tier Illegal: {}", illegal_tier);
    
    // Test the existence function
    let count = dex.species.values().filter(|s| {
        s.exists && s.is_nonstandard.is_none() && s.tier.as_deref() != Some("Illegal")
    }).count();
    println!("Pokemon matching existence function: {}", count);
}
