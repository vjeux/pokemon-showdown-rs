//! Own Tempo Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.volatiles['confusion']) {
///         this.add('-activate', pokemon, 'ability: Own Tempo');
///         pokemon.removeVolatile('confusion');
///     }
/// }
pub fn on_update(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryAddVolatile(status, pokemon) {
///     if (status.id === 'confusion') return null;
/// }
pub fn on_try_add_volatile(_battle: &mut Battle, status_id: &str, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    if status_id == "confusion" {
        return EventResult::Null;
    }
    EventResult::Continue
}

/// onHit(target, source, move) {
///     if (move?.volatileStatus === 'confusion') {
///         this.add('-immune', target, 'confusion', '[from] ability: Own Tempo');
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if let Some(ref volatile_status) = move_data.volatile_status {
            if volatile_status.as_str() == "confusion" {
                let target_ident = {
                    let target = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    target.get_slot()
                };

                battle.add(
                    "-immune",
                    &[
                        target_ident.as_str().into(),
                        "confusion".into(),
                        "[from] ability: Own Tempo".into(),
                    ],
                );
            }
        }
    }
    EventResult::Continue
}

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Own Tempo', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(_battle: &mut Battle, _boost: &str, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

