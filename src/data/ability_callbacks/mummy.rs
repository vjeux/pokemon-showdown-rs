//! Mummy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     const sourceAbility = source.getAbility();
///     if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'mummy') {
///         return;
///     }
///     if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
///         const oldAbility = source.setAbility('mummy', target);
///         if (oldAbility) {
///             this.add('-activate', target, 'ability: Mummy', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;
    use crate::Pokemon;

    // Get target and source positions
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const sourceAbility = source.getAbility();
    let (source_ability_id, source_ability_cantsuppress) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let ability_id = source.ability.clone();

        // Check if source ability has cantsuppress flag
        let cantsuppress = battle.dex.abilities()
            .get_by_id(&ability_id)
            .and_then(|ability| ability.flags.get("cantsuppress"))
            .map(|v| *v != 0)
            .unwrap_or(false);

        (ability_id, cantsuppress)
    };

    // if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'mummy')
    if source_ability_cantsuppress || source_ability_id.as_str() == "mummy" {
        return EventResult::Continue;
    }

    // if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target)))
    let is_ally = battle.is_ally(source_pos, target_pos);
    let makes_contact = battle.check_move_makes_contact(&ID::from(move_id), source_pos, target_pos, !is_ally);

    if makes_contact {
        // const oldAbility = source.setAbility('mummy', target);
        let old_ability = Pokemon::set_ability(
            battle,
            source_pos,
            ID::from("mummy"),
            Some(target_pos),
            None,
            false,
            false,
        );

        // if (oldAbility)
        if !old_ability.is_empty() {
            // this.add('-activate', target, 'ability: Mummy', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
            let (target_slot, source_slot, old_ability_name) = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                let old_ability_name = battle.dex.abilities()
                    .get_by_id(&old_ability)
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| old_ability.to_string());

                (target.get_slot(), source.get_slot(), old_ability_name)
            };

            battle.add("-activate", &[
                Arg::String(target_slot),
                Arg::Str("ability: Mummy"),
                Arg::String(old_ability_name),
                Arg::String(format!("[of] {}", source_slot)),
            ]);
        }
    }

    EventResult::Continue
}

