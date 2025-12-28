//! Mind Reader Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     if (source.volatiles['lockon']) return false;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), _target_pos: (usize, usize)) -> EventResult {
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
///     this.add('-activate', source, 'move: Mind Reader', `[of] ${target}`);
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
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.add_volatile(ID::from("lockon"));
    }

    // this.add('-activate', source, 'move: Mind Reader', `[of] ${target}`);
    let (source_arg, target_arg) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.get_slot(), target_pokemon.get_slot())
    };

    battle.add("-activate", &[
        source_arg.into(),
        "move: Mind Reader".into(),
        format!("[of] {}", target_arg).into(),
    ]);

    EventResult::Continue
}

