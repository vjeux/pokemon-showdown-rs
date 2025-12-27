//! Doodle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     let success: boolean | null = false;
///     if (!target.getAbility().flags['failroleplay']) {
///         for (const pokemon of source.alliesAndSelf()) {
///             if (pokemon.ability === target.ability || pokemon.getAbility().flags['cantsuppress']) continue;
///             const oldAbility = pokemon.setAbility(target.ability, null, move);
///             if (oldAbility) {
///                 success = true;
///             } else if (!success && oldAbility === null) {
///                 success = null;
///             }
///         }
///     }
///     if (!success) {
///         if (success === false) {
///             this.add('-fail', source);
///         }
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // Get source and target
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let success: boolean | null = false;
    // Using Option<bool> where None represents null, Some(false) represents false, Some(true) represents true
    let mut success: Option<bool> = Some(false);

    // if (!target.getAbility().flags['failroleplay']) {
    let target_ability = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_ability(battle)
    };

    if !target_ability.flags.contains_key("failroleplay") {
        // for (const pokemon of source.alliesAndSelf()) {
        let allies_and_self = battle.get_allies_and_self(source);

        for ally_pos in allies_and_self {
            // if (pokemon.ability === target.ability || pokemon.getAbility().flags['cantsuppress']) continue;
            let (ally_ability, cant_suppress) = {
                let ally_pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let ability = ally_pokemon.get_ability(battle);
                (ally_pokemon.ability.clone(), ability.flags.contains_key("cantsuppress"))
            };

            if ally_ability == target_ability.id || cant_suppress {
                continue;
            }

            // const oldAbility = pokemon.setAbility(target.ability, null, move);
            let old_ability = {
                let ally_pokemon = match battle.pokemon_at_mut(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                // setAbility returns Option<ID> - Some(old_ability) if successful, None if failed
                ally_pokemon.set_ability(target_ability.id.clone(), None, None)
            };

            // if (oldAbility) {
            //     success = true;
            // } else if (!success && oldAbility === null) {
            //     success = null;
            // }
            if old_ability.is_some() {
                success = Some(true);
            } else if success == Some(false) {
                // oldAbility === null case
                success = None;
            }
        }
    }

    // if (!success) {
    //     if (success === false) {
    //         this.add('-fail', source);
    //     }
    //     this.attrLastMove('[still]');
    //     return this.NOT_FAIL;
    // }
    if success != Some(true) {
        // if (success === false)
        if success == Some(false) {
            // this.add('-fail', source);
            let source_arg = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(source_pokemon)
            };
            battle.add("-fail", &[source_arg]);
        }

        // this.attrLastMove('[still]');
        battle.attr_last_move("[still]");

        // return this.NOT_FAIL;
        return EventResult::Bool(true);
    }

    EventResult::Continue
}

