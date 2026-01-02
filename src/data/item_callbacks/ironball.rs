//! Iron Ball Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onEffectiveness(typeMod, target, type, move) {
///     if (!target) return;
///     if (target.volatiles['ingrain'] || target.volatiles['smackdown'] || this.field.getPseudoWeather('gravity')) return;
///     if (move.type === 'Ground' && target.hasType('Flying')) return 0;
/// }
pub fn on_effectiveness(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!target) return;
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.volatiles['ingrain'] || target.volatiles['smackdown'] || this.field.getPseudoWeather('gravity')) return;
    let has_grounding_effect = {
        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        use crate::dex_data::ID;
        pokemon.volatiles.contains_key(&ID::from("ingrain"))
            || pokemon.volatiles.contains_key(&ID::from("smackdown"))
            || battle.field.get_pseudo_weather(&ID::from("gravity")).is_some()
    };

    if has_grounding_effect {
        return EventResult::Continue;
    }

    // if (move.type === 'Ground' && target.hasType('Flying')) return 0;
    let (move_type, target_has_flying) = {
        let move_type = battle.active_move.as_ref().map(|m| m.move_type.clone()).unwrap_or_default();

        let target_has_flying = {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_type(battle, "Flying")
        };

        (move_type, target_has_flying)
    };

    if move_type == "Ground" && target_has_flying {
        // return 0;
        return EventResult::Number(0);
    }

    EventResult::Continue
}

/// onModifySpe(spe) {
///     return this.chainModify(0.5);
/// }
pub fn on_modify_spe(battle: &mut Battle) -> EventResult {
    // return this.chainModify(0.5);
    battle.chain_modify(0.5);
    EventResult::Continue
}
