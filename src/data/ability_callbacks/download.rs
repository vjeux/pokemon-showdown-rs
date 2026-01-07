//! Download Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     let totaldef = 0;
///     let totalspd = 0;
///     for (const target of pokemon.foes()) {
///         totaldef += target.getStat('def', false, true);
///         totalspd += target.getStat('spd', false, true);
///     }
///     if (totaldef && totaldef >= totalspd) {
///         this.boost({ spa: 1 });
///     } else if (totalspd) {
///         this.boost({ atk: 1 });
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::dex_data::StatID;

    // let totaldef = 0;
    // let totalspd = 0;
    // for (const target of pokemon.foes())
    let foe_positions = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.foes(battle, false) // don't include fainted
    };

    let mut total_def = 0;
    let mut total_spd = 0;

    for foe_pos in foe_positions {
        // totaldef += target.getStat('def', false, true);
        // totalspd += target.getStat('spd', false, true);
        total_def += battle.get_pokemon_stat(foe_pos, StatID::Def, false, true);
        total_spd += battle.get_pokemon_stat(foe_pos, StatID::SpD, false, true);
    }

    // if (totaldef && totaldef >= totalspd) {
    //     this.boost({ spa: 1 });
    // } else if (totalspd) {
    //     this.boost({ atk: 1 });
    // }
    if total_def > 0 && total_def >= total_spd {
        battle.boost(&[("spa", 1)], pokemon_pos, Some(pokemon_pos), None, false, false);
    } else if total_spd > 0 {
        battle.boost(&[("atk", 1)], pokemon_pos, Some(pokemon_pos), None, false, false);
    }

    EventResult::Continue
}

