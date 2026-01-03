//! Wandering Spirit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (source.getAbility().flags['failskillswap'] || target.volatiles['dynamax']) return;
///
///     if (this.checkMoveMakesContact(move, source, target)) {
///         const targetCanBeSet = this.runEvent('SetAbility', target, source, this.effect, source.ability);
///         if (!targetCanBeSet) return targetCanBeSet;
///         const sourceAbility = source.setAbility('wanderingspirit', target);
///         if (!sourceAbility) return;
///         if (target.isAlly(source)) {
///             this.add('-activate', target, 'Skill Swap', '', '', `[of] ${source}`);
///         } else {
///             this.add('-activate', target, 'ability: Wandering Spirit', this.dex.abilities.get(sourceAbility).name, 'Wandering Spirit', `[of] ${source}`);
///         }
///         target.setAbility(sourceAbility);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    let (target_pos, source_pos) = match (target_pos, source_pos) {
        (Some(t), Some(s)) => (t, s),
        _ => return EventResult::Continue,
    };

    // if (source.getAbility().flags['failskillswap'] || target.volatiles['dynamax']) return;
    let (source_ability_id, target_has_dynamax) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source.ability.clone(), target.has_volatile(&ID::from("dynamax")))
    };

    // Check if source ability has failskillswap flag
    let has_failskillswap = if let Some(ability_data) = battle.dex.abilities().get(source_ability_id.as_str()) {
        ability_data.flags.get("failskillswap").map(|v| *v != 0).unwrap_or(false)
    } else {
        false
    };

    if has_failskillswap || target_has_dynamax {
        return EventResult::Continue;
    }

    // Get the move ID from active_move
    let move_id = if let Some(ref active_move) = battle.active_move {
        active_move.id.clone()
    } else {
        return EventResult::Continue;
    };

    // if (this.checkMoveMakesContact(move, source, target))
    if !battle.check_move_makes_contact(&move_id, source_pos, target_pos, false) {
        return EventResult::Continue;
    }

    // const sourceAbility = source.setAbility('wanderingspirit', target);
    let source_ability_id = Pokemon::set_ability(battle, source_pos, ID::from("wanderingspirit"), Some(target_pos), None, false, false);

    // if (!sourceAbility) return;
    if source_ability_id.is_empty() {
        return EventResult::Continue;
    }

    // if (target.isAlly(source))
    let is_ally = battle.is_ally(target_pos, source_pos);

    let (target_slot, source_slot, source_ability_name) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ability_name = if let Some(ability_data) = battle.dex.abilities().get(source_ability_id.as_str()) {
            ability_data.name.clone()
        } else {
            source_ability_id.to_string()
        };
        (target.get_slot(), source.get_slot(), ability_name)
    };

    if is_ally {
        // this.add('-activate', target, 'Skill Swap', '', '', `[of] ${source}`);
        battle.add("-activate", &[
            Arg::String(target_slot.clone()),
            Arg::Str("Skill Swap"),
            Arg::Str(""),
            Arg::Str(""),
            Arg::String(format!("[of] {}", source_slot)),
        ]);
    } else {
        // this.add('-activate', target, 'ability: Wandering Spirit', this.dex.abilities.get(sourceAbility).name, 'Wandering Spirit', `[of] ${source}`);
        battle.add("-activate", &[
            Arg::String(target_slot.clone()),
            Arg::Str("ability: Wandering Spirit"),
            Arg::String(source_ability_name),
            Arg::Str("Wandering Spirit"),
            Arg::String(format!("[of] {}", source_slot)),
        ]);
    }

    // target.setAbility(sourceAbility);
    Pokemon::set_ability(battle, target_pos, source_ability_id, Some(source_pos), None, false, false);

    EventResult::Continue
}

