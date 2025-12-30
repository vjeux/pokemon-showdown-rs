//! BattleActions::switchIn - Switch a Pokemon in
//!
//! 1:1 port of switchIn from battle-actions.ts

use crate::*;
use crate::battle::SwitchResult;
use crate::event_system::EffectState;

/// Switch a Pokemon in
/// Equivalent to battle-actions.ts switchIn()
/// Returns false if switch failed, true if successful, or "pursuitfaint" string converted to SwitchResult
pub fn switch_in(
    battle: &mut Battle,
    side_index: usize,
    pos: usize,
    pokemon_index: usize,
    source_effect: Option<&ID>,
    is_drag: bool,
) -> SwitchResult {
    // Check if pokemon exists and is not already active
    let side = match battle.sides.get(side_index) {
        Some(s) => s,
        None => return SwitchResult::Failed,
    };

    let pokemon_is_active = match side.pokemon.get(pokemon_index) {
        Some(p) => p.is_active,
        None => return SwitchResult::Failed,
    };

    if pokemon_is_active {
        battle.hint(
            "A switch failed because the Pokémon trying to switch in is already in.",
            false,
            None,
        );
        return SwitchResult::Failed;
    }

    if pos >= side.active.len() {
        return SwitchResult::Failed;
    }

    // Get the old active Pokemon index if any
    let old_active_idx = side.active.get(pos).and_then(|&opt| opt);

    // Handle old active Pokemon switching out
    if let Some(old_idx) = old_active_idx {
        let side = &battle.sides[side_index];
        let old_pokemon = &side.pokemon[old_idx];

        if old_pokemon.hp > 0 {
            // Mark as being called back
            battle.sides[side_index].pokemon[old_idx].being_called_back = true;

            // Run BeforeSwitchOut event (unless skipBeforeSwitchOutEventFlag or is_drag)
            let skip_event =
                battle.sides[side_index].pokemon[old_idx].skip_before_switch_out_event_flag;
            if !skip_event && !is_drag {
                battle.run_event(
                    "BeforeSwitchOut",
                    Some((side_index, old_idx)),
                    None,
                    None,
                    None,
                );
                if battle.gen >= 5 {
                    // JS: this.battle.eachEvent("Update");
                    battle.each_event("Update", None);
                }
            }

            battle.sides[side_index].pokemon[old_idx].skip_before_switch_out_event_flag = false;

            // Run SwitchOut event
            if !battle.run_event_bool("SwitchOut", Some((side_index, old_idx)), None, None) {
                return SwitchResult::Failed;
            }

            // Check if fainted from Pursuit
            if battle.sides[side_index].pokemon[old_idx].hp == 0 {
                return SwitchResult::PursuitFaint;
            }

            // Will definitely switch out at this point
            battle.sides[side_index].pokemon[old_idx].illusion = None;

            // Trigger End events for ability and item
            let ability_id = battle.sides[side_index].pokemon[old_idx].ability.clone();
            battle.single_event("End", &ability_id, Some((side_index, old_idx)), None, None);
            let item_id = battle.sides[side_index].pokemon[old_idx].item.clone();
            battle.single_event("End", &item_id, Some((side_index, old_idx)), None, None);

            // Cancel any queued action
            battle.queue.cancel_action(side_index, old_idx);

            // Clear volatiles on old Pokemon
            battle.sides[side_index].pokemon[old_idx].clear_volatiles();
        }

        // Update old active state
        let old_position = battle.sides[side_index].pokemon[pokemon_index].position;
        {
            let old_pokemon = &mut battle.sides[side_index].pokemon[old_idx];
            old_pokemon.is_active = false;
            old_pokemon.is_started = false;
            old_pokemon.used_item_this_turn = false;
            old_pokemon.stats_raised_this_turn = false;
            old_pokemon.stats_lowered_this_turn = false;
            old_pokemon.position = old_position;
            if old_pokemon.fainted {
                old_pokemon.status = ID::empty();
            }
        }

        // Swap positions in active array
        // Note: JavaScript swaps pokemon in the pokemon array to maintain index=position invariant.
        // Rust uses an index-based architecture (borrow-checker workaround), so we don't swap
        // the pokemon Vec, only update the active array and position fields.
        let new_position = pos;
        battle.sides[side_index].pokemon[pokemon_index].position = new_position;
    }

    // Set up new active Pokemon
    {
        let side = &mut battle.sides[side_index];
        let pokemon = &mut side.pokemon[pokemon_index];

        pokemon.is_active = true;
        side.active[pos] = Some(pokemon_index);
        pokemon.active_turns = 0;
        pokemon.active_move_actions = 0;

        // Reset move.used for all moves
        for move_slot in &mut pokemon.move_slots {
            move_slot.used = false;
        }

        // Initialize ability and item state
        pokemon.ability_state = EffectState::new(pokemon.ability.clone());
        pokemon.item_state = EffectState::new(pokemon.item.clone());
    }

    // Run BeforeSwitchIn event
    battle.run_event(
        "BeforeSwitchIn",
        Some((side_index, pokemon_index)),
        None,
        None,
        None,
    );

    // Log the switch
    let (details, hp_display) = {
        let pokemon = &battle.sides[side_index].pokemon[pokemon_index];
        let details = pokemon.details();
        let hp = format!("{}/{}", pokemon.hp, pokemon.maxhp);
        (details, hp)
    };
    let side_id = battle.sides[side_index].id_str().to_string();
    let pokemon_name = battle.sides[side_index].pokemon[pokemon_index].name.clone();

    let event_type = if is_drag { "drag" } else { "switch" };
    if let Some(effect) = source_effect {
        battle.log.push(format!(
            "|{}|{}: {}|{}|{}|[from] {}",
            event_type,
            side_id,
            pokemon_name,
            details,
            hp_display,
            effect.as_str()
        ));
    } else {
        battle.log.push(format!(
            "|{}|{}: {}|{}|{}",
            event_type, side_id, pokemon_name, details, hp_display
        ));
    }

    // Gen 2 drag tracking
    if is_drag && battle.gen == 2 {
        battle.sides[side_index].pokemon[pokemon_index].dragged_in = Some(battle.turn as usize);
    }
    battle.sides[side_index].pokemon[pokemon_index].previously_switched_in += 1;

    // TODO: Hazards should be applied via side condition onSwitchIn callbacks
    // (triggered by field_event_switch_in in run_switch), not directly here.
    // TypeScript: fieldEvent('SwitchIn') → stealthrock.onSwitchIn(), spikes.onSwitchIn(), etc.
    // Rust: field_event_switch_in() should trigger condition_callbacks for hazards

    // Run switch or queue it
    if is_drag && battle.gen >= 5 {
        // runSwitch happens immediately so that Mold Breaker can make hazards bypass Clear Body and Levitate
        battle.run_switch(side_index, pokemon_index);
    } else {
        // JS: this.battle.queue.insertChoice({ choice: "runSwitch", pokemon });
        battle.insert_run_switch_action(side_index, pokemon_index);
    }

    SwitchResult::Success
}
