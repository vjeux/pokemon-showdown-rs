//! Gravity Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;
use super::{Status, Effect};



// Condition handlers
pub mod condition {
    use super::*;

    /// durationCallback(source, effect) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Gravity');
    ///         return 7;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(battle: &mut Battle, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldStart(target, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-fieldstart', 'move: Gravity', '[persistent]');
    ///     } else {
    ///         this.add('-fieldstart', 'move: Gravity');
    ///     }
    ///     for (const pokemon of this.getAllActive()) {
    ///         let applies = false;
    ///         if (pokemon.removeVolatile('bounce') || pokemon.removeVolatile('fly')) {
    ///             applies = true;
    ///             this.queue.cancelMove(pokemon);
    ///             pokemon.removeVolatile('twoturnmove');
    ///         }
    ///         if (pokemon.volatiles['skydrop']) {
    ///             applies = true;
    ///             this.queue.cancelMove(pokemon);
    /// 
    ///             if (pokemon.volatiles['skydrop'].source) {
    ///                 this.add('-end', pokemon.volatiles['twoturnmove'].source, 'Sky Drop', '[interrupt]');
    ///             }
    ///             pokemon.removeVolatile('skydrop');
    ///             pokemon.removeVolatile('twoturnmove');
    ///         }
    ///         if (pokemon.volatiles['magnetrise']) {
    ///             applies = true;
    ///             delete pokemon.volatiles['magnetrise'];
    ///         }
    ///         if (pokemon.volatiles['telekinesis']) {
    ///             applies = true;
    ///             delete pokemon.volatiles['telekinesis'];
    ///         }
    ///         if (applies) this.add('-activate', pokemon, 'move: Gravity');
    ///     }
    /// }
    pub fn on_field_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onModifyAccuracy(accuracy) {
    ///     if (typeof accuracy !== 'number') return;
    ///     return this.chainModify([6840, 4096]);
    /// }
    pub fn on_modify_accuracy(battle: &mut Battle, accuracy: i32) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (this.dex.moves.get(moveSlot.id).flags['gravity']) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (move.flags['gravity'] && !move.isZ) {
    ///         this.add('cant', pokemon, 'move: Gravity', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onModifyMove(move, pokemon, target) {
    ///     if (move.flags['gravity'] && !move.isZ) {
    ///         this.add('cant', pokemon, 'move: Gravity', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Gravity');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
