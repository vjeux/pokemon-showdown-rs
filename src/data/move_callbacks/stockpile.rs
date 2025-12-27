//! Stockpile Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry(source) {
///     if (source.volatiles['stockpile'] && source.volatiles['stockpile'].layers >= 3) return false;
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.effectState.layers = 1;
    ///     this.effectState.def = 0;
    ///     this.effectState.spd = 0;
    ///     this.add('-start', target, 'stockpile' + this.effectState.layers);
    ///     const [curDef, curSpD] = [target.boosts.def, target.boosts.spd];
    ///     this.boost({ def: 1, spd: 1 }, target, target);
    ///     if (curDef !== target.boosts.def) this.effectState.def--;
    ///     if (curSpD !== target.boosts.spd) this.effectState.spd--;
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onRestart(target) {
    ///     if (this.effectState.layers >= 3) return false;
    ///     this.effectState.layers++;
    ///     this.add('-start', target, 'stockpile' + this.effectState.layers);
    ///     const curDef = target.boosts.def;
    ///     const curSpD = target.boosts.spd;
    ///     this.boost({ def: 1, spd: 1 }, target, target);
    ///     if (curDef !== target.boosts.def) this.effectState.def--;
    ///     if (curSpD !== target.boosts.spd) this.effectState.spd--;
    /// }
    pub fn on_restart(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onEnd(target) {
    ///     if (this.effectState.def || this.effectState.spd) {
    ///         const boosts: SparseBoostsTable = {};
    ///         if (this.effectState.def) boosts.def = this.effectState.def;
    ///         if (this.effectState.spd) boosts.spd = this.effectState.spd;
    ///         this.boost(boosts, target, target);
    ///     }
    ///     this.add('-end', target, 'Stockpile');
    ///     if (this.effectState.def !== this.effectState.layers * -1 || this.effectState.spd !== this.effectState.layers * -1) {
    ///         this.hint("In Gen 7, Stockpile keeps track of how many times it successfully altered each stat individually.");
    ///     }
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
