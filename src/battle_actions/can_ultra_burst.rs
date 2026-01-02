//! BattleActions::canUltraBurst - Check if Pokemon can Ultra Burst
//!
//! 1:1 port of canUltraBurst from battle-actions.ts

use crate::*;

/// Check if Pokemon can Ultra Burst
/// Equivalent to battle-actions.ts canUltraBurst()
///
/// canUltraBurst(pokemon: Pokemon) {
///     if (['Necrozma-Dawn-Wings', 'Necrozma-Dusk-Mane'].includes(pokemon.baseSpecies.name) &&
///         pokemon.getItem().id === 'ultranecroziumz') {
///         return "Necrozma-Ultra";
///     }
///     return null;
/// }
pub fn can_ultra_burst(
    battle: &Battle,
    side_index: usize,
    pokemon_index: usize,
) -> Option<String> {
    // Get pokemon
    let pokemon = match battle.sides.get(side_index)
        .and_then(|s| s.pokemon.get(pokemon_index)) {
        Some(p) => p,
        None => return None,
    };

    // Get base species data
    let species = match battle.dex.species().get(&pokemon.base_species.as_str()) {
        Some(s) => s,
        None => return None,
    };

    // if (['Necrozma-Dawn-Wings', 'Necrozma-Dusk-Mane'].includes(pokemon.baseSpecies.name) &&
    //     pokemon.getItem().id === 'ultranecroziumz') {
    //     return "Necrozma-Ultra";
    // }
    if (species.name == "Necrozma-Dawn-Wings" || species.name == "Necrozma-Dusk-Mane")
        && pokemon.item.as_str() == "ultranecroziumz"
    {
        return Some("Necrozma-Ultra".to_string());
    }

    // return null;
    None
}
