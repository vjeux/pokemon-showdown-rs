# Missing Methods - Pokemon Showdown Rust Port

This file tracks all methods from the TypeScript `sim/` files that are not yet implemented in the Rust port.

## Summary

| File | TS Methods | Implemented in RS | Status |
|------|------------|-------------------|--------|
| battle.ts | ~100+ | ~30 | Partial |
| battle-actions.ts | 42 | ~15 | Partial |
| pokemon.ts | ~80+ | ~50 | ✓ Core Complete |
| side.ts | ~40+ | ~35 | ✓ Core Complete |
| field.ts | 17 | ~17 | ✓ Complete |
| battle-queue.ts | 20 | ~18 | ✓ Core Complete |
| prng.ts | 22 | ~20 | ✓ Complete |
| battle-stream.ts | ~20 | 0 | Not Started |
| state.ts | 22 | 0 | Not Started |
| teams.ts | ~10 | ~5 | Partial |
| team-validator.ts | 27 | 0 | Not Started |
| dex.ts | 26 | ~10 | Partial |
| dex-data.ts | 20 | ~15 | Partial |

---

## battle.ts → battle.rs

### Implemented Methods:
- [x] switch_in (switch pokemon in) - Full implementation
- [x] drag_in (force drag pokemon in) - Full implementation
- [x] run_switch (execute switch queue) - Full implementation
- [x] run_move (execute move action) - Basic implementation
- [x] use_move (use a move) - Basic implementation
- [x] run_event (run battle events) - Full implementation
- [x] single_event (run single event) - Full implementation
- [x] run_event_bool (run event returning bool) - Full implementation
- [x] each_event (run event on all targets) - Full implementation
- [x] hint (send hint message) - Full implementation
- [x] add (add protocol message) - Full implementation
- [x] debug (debug logging) - Full implementation

### Remaining Methods:
- [ ] addSplit (split battle messages to multiple players)
- [ ] attrLastMove (attribute damage to last move)
- [ ] chainModify (chain multiple damage modifiers)
- [ ] checkEVBalance (check if teams have balanced EVs)
- [ ] clearEffectState (clear effect state data)
- [ ] debugError (log debug errors)
- [ ] faintMessages (output faint messages)
- [ ] fieldEvent (run event on all field effects)
- [ ] getCallback (get event callback for effect)
- [ ] getOverflowedTurnCount (get turn overflow count)
- [ ] getRequests (get battle requests for all players)
- [ ] getTeam (get team based on player options)
- [ ] join (join player to battle)
- [ ] onEvent (register event listener)
- [ ] resetRNG (reset pseudo-random number generator with seed)
- [ ] resolvePriority (resolve event priority ordering)
- [ ] retargetLastMove (retarget the last executed move)
- [ ] runPickTeam (run team preview phase)
- [ ] sendUpdates (send updates to connected players)
- [ ] showOpenTeamSheets (show team sheets to players)
- [ ] spreadModify (spread damage modifier across multiple targets)
- [ ] statModify (modify pokemon stat)
- [ ] tiebreak (execute tiebreak logic when sides are tied)
- [ ] toJSON (serialize battle to JSON)
- [ ] toString (convert battle to string representation)

---

## battle-actions.ts → battle_actions.rs

### Implemented Methods:
- [x] switchIn (switch pokemon in) - Implemented in battle.rs
- [x] dragIn (force drag pokemon in) - Implemented in battle.rs
- [x] runSwitch (execute switch queue) - Implemented in battle.rs
- [x] runMove (execute move action) - Basic implementation in battle.rs
- [x] useMove (use a move) - Basic implementation in battle.rs
- [x] calcRecoilDamage (calculate recoil damage) - Implemented in battle_actions.rs
- [x] getZMove (get Z-move variant) - Implemented in battle_actions.rs
- [x] canZMove (check if Z-move is possible) - Implemented in battle_actions.rs
- [x] getMaxMove (get Max move variant) - Implemented in battle_actions.rs
- [x] targetTypeChoices (determine valid targets) - Implemented in battle_actions.rs
- [x] getConfusionDamage (calculate confusion damage) - Implemented in battle_actions.rs
- [x] canMegaEvo (check if Mega Evolution is possible) - Implemented in battle_actions.rs
- [x] canUltraBurst (check if Ultra Burst is possible) - Implemented in battle_actions.rs
- [x] canTerastallize (check if Terastallization is possible) - Implemented in battle_actions.rs
- [x] calculate_damage (calculate damage) - Implemented in battle_actions.rs

### Remaining Methods:
- [ ] useMoveInner (internal move execution)
- [ ] trySpreadMoveHit (try hitting multiple targets)
- [ ] hitStepInvulnerabilityEvent (check invulnerability)
- [ ] hitStepTryHitEvent (trigger tryHit event)
- [ ] hitStepTypeImmunity (check type immunity)
- [ ] hitStepTryImmunity (check immunity mechanics)
- [ ] hitStepAccuracy (accuracy calculation step)
- [ ] hitStepBreakProtect (protect breaking check)
- [ ] hitStepStealBoosts (steal stat boosts)
- [ ] afterMoveSecondaryEvent (handle secondary effects)
- [ ] tryMoveHit (attempt move hit)
- [ ] hitStepMoveHitLoop (execute hit loop for multiple targets)
- [ ] spreadMoveHit (apply damage to multiple targets)
- [ ] tryPrimaryHitEvent (trigger primary hit event)
- [ ] getSpreadDamage (calculate spread damage)
- [ ] runMoveEffects (execute move effects)
- [ ] selfDrops (apply self-inflicted stat drops)
- [ ] secondaries (handle secondary effects)
- [ ] forceSwitch (force switch out)
- [ ] moveHit (execute move hit)
- [ ] getActiveZMove (get active Z-move)
- [ ] getActiveMaxMove (get active Max move)
- [ ] runZPower (execute Z-power effect)
- [ ] modifyDamage (apply damage modifiers)
- [ ] runMegaEvo (execute Mega Evolution)
- [ ] terastallize (execute Terastallization)

---

## pokemon.ts → pokemon.rs ✓ CORE COMPLETE

### Implemented Methods:
- [x] get_slot - Get slot identifier
- [x] to_string - String representation
- [x] get_updated_details - Get protocol details
- [x] calculate_stat - Calculate stat with boost
- [x] get_best_stat - Get best stat (for Quark Drive, etc.)
- [x] has_type - Check if has specific type
- [x] has_any_type - Check if has any of given types
- [x] faint - Mark as fainted
- [x] damage - Apply damage
- [x] is_ally - Check if ally
- [x] is_adjacent - Check if adjacent
- [x] get_capped_boost - Get capped boost value
- [x] boost_by - Boost stat with tracking
- [x] set_boost - Set boost value
- [x] clear_ability - Clear ability
- [x] set_ability - Set ability
- [x] get_ability - Get ability
- [x] clear_item - Clear item
- [x] set_item - Set item
- [x] take_item - Remove and return item
- [x] get_item - Get item
- [x] set_hp - Set HP directly
- [x] move_used - Record move usage
- [x] is_last_active - Check if last active
- [x] ignoring_ability - Check if ability ignored
- [x] ignoring_item - Check if item ignored
- [x] update_max_hp - Update max HP
- [x] is_sky_dropped - Check if in Sky Drop
- [x] get_move_data - Get move slot data
- [x] get_move_data_mut - Get mutable move slot
- [x] get_positive_boosts - Get positive boost count
- [x] get_undynamaxed_hp - Get HP as if not Dynamaxed
- [x] try_trap - Attempt to trap
- [x] got_attacked - Record attack received
- [x] get_locked_move - Get locked move
- [x] max_move_disabled - Check if max move disabled
- [x] transform_into - Transform into another Pokemon
- [x] copy_volatile_from_full - Copy volatiles (Baton Pass)
- [x] set_species - Set species
- [x] forme_change - Change forme
- [x] clear_turn_state_full - Clear all turn state

### Remaining Methods (Low Priority):
- [ ] getAtLoc (Get Pokemon at a specific field location)
- [ ] getCombatPower (Calculate combat power value)
- [ ] getDynamaxRequest (Get Dynamax request data for protocol)
- [ ] getLastAttackedBy (Get last Pokemon that attacked this one)
- [ ] getLastDamagedBy (Get last Pokemon that damaged this one)
- [ ] getLocOf (Get field location of a Pokemon)
- [ ] getMoveHitData (Get hit data for a move)
- [ ] getMoveRequestData (Get move request data for protocol)
- [ ] getNature (Get the Pokemon's nature)
- [ ] getSmartTargets (Get smart targeting for moves)
- [ ] getStatus (Get the Pokemon's status condition)
- [ ] removeLinkedVolatiles (Remove linked volatile conditions)
- [ ] runEffectiveness (Run move type effectiveness check)
- [ ] runImmunity (Run type/effect immunity check)
- [ ] runStatusImmunity (Run status effect immunity check)
- [ ] trySetStatus (Attempt to set status with immunity checks)
- [ ] useItem (Use held item)
- [ ] eatItem (Handle Pokemon eating a held item)
- [ ] deductPP (Deduct PP from move)
- [ ] destroy (Clean up and destroy Pokemon instance)

---

## side.ts → side.rs ✓ CORE COMPLETE

### Implemented Methods:
- [x] to_string - String representation
- [x] can_dynamax_now - Check if can dynamax (Gen 8)
- [x] allies - Get allied Pokemon
- [x] foes_active - Get foe Pokemon (stub)
- [x] has_ally - Check if ally
- [x] add_pokemon - Add Pokemon to team
- [x] random_foe - Get random foe (stub)
- [x] foe_pokemon_left - Get foe count (stub)
- [x] get_slot_condition - Get slot condition
- [x] get_slot_condition_mut - Get mutable slot condition
- [x] clear_choice - Clear choice state
- [x] get_choice_index - Get choice action index
- [x] choose_pass - Pass action
- [x] choose_switch - Switch action
- [x] choose_move - Move action with validations
- [x] choose_team - Team preview selection
- [x] choose_shift - Triples shift action
- [x] auto_choose - Auto-complete choices
- [x] picked_team_size - Team preview size
- [x] destroy - Cleanup
- [x] is_choice_done - Check if choice complete
- [x] get_choice - Get choice as string
- [x] add_side_condition - Add side condition
- [x] has_side_condition - Check side condition
- [x] get_side_condition - Get side condition
- [x] remove_side_condition - Remove side condition
- [x] add_slot_condition - Add slot condition
- [x] has_slot_condition - Check slot condition
- [x] remove_slot_condition - Remove slot condition
- [x] add_hazard - Add hazard with layers
- [x] get_hazard_layers - Get hazard layer count

### Remaining Methods (Low Priority):
- [ ] getRequestData (get side request data for protocol)
- [ ] send (send message to clients)
- [ ] emitRequest (emit request to client)
- [ ] emitChoiceError (emit choice error to client)
- [ ] updateDisabledRequest (update request with disabled moves)
- [ ] updateRequestForPokemon (update request for specific Pokemon)
- [ ] activeTeam (get active team, handling multi battles)
- [ ] foeSidesWithConditions (iterate through all foe side conditions)

---

## field.ts → field.rs ✓ COMPLETE

### All Methods Implemented:
- [x] has_weather - Check if weather active
- [x] is_weather_active - Check any weather
- [x] set_weather - Set weather
- [x] clear_weather - Clear weather
- [x] effective_weather - Get effective weather
- [x] is_weather - Check weather match
- [x] is_weather_any - Check weather in list
- [x] get_weather - Get weather ID
- [x] get_weather_state - Get weather state
- [x] has_terrain - Check terrain active
- [x] is_terrain_active - Check any terrain
- [x] set_terrain - Set terrain
- [x] clear_terrain - Clear terrain
- [x] effective_terrain - Get effective terrain
- [x] is_terrain - Check terrain match
- [x] is_terrain_any - Check terrain in list
- [x] get_terrain - Get terrain ID
- [x] get_terrain_state - Get terrain state
- [x] add_pseudo_weather - Add pseudo-weather
- [x] has_pseudo_weather - Check pseudo-weather
- [x] get_pseudo_weather - Get pseudo-weather
- [x] remove_pseudo_weather - Remove pseudo-weather
- [x] decrement_durations - Decrement all durations
- [x] suppressing_weather - Check if suppressing (stub)
- [x] destroy - Cleanup

---

## battle-queue.ts → battle_queue.rs ✓ CORE COMPLETE

### Implemented Methods:
- [x] shift - Get next action
- [x] peek - Peek next action
- [x] peek_end - Peek last action
- [x] push - Push action
- [x] unshift - Unshift action
- [x] len - Get length
- [x] is_empty - Check empty
- [x] clear - Clear queue
- [x] cancel_action - Cancel action for Pokemon
- [x] cancel_move - Cancel move for Pokemon
- [x] will_move - Check if Pokemon will move
- [x] will_switch - Check if Pokemon will switch
- [x] will_act - Check if any action
- [x] will_act_full - Check with full details
- [x] insert_run_switch - Insert runSwitch action
- [x] insert_choice - Insert at front
- [x] insert_in_order - Insert maintaining order
- [x] sort - Sort by priority/speed
- [x] prioritize_action - Move action to front
- [x] prioritize_action_ref - Prioritize with order change
- [x] change_action - Cancel and reinsert
- [x] add_choice - Add action
- [x] iter - Get iterator
- [x] iter_mut - Get mutable iterator
- [x] entries - Get entries with indices
- [x] find - Find by predicate
- [x] remove_where - Remove matching
- [x] debug - Debug output

### Remaining Methods:
- [ ] resolveAction (resolve action choices into full objects - needs battle context)

---

## prng.ts → prng.rs ✓ COMPLETE

### All Methods Implemented:
- [x] PRNGSeed enum (Sodium, Gen5)
- [x] Gen5RNG struct (full LCG implementation)
- [x] SodiumRNG struct (ChaCha20 implementation)
- [x] PRNG struct with:
  - [x] new - Create from seed
  - [x] from_seed_string - Parse seed string
  - [x] get_seed - Get current seed
  - [x] set_seed - Set seed
  - [x] clone_with_current_seed - Clone PRNG
  - [x] random - Get random number (various forms)
  - [x] random_int - Random integer in range
  - [x] random_range - Random in range
  - [x] random_float - Random float [0, 1)
  - [x] random_chance - Flip weighted coin
  - [x] sample - Random element
  - [x] sample_remove - Sample and remove
  - [x] sample_n - Sample n unique
  - [x] shuffle - Fisher-Yates shuffle
  - [x] shuffle_range - Shuffle range
  - [x] generate_seed - Generate random seed
  - [x] convert_seed - Seed to string
  - [x] get - Factory method

---

## Files Not Started (Lower Priority)

### battle-stream.ts → battle_stream.rs
- Stream handling for battle communication
- Not critical for core battle simulation

### state.ts → state.rs
- Serialization/deserialization
- Can use serde derive macros for most functionality

### team-validator.ts → team_validator.rs
- Team validation rules
- Complex but not required for battle execution

### teams.ts → teams.rs
- Team import/export
- Basic team parsing already implemented

---

## Implementation Priority

### ✓ Completed (Core Battle Logic):
1. pokemon.ts methods - Core complete
2. side.ts methods - Core complete
3. field.ts methods - Complete
4. battle-queue.ts methods - Core complete
5. prng.ts methods - Complete

### In Progress:
6. battle-actions.ts methods - Hit step methods needed
7. battle.ts remaining methods - Protocol/serialization

### Lower Priority:
8. state.ts methods - Serialization
9. teams.ts methods - Team parsing
10. team-validator.ts methods - Validation
11. battle-stream.ts methods - Streaming
12. dex.ts methods - Data loading
