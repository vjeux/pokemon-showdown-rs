//! Phantom Force Condition (volatile applied during two-turn move)
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/moves.ts - phantomforce condition

use crate::battle::Battle;
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
/// Unlike Fly/Dig/Dive which check specific moves, Phantom Force makes
/// the user completely invulnerable to ALL moves.
pub fn on_any_invulnerability(
    _battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _attacking_move_id: &str,
) -> EventResult {
    // onInvulnerability: false means always invulnerable
    EventResult::Boolean(false)
}
