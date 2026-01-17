//! Axe Kick Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{hp_fraction, Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;

/// onMoveFail(target, source, move) {
///     this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('High Jump Kick'));
/// }
pub fn on_move_fail(
    battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get the source
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get source pokemon to access base_maxhp
    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('High Jump Kick'));
    let damage_amount = hp_fraction(source_pokemon.base_maxhp, 2);

    battle.damage(
        damage_amount,
        Some(source),
        Some(source),
        Some(&Effect::move_(ID::from("highjumpkick"))),
        false,
    );

    EventResult::Continue
}
