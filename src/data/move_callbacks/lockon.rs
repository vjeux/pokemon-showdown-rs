//! Lock-On Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     if (source.volatiles['lockon']) return false;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (source.volatiles['lockon']) return false;
    let has_lockon = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.volatiles.contains_key(&ID::from("lockon"))
    };

    if has_lockon {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     source.addVolatile('lockon', target);
///     this.add('-activate', source, 'move: Lock-On', `[of] ${target}`);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source.addVolatile('lockon', target);
    {

        let pokemon = match battle.pokemon_at_mut(source.0, source.1) {

            Some(p) => p,

            None => return EventResult::Continue,

        };

        pokemon.add_volatile(ID::from("lockon"));

    }

    // this.add('-activate', source, 'move: Lock-On', `[of] ${target}`);
    let source_arg = {

        let pokemon = match battle.pokemon_at(source.0, source.1) {

            Some(p) => p,

            None => return EventResult::Continue,

        };

        crate::battle::Arg::from(pokemon)

    };
    let target_arg = {

        let pokemon = match battle.pokemon_at(target.0, target.1) {

            Some(p) => p,

            None => return EventResult::Continue,

        };

        crate::battle::Arg::from(pokemon)

    };
    battle.add("-activate", &[source_arg, "move: Lock-On".into(), format!("[of] {}", target_arg).into()]);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSourceInvulnerability(target, source, move) {
    ///     if (move && source === this.effectState.target && target === this.effectState.source) return 0;
    /// }
    pub fn on_source_invulnerability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (move && source === this.effectState.target && target === this.effectState.source) return 0;
        let (effect_target, effect_source) = match &battle.current_effect_state {
            Some(es) => (es.target, es.source),
            None => return EventResult::Continue,
        };

        if Some(source) == effect_target && Some(target) == effect_source {
            return EventResult::Number(0);
        }

        EventResult::Continue
    }

    /// onSourceAccuracy(accuracy, target, source, move) {
    ///     if (move && source === this.effectState.target && target === this.effectState.source) return true;
    /// }
    pub fn on_source_accuracy(battle: &mut Battle, accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (move && source === this.effectState.target && target === this.effectState.source) return true;
        let (effect_target, effect_source) = match &battle.current_effect_state {
            Some(es) => (es.target, es.source),
            None => return EventResult::Continue,
        };

        if Some(source) == effect_target && Some(target) == effect_source {
            return EventResult::Boolean(true);
        }

        EventResult::Continue
    }
}
