//! Pain Split Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;
use super::{Status, Effect};

/// onHit(target, pokemon) {
///     const targetHP = target.getUndynamaxedHP();
///     const averagehp = Math.floor((targetHP + pokemon.hp) / 2) || 1;
///     const targetChange = targetHP - averagehp;
///     target.sethp(target.hp - targetChange);
///     this.add('-sethp', target, target.getHealth, '[from] move: Pain Split', '[silent]');
///     pokemon.sethp(averagehp);
///     this.add('-sethp', pokemon, pokemon.getHealth, '[from] move: Pain Split');
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

