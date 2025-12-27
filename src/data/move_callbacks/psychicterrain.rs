//! Psychic Terrain Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// durationCallback(source, effect) {
    ///     if (source?.hasItem('terrainextender')) {
    ///         return 8;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onTryHit(target, source, effect) {
    ///     if (effect && (effect.priority <= 0.1 || effect.target === 'self')) {
    ///         return;
    ///     }
    ///     if (target.isSemiInvulnerable() || target.isAlly(source)) return;
    ///     if (!target.isGrounded()) {
    ///         const baseMove = this.dex.moves.get(effect.id);
    ///         if (baseMove.priority > 0) {
    ///             this.hint("Psychic Terrain doesn't affect Pokémon immune to Ground.");
    ///         }
    ///         return;
    ///     }
    ///     this.add('-activate', target, 'move: Psychic Terrain');
    ///     return null;
    /// }
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize), move_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Psychic' && attacker.isGrounded() && !attacker.isSemiInvulnerable()) {
    ///         this.debug('psychic terrain boost');
    ///         return this.chainModify([5325, 4096]);
    ///     }
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldStart(field, source, effect) {
    ///     if (effect?.effectType === 'Ability') {
    ///         this.add('-fieldstart', 'move: Psychic Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
    ///     } else {
    ///         this.add('-fieldstart', 'move: Psychic Terrain');
    ///     }
    /// }
    pub fn on_field_start(battle: &mut Battle, field_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Psychic Terrain');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
