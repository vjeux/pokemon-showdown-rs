//! Spider Web Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     return target.addVolatile('trapped', source, move, 'trapper');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onHit(target, source, move) {
    //     return target.addVolatile('trapped', source, move, 'trapper');
    // }
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.addVolatile('trapped', source, move, 'trapper');
    let result = {
     let pokemon = match battle.pokemon_at_mut(target.0, target.1) {
         Some(p) => p,
         None => return EventResult::Continue,
     };
     pokemon.add_volatile(ID::from("trapped"));
 }

    EventResult::Boolean(result)
}

