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
    debug_elog!("[COUNTER] damage_callback called for pokemon_pos={:?}", pokemon_pos);
    // if (!pokemon.volatiles['counter']) return 0;
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let counter_id = ID::from("counter");
    if !pokemon.volatiles.contains_key(&counter_id) {
        debug_elog!("[COUNTER] damage_callback: no counter volatile, returning 0");
        return EventResult::Number(0);
    }

    // return pokemon.volatiles['counter'].damage || 1;
    // Note: In JS, `damage || 1` treats 0 as falsy and returns 1
    // So if damage is 0 (e.g., False Swipe couldn't deal damage), Counter still deals 1
    let damage = pokemon
        .volatiles
        .get(&counter_id)
        .and_then(|v| v.borrow().damage)
        .unwrap_or(0);
    debug_elog!("[COUNTER] damage_callback: volatile damage={}", damage);
    let damage = if damage == 0 { 1 } else { damage };

    debug_elog!("[COUNTER] damage_callback: returning {}", damage);
    EventResult::Number(damage)
}

/// beforeTurnCallback(pokemon) {
///     pokemon.addVolatile('counter');
/// }
pub fn before_turn_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.addVolatile('counter');
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("counter"), None, None, None, None);

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
    debug_elog!("[COUNTER] on_try called for source_pos={:?}", source_pos);
    // if (!source.volatiles['counter']) return false;
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let counter_id = ID::from("counter");
    if !source.volatiles.contains_key(&counter_id) {
        debug_elog!("[COUNTER] on_try: no counter volatile, returning false");
        return EventResult::Boolean(false);
    }

    // if (source.volatiles['counter'].slot === null) return false;
    let slot = source
        .volatiles
        .get(&counter_id)
        .and_then(|v| v.borrow().slot);

    debug_elog!("[COUNTER] on_try: volatile slot={:?}, damage={:?}",
        slot, source.volatiles.get(&counter_id).and_then(|v| v.borrow().damage));

    match slot {
        None => {
            debug_elog!("[COUNTER] on_try: slot is None, returning false");
            EventResult::Boolean(false)
        },
        _ => {
            debug_elog!("[COUNTER] on_try: slot is set, returning Continue");
            EventResult::Continue
        },
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
        _pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // this.effectState.slot = null;
        // this.effectState.damage = 0;
        battle.with_effect_state(|state| {
            state.slot = None;
            state.damage = Some(0);
        });

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
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
        // if (move.id !== 'counter') return;
        if move_id != "counter" {
            return EventResult::Continue;
        }

        // if (source !== this.effectState.target || !this.effectState.slot) return;
        let (effect_target, slot) = battle.with_effect_state_ref(|state| {
            (state.target, state.slot)
        }).unwrap_or((None, None));

        if source_pos != effect_target {
            return EventResult::Continue;
        }

        match slot {
            None => EventResult::Continue,
            Some(slot_num) => {
                // return this.getAtSlot(this.effectState.slot);
                let slot_str = slot_num.to_string();
                if let Some(new_target) = battle.get_at_slot(Some(&slot_str)) {
                    // Return the new target position for move redirection
                    let target_pos = (new_target.side_index, new_target.position);
                    return EventResult::Position(target_pos);
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
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        debug_elog!("[COUNTER] on_damaging_hit called: damage={}, target_pos={:?}, source_pos={:?}", damage, target_pos, source_pos);
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

        // Use active_move category (runtime) instead of static dex data
        // JS uses this.getCategory(move) which returns the effective category
        // Moves like Photon Geyser can change category based on stats
        let category = active_move.map(|m| m.category.as_str()).unwrap_or("");

        if !is_ally && category == "Physical" {
            debug_elog!("[COUNTER] on_damaging_hit: is_ally=false, category=Physical - storing damage * 2 = {}", 2 * damage);
            debug_elog!("[COUNTER] battle.effect = {:?}", battle.effect);
            // this.effectState.slot = source.getSlot();
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let _slot = source_pokemon.get_slot();

            // this.effectState.damage = 2 * damage;
            let _update_result = battle.with_effect_state(|state| {
                // Store slot as integer (parse from slot string like "a: Pikachu" -> position)
                // Actually, slot is the string identifier, but we need to store it
                // The slot field is i32, but Pokemon::get_slot() returns a string
                // We'll store the position instead
                // Looking at the JavaScript, slot is stored for use in getAtSlot
                // In Rust, we may need to adjust - let's store it as string in a different way
                // Actually wait - looking at the redirect code, it expects an i32
                // Let me store the numeric position
                state.slot = Some(source.1 as i32);
                state.damage = Some(2 * damage);
                debug_elog!("[COUNTER] on_damaging_hit: set effect_state: slot={:?}, damage={:?}", state.slot, state.damage);
            });
            debug_elog!("[COUNTER] with_effect_state returned: {:?}", _update_result);
        } else {
            debug_elog!("[COUNTER] on_damaging_hit: is_ally={}, category='{}' - NOT storing damage", is_ally, category);
        }

        EventResult::Continue
    }
}
