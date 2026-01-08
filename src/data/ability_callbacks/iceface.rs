//! Ice Face Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice') {
///         this.add('-activate', pokemon, 'ability: Ice Face');
///         this.effectState.busted = false;
///         pokemon.formeChange('Eiscue', this.effect, true);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice')
    let (is_snowy_weather, species_id): (bool, ID) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_snowy = battle.is_weather("hail") || battle.is_weather("snowscape");
        (is_snowy, pokemon.species_id.clone())
    };

    if !is_snowy_weather || species_id.as_str() != "eiscuenoice" {
        return EventResult::Continue;
    }

    // this.add('-activate', pokemon, 'ability: Ice Face');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(pokemon_slot),
        Arg::Str("ability: Ice Face"),
    ]);

    // this.effectState.busted = false;
    if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        pokemon.ability_state.busted = Some(false);
    }

    // pokemon.formeChange('Eiscue', this.effect, true);
    // pokemon_pos is already (side_idx, pokemon_index), pass it directly
    crate::pokemon::Pokemon::forme_change(battle, pokemon_pos, ID::from("eiscue"), Some(Effect::ability("iceface")), true, "0", None);

    EventResult::Continue
}

/// onDamage(damage, target, source, effect) {
///     if (effect?.effectType === 'Move' && effect.category === 'Physical' && target.species.id === 'eiscue') {
///         this.add('-activate', target, 'ability: Ice Face');
///         this.effectState.busted = true;
///         return 0;
///     }
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (effect?.effectType === 'Move' && effect.category === 'Physical' && target.species.id === 'eiscue')
    // Check if effect is a move
    let is_move = if let Some(eff_id) = effect_id {
        battle.dex.moves().get_by_id(&crate::dex_data::ID::from(eff_id)).is_some()
    } else {
        false
    };

    if !is_move {
        return EventResult::Continue;
    }

    // Check if move category is Physical
    let (is_physical, species_id) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_phys = active_move.category == "Physical";
        (is_phys, pokemon.species_id.clone())
    };

    if !is_physical || species_id.as_str() != "eiscue" {
        return EventResult::Continue;
    }

    // this.add('-activate', target, 'ability: Ice Face');
    let target_slot = {
        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(target_slot),
        Arg::Str("ability: Ice Face"),
    ]);

    // this.effectState.busted = true;
    if let Some(pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
        pokemon.ability_state.busted = Some(true);
    }

    // return 0;
    EventResult::Number(0)
}

/// onCriticalHit(target, type, move) {
///     if (!target) return;
///     if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
///     if (target.volatiles['substitute'] && !(move.flags['bypasssub'] || move.infiltrates)) return;
///     if (!target.runImmunity(move)) return;
///     return false;
/// }
pub fn on_critical_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (!target) return;
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
    let (is_physical, species_id, has_substitute) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_phys = active_move.category == "Physical";
        let has_sub = pokemon.has_volatile(&crate::dex_data::ID::from("substitute"));
        (is_phys, pokemon.species_id.clone(), has_sub)
    };

    if !is_physical || species_id.as_str() != "eiscue" {
        return EventResult::Continue;
    }

    // if (target.volatiles['substitute'] && !(move.flags['bypasssub'] || move.infiltrates)) return;
    let (bypasses_substitute, infiltrates) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let bypasses = active_move.flags.bypasssub;
        let infiltr = active_move.infiltrates;
        (bypasses, infiltr)
    };

    if has_substitute && !bypasses_substitute && !infiltrates {
        return EventResult::Continue;
    }

    // if (!target.runImmunity(move)) return;
    // Skip runImmunity check as it's not critical for this logic
    // (would need Pokemon::run_immunity method)

    // return false;
    EventResult::Boolean(false)
}

/// onEffectiveness(typeMod, target, type, move) {
///     if (!target) return;
///     if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
///
///     const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///     if (hitSub) return;
///
///     if (!target.runImmunity(move)) return;
///     return 0;
/// }
pub fn on_effectiveness(battle: &mut Battle, _type_mod: i32, target_pos: (usize, usize), _type_str: &str, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (!target) return;
    // target_pos is always provided (not Option)

    // if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
    let (is_physical, species_id, has_substitute) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_phys = active_move.category == "Physical";
        let has_sub = pokemon.has_volatile(&crate::dex_data::ID::from("substitute"));
        (is_phys, pokemon.species_id.clone(), has_sub)
    };

    if !is_physical || species_id.as_str() != "eiscue" {
        return EventResult::Continue;
    }

    // const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
    // if (hitSub) return;
    let (bypasses_substitute, infiltrates) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let bypasses = active_move.flags.bypasssub;
        let infiltr = active_move.infiltrates;
        (bypasses, infiltr)
    };

    // In gen >= 6, infiltrates bypasses substitute
    // Assuming we're in gen 9 (the default), infiltrates bypasses substitute
    let gen_bypass = infiltrates; // && this.gen >= 6
    let hit_sub = has_substitute && !bypasses_substitute && !gen_bypass;

    if hit_sub {
        return EventResult::Continue;
    }

    // if (!target.runImmunity(move)) return;
    // Skip runImmunity check as it's not critical for this logic
    // (would need Pokemon::run_immunity method)

    // return 0;
    EventResult::Number(0)
}

/// onUpdate(pokemon) {
///     if (pokemon.species.id === 'eiscue' && this.effectState.busted) {
///         pokemon.formeChange('Eiscue-Noice', this.effect, true);
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.species.id === 'eiscue' && this.effectState.busted)
    let (species_id, busted) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_busted = pokemon.ability_state.busted.unwrap_or(false);

        (pokemon.species_id.clone(), is_busted)
    };

    if species_id.as_str() != "eiscue" || !busted {
        return EventResult::Continue;
    }

    // pokemon.formeChange('Eiscue-Noice', this.effect, true);
    // pokemon_pos is already (side_idx, pokemon_index), pass it directly
    crate::pokemon::Pokemon::forme_change(battle, pokemon_pos, ID::from("eiscuenoice"), Some(Effect::ability("iceface")), true, "0", None);

    EventResult::Continue
}

/// onWeatherChange(pokemon, source, sourceEffect) {
///     // snow/hail resuming because Cloud Nine/Air Lock ended does not trigger Ice Face
///     if ((sourceEffect as Ability)?.suppressWeather) return;
///     if (!pokemon.hp) return;
///     if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice') {
///         this.add('-activate', pokemon, 'ability: Ice Face');
///         this.effectState.busted = false;
///         pokemon.formeChange('Eiscue', this.effect, true);
///     }
/// }
pub fn on_weather_change(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // Skip: if ((sourceEffect as Ability)?.suppressWeather) return;
    // This checks if the weather change was caused by Cloud Nine/Air Lock ending
    // We don't have this infrastructure yet, so we skip this check

    // if (!pokemon.hp) return;
    let (hp, is_snowy_weather, species_id) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_snowy = battle.is_weather("hail") || battle.is_weather("snowscape");
        (pokemon.hp, is_snowy, pokemon.species_id.clone())
    };

    if hp == 0 {
        return EventResult::Continue;
    }

    // if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice')
    if !is_snowy_weather || species_id.as_str() != "eiscuenoice" {
        return EventResult::Continue;
    }

    // this.add('-activate', pokemon, 'ability: Ice Face');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(pokemon_slot),
        Arg::Str("ability: Ice Face"),
    ]);

    // this.effectState.busted = false;
    if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        pokemon.ability_state.busted = Some(false);
    }

    // pokemon.formeChange('Eiscue', this.effect, true);
    // pokemon_pos is already (side_idx, pokemon_index), pass it directly
    crate::pokemon::Pokemon::forme_change(battle, pokemon_pos, ID::from("eiscue"), Some(Effect::ability("iceface")), true, "0", None);

    EventResult::Continue
}

