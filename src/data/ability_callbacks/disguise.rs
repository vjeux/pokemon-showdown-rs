//! Disguise Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect?.effectType === 'Move' && ['mimikyu', 'mimikyutotem'].includes(target.species.id)) {
///         this.add('-activate', target, 'ability: Disguise');
///         this.effectState.busted = true;
///         return 0;
///     }
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (effect?.effectType === 'Move' ...)
    let is_move = if let Some(eff_id) = effect_id {
        battle.dex.moves().get_by_id(&ID::from(eff_id)).is_some()
    } else {
        false
    };

    if !is_move {
        return EventResult::Continue;
    }

    // ['mimikyu', 'mimikyutotem'].includes(target.species.id)
    let species_id = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.species_id.clone()
    };

    let is_mimikyu = species_id.as_str() == "mimikyu" || species_id.as_str() == "mimikyutotem";

    if !is_mimikyu {
        return EventResult::Continue;
    }

    // this.add('-activate', target, 'ability: Disguise');
    let target_slot = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(target_slot),
        Arg::Str("ability: Disguise"),
    ]);

    // this.effectState.busted = true;
    battle.effect_state.data.insert(
        "busted".to_string(),
        serde_json::Value::Bool(true),
    );

    // return 0;
    EventResult::Number(0)
}

/// onCriticalHit(target, source, move) {
///     if (!target) return;
///     if (!['mimikyu', 'mimikyutotem'].includes(target.species.id)) {
///         return;
///     }
///     const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///     if (hitSub) return;
///
///     if (!target.runImmunity(move)) return;
///     return false;
/// }
pub fn on_critical_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    // if (!target) return;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!['mimikyu', 'mimikyutotem'].includes(target.species.id)) return;
    let species_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.species_id.clone()
    };

    let is_mimikyu = species_id.as_str() == "mimikyu" || species_id.as_str() == "mimikyutotem";

    if !is_mimikyu {
        return EventResult::Continue;
    }

    // const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
    let hit_sub = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_substitute = target_pokemon.has_volatile(&ID::from("substitute"));
        let bypasssub = battle.active_move.as_ref().map(|m| m.flags.bypasssub).unwrap_or(false);
        let infiltrates = battle.active_move.as_ref().map(|m| m.infiltrates).unwrap_or(false);
        let gen = battle.gen;

        has_substitute && !bypasssub && !(infiltrates && gen >= 6)
    };

    // if (hitSub) return;
    if hit_sub {
        return EventResult::Continue;
    }

    // if (!target.runImmunity(move)) return;
    let immune = {
        let move_type = if let Some(ref active_move) = battle.active_move {
            active_move.move_type.clone()
        } else {
            return EventResult::Continue;
        };

        !crate::Pokemon::run_immunity(battle, target, &move_type, false)
    };

    if immune {
        return EventResult::Continue;
    }

    // return false;
    EventResult::Boolean(false)
}

/// onEffectiveness(typeMod, target, type, move) {
///     if (!target || move.category === 'Status') return;
///     if (!['mimikyu', 'mimikyutotem'].includes(target.species.id)) {
///         return;
///     }
///
///     const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///     if (hitSub) return;
///
///     if (!target.runImmunity(move)) return;
///     return 0;
/// }
pub fn on_effectiveness(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _type_str: &str, _move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    // if (move.category === 'Status') return;
    let is_status = battle.active_move.as_ref().map(|m| m.category == "Status").unwrap_or(false);
    if is_status {
        return EventResult::Continue;
    }

    // if (!['mimikyu', 'mimikyutotem'].includes(target.species.id)) return;
    let species_id = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.species_id.clone()
    };

    let is_mimikyu = species_id.as_str() == "mimikyu" || species_id.as_str() == "mimikyutotem";

    if !is_mimikyu {
        return EventResult::Continue;
    }

    // const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
    let hit_sub = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_substitute = target.has_volatile(&ID::from("substitute"));
        let bypasssub = battle.active_move.as_ref().map(|m| m.flags.bypasssub).unwrap_or(false);
        let infiltrates = battle.active_move.as_ref().map(|m| m.infiltrates).unwrap_or(false);
        let gen = battle.gen;

        has_substitute && !bypasssub && !(infiltrates && gen >= 6)
    };

    // if (hitSub) return;
    if hit_sub {
        return EventResult::Continue;
    }

    // if (!target.runImmunity(move)) return;
    let immune = {
        let move_type = if let Some(ref active_move) = battle.active_move {
            active_move.move_type.clone()
        } else {
            return EventResult::Continue;
        };

        !crate::Pokemon::run_immunity(battle, target_pos, &move_type, false)
    };

    if immune {
        return EventResult::Continue;
    }

    // return 0;
    EventResult::Number(0)
}

/// onUpdate(pokemon) {
///     if (['mimikyu', 'mimikyutotem'].includes(pokemon.species.id) && this.effectState.busted) {
///         const speciesid = pokemon.species.id === 'mimikyutotem' ? 'Mimikyu-Busted-Totem' : 'Mimikyu-Busted';
///         pokemon.formeChange(speciesid, this.effect, true);
///         this.damage(pokemon.baseMaxhp / 8, pokemon, pokemon, this.dex.species.get(speciesid));
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // Check this.effectState.busted
    let busted = battle
        .effect_state
        .data
        .get("busted")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !busted {
        return EventResult::Continue;
    }

    // if (['mimikyu', 'mimikyutotem'].includes(pokemon.species.id))
    let (species_id, base_maxhp) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.species_id.clone(), pokemon.base_maxhp)
    };

    let is_mimikyu = species_id.as_str() == "mimikyu" || species_id.as_str() == "mimikyutotem";

    if !is_mimikyu {
        return EventResult::Continue;
    }

    // const speciesid = pokemon.species.id === 'mimikyutotem' ? 'Mimikyu-Busted-Totem' : 'Mimikyu-Busted';
    let new_species_id = if species_id.as_str() == "mimikyutotem" {
        "mimikyubusttotem"
    } else {
        "mimikyubusted"
    };

    // pokemon.formeChange(speciesid, this.effect, true);
    unsafe {
        let battle_ptr = battle as *mut Battle;
        let battle_ref1 = &mut *battle_ptr;
        let battle_ref2 = &mut *battle_ptr;

        let side = &mut battle_ref1.sides[pokemon_pos.0];
        let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
        if let Some(pokemon_index) = active_slot {
            if pokemon_index < side.pokemon.len() {
                crate::pokemon::Pokemon::forme_change(
                    battle_ref2,
                    (pokemon_pos.0, pokemon_index),
                    ID::from(new_species_id),
                    Some(ID::from("disguise")),
                    true,
                    "0",
                    None,
                );
            }
        }
    }

    // this.damage(pokemon.baseMaxhp / 8, pokemon, pokemon, this.dex.species.get(speciesid));
    let damage_amount = base_maxhp / 8;
    battle.damage(damage_amount, Some(pokemon_pos), Some(pokemon_pos), Some(&ID::from(new_species_id)), false);

    EventResult::Continue
}

