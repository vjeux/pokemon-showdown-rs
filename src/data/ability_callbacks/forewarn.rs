//! Forewarn Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     let warnMoves: (Move | Pokemon)[][] = [];
///     let warnBp = 1;
///     for (const target of pokemon.foes()) {
///         for (const moveSlot of target.moveSlots) {
///             const move = this.dex.moves.get(moveSlot.move);
///             let bp = move.basePower;
///             if (move.ohko) bp = 150;
///             if (move.id === 'counter' || move.id === 'metalburst' || move.id === 'mirrorcoat') bp = 120;
///             if (bp === 1) bp = 80;
///             if (!bp && move.category !== 'Status') bp = 80;
///             if (bp > warnBp) {
///                 warnMoves = [[move, target]];
///                 warnBp = bp;
///             } else if (bp === warnBp) {
///                 warnMoves.push([move, target]);
///             }
///         }
///     }
///     if (!warnMoves.length) return;
///     const [warnMoveName, warnTarget] = this.sample(warnMoves);
///     this.add('-activate', pokemon, 'ability: Forewarn', warnMoveName, `[of] ${warnTarget}`);
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

