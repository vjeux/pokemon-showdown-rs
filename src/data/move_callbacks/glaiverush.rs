//! Glaive Rush Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singlemove', pokemon, 'Glaive Rush', '[silent]');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-singlemove', pokemon, 'Glaive Rush', '[silent]');
        let pokemon_ident = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };
        battle.add(
            "-singlemove",
            &[
                pokemon_ident.as_str().into(),
                "Glaive Rush".into(),
                "[silent]".into(),
            ],
        );

        EventResult::Continue
    }

    /// onAccuracy() {
    ///     return true;
    /// }
    pub fn on_accuracy(
        _battle: &mut Battle,
        _accuracy: i32,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        // return true;
        EventResult::Boolean(true)
    }

    /// onSourceModifyDamage() {
    ///     return this.chainModify(2);
    /// }
    pub fn on_source_modify_damage(battle: &mut Battle) -> EventResult {
        // return this.chainModify(2);
        { battle.chain_modify(2.0_f32); EventResult::Continue }
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
        Pokemon::remove_volatile(battle, pokemon, &ID::from("glaiverush"));

        EventResult::Continue
    }
}
