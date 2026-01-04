//! Imposter Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     // Imposter does not activate when Skill Swapped or when Neutralizing Gas leaves the field
///     // Imposter copies across in doubles/triples
///     // (also copies across in multibattle and diagonally in free-for-all,
///     // but side.foe already takes care of those)
///     const target = pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
///     if (target) {
///         pokemon.transformInto(target, this.dex.abilities.get('imposter'));
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // const target = pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
    // Get the target position (mirror position on the foe's side)
    let (pokemon_side_idx, pokemon_pokemon_idx) = pokemon_pos;

    let (pokemon_position, foe_side_idx) = {
        let pokemon = match battle.pokemon_at(pokemon_side_idx, pokemon_pokemon_idx) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let foe_idx = 1 - pokemon_side_idx;
        (pokemon.position, foe_idx)
    };

    // Get the mirror position on the foe's side
    // JavaScript: pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position]
    let foe_active = battle.get_all_active(false);
    let foe_active_on_side: Vec<_> = foe_active.iter()
        .filter(|&&(side_idx, _)| side_idx == foe_side_idx)
        .collect();

    if foe_active_on_side.is_empty() {
        return EventResult::Continue;
    }

    // Mirror position: length - 1 - position
    let mirror_index = foe_active_on_side.len().saturating_sub(1).saturating_sub(pokemon_position);

    let target_pos = if let Some(&&pos) = foe_active_on_side.get(mirror_index) {
        pos
    } else {
        return EventResult::Continue;
    };

    // if (target) {
    //     pokemon.transformInto(target, this.dex.abilities.get('imposter'));
    // }
    let imposter_id = ID::from("imposter");
    Pokemon::transform_into(battle, pokemon_pos, target_pos, Some(&imposter_id));

    EventResult::Continue
}

