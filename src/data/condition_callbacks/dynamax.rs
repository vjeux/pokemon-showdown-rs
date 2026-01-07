//! Dynamax Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::event::EventResult;
use crate::dex_data::ID;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(pokemon) {
///     this.effectState.turns = 0;
///     pokemon.removeVolatile('minimize');
///     pokemon.removeVolatile('substitute');
///     if (pokemon.volatiles['torment']) {
///         delete pokemon.volatiles['torment'];
///         this.add('-end', pokemon, 'Torment', '[silent]');
///     }
///     if (['cramorantgulping', 'cramorantgorging'].includes(pokemon.species.id) && !pokemon.transformed) {
///         pokemon.formeChange('cramorant');
///     }
///     this.add('-start', pokemon, 'Dynamax', pokemon.gigantamax ? 'Gmax' : '');
///     if (pokemon.baseSpecies.name === 'Shedinja') return;
///
///     // Changes based on dynamax level, 2 is max (at LVL 10)
///     const ratio = 1.5 + (pokemon.dynamaxLevel * 0.05);
///
///     pokemon.maxhp = Math.floor(pokemon.maxhp * ratio);
///     pokemon.hp = Math.floor(pokemon.hp * ratio);
///     this.add('-heal', pokemon, pokemon.getHealth, '[silent]');
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // this.effectState.turns = 0;
    // JavaScript: this.effectState.turns = 0
    battle.with_effect_state(|state| {
        state.data.insert("turns".to_string(), serde_json::json!(0));
    });

    // pokemon.removeVolatile('minimize');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("minimize"));

    // pokemon.removeVolatile('substitute');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("substitute"));

    // if (pokemon.volatiles['torment'])
    let has_torment = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.volatiles.contains_key(&ID::from("torment"))
    };

    if has_torment {
        // delete pokemon.volatiles['torment'];
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.volatiles.remove(&ID::from("torment"));
            pokemon.get_slot()
        };

        // this.add('-end', pokemon, 'Torment', '[silent]');
        battle.add("-end", &[Arg::String(pokemon_ident), Arg::Str("Torment"), Arg::Str("[silent]")]);
    }

    // if (['cramorantgulping', 'cramorantgorging'].includes(pokemon.species.id) && !pokemon.transformed)
    let (is_cramorant_form, transformed) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let species_id = pokemon.species_id.as_str();
        (species_id == "cramorantgulping" || species_id == "cramorantgorging", pokemon.transformed)
    };

    if is_cramorant_form && !transformed {
        // pokemon.formeChange('cramorant');
        crate::pokemon::Pokemon::forme_change(
            battle,
            pokemon_pos,
            ID::from("cramorant"),
            None,
            false,
            "0",
            None,
        );
    }

    // this.add('-start', pokemon, 'Dynamax', pokemon.gigantamax ? 'Gmax' : '');
    let (pokemon_ident, gigantamax) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.get_slot(), pokemon.gigantamax)
    };

    if gigantamax {
        battle.add("-start", &[Arg::String(pokemon_ident.clone()), Arg::Str("Dynamax"), Arg::Str("Gmax")]);
    } else {
        battle.add("-start", &[Arg::String(pokemon_ident.clone()), Arg::Str("Dynamax")]);
    }

    // if (pokemon.baseSpecies.name === 'Shedinja') return;
    let is_shedinja = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_species.as_str() == "shedinja"
    };

    if is_shedinja {
        return EventResult::Continue;
    }

    // const ratio = 1.5 + (pokemon.dynamaxLevel * 0.05);
    let (dynamax_level, old_maxhp, old_hp) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.dynamax_level, pokemon.maxhp, pokemon.hp)
    };

    let ratio = 1.5 + (dynamax_level as f64 * 0.05);

    // pokemon.maxhp = Math.floor(pokemon.maxhp * ratio);
    // pokemon.hp = Math.floor(pokemon.hp * ratio);
    let new_maxhp = (old_maxhp as f64 * ratio).floor() as i32;
    let new_hp = (old_hp as f64 * ratio).floor() as i32;

    let health_str = {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.maxhp = new_maxhp;
        pokemon.hp = new_hp;
        pokemon.get_health()
    };

    // this.add('-heal', pokemon, pokemon.getHealth, '[silent]');
    battle.add("-heal", &[Arg::String(pokemon_ident), Arg::String(health_str), Arg::Str("[silent]")]);

    EventResult::Continue
}

/// onTryAddVolatile
/// JavaScript source (data/conditions.ts):
/// ```js
/// onTryAddVolatile(status, pokemon) {
///     if (status.id === 'flinch') return null;
/// }
/// ```
pub fn on_try_add_volatile(
    _battle: &mut Battle,
    status: Option<&str>,
    _pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // if (status.id === 'flinch') return null;
    if let Some(status_id) = status {
        if status_id == "flinch" {
            return EventResult::Null;
        }
    }

    EventResult::Continue
}

/// onBeforeSwitchOut
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeSwitchOutPriority: -1,
/// onBeforeSwitchOut(pokemon) {
///     pokemon.removeVolatile('dynamax');
/// }
/// ```
pub fn on_before_switch_out(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // pokemon.removeVolatile('dynamax');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("dynamax"));

    EventResult::Continue
}

/// onSourceModifyDamage
/// JavaScript source (data/conditions.ts):
/// ```js
/// onSourceModifyDamage(damage, source, target, move) {
///     if (move.id === 'behemothbash' || move.id === 'behemothblade' || move.id === 'dynamaxcannon') {
///         return this.chainModify(2);
///     }
/// }
/// ```
pub fn on_source_modify_damage(
    battle: &mut Battle,
    _damage: i32,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
    move_id: &str,
) -> EventResult {
    // if (move.id === 'behemothbash' || move.id === 'behemothblade' || move.id === 'dynamaxcannon')
    if move_id == "behemothbash" || move_id == "behemothblade" || move_id == "dynamaxcannon" {
        // return this.chainModify(2);
        battle.chain_modify(2.0); return EventResult::Continue;
    }

    EventResult::Continue
}

/// onDragOut
/// JavaScript source (data/conditions.ts):
/// ```js
/// onDragOutPriority: 2,
/// onDragOut(pokemon) {
///     this.add('-block', pokemon, 'Dynamax');
///     return null;
/// }
/// ```
pub fn on_drag_out(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // this.add('-block', pokemon, 'Dynamax');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-block", &[Arg::String(pokemon_ident), Arg::Str("Dynamax")]);

    // return null;
    EventResult::Null
}

/// onResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onResidualPriority: -100,
/// onResidual() {
///     this.effectState.turns++;
/// }
/// ```
pub fn on_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // this.effectState.turns++;
    // JavaScript: this.effectState.turns++
    battle.with_effect_state(|state| {
        let turns = state.data.get("turns")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        state.data.insert("turns".to_string(), serde_json::json!(turns + 1));
    });

    EventResult::Continue
}

/// onEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEnd(pokemon) {
///     this.add('-end', pokemon, 'Dynamax');
///     if (pokemon.baseSpecies.name === 'Shedinja') return;
///     pokemon.hp = pokemon.getUndynamaxedHP();
///     pokemon.maxhp = pokemon.baseMaxhp;
///     this.add('-heal', pokemon, pokemon.getHealth, '[silent]');
/// }
/// ```
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-end', pokemon, 'Dynamax');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-end", &[Arg::String(pokemon_ident.clone()), Arg::Str("Dynamax")]);

    // if (pokemon.baseSpecies.name === 'Shedinja') return;
    let is_shedinja = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_species.as_str() == "shedinja"
    };

    if is_shedinja {
        return EventResult::Continue;
    }

    // pokemon.hp = pokemon.getUndynamaxedHP();
    // pokemon.maxhp = pokemon.baseMaxhp;
    let (undynamaxed_hp, base_maxhp) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.get_undynamaxed_hp(None), pokemon.base_maxhp)
    };

    let health_str = {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.hp = undynamaxed_hp;
        pokemon.maxhp = base_maxhp;
        pokemon.get_health()
    };

    // this.add('-heal', pokemon, pokemon.getHealth, '[silent]');
    battle.add("-heal", &[Arg::String(pokemon_ident), Arg::String(health_str), Arg::Str("[silent]")]);

    EventResult::Continue
}

