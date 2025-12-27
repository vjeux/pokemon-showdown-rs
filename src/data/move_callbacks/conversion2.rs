//! Conversion 2 Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// onHit(target, source) {
///     if (!target.lastMoveUsed) {
///         return false;
///     }
///     const possibleTypes = [];
///     const attackType = target.lastMoveUsed.type;
///     for (const typeName of this.dex.types.names()) {
///         if (source.hasType(typeName)) continue;
///         const typeCheck = this.dex.types.get(typeName).damageTaken[attackType];
///         if (typeCheck === 2 || typeCheck === 3) {
///             possibleTypes.push(typeName);
///         }
///     }
///     if (!possibleTypes.length) {
///         return false;
///     }
///     const randomType = this.sample(possibleTypes);
///
///     if (!source.setType(randomType)) return false;
///     this.add('-start', source, 'typechange', randomType);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if target used a move
    let last_move_used = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        match &target.last_move_used {
            Some(id) => id.clone(),
            None => return EventResult::Bool(false),
        }
    };

    // Get the attack type from the last move used
    let attack_type = battle.dex.moves.get(last_move_used.as_str())
        .map(|m| m.move_type.clone())
        .unwrap_or_else(|| "Normal".to_string());

    // Collect possible types that resist or are immune to the attack type
    let possible_types: Vec<String> = {
        let source = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.get_all_type_names()
            .filter(|type_name| {
                // Skip if source already has this type
                if source.has_type(type_name) {
                    return false;
                }

                // Check type effectiveness: 2 = not very effective, 3 = immune
                let type_check = battle.dex.get_type_damage_taken(type_name, &attack_type);
                type_check == 2 || type_check == 3
            })
            .cloned()
            .collect()
    };

    // If no possible types, fail
    if possible_types.is_empty() {
        return EventResult::Bool(false);
    }

    // Sample a random type from possible types
    let random_type = match battle.sample(&possible_types) {
        Some(t) => t.clone(),
        None => return EventResult::Bool(false),
    };

    // Set the source's type
    let success = {
        let source = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        source.set_type(&random_type)
    };

    if !success {
        return EventResult::Bool(false);
    }

    // Add typechange message
    let source = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    battle.add("-start", &[
        Arg::Pokemon(source),
        Arg::Str("typechange"),
        Arg::String(random_type),
    ]);

    EventResult::Continue
}

