//! As One (Spectrier) Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.effectState.unnerved) return;
///     this.add('-ability', pokemon, 'As One');
///     this.add('-ability', pokemon, 'Unnerve');
///     this.effectState.unnerved = true;
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // if (this.effectState.unnerved) return;
    let already_unnerved = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.get("unnerved").and_then(|v| v.as_bool()).unwrap_or(false)
    };

    if already_unnerved {
        return EventResult::Continue;
    }

    // this.add('-ability', pokemon, 'As One');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-ability", &[
        Arg::String(pokemon_slot.clone()),
        Arg::Str("As One"),
    ]);

    // this.add('-ability', pokemon, 'Unnerve');
    battle.add("-ability", &[
        Arg::String(pokemon_slot),
        Arg::Str("Unnerve"),
    ]);

    // this.effectState.unnerved = true;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.insert("unnerved".to_string(), serde_json::json!(true));
    }

    EventResult::Continue
}

/// onEnd() {
///     this.effectState.unnerved = false;
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.effectState.unnerved = false;
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon.ability_state.data.insert("unnerved".to_string(), serde_json::json!(false));

    EventResult::Continue
}

/// onFoeTryEatItem() {
///     return !this.effectState.unnerved;
/// }
pub fn on_foe_try_eat_item(_battle: &mut Battle) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSourceAfterFaint(length, target, source, effect) {
///     if (effect && effect.effectType === 'Move') {
///         this.boost({ spa: length }, source, source, this.dex.abilities.get('grimneigh'));
///     }
/// }
pub fn on_source_after_faint(battle: &mut Battle, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::dex_data::ID;

    // if (effect && effect.effectType === 'Move')
    if let Some(eff_id) = effect_id {
        if battle.dex.moves().get(eff_id).is_some() {
            // Effect is a move, boost Special Attack by 1 (length parameter not available in current dispatcher)
            if let Some(src_pos) = source_pos {
                battle.boost(&[("spa", 1)], src_pos, Some(src_pos), Some(&ID::from("grimneigh").to_string()), false, false);
            }
        }
    }
    EventResult::Continue
}

