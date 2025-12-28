//! Glaive Rush Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singlemove', pokemon, 'Glaive Rush', '[silent]');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-singlemove', pokemon, 'Glaive Rush', '[silent]');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };
        battle.add("-singlemove", &[pokemon_arg, "Glaive Rush".into(), "[silent]".into()]);

        EventResult::Continue
    }

    /// onAccuracy() {
    ///     return true;
    /// }
    pub fn on_accuracy(battle: &mut Battle, accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // return true;
        EventResult::Boolean(true)
    }

    /// onSourceModifyDamage() {
    ///     return this.chainModify(2);
    /// }
    pub fn on_source_modify_damage(battle: &mut Battle) -> EventResult {
        // return this.chainModify(2);
        EventResult::Number(battle.chain_modify(2.0 as f32))
    }

    /// onBeforeMove(pokemon) {
    ///     this.debug('removing Glaive Rush drawback before attack');
    ///     pokemon.removeVolatile('glaiverush');
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // this.debug('removing Glaive Rush drawback before attack');
        battle.debug("removing Glaive Rush drawback before attack");

        // pokemon.removeVolatile('glaiverush');
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.remove_volatile(&ID::from("glaiverush"));

        EventResult::Continue
    }
}
