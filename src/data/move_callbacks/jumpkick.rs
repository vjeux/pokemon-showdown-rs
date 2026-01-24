//! Jump Kick Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{hp_fraction, Battle};
use crate::event::EventResult;

/// onMoveFail(target, source, move) {
///     this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('Jump Kick'));
/// }
pub fn on_move_fail(
    battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('Jump Kick'));
    let damage_amount = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        hp_fraction(source_pokemon.base_maxhp, 2)
    };

    // this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('Jump Kick'));
    // JavaScript passes a CONDITION effect, not a MOVE effect. This prevents Disguise from
    // blocking the crash damage (Disguise only blocks damage from effectType === 'Move').
    let condition_effect = battle.make_condition_effect(&ID::from("jumpkick"));
    battle.damage(
        damage_amount,
        Some(source),
        Some(source),
        Some(&condition_effect),
        false,
    );

    EventResult::Continue
}
