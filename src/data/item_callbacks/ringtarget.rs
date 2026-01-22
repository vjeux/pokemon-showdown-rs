//! Ring Target Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onNegateImmunity: false
///
/// JavaScript source (data/items.ts):
/// ```js
/// ringtarget: {
///     name: "Ring Target",
///     fling: {
///         basePower: 10,
///     },
///     onNegateImmunity: false,
///     num: 543,
///     gen: 5,
/// },
/// ```
///
/// When onNegateImmunity is set to the static value `false`, it causes
/// the holder's type immunities to be negated.
///
/// The logic in pokemon.ts is:
/// ```js
/// const negateImmunity = !this.battle.runEvent('NegateImmunity', this, type);
/// ```
///
/// If runEvent returns false, negateImmunity becomes true, which means
/// the Pokemon is NOT immune to the type (immunity is negated).
pub fn on_negate_immunity(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _type_str: &str,
) -> EventResult {
    // onNegateImmunity: false causes the holder to lose type immunities
    EventResult::Boolean(false)
}
