//! BattleActions::switchIn - Switch a Pokemon in
//!
//! 1:1 port of switchIn from battle-actions.ts

use crate::*;
use crate::event::EventResult;
use crate::battle::SwitchResult;
use crate::event_system::EffectState;

/// Switch a Pokemon in
/// Equivalent to battle-actions.ts switchIn()
/// switchIn(pokemon: Pokemon, pos: number, sourceEffect: Effect | null = null, isDrag?: boolean) {
///     if (!pokemon || pokemon.isActive) {
///         this.battle.hint("A switch failed because the Pokémon trying to switch in is already in.");
///         return false;
///     }
///
///     const side = pokemon.side;
///     if (pos >= side.active.length) {
///         throw new Error(`Invalid switch position ${pos} / ${side.active.length}`);
///     }
///     const oldActive = side.active[pos];
///     const unfaintedActive = oldActive?.hp ? oldActive : null;
///     if (unfaintedActive) {
///         oldActive.beingCalledBack = true;
///         let switchCopyFlag: 'copyvolatile' | 'shedtail' | boolean = false;
///         if (sourceEffect && typeof (sourceEffect as Move).selfSwitch === 'string') {
///             switchCopyFlag = (sourceEffect as Move).selfSwitch!;
///         }
///         if (!oldActive.skipBeforeSwitchOutEventFlag && !isDrag) {
///             this.battle.runEvent('BeforeSwitchOut', oldActive);
///             if (this.battle.gen >= 5) {
///                 this.battle.eachEvent('Update');
///             }
///         }
///         oldActive.skipBeforeSwitchOutEventFlag = false;
///         if (!this.battle.runEvent('SwitchOut', oldActive)) {
///             // Warning: DO NOT interrupt a switch-out if you just want to trap a pokemon.
///             // To trap a pokemon and prevent it from switching out, (e.g. Mean Look, Magnet Pull)
///             // use the 'trapped' flag instead.
///
///             // Note: Nothing in the real games can interrupt a switch-out (except Pursuit KOing,
///             // which is handled elsewhere); this is just for custom formats.
///             return false;
///         }
///         if (!oldActive.hp) {
///             // a pokemon fainted from Pursuit before it could switch
///             return 'pursuitfaint';
///         }
///
///         // will definitely switch out at this point
///
///         oldActive.illusion = null;
///         this.battle.singleEvent('End', oldActive.getAbility(), oldActive.abilityState, oldActive);
///         this.battle.singleEvent('End', oldActive.getItem(), oldActive.itemState, oldActive);
///
///         // if a pokemon is forced out by Whirlwind/etc or Eject Button/Pack, it can't use its chosen move
///         this.battle.queue.cancelAction(oldActive);
///
///         let newMove = null;
///         if (this.battle.gen === 4 && sourceEffect) {
///             newMove = oldActive.lastMove;
///         }
///         if (switchCopyFlag) {
///             pokemon.copyVolatileFrom(oldActive, switchCopyFlag);
///         }
///         if (newMove) pokemon.lastMove = newMove;
///         oldActive.clearVolatile();
///     }
///     if (oldActive) {
///         oldActive.isActive = false;
///         oldActive.isStarted = false;
///         oldActive.usedItemThisTurn = false;
///         oldActive.statsRaisedThisTurn = false;
///         oldActive.statsLoweredThisTurn = false;
///         oldActive.position = pokemon.position;
///         if (oldActive.fainted) oldActive.status = '';
///         if (this.battle.gen <= 4) {
///             pokemon.lastItem = oldActive.lastItem;
///             oldActive.lastItem = '';
///         }
///         pokemon.position = pos;
///         side.pokemon[pokemon.position] = pokemon;
///         side.pokemon[oldActive.position] = oldActive;
///     }
///     pokemon.isActive = true;
///     side.active[pos] = pokemon;
///     pokemon.activeTurns = 0;
///     pokemon.activeMoveActions = 0;
///     for (const moveSlot of pokemon.moveSlots) {
///         moveSlot.used = false;
///     }
///     pokemon.abilityState = this.battle.initEffectState({ id: pokemon.ability, target: pokemon });
///     pokemon.itemState = this.battle.initEffectState({ id: pokemon.item, target: pokemon });
///     this.battle.runEvent('BeforeSwitchIn', pokemon);
///     if (sourceEffect) {
///         this.battle.add(isDrag ? 'drag' : 'switch', pokemon, pokemon.getFullDetails, `[from] ${sourceEffect}`);
///     } else {
///         this.battle.add(isDrag ? 'drag' : 'switch', pokemon, pokemon.getFullDetails);
///     }
///     if (isDrag && this.battle.gen === 2) pokemon.draggedIn = this.battle.turn;
///     pokemon.previouslySwitchedIn++;
///
///     if (isDrag && this.battle.gen >= 5) {
///         // runSwitch happens immediately so that Mold Breaker can make hazards bypass Clear Body and Levitate
///         this.runSwitch(pokemon);
///     } else {
///         this.battle.queue.insertChoice({ choice: 'runSwitch', pokemon });
///     }
///
///     return true;
/// }
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

            // JS: let switchCopyFlag: 'copyvolatile' | 'shedtail' | boolean = false;
            // JS: if (sourceEffect && typeof (sourceEffect as Move).selfSwitch === 'string') {
            // JS:     switchCopyFlag = (sourceEffect as Move).selfSwitch!;
            // JS: }
            // Check if source effect is a move with selfSwitch property
            let switch_copy_flag: Option<String> = if let Some(effect_id) = source_effect {
                // Check if effect is a move and has selfSwitch property
                if let Some(move_data) = battle.dex.moves().get(effect_id.as_str()) {
                    // self_switch is Option<Value>, extract string if present
                    move_data.self_switch.as_ref().and_then(|v| v.as_str()).map(|s| s.to_string())
                } else {
                    None
                }
            } else {
                None
            };

            // Run BeforeSwitchOut event (unless skipBeforeSwitchOutEventFlag or is_drag)
            let skip_event =
                battle.sides[side_index].pokemon[old_idx].skip_before_switch_out_event_flag;
            if !skip_event && !is_drag {
                battle.run_event(
                "BeforeSwitchOut",
                Some(crate::event::EventTarget::Pokemon((side_index, old_idx))),
                    None,
                    None,
                    EventResult::Continue,
                    false,
                    false,
                );
                if battle.gen >= 5 {
                    // JS: this.battle.eachEvent("Update");
                    battle.each_event("Update", None, None);
                }
            }

            battle.sides[side_index].pokemon[old_idx].skip_before_switch_out_event_flag = false;

            // Clear switch_flag since we're now handling the switch
            // In JavaScript, switchFlag is cleared when processing the switch action
            battle.sides[side_index].pokemon[old_idx].switch_flag = None;

            // Run SwitchOut event
            if !battle.run_event(
                "SwitchOut",
                Some(crate::event::EventTarget::Pokemon((side_index, old_idx))),
                None,
                None,
                crate::event::EventResult::Number(1),
                false,
                false,
            ).is_truthy() {
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
            battle.single_event("End", &crate::battle::Effect::ability(ability_id), Some((side_index, old_idx)), None, None, None);
            let item_id = battle.sides[side_index].pokemon[old_idx].item.clone();
            battle.single_event("End", &crate::battle::Effect::item(item_id), Some((side_index, old_idx)), None, None, None);

            // Cancel any queued action
            battle.queue.cancel_action(side_index, old_idx);

            // JS: let newMove = null;
            // JS: if (this.battle.gen === 4 && sourceEffect) {
            // JS:     newMove = oldActive.lastMove;
            // JS: }
            let new_move = if battle.gen == 4 && source_effect.is_some() {
                Some(battle.sides[side_index].pokemon[old_idx].last_move.clone())
            } else {
                None
            };

            // JS: if (switchCopyFlag) {
            // JS:     pokemon.copyVolatileFrom(oldActive, switchCopyFlag);
            // JS: }
            if let Some(ref copy_flag) = switch_copy_flag {
                // pokemon.copyVolatileFrom(oldActive, switchCopyFlag);
                Pokemon::copy_volatile_from(
                    battle,
                    (side_index, pokemon_index),
                    (side_index, old_idx),
                    Some(copy_flag.as_str()),
                );
            }

            // JS: if (newMove) pokemon.lastMove = newMove;
            if let Some(ref last_move_id) = new_move {
                battle.sides[side_index].pokemon[pokemon_index].last_move = last_move_id.clone();
            }

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

        // JS: if (this.battle.gen <= 4) {
        // JS:     pokemon.lastItem = oldActive.lastItem;
        // JS:     oldActive.lastItem = '';
        // JS: }
        if battle.gen <= 4 {
            let old_last_item = battle.sides[side_index].pokemon[old_idx].last_item.clone();
            battle.sides[side_index].pokemon[pokemon_index].last_item = old_last_item;
            battle.sides[side_index].pokemon[old_idx].last_item = ID::empty();
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
        eprintln!("[SWITCH_IN] Before update: side_index={}, pos={}, pokemon_index={}, side.active={:?}",
            side_index, pos, pokemon_index, side.active);

        let pokemon = &mut side.pokemon[pokemon_index];

        pokemon.is_active = true;
        side.active[pos] = Some(pokemon_index);

        eprintln!("[SWITCH_IN] After update: side.active={:?}", side.active);

        pokemon.active_turns = 0;
        pokemon.active_move_actions = 0;

        // Reset move.used for all moves
        for move_slot in &mut pokemon.move_slots {
            move_slot.used = false;
        }

        // Initialize ability and item state
        // JS: pokemon.abilityState = this.battle.initEffectState({ id: pokemon.ability, target: pokemon });
        // JS: pokemon.itemState = this.battle.initEffectState({ id: pokemon.item, target: pokemon });
        pokemon.ability_state = EffectState::new(pokemon.ability.clone());
        pokemon.ability_state.target = Some((side_index, pokemon_index));
        pokemon.item_state = EffectState::new(pokemon.item.clone());
        pokemon.item_state.target = Some((side_index, pokemon_index));
    }

    // Run BeforeSwitchIn event
    battle.run_event(
                "BeforeSwitchIn",
                Some(crate::event::EventTarget::Pokemon((side_index, pokemon_index))),
        None,
        None,
        EventResult::Continue,
        false,
        false,
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
        // JavaScript: pokemon.draggedIn = this.turn (number | null type)
        battle.sides[side_index].pokemon[pokemon_index].dragged_in = Some(battle.turn);
    }
    battle.sides[side_index].pokemon[pokemon_index].previously_switched_in += 1;

    // Note: Hazards are applied via side condition onSwitchIn callbacks
    // (triggered by field_event_switch_in in run_switch), not directly here.
    // TypeScript: fieldEvent('SwitchIn') → stealthrock.onSwitchIn(), spikes.onSwitchIn(), etc.
    // Rust: field_event_switch_in() triggers condition_callbacks for hazards

    // Run switch or queue it
    if is_drag && battle.gen >= 5 {
        // runSwitch happens immediately so that Mold Breaker can make hazards bypass Clear Body and Levitate
        crate::battle_actions::run_switch(battle, side_index, pokemon_index);
    } else {
        // JS: this.battle.queue.insertChoice({ choice: "runSwitch", pokemon });
        battle.insert_run_switch_action(side_index, pokemon_index);
    }

    SwitchResult::Success
}
