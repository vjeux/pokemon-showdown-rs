//! Lucky Chant Side Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/moves.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onCriticalHit: false
///
/// JavaScript source (data/moves.ts):
/// ```js
/// luckychant: {
///     condition: {
///         duration: 5,
///         onSideStart(side) {
///             this.add('-sidestart', side, 'move: Lucky Chant');
///         },
///         onCriticalHit: false,
///         onSideResidualOrder: 26,
///         onSideResidualSubOrder: 6,
///         onSideEnd(side) {
///             this.add('-sideend', side, 'move: Lucky Chant');
///         },
///     },
/// },
/// ```
///
/// When onCriticalHit is set to the static value `false`, it means
/// critical hits are ALWAYS blocked against Pokemon on this side.
pub fn on_critical_hit(
    _battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // onCriticalHit: false means always block critical hits
    EventResult::Boolean(false)
}
