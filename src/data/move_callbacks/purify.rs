//! Purify Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (!target.cureStatus()) {
///         this.add('-fail', source);
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
///     this.heal(Math.ceil(source.maxhp * 0.5), source);
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!target.cureStatus()) {
    let (target_ident, target_name) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target_pokemon.get_slot(), target_pokemon.name.clone())
    };

    let target_mut = match battle.pokemon_at_mut(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let cure_result = target_mut.cure_status();

    if let Some((status, removed_nightmare)) = cure_result {
        let full_name = format!("{}: {}", target_ident, target_name);
        battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
        if removed_nightmare {
            battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
        }
    } else {
        // this.add('-fail', source);
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add("-fail", &[source_arg.into()]);

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // this.heal(Math.ceil(source.maxhp * 0.5), source);
    let heal_amount = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.maxhp + 1) / 2 // Math.ceil equivalent for integer division
    };

    battle.heal(heal_amount, Some(source), None, None);

    EventResult::Continue
}
