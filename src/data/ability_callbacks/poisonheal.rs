//! Poison Heal Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect.id === 'psn' || effect.id === 'tox') {
///         this.heal(target.baseMaxhp / 8);
///         return false;
///     }
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // if (effect.id === 'psn' || effect.id === 'tox')
    if let Some(eff_id) = effect_id {
        if eff_id == "psn" || eff_id == "tox" {
            // this.heal(target.baseMaxhp / 8);
            let heal_amount = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Boolean(false),
                };
                hp_fraction(target.base_maxhp, 8)
            };

            battle.heal(heal_amount, Some(target_pos), None, None);

            // return false;
            return EventResult::Boolean(false);
        }
    }

    EventResult::Continue
}

