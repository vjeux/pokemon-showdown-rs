//! Aerilate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     const noModifyType = [
///         'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
///     ];
///     if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
///         !(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
///         move.type = 'Flying';
///         move.typeChangerBoosted = this.effect;
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, _move_id: &str, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // const noModifyType = [...]
    let no_modify_type = [
        "judgment",
        "multiattack",
        "naturalgift",
        "revelationdance",
        "technoblast",
        "terrainpulse",
        "weatherball",
    ];

    // Get pokemon.terastallized
    let is_terastallized = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.terastallized.is_some()
    };

    // Check move properties
    if let Some(ref mut active_move) = battle.active_move {
        // if (move.type === 'Normal' && ...)
        let is_normal_type = active_move.move_type == "Normal";
        let in_no_modify_list = no_modify_type.contains(&active_move.id.as_str());
        let is_z_non_status = active_move.is_z && active_move.category != "Status";
        let is_tera_blast_terastallized = active_move.name == "Tera Blast" && is_terastallized;

        // if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
        //     !(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized))
        if is_normal_type && (!in_no_modify_list || active_move.is_max) && !is_z_non_status && !is_tera_blast_terastallized {
            // move.type = 'Flying';
            active_move.move_type = "Flying".to_string();
            // move.typeChangerBoosted = this.effect;
            active_move.type_changer_boosted = Some(Effect::ability("aerilate"));
        }
    }

    EventResult::Continue
}

/// onBasePower(basePower, pokemon, target, move) {
///     if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // if (move.typeChangerBoosted === this.effect)
    let is_boosted = if let Some(ref active_move) = battle.active_move {
        active_move.type_changer_boosted.as_ref().map(|e| e.as_str()) == Some("aerilate")
    } else {
        false
    };

    if is_boosted {
        // return this.chainModify([4915, 4096]);
        battle.chain_modify_fraction(4915, 4096); return EventResult::Continue;
    }

    EventResult::Continue
}

