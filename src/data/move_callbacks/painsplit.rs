//! Pain Split Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, pokemon) {
///     const targetHP = target.getUndynamaxedHP();
///     const averagehp = Math.floor((targetHP + pokemon.hp) / 2) || 1;
///     const targetChange = targetHP - averagehp;
///     target.sethp(target.hp - targetChange);
///     this.add('-sethp', target, target.getHealth, '[from] move: Pain Split', '[silent]');
///     pokemon.sethp(averagehp);
///     this.add('-sethp', pokemon, pokemon.getHealth, '[from] move: Pain Split');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

