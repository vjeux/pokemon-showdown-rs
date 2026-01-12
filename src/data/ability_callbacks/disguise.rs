//! Disguise Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect?.effectType === 'Move' && ['mimikyu', 'mimikyutotem'].includes(target.species.id)) {
///         this.add('-activate', target, 'ability: Disguise');
///         this.effectState.busted = true;
///         return 0;
///     }
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::battle::EffectType;

    // if (effect?.effectType === 'Move' ...)
    // JavaScript checks the effect parameter's effectType, not this.effect (the handler's effect).
    // The effect parameter is the effect that caused the damage, stored in battle.event.effect.
    // battle.effect is the current handler's effect (the ability), not the damage source.
    let is_move = if let Some(ref event) = battle.event {
        if let Some(ref eff) = event.effect {
            eff.effect_type == EffectType::Move
        } else {
            false
        }
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
    // Use with_effect_state to persist in the ability's effect state
    battle.with_effect_state(|state| {
        state.busted = Some(true);
    });

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
pub fn on_critical_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    use crate::dex_data::ID;

    // if (!target) return;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
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
        has_substitute && !active_move_ref.flags.bypasssub && !(active_move_ref.infiltrates && battle.gen >= 6)
    };

    // if (hitSub) return;
    if hit_sub {
        return EventResult::Continue;
    }

    // if (!target.runImmunity(move)) return;
    let immune = !crate::Pokemon::run_immunity(battle, target, &active_move_ref.move_type, false);

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
pub fn on_effectiveness(battle: &mut Battle, _type_mod: i32, target_pos: (usize, usize), _type_str: &str, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    use crate::dex_data::ID;

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.category === 'Status') return;
    if active_move_ref.category == "Status" {
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
        has_substitute && !active_move_ref.flags.bypasssub && !(active_move_ref.infiltrates && battle.gen >= 6)
    };

    // if (hitSub) return;
    if hit_sub {
        return EventResult::Continue;
    }

    // if (!target.runImmunity(move)) return;
    let immune = !crate::Pokemon::run_immunity(battle, target_pos, &active_move_ref.move_type, false);

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

    // Check this.effectState.busted using with_effect_state_ref
    let busted = battle.with_effect_state_ref(|state| {
        state.busted
    }).flatten().unwrap_or(false);

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
        "mimikyubustedtotem"
    } else {
        "mimikyubusted"
    };

    // pokemon.formeChange(speciesid, this.effect, true);
    // pokemon_pos is already (side_idx, pokemon_index), pass it directly
    crate::pokemon::Pokemon::forme_change(
        battle,
        pokemon_pos,
        ID::from(new_species_id),
        Some(Effect::ability("disguise")),
        true,
        "0",
        None,
    );

    // this.damage(pokemon.baseMaxhp / 8, pokemon, pokemon, this.dex.species.get(speciesid));
    // In JS, species.get returns an Effect. Species effects don't have onDamage callbacks,
    // so passing them prevents the fallback to self.effect (which would be Disguise).
    // We create a species Effect to match JavaScript behavior.
    let species_effect = Effect::species(new_species_id);
    let damage_amount = base_maxhp / 8;
    battle.damage(damage_amount, Some(pokemon_pos), Some(pokemon_pos), Some(&species_effect), false);

    EventResult::Continue
}

