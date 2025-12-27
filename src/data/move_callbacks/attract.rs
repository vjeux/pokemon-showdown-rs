//! Attract Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target, source) {
///     return (target.gender === 'M' && source.gender === 'F') || (target.gender === 'F' && source.gender === 'M');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    use crate::dex_data::Gender;

    // Only works if target and source have opposite genders
    let opposite_genders = (target.gender == Gender::Male && source.gender == Gender::Female) ||
                          (target.gender == Gender::Female && source.gender == Gender::Male);

    EventResult::Bool(opposite_genders)
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     if (!(pokemon.gender === 'M' && source.gender === 'F') && !(pokemon.gender === 'F' && source.gender === 'M')) {
    ///         this.debug('incompatible gender');
    ///         return false;
    ///     }
    ///     if (!this.runEvent('Attract', pokemon, source)) {
    ///         this.debug('Attract event failed');
    ///         return false;
    ///     }
    /// 
    ///     if (effect.name === 'Cute Charm') {
    ///         this.add('-start', pokemon, 'Attract', '[from] ability: Cute Charm', `[of] ${source}`);
    ///     } else if (effect.name === 'Destiny Knot') {
    ///         this.add('-start', pokemon, 'Attract', '[from] item: Destiny Knot', `[of] ${source}`);
    ///     } else {
    ///         this.add('-start', pokemon, 'Attract');
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onUpdate(pokemon) {
    ///     if (this.effectState.source && !this.effectState.source.isActive && pokemon.volatiles['attract']) {
    ///         this.debug(`Removing Attract volatile on ${pokemon}`);
    ///         pokemon.removeVolatile('attract');
    ///     }
    /// }
    pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     this.add('-activate', pokemon, 'move: Attract', '[of] ' + this.effectState.source);
    ///     if (this.randomChance(1, 2)) {
    ///         this.add('cant', pokemon, 'Attract');
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Attract', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
