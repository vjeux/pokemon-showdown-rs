WARNING: Found inline has_type in roost.rs line 1: WARNING: Found inline has_type in roost.rs line 1: Fixed in roost.rs: has_flying = battle.has_type(target, "Flying")
  This needs manual fixing
WARNING: Found inline has_type in roost.rs line 3: Fixed in roost.rs: has_flying = battle.has_type(target, "Flying")
  This needs manual fixing
WARNING: Found inline has_type in roost.rs line 1: Fixed in roost.rs: has_flying = battle.has_type(target, "Flying")
  This needs manual fixing
Fixed in roost.rs: has_flying = battle.has_type(target, "Flying")
Total changes in roost.rs: 1
//! Roost Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     if (target.terastallized) {
    ///         if (target.hasType('Flying')) {
    ///             this.add('-hint', "If a Terastallized Pokemon uses Roost, it remains Flying-type.");
    ///         }
    ///         return false;
    ///     }
    ///     this.add('-singleturn', target, 'move: Roost');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.terastallized) {
        let terastallized = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.terastallized
        };

        if terastallized.is_some() {
            // if (target.hasType('Flying')) {
            //     this.add('-hint', "If a Terastallized Pokemon uses Roost, it remains Flying-type.");
            // }
            let has_flying = {
                let pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.has_type("Flying")
            };

            if has_flying {
                battle.add("-hint", &["If a Terastallized Pokemon uses Roost, it remains Flying-type.".into()]);
            }

            // return false;
            return EventResult::Boolean(false);
        }

        // this.add('-singleturn', target, 'move: Roost');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-singleturn", &[
            target_arg,
            "move: Roost".into(),
        ]);

        EventResult::Continue
    }

    /// onType(types, pokemon) {
    ///     this.effectState.typeWas = types;
    ///     return types.filter(type => type !== 'Flying');
    /// }
    pub fn on_type(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.effectState.typeWas = types;
        // return types.filter(type => type !== 'Flying');
        let types = battle.get_types(pokemon, false);

        let effect_state = match &mut battle.current_effect_state {
            Some(es) => es,
            None => return EventResult::Continue,
        };
        effect_state.data.insert("typeWas".to_string(), serde_json::json!(types.clone()));

        let filtered_types: Vec<String> = types.into_iter()
            .filter(|t| t != "Flying")
            .collect();

        EventResult::Types(filtered_types)
    }
}



