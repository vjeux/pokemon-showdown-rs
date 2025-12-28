//! Floral Healing Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     let success = false;
///     if (this.field.isTerrain('grassyterrain')) {
///         success = !!this.heal(this.modify(target.baseMaxhp, 0.667));
///     } else {
///         success = !!this.heal(Math.ceil(target.baseMaxhp * 0.5));
///     }
///     if (success && !target.isAlly(source)) {
///         target.staleness = 'external';
///     }
///     if (!success) {
///         this.add('-fail', target, 'heal');
///         return this.NOT_FAIL;
///     }
///     return success;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let success = false;
    // if (this.field.isTerrain('grassyterrain')) {
    //     success = !!this.heal(this.modify(target.baseMaxhp, 0.667));
    // } else {
    //     success = !!this.heal(Math.ceil(target.baseMaxhp * 0.5));
    // }
    let is_grassy_terrain = battle.field.is_terrain(&ID::from("grassyterrain"));

    let heal_amount = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let base_max_hp = target_pokemon.base_maxhp;

        if is_grassy_terrain {
            // this.modify(target.baseMaxhp, 0.667) - multiply by 2/3
            battle.modify_value(base_max_hp, 0.667)
        } else {
            // Math.ceil(target.baseMaxhp * 0.5) - 50% rounded up
            ((base_max_hp as f64 * 0.5).ceil()) as i32
        }
    };

    let success = battle.heal(heal_amount, target, None, None);

    // if (success && !target.isAlly(source)) {
    //     target.staleness = 'external';
    // }
    if success {
        let is_ally = battle.is_ally(target, source);
        if !is_ally {
            let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.staleness = Some("external".to_string());
        }
    }

    // if (!success) {
    //     this.add('-fail', target, 'heal');
    //     return this.NOT_FAIL;
    // }
    if !success {
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-fail", &[target_arg, "heal".into()]);

        return EventResult::NotFail;
    }

    // return success;
    EventResult::Boolean(success)
}

