//! Pixilate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Converts Normal-type moves to Fairy-type and boosts their power by 1.2x (Gen 8+)

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     const noModifyType = [
///         'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
///     ];
///     if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
///         !(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
///         move.type = 'Fairy';
///         move.typeChangerBoosted = this.effect;
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    let active_move = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if move type is Normal
    if active_move.move_type != "Normal" {
        return EventResult::Continue;
    }

    // List of moves that can't be converted (unless Max move)
    let no_modify_type = [
        "judgment", "multiattack", "naturalgift", "revelationdance",
        "technoblast", "terrainpulse", "weatherball"
    ];

    // Check exclusions
    if no_modify_type.contains(&active_move.id.as_str()) && !active_move.is_max {
        return EventResult::Continue;
    }

    // Don't change Z-moves (except Status category)
    if active_move.is_z.is_some() && active_move.category != "Status" {
        return EventResult::Continue;
    }

    // Don't change Tera Blast if Pokemon is terastallized
    if active_move.name == "Tera Blast" {
        // Check if Pokemon is terastallized (terastallized field is Option<String>)
        let is_terastallized = if let Some(side) = battle.sides.get(pokemon_pos.0) {
            if let Some(pokemon) = side.pokemon.get(pokemon_pos.1) {
                pokemon.terastallized.is_some()
            } else {
                false
            }
        } else {
            false
        };

        if is_terastallized {
            return EventResult::Continue;
        }
    }

    // Convert to Fairy type and mark as boosted
    active_move.move_type = "Fairy".to_string();
    active_move.type_changer_boosted = Some(Effect::ability("pixilate"));
    eprintln!("[PIXILATE] Changed {} from Normal to Fairy", active_move.name);

    EventResult::Continue
}

/// onBasePower(basePower, pokemon, target, move) {
///     if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
/// }
pub fn on_base_power(_battle: &mut Battle, base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    eprintln!("[PIXILATE on_base_power] CALLED! base_power={}", base_power);

    // Check if this move was boosted by Pixilate
    let should_boost = active_move
        .and_then(|m| m.type_changer_boosted.as_ref())
        .map(|booster| {
            eprintln!("[PIXILATE on_base_power] booster={}, checking if pixilate", booster.as_str());
            booster.as_str() == "pixilate"
        })
        .unwrap_or(false);

    eprintln!("[PIXILATE on_base_power] should_boost={}", should_boost);

    if !should_boost {
        return EventResult::Continue;
    }

    // Return the modifier 4915/4096 (approximately 1.2x, used in Gen 8+)
    // JavaScript: this.chainModify([4915, 4096])
    // The event system will multiply: base_power * (4915 / 4096) = 90 * 1.1999... = 107.95
    eprintln!("[PIXILATE] Applying power boost modifier 4915/4096 (≈1.2x)");

    EventResult::Number(4915)
}
