//! Tera Blast Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (pokemon.terastallized === 'Stellar') {
///         return 100;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onPrepareHit(target, source, move) {
///     if (source.terastallized) {
///         this.attrLastMove('[anim] Tera Blast ' + source.teraType);
///     }
/// }
pub fn on_prepare_hit(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyType(move, pokemon, target) {
///     if (pokemon.terastallized) {
///         move.type = pokemon.teraType;
///     }
/// }
pub fn on_modify_type(
    _battle: &mut Battle,
    _move_id: &str,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     if (pokemon.terastallized && pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) {
///         move.category = 'Physical';
///     }
///     if (pokemon.terastallized === 'Stellar') {
///         move.self = { boosts: { atk: -1, spa: -1 } };
///     }
/// }
pub fn on_modify_move(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
