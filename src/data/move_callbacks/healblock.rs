//! Heal Block Move
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

    /// durationCallback(target, source, effect) {
    ///     if (effect?.name === "Psychic Noise") {
    ///         return 2;
    ///     }
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Heal Block');
    ///         return 7;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onStart(pokemon, source) {
    ///     this.add('-start', pokemon, 'move: Heal Block');
    ///     source.moveThisTurnResult = true;
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (this.dex.moves.get(moveSlot.id).flags['heal']) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (move.flags['heal'] && !move.isZ && !move.isMax) {
    ///         this.add('cant', pokemon, 'move: Heal Block', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onModifyMove(move, pokemon, target) {
    ///     if (move.flags['heal'] && !move.isZ && !move.isMax) {
    ///         this.add('cant', pokemon, 'move: Heal Block', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'move: Heal Block');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onTryHeal(damage, target, source, effect) {
    ///     if (effect && (effect.id === 'zpower' || (effect as Move).isZ)) return damage;
    ///     if (source && target !== source && target.hp !== target.maxhp && effect.name === "Pollen Puff") {
    ///         this.attrLastMove('[still]');
    ///         // FIXME: Wrong error message, correct one not supported yet
    ///         this.add('cant', source, 'move: Heal Block', effect);
    ///         return null;
    ///     }
    ///     return false;
    /// }
    pub fn on_try_heal(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onRestart(target, source, effect) {
    ///     if (effect?.name === 'Psychic Noise') return;
    /// 
    ///     this.add('-fail', target, 'move: Heal Block'); // Succeeds to suppress downstream messages
    ///     if (!source.moveThisTurnResult) {
    ///         source.moveThisTurnResult = false;
    ///     }
    /// }
    pub fn on_restart(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
