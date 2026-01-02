//! Counter Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// damageCallback(pokemon) {
///     if (!pokemon.volatiles['counter']) return 0;
///     return pokemon.volatiles['counter'].damage || 1;
/// }
pub fn damage_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (!pokemon.volatiles['counter']) return 0;
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let counter_id = ID::from("counter");
    if !pokemon.volatiles.contains_key(&counter_id) {
        return EventResult::Number(0);
    }

    // return pokemon.volatiles['counter'].damage || 1;
    let damage = pokemon
        .volatiles
        .get(&counter_id)
        .and_then(|v| v.data.get("damage"))
        .and_then(|d| d.as_i64())
        .unwrap_or(1) as i32;

    EventResult::Number(damage)
}

/// beforeTurnCallback(pokemon) {
///     pokemon.addVolatile('counter');
/// }
pub fn before_turn_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.addVolatile('counter');
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("counter"), None, None, None);

    EventResult::Continue
}

/// onTry(source) {
///     if (!source.volatiles['counter']) return false;
///     if (source.volatiles['counter'].slot === null) return false;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (!source.volatiles['counter']) return false;
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let counter_id = ID::from("counter");
    if !source.volatiles.contains_key(&counter_id) {
        return EventResult::Boolean(false);
    }

    // if (source.volatiles['counter'].slot === null) return false;
    let slot = source
        .volatiles
        .get(&counter_id)
        .and_then(|v| v.data.get("slot"))
        .cloned();

    match slot {
        None | Some(serde_json::Value::Null) => EventResult::Boolean(false),
        _ => EventResult::Continue,
    }
}

pub mod condition {
    use super::*;

    /// onStart(target, source, move) {
    ///     this.effectState.slot = null;
    ///     this.effectState.damage = 0;
    /// }
    pub fn on_start(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _move_id: &str,
    ) -> EventResult {
        // this.effectState.slot = null;
        // this.effectState.damage = 0;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state
                .data
                .insert("slot".to_string(), serde_json::Value::Null);
            effect_state
                .data
                .insert("damage".to_string(), serde_json::Value::Number(0.into()));
        }

        EventResult::Continue
    }

    /// onRedirectTarget(target, source, source2, move) {
    ///     if (move.id !== 'counter') return;
    ///     if (source !== this.effectState.target || !this.effectState.slot) return;
    ///     return this.getAtSlot(this.effectState.slot);
    /// }
    pub fn on_redirect_target(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        // if (move.id !== 'counter') return;
        if move_id != "counter" {
            return EventResult::Continue;
        }

        // if (source !== this.effectState.target || !this.effectState.slot) return;
        let (effect_target, slot) = if let Some(ref effect_state) = battle.current_effect_state {
            let target = effect_state
                .data
                .get("target")
                .and_then(|v| serde_json::from_value::<(usize, usize)>(v.clone()).ok());
            let slot = effect_state.data.get("slot").cloned();
            (target, slot)
        } else {
            (None, None)
        };

        if source_pos != effect_target {
            return EventResult::Continue;
        }

        match slot {
            None | Some(serde_json::Value::Null) => EventResult::Continue,
            Some(slot_value) => {
                if let Ok(slot_str) = serde_json::from_value::<String>(slot_value) {
                    // return this.getAtSlot(this.effectState.slot);
                    if let Some(_new_target) = battle.get_at_slot(Some(&slot_str)) {
                        // TODO: Return the new target position
                        // For now, we'll just continue
                    }
                }
                EventResult::Continue
            }
        }
    }

    /// onDamagingHit(damage, target, source, move) {
    ///     if (!source.isAlly(target) && this.getCategory(move) === 'Physical') {
    ///         this.effectState.slot = source.getSlot();
    ///         this.effectState.damage = 2 * damage;
    ///     }
    /// }
    pub fn on_damaging_hit(
        battle: &mut Battle,
        damage: i32,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        // Get target and source
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!source.isAlly(target) && this.getCategory(move) === 'Physical') {
        let is_ally = battle.is_ally(source, target);

        let move_data = match battle.dex.moves().get_by_id(&ID::from(move_id)) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        if !is_ally && move_data.category == "Physical" {
            // this.effectState.slot = source.getSlot();
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let slot = source_pokemon.get_slot();

            // this.effectState.damage = 2 * damage;
            if let Some(ref mut effect_state) = battle.current_effect_state {
                effect_state.data.insert(
                    "slot".to_string(),
                    serde_json::to_value(slot).unwrap_or(serde_json::Value::Null),
                );
                effect_state.data.insert(
                    "damage".to_string(),
                    serde_json::Value::Number((2 * damage).into()),
                );
            }
        }

        EventResult::Continue
    }
}
