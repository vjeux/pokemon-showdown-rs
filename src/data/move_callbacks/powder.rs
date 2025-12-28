//! Powder Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'Powder');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', target, 'Powder');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-singleturn", &[target_arg, "Powder".into()]);

        EventResult::Continue
    }

    /// onTryMove(pokemon, target, move) {
    ///     if (move.type === 'Fire') {
    ///         this.add('-activate', pokemon, 'move: Powder');
    ///         this.damage(this.clampIntRange(Math.round(pokemon.maxhp / 4), 1));
    ///         this.attrLastMove('[still]');
    ///         return false;
    ///     }
    /// }
    pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        let pokemon = source_pos;

        // if (move.type === 'Fire') {
        let move_type = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.move_type.clone()
        };

        if move_type.as_deref() == Some("Fire") {
            // this.add('-activate', pokemon, 'move: Powder');
            let pokemon_arg = {
                let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(pokemon_pokemon)
            };

            battle.add("-activate", &[pokemon_arg, "move: Powder".into()]);

            // this.damage(this.clampIntRange(Math.round(pokemon.maxhp / 4), 1));
            let damage = {
                let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                let calculated_damage = (pokemon_pokemon.maxhp + 2) / 4; // Math.round equivalent for integer division
                std::cmp::max(calculated_damage, 1)
            };

            battle.damage(damage, pokemon, None, None);

            // this.attrLastMove('[still]');
            battle.attr_last_move(&["[still]"]);

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }
}
