//! Oblivious Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.volatiles['attract']) {
///         this.add('-activate', pokemon, 'ability: Oblivious');
///         pokemon.removeVolatile('attract');
///         this.add('-end', pokemon, 'move: Attract', '[from] ability: Oblivious');
///     }
///     if (pokemon.volatiles['taunt']) {
///         this.add('-activate', pokemon, 'ability: Oblivious');
///         pokemon.removeVolatile('taunt');
///         // Taunt's volatile already sends the -end message when removed
///     }
/// }
pub fn on_update(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onImmunity(type, pokemon) {
///     if (type === 'attract') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, type_or_status: &str, _pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "attract" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

/// onTryHit(pokemon, target, move) {
///     if (move.id === 'attract' || move.id === 'captivate' || move.id === 'taunt') {
///         this.add('-immune', pokemon, '[from] ability: Oblivious');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), _source_pos: (usize, usize), move_id: &str) -> EventResult {
    if move_id == "attract" || move_id == "captivate" || move_id == "taunt" {
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            target.get_slot()
        };

        battle.add(
            "-immune",
            &[
                target_ident.as_str().into(),
                "[from] ability: Oblivious".into(),
            ],
        );
        return EventResult::Null;
    }
    EventResult::Continue
}

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Oblivious', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(_battle: &mut Battle, _boost: &str, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

