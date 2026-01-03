//! Mirror Coat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// damageCallback(pokemon) {
///     if (!pokemon.volatiles['mirrorcoat']) return 0;
///     return pokemon.volatiles['mirrorcoat'].damage || 1;
/// }
pub fn damage_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (!pokemon.volatiles['mirrorcoat']) return 0;
    let damage = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Number(0),
        };

        match pokemon_pokemon.volatiles.get(&ID::from("mirrorcoat")) {
            Some(volatile) => {
                // return pokemon.volatiles['mirrorcoat'].damage || 1;
                volatile
                    .data
                    .get("damage")
                    .and_then(|v| v.as_i64())
                    .map(|d| d as i32)
                    .unwrap_or(1)
            }
            None => {
                // return 0;
                return EventResult::Number(0);
            }
        }
    };

    EventResult::Number(damage)
}

/// beforeTurnCallback(pokemon) {
///     pokemon.addVolatile('mirrorcoat');
/// }
pub fn before_turn_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = pokemon_pos;

    // pokemon.addVolatile('mirrorcoat');
    Pokemon::add_volatile(battle, pokemon, ID::from("mirrorcoat"), None, None, None);

    EventResult::Continue
}

/// onTry(source) {
///     if (!source.volatiles['mirrorcoat']) return false;
///     if (source.volatiles['mirrorcoat'].slot === null) return false;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = source_pos;

    // if (!source.volatiles['mirrorcoat']) return false;
    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Boolean(false),
    };

    let volatile = match source_pokemon.volatiles.get(&ID::from("mirrorcoat")) {
        Some(v) => v,
        None => return EventResult::Boolean(false),
    };

    // if (source.volatiles['mirrorcoat'].slot === null) return false;
    let slot = volatile.data.get("slot");
    if slot.is_none() || slot == Some(&serde_json::Value::Null) {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
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
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state
                .data
                .insert("slot".to_string(), serde_json::Value::Null);
            // this.effectState.damage = 0;
            effect_state
                .data
                .insert("damage".to_string(), serde_json::to_value(0).unwrap());
        }

        EventResult::Continue
    }

    /// onRedirectTarget(target, source, source2, move) {
    ///     if (move.id !== 'mirrorcoat') return;
    ///     if (source !== this.effectState.target || !this.effectState.slot) return;
    ///     return this.getAtSlot(this.effectState.slot);
    /// }
    pub fn on_redirect_target(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        // if (move.id !== 'mirrorcoat') return;
        if move_id != "mirrorcoat" {
            return EventResult::Continue;
        }

        // if (source !== this.effectState.target || !this.effectState.slot) return;
        let (effect_target, effect_slot) = {
            if let Some(ref effect_state) = battle.current_effect_state {
                let target = effect_state.target;
                let slot = effect_state.data.get("slot");
                (target, slot.cloned())
            } else {
                return EventResult::Continue;
            }
        };

        if source_pos != effect_target
            || effect_slot.is_none()
            || effect_slot == Some(serde_json::Value::Null)
        {
            return EventResult::Continue;
        }

        // return this.getAtSlot(this.effectState.slot);
        if let Some(serde_json::Value::Number(slot_num)) = effect_slot {
            if let Some(slot_val) = slot_num.as_u64() {
                let new_target = battle.get_at_slot(Some(&slot_val.to_string()));
                if let Some(target) = new_target {
                    // Return the new target position for move redirection
                    let target_pos = (target.side_index, target.position);
                    return EventResult::Position(target_pos);
                }
            }
        }

        EventResult::Continue
    }

    /// onDamagingHit(damage, target, source, move) {
    ///     if (!source.isAlly(target) && this.getCategory(move) === 'Special') {
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
        use crate::dex_data::ID;

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!source.isAlly(target) && this.getCategory(move) === 'Special') {
        let is_ally = source.0 == target.0; // Same side

        if is_ally {
            return EventResult::Continue;
        }

        let move_data = battle.dex.moves().get_by_id(&ID::from(move_id));
        let is_special = move_data.map(|m| m.category == "Special").unwrap_or(false);

        if !is_special {
            return EventResult::Continue;
        }

        // this.effectState.slot = source.getSlot();
        // this.effectState.damage = 2 * damage;
        let slot = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state
                .data
                .insert("slot".to_string(), serde_json::to_value(slot).unwrap());
            effect_state.data.insert(
                "damage".to_string(),
                serde_json::to_value(2 * damage).unwrap(),
            );
        }

        EventResult::Continue
    }
}
