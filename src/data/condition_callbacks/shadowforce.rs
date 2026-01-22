//! Shadow Force Condition (volatile applied during two-turn move)
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/moves.ts - shadowforce condition

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onInvulnerability
/// JavaScript source (data/moves.ts):
/// ```js
/// condition: {
///     duration: 2,
///     onInvulnerability: false,
/// }
/// ```
///
/// When onInvulnerability is set to the static value `false`, it means
/// the Pokemon is ALWAYS invulnerable while this condition is active.
/// Shadow Force makes the user completely invulnerable to ALL moves,
/// just like Phantom Force.
pub fn on_invulnerability(
    _battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _attacking_active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // onInvulnerability: false means always invulnerable
    EventResult::Boolean(false)
}
