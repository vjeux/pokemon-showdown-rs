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
    pub fn duration_callback(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
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
    pub fn on_try_hit(
        _battle: &mut Battle,
        _source_pos: (usize, usize),
        _target_pos: (usize, usize),
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Psychic' && attacker.isGrounded() && !attacker.isSemiInvulnerable()) {
    ///         this.debug('psychic terrain boost');
    ///         return this.chainModify([5325, 4096]);
    ///     }
    /// }
    pub fn on_base_power(
        _battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
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
    pub fn on_field_start(
        _battle: &mut Battle,
        _field_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Psychic Terrain');
    /// }
    pub fn on_field_end(_battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
