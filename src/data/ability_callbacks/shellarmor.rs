//! Shell Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onCriticalHit: false
///
/// JavaScript source (data/abilities.ts):
/// ```js
/// shellarmor: {
///     onCriticalHit: false,
///     isBreakable: true,
///     name: "Shell Armor",
///     rating: 1,
///     num: 75,
/// },
/// ```
///
/// When onCriticalHit is set to the static value `false`, it means
/// critical hits are ALWAYS blocked against this Pokemon.
pub fn on_critical_hit(
    _battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // onCriticalHit: false means always block critical hits
    EventResult::Boolean(false)
}
