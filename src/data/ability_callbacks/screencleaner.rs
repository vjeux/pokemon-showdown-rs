//! Screen Cleaner Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(pokemon) {
///     let activated = false;
///     for (const sideCondition of ['reflect', 'lightscreen', 'auroraveil']) {
///         for (const side of [pokemon.side, ...pokemon.side.foeSidesWithConditions()]) {
///             if (side.getSideCondition(sideCondition)) {
///                 if (!activated) {
///                     this.add('-activate', pokemon, 'ability: Screen Cleaner');
///                     activated = true;
///                 }
///                 side.removeSideCondition(sideCondition);
///             }
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // let activated = false;
    let mut activated = false;

    // Get pokemon's side index
    let pokemon_side = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.side_index
    };

    // const sides = [pokemon.side, ...pokemon.side.foeSidesWithConditions()]
    let mut sides_to_check = vec![pokemon_side];
    let foe_sides = {
        let pokemon_side_ref = &battle.sides[pokemon_side];
        pokemon_side_ref.foe_sides_with_conditions(&battle.sides)
    };
    for foe_side in foe_sides {
        sides_to_check.push(foe_side.n);
    }

    // for (const sideCondition of ['reflect', 'lightscreen', 'auroraveil'])
    let side_conditions = ["reflect", "lightscreen", "auroraveil"];

    for condition_str in side_conditions.iter() {
        let condition_id = ID::from(*condition_str);

        // for (const side of [pokemon.side, ...pokemon.side.foeSidesWithConditions()])
        for &side_index in &sides_to_check {
            // if (side.getSideCondition(sideCondition))
            let has_condition = battle.sides[side_index].get_side_condition(&condition_id).is_some();

            if has_condition {
                // if (!activated)
                if !activated {
                    // this.add('-activate', pokemon, 'ability: Screen Cleaner');
                    let pokemon_slot = {
                        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        pokemon.get_slot()
                    };

                    battle.add("-activate", &[
                        Arg::String(pokemon_slot),
                        Arg::Str("ability: Screen Cleaner"),
                    ]);

                    // activated = true;
                    activated = true;
                }

                // side.removeSideCondition(sideCondition);
                battle.sides[side_index].remove_side_condition(&condition_id);
            }
        }
    }

    EventResult::Continue
}

