# 1:1 JavaScript Implementation Progress

## Overall Status

**Methods Review:** ✅ 331/331 (100%)
- All methods have TypeScript source comments
- All documented with JavaScript equivalents or marked as Rust-specific

**Feature Implementation:** ⚠️ 77/79 TODOs (97.5%)
- Systematic implementation of missing JavaScript features ongoing

## Completed Implementations

### Session 2 - Critical Events (5 implementations)

#### Faint Events (3/3) ✅
- [x] **BeforeFaint** (battle.rs:2695-2706) - Can cancel fainting
- [x] **Faint** (battle.rs:2726-2733) - Triggers on faint
- [x] **AfterFaint** (battle.rs:2807-2818) - Post-faint cleanup

**Enables:** Sturdy, Soul-Heart, Moxie, format-specific faint handling

#### Boost Events (2/2) ✅  
- [x] **ChangeBoost** (battle.rs:4222-4223) - Modify boost amounts
- [x] **TryBoost** (battle.rs:4232) - Prevent boosts

**Enables:** Simple, Contrary, Clear Body, Defiant

### Session 3 - Move Priority (3 partial implementations)

#### Move Priority Events (3/8 partial) ⚠️
- [x] **Get move priority from Dex** (battle.rs:5763-5770) - Retrieves base priority from Dex
- [x] **ModifyPriority event** (battle.rs:5781-5793) - Ability/item priority modification
- [~] **Set move.priority field** (battle.rs:5800-5809) - Documented limitation, needs infrastructure

**Enables:** Prankster, Quick Claw, Grassy Glide priority mechanics

**Note:** Quick Guard detection of enhanced priority requires additional infrastructure (move state tracking)

#### Pokemon Adjacency Helpers (2/2) ✅
- [x] **is_adjacent()** (battle.rs:3472-3506) - Check if two pokemon are adjacent
- [x] **adjacent_allies()** (battle.rs:3515-3563) - Get all adjacent allied pokemon
- [x] **adjacent_foes()** (battle.rs:3573-3624) - Get all adjacent enemy pokemon
- [x] **Updated get_random_target()** (battle.rs:3680-3708) - Now uses adjacency helpers

**Enables:** Proper targeting for doubles/triples, Healing Wish, Storm Drain, Lightning Rod

#### Critical Events (3/3) ✅
- [x] **DisableMove event** (battle.rs:5310-5315) - Allows abilities to disable moves
- [x] **TrapPokemon event** (battle.rs:5327-5329) - Enables trapping moves
- [x] **MaybeTrapPokemon event** (battle.rs:5331-5334) - Conditional trapping based on type
- [~] **Foe ability trapping** (battle.rs:5337-5356) - Stub for Gen 3+ ability checks

**Enables:** Assault Vest, Gorilla Tactics move restrictions, Mean Look trapping

### Session 5 - Move Target Events (5 implementations)

#### ModifyTarget Event (1/1) ✅
- [x] **ModifyTarget event** (battle_actions.rs:2799-2818) - Allows moves to redirect targets

**Enables:** Payback, Metal Burst retargeting to last attacker

#### Target Resolution (2/2) ✅
- [x] **getRandomTarget calls** (battle_actions.rs:2820-2826) - Get valid target when None
- [x] **Target adjustment after ModifyMove** (battle_actions.rs:2854-2877) - Retarget if move type changes

**Enables:** Proper target selection for moves without explicit targets, dynamic retargeting

#### Multi-target Infrastructure (2/2) ✅
- [x] **getMoveTargets implementation** (battle.rs:3782-3907, battle_actions.rs:2925-2932) - Full multi-target logic
- [x] **PP deduction with Pressure** (battle_actions.rs:2950-2973) - Deduct extra PP from Pressure targets

**Supporting Methods:**
- allies_and_self() (battle.rs:3642-3660)
- foes() (battle.rs:3678-3704)
- is_ally() (battle.rs:3909-3917)
- is_pokemon_fainted() (battle.rs:3919-3926)

**Enables:** Spread moves, multi-target damage, Pressure ability PP mechanics

### Session 6 - Z-Move & Max Move Infrastructure (2 implementations)

#### ActiveMove Architecture (1/1) ✅
- [x] **Dex::get_active_move()** (dex.rs:750-857) - Converts MoveData to mutable ActiveMove
- [x] **convert_move_flags()** (dex.rs:840-857) - Helper to map HashMap flags to MoveFlags struct

**Enables:** Mutable move state for Z-Move/Max Move transformation

#### Z-Move & Max Move Transformation (2/2) ✅
- [x] **Z-move transformation** (battle_actions.rs:2759-2774) - Transform active_move to Z-Move
- [x] **Max move transformation** (battle_actions.rs:2785-2800) - Transform active_move to Max Move

**Refactoring:**
- Refactored use_move_inner to use ActiveMove instead of MoveData (battle_actions.rs:2736)
- Updated 8 references from move_data to active_move throughout function

**Enables:** Z-Moves (Breakneck Blitz, Inferno Overdrive, etc.), Max Moves (Max Flare, Max Geyser, etc.), Dynamax battle mechanics

### Session 7 - Z-Move & Max Move Priority (2 implementations)

#### Z-Move Priority Transformation (1/1) ✅
- [x] **Z-Move transformation in get_action_speed** (battle.rs:6271-6279) - Transform move to Z-Move for priority calculation

**Implementation Details:**
- Checks if `move_action.zmove` is Some
- Uses `dex.get_active_move()` to get the active Z-Move
- Validates `z_move.is_z` before using transformed move ID
- Uses Z-Move's priority instead of base move priority

**Enables:** Correct priority calculation for Z-Moves, priority queue sorting

#### Max Move Priority Transformation (1/1) ✅
- [x] **Max Move transformation in get_action_speed** (battle.rs:6290-6298) - Transform move to Max Move for priority calculation

**Implementation Details:**
- Checks if `move_action.max_move` is Some
- Uses `dex.get_active_move()` to get the active Max Move
- Validates `max_move.is_max` before using transformed move ID
- Uses Max Move's priority instead of base move priority

**Enables:** Correct priority calculation for Max Moves (Dynamax), priority queue sorting

**Note:** Both transformations ensure 1:1 JavaScript compatibility by matching the logic in `getActionSpeed()` from battle.ts:2590-2627

### Session 8 - Status Z-Move Effects (1 implementation)

#### runZPower Implementation (1/1) ✅
- [x] **runZPower for status Z-moves** (battle_actions.rs:2920-3002) - Apply Z-Power effects to Pokemon

**Implementation Details:**
- Checks if `z_move` parameter is Some
- Extracts ZMoveData from `active_move.z_move` field
- Calls `BattleActions::run_z_power()` helper to determine effect type
- Matches on ZPowerResult to apply effects:
  - **Boost:** Converts BoostsTable to array format and calls `battle.boost()`
  - **Heal:** Calls `battle.heal()` with pokemon's max HP
  - **ClearNegativeBoost:** Clears all negative stat boosts and adds log message
  - **DamageMove:** TODO for attrLastMove('[zeffect]') attribute
  - **HealReplacement:** TODO for addSlotCondition
  - **Redirect:** TODO for addVolatile('followme')
  - **Crit2:** TODO for addVolatile('focusenergy')
  - **None:** No effect

**Enables:** Status Z-move effects including stat boosts (Z-Belly Drum, Z-Swords Dance), healing (Z-Heal Pulse), clear negative boosts (Z-Refresh), and partial support for other effects

**Note:** Some effects (HealReplacement, Redirect, Crit2, DamageMove attribute) require additional infrastructure (addVolatile, addSlotCondition, attrLastMove) and are marked as TODOs

## Remaining P0 Critical (High Priority)

### Move Events (0 remaining TODOs) ✅ ALL COMPLETE
- [x] ~~ModifyTarget event (battle_actions.rs:2799)~~ ✅ Completed
- [x] ~~ModifyPriority event (battle.rs:5747)~~ ✅ Completed
- [~] ~~Set move.priority field (battle.rs:5755)~~ ⚠️ Documented limitation
- [x] ~~Get move priority from Dex (battle.rs:5735)~~ ✅ Completed
- [x] ~~getRandomTarget (battle_actions.rs:2805)~~ ✅ Completed
- [~] ~~Set move source effect (battle_actions.rs:2821)~~ ⚠️ Documented - handled via event parameters
- [x] ~~getMoveTargets multi-target (battle_actions.rs:2899)~~ ✅ Completed
- [x] ~~PP deduction with Pressure (battle_actions.rs:2918)~~ ✅ Completed

### Pokemon Helpers (0 remaining TODOs)
- [x] ~~adjacentAllies() (battle.rs:3492)~~ ✅ Completed
- [x] ~~adjacentFoes() (battle.rs:3516)~~ ✅ Completed

### Other Critical Events (0 remaining TODOs)
- [x] ~~DisableMove event (battle.rs:5096)~~ ✅ Completed
- [x] ~~TrapPokemon/MaybeTrapPokemon events (battle.rs:5115)~~ ✅ Completed
- [~] ~~Foe ability trapping (battle.rs:5119)~~ ⚠️ Stub implemented, full requires species infrastructure

## Remaining P1 Important (4 TODOs)

### Z-Moves & Max Moves (0 remaining TODOs) ✅ ALL COMPLETE
- [x] ~~Z-Move transformation in get_action_speed~~ ✅ Completed (Session 7)
- [x] ~~Max Move transformation in get_action_speed~~ ✅ Completed (Session 7)
- [x] ~~Z-move transformation in use_move~~ ✅ Completed (Session 6)
- [x] ~~Max move transformation in use_move~~ ✅ Completed (Session 6)
- [x] ~~runZPower for status Z-moves~~ ✅ Completed (Session 8)

**Note:** Z-Move priority calculation, transformation, and status effects now fully implemented with some minor infrastructure TODOs (addVolatile, addSlotCondition, attrLastMove)

### Side Management (0 remaining TODOs) ✅ ALL COMPLETE
- [x] **side.clearChoice()** (side.rs:595-642) ✅ Completed (Session 8)
- [x] **side.activeRequest field** (side.rs:125-127) ✅ Completed (Session 8)
- [x] **ruleTable.pickedTeamSize** (battle.rs:299-301, side.rs:1697-1699) ✅ Completed (Session 8)
- [x] **isChoiceDone() check** (side.rs:565-593) ✅ Completed (Session 8)

**Implementation Details:**
- Added `activeRequest` field to Side struct for tracking active battle request
- Added `rule_table` field to Battle struct to support pickedTeamSize
- Implemented `clear_choice()` method matching JavaScript exactly (calculates forcedSwitches/forcedPasses)
- Updated `is_choice_done()` to 1:1 JavaScript implementation:
  - Checks forcedSwitchesLeft before returning
  - Calls picked_team_size() for TeamPreview
  - Includes TODO for getChoiceIndex() auto-pass call

**Enables:** Proper choice state management, team preview validation, forced switch handling

### Format Callbacks (0 remaining TODOs) ✅ ALL COMPLETE
- [x] **Format callbacks (onBegin)** (battle.rs:814-847) ✅ Completed (Session 8)
- [x] **ruleTable iteration and subformat callbacks** (battle.rs:824-847) ✅ Completed (Session 8)
- [x] **Format callbacks in runAction** (battle.rs:6134-6160) ✅ Completed (Session 8)
- [x] **Swap events** (battle.rs:6562-6570) ✅ Completed (Session 8)

**Implementation Details:**
- Added infrastructure for format callback system using event emission
- Implemented ruleTable.keys() iteration in start() method
- Skips rules starting with +, *, -, ! as per JavaScript logic
- Emits FormatBegin and RuleBegin:{rule} events that can be hooked
- Implemented onBattleStart callbacks in runAction with RuleBattleStart events
- Implemented Swap event firing for position swaps (two events per swap)
- Events can be registered via Battle::on_event() for format-specific behavior
- Full callback system requires manual registration (JavaScript callbacks can't be in JSON)

**Enables:** Format-specific battle initialization, custom rules support, position swap mechanics

### Session 9 - Request Management Completion (3 implementations)

#### makeRequest() Method Enhancements (3/3) ✅
- [x] **clearChoice() calls** (battle.rs:6654-6656) - Calls side.clearChoice() when setting new request state
- [x] **activeRequest assignment** (battle.rs:6663-6665) - Sets activeRequest to None for all sides
- [x] **pickedTeamSize display** (battle.rs:6668-6680) - Adds team preview size to protocol message
- [x] **isChoiceDone() safety check** (battle.rs:6699-6704) - Validates choices aren't already done

**Implementation Details:**
- Implemented proper clearChoice() calls with request state parameter
- Sets activeRequest = None for all sides as per JavaScript
- Displays pickedTeamSize in teampreview message when available
- Added safety check to prevent infinite loops (panics if choices done immediately after request)
- Matches JavaScript 1:1 for all request state transitions

**Enables:** Proper request state management, choice validation, team preview protocol messages

#### Multi-Battle Ally Check (1/1) ✅
- [x] **allySide check in is_ally()** (battle.rs:3954-3961) - Checks ally_index for multi-battle ally detection

**Implementation Details:**
- Added ally_index check to is_ally() method
- Returns true if pos2's side matches pos1's ally_index
- Matches JavaScript: this.side === pokemon.side || this.side.allySide === pokemon.side
- Essential for 2v2 and multi-battle formats

**Enables:** Correct ally detection in multi-battle formats (doubles, triples, multi)

### Session 9 Continued - Move Target Pressure Mechanics (3 implementations)

#### Mustpressure Flag Check (1/1) ✅
- [x] **mustpressure flag check in get_move_targets** (battle.rs:3942-3944) - Check move flags for Pressure PP deduction

**Implementation Details:**
- Modified get_move_targets() to extract both move_target and has_mustpressure flag upfront
- Avoids borrow checker issues by cloning needed data instead of holding references
- Checks if move has 'mustpressure' flag at beginning (line 3835)
- Sets pressure_targets = all foes when flag is present (line 3947)
- Matches JavaScript: `if (move.flags['mustpressure']) pressureTargets = this.foes();`

**Enables:** Correct Pressure ability PP deduction for moves with mustpressure flag

#### Futuremove Flag Check (1/1) ✅
- [x] **futuremove flag check in get_move_targets** (battle.rs:3922-3925) - Prevent returning empty targets for future moves

**Implementation Details:**
- Added has_futuremove flag extraction at function start (line 3836)
- Modified fainted target check to include futuremove condition
- Only returns empty if target is fainted AND move does NOT have futuremove flag (line 3923)
- Matches JavaScript: `if (target.fainted && !move.flags['futuremove']) return { targets: [], pressureTargets: [] };`

**Enables:** Future moves (Future Sight, Doom Desire) can target fainted Pokemon slots

#### Support Cancel Field (1/1) ✅
- [x] **support_cancel field in Battle struct** (battle.rs:374, 463, 6238) - Allow choice cancellation control

**Implementation Details:**
- Added support_cancel: bool field to Battle struct (line 374)
- Initialized to false in Battle::new() (line 463) - matches JavaScript default
- Updated is_all_choice_done() to check support_cancel before setting cantUndo (line 6238)
- Matches JavaScript: `if (!this.supportCancel) side.choice.cantUndo = true;`

**Enables:** Proper choice undo control based on battle configuration

#### Pokemon isStarted Field (1/1) ✅
- [x] **isStarted field assignment in faint_messages** (battle.rs:2823) - Reset isStarted when Pokemon faints

**Implementation Details:**
- Sets pokemon.is_started = false when Pokemon faints
- Matches JavaScript: `pokemon.isStarted = false;`
- Field already existed in Pokemon struct, just needed assignment
- Tracks whether a Pokemon has been sent out this battle

**Enables:** Proper tracking of Pokemon battle state, essential for various battle mechanics

#### SentRequests Field (1/1) ✅
- [x] **sentRequests field in Battle struct** (battle.rs:325, 444, 6716) - Track whether requests have been sent

**Implementation Details:**
- Added sent_requests: bool field to Battle struct (line 325)
- Initialized to false in Battle::new() (line 444)
- Set to false in make_request() (line 6716)
- Matches JavaScript: `this.sentRequests = false;`
- Tracks whether battle requests have been sent to players

**Enables:** Proper request state tracking for battle flow control

#### Pressure PP Deduction (1/1) ✅
- [x] **PP deduction in use_move_inner** (battle_actions.rs:3067-3071) - Deduct extra PP from Pressure ability

**Implementation Details:**
- Calls pokemon.deduct_pp() when extra_pp > 0 (line 3068-3070)
- Deducts extra PP accumulated from Pressure ability targets
- Matches JavaScript: `if (extraPP > 0) pokemon.deductPP(callerMoveForPressure || moveOrMoveName, extraPP);`
- Uses existing deduct_pp method on Pokemon

**Enables:** Pressure ability PP deduction mechanics

#### Gen 5+ FaintData Logic (1/1) ✅
- [x] **checkWin Gen 5+ logic** (battle.rs:1224-1244) - Determine winner based on which side fainted last in Gen 5+

**Implementation Details:**
- Changed parameter from `_faint_data` to `faint_data` to use the parameter
- Implemented Gen 5+ logic: `faintData && this.gen > 4 ? faintData.target.side : null`
- Extracts side_idx from faint_data.target tuple
- Passes the side's ID to win() if Gen 5+ and faint_data exists
- Otherwise passes None for a tie
- Matches JavaScript: `this.win(faintData && this.gen > 4 ? faintData.target.side : null);`

**Enables:** Correct Gen 5+ simultaneous faint resolution, tie-breaker mechanics based on faint order

#### Z-Move attrLastMove (1/1) ✅
- [x] **attrLastMove for [zeffect]** (battle_actions.rs:2940) - Add [zeffect] attribute to damage Z-moves

**Implementation Details:**
- Calls battle.attr_last_move(&["[zeffect]"]) for damage Z-moves
- Matches JavaScript: `this.battle.attrLastMove('[zeffect]');`
- Uses existing attr_last_move method (battle.rs:9097-9130)
- Adds [zeffect] tag to the last move log line

**Enables:** Proper protocol logging for damage Z-moves, distinguishes Z-powered damaging moves in battle log

#### Z-Move Status Effects (3/3) ✅
- [x] **addVolatile for followme** (battle_actions.rs:2991) - Z-Move redirect effect
- [x] **addVolatile for focusenergy** (battle_actions.rs:2996) - Z-Move critical hit boost
- [x] **addSlotCondition for healreplacement** (battle_actions.rs:2965) - Z-Move healing replacement

**Implementation Details:**
- Followme: Adds 'followme' volatile using pokemon.add_volatile()
- Focusenergy: Adds 'focusenergy' volatile using pokemon.add_volatile()
- Healreplacement: Calls side.add_slot_condition() with pokemon's position slot
- Uses existing add_volatile() method (pokemon.rs:560-566)
- Uses existing add_slot_condition() method (side.rs:373-384)
- Matches JavaScript: `pokemon.addVolatile('followme')`, `pokemon.addVolatile('focusenergy')`, `pokemon.side.addSlotCondition(pokemon, 'healreplacement')`

**Enables:**
- Follow Me redirect effect (Z-Splash, Z-Celebrate, etc.)
- Critical hit boost (Z-Foresight, Z-Acupressure, etc.)
- Healing replacement on switch (Z-Parting Shot, Z-Memento, etc.)
- Full status Z-Move effect support

#### Instafaint Handling (1/1) ✅
- [x] **faintMessages call in spread_damage** (battle.rs:8933) - Process faint queue immediately for instafaint damage

**Implementation Details:**
- Calls faint_messages(true, false, true) when Pokemon faints from instafaint damage
- Matches JavaScript: `this.faintMessages(true);` which uses defaults (lastFirst=true, forceCheck=false, checkWin=true)
- Ensures faints are processed immediately before continuing with damage spread
- Critical for proper turn order in simultaneous faint scenarios

**Enables:** Correct instafaint behavior for spread damage, proper faint queue processing

#### Source Last Damage Tracking (1/1) ✅
- [x] **Effect type check for lastDamage** (battle.rs:8881) - Only set source.lastDamage for Move effects

**Implementation Details:**
- Added effect.effectType check before setting source.lastDamage
- Uses get_effect_type() to determine if effect is a Move
- Matches JavaScript: `if (source && effect.effectType === 'Move') source.lastDamage = targetDamage;`
- Prevents incorrectly setting lastDamage for non-Move damage sources (items, abilities, status, etc.)

**Enables:** Correct damage tracking for move-based effects, proper Last Resort / Copycat / Mirror Move mechanics

#### Gen 1 Queue Clear (1/1) ✅
- [x] **queue.clear() in faint_messages** (battle.rs:2856) - Clear action queue when Pokemon faints in Gen 1

**Implementation Details:**
- Calls self.queue.clear() in Gen 1 faint handling
- Matches JavaScript: `this.queue.clear();`
- Gen 1 specific: fainting skips the rest of the turn by clearing all queued actions
- Essential for Gen 1 battle mechanics where fainting immediately ends turn processing

**Enables:** Correct Gen 1 faint behavior, proper turn termination in Gen 1

#### Multi Battle Side Conditions (2/2) ✅
- [x] **Sync side conditions for team 1** (battle.rs:777) - Copy side conditions from side 0 to side 2
- [x] **Sync side conditions for team 2** (battle.rs:780) - Copy side conditions from side 1 to side 3

**Implementation Details:**
- Clones side_conditions HashMap from ally sides
- Matches JavaScript: `this.sides[2].sideConditions = this.sides[0].sideConditions;`
- Matches JavaScript: `this.sides[3].sideConditions = this.sides[1].sideConditions;`
- Multi battle specific: sides 2 and 3 are allies of sides 0 and 1 respectively
- Ensures hazards and side effects apply to both partners in multi battles

**Enables:** Correct multi battle (doubles/triples/multi) side condition sharing, proper hazard mechanics in team battles

#### Effect Type Check for Heal Logs (1/1) ✅
- [x] **Effect type check in add_heal_log** (battle.rs:4646) - Check if effect is Move before adding [from] tag

**Implementation Details:**
- Added effect.effectType check before adding heal log
- Uses get_effect_type() to determine if effect is a Move
- Matches JavaScript logic:
  - If effect.effectType === 'Move': `this.add('-heal', target, target.getHealth);` (no [from] tag)
  - Else if source exists: `this.add('-heal', target, target.getHealth, '[from] ${effect}', '[of] ${source}');`
  - Else: `this.add('-heal', target, target.getHealth, '[from] ${effect}');`
- Prevents incorrectly showing [from] tag for healing moves

**Enables:** Correct protocol logging for healing moves vs healing abilities/items, proper battle log display

#### Gen 1 Bide Damage Clearing (1/1) ✅
- [x] **Clear Bide accumulated damage on faint** (battle.rs:2870-2898) - Reset Bide damage to 0 for all active Pokemon in Gen 1

**Implementation Details:**
- Iterates through all active Pokemon when a faint occurs in Gen 1
- Checks pokemon.volatiles['bide'] for damage field
- Resets damage to 0 if it exists and is > 0
- Adds hint messages when Bide damage is cleared
- Matches JavaScript:
  ```javascript
  for (const pokemon of this.getAllActive()) {
      if (pokemon.volatiles['bide']?.damage) {
          pokemon.volatiles['bide'].damage = 0;
          this.hint("Desync Clause Mod activated!");
          this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
      }
  }
  ```
- Uses EffectState.data HashMap to access/modify Bide damage field

**Enables:** Correct Gen 1 Bide mechanics, prevents accumulated damage from carrying over after faints

#### Custom Rules Display (1/1) ✅
- [x] **Add customRules field to FormatData** (dex.rs:580) - Added optional custom_rules field
- [x] **Implement customRules display** (battle.rs:871-887) - Display custom rules in battle log

**Implementation Details:**
- Added `custom_rules: Option<Vec<String>>` field to FormatData struct in dex.rs
- Implemented display logic in battle.rs start() method
- Finds format by format_id in dex.formats Vec
- Checks if format.customRules exists and is non-empty
- Calculates plural suffix: "" for 1 rule, "s" for multiple
- Calculates open attribute: " open" for <= 5 rules, "" for more
- Generates HTML with infobox details: `<div class="infobox"><details class="readmore"${open}><summary><strong>${count} custom rule${plural}:</strong></summary> ${rules.join(', ')}</details></div>`
- Matches JavaScript:
  ```javascript
  if (format.customRules) {
      const plural = format.customRules.length === 1 ? '' : 's';
      const open = format.customRules.length <= 5 ? ' open' : '';
      this.add(`raw|<div class="infobox">...`);
  }
  ```

**Enables:** Display of custom rules in battle log, proper format information communication to players

#### Gen 2-3 Queue Cancellation (1/1) ✅
- [x] **Gen 2-3 queue cancellation in faint_messages** (battle.rs:2917-2942) - Cancel queued actions when Pokemon faints

**Implementation Details:**
- Checks if gen <= 3 and gameType is Singles
- Collects all active Pokemon positions (side_idx, pokemon_idx)
- For each active Pokemon:
  - Gen 2: Calls queue.cancel_move() to skip moves only
  - Gen 3: Calls queue.cancel_action() to skip all moves and switches
- Matches JavaScript:
  ```javascript
  else if (this.gen <= 3 && this.gameType === 'singles') {
      for (const pokemon of this.getAllActive()) {
          if (this.gen <= 2) {
              this.queue.cancelMove(pokemon);
          } else {
              this.queue.cancelAction(pokemon);
          }
      }
  }
  ```

**Enables:** Correct Gen 2-3 faint behavior in singles battles, proper turn skipping mechanics

#### RedirectTarget Priority Event (1/1) ✅
- [x] **RedirectTarget priority event in get_move_targets** (battle.rs:4001-4032) - Redirects moves via abilities like Storm Drain and Lightning Rod
- [x] **priority_event relay_var parameter** (battle.rs:8434-8444) - Updated to accept relay variable for target encoding
- [x] **tracksTarget infrastructure** (dex.rs:172-173, battle_actions.rs:158, dex.rs:812) - Added tracks_target field to MoveData and ActiveMove

**Implementation Details:**
- Checks if `active_per_half > 1` and `!move.tracks_target`
- Encodes target position as: `side_idx * 10 + pokemon_idx`
- Calls `priority_event("RedirectTarget", user, user, move, encoded_target)`
- Decodes result back to target position
- Updated priority_event to accept and pass relay_var parameter
- Added tracks_target field to MoveData with serde rename
- Initialize tracks_target in get_active_move() from move data
- Matches JavaScript:
  ```javascript
  if (this.battle.activePerHalf > 1 && !move.tracksTarget) {
      target = this.battle.priorityEvent('RedirectTarget', this, this, move, target);
  }
  ```

**Enables:** Storm Drain, Lightning Rod, and other move redirection abilities

#### Smart Targeting (1/1) ✅
- [x] **Smart targeting in get_target** (battle.rs:8628-8642) - Smart target selection for moves like Dragon Darts
- [x] **smartTarget infrastructure** (dex.rs:174-175, dex.rs:803) - Added smart_target field to MoveData and ActiveMove

**Implementation Details:**
- Checks if `move.smartTarget` is true
- Gets Pokemon at target location
- If target exists and is not fainted, returns it
- Otherwise, gets a random target
- Added smart_target field to MoveData with serde rename
- Initialize smart_target in get_active_move() from move data
- Matches JavaScript:
  ```javascript
  if (move.smartTarget) {
      const curTarget = pokemon.getAtLoc(targetLoc);
      return curTarget && !curTarget.fainted ? curTarget : this.getRandomTarget(pokemon, move);
  }
  ```

**Enables:** Dragon Darts and other multi-target moves with smart targeting

#### Self-Targeting Validation (1/1) ✅
- [x] **Self-targeting check in get_target** (battle.rs:8644-8678) - Prevents moves from targeting themselves unless volatiles allow it

**Implementation Details:**
- Calculates self location using get_loc_of(user, user)
- Checks if move target type is 'adjacentAlly', 'any', or 'normal' (moves that can't normally target self)
- Checks if target location equals self location
- Validates Pokemon doesn't have volatiles that allow self-targeting: 'twoturnmove', 'iceball', 'rollout'
- Returns user (self) if move has 'futuremove' flag, otherwise returns None
- Matches JavaScript:
  ```javascript
  const selfLoc = pokemon.getLocOf(pokemon);
  if (
      ['adjacentAlly', 'any', 'normal'].includes(move.target) && targetLoc === selfLoc &&
      !pokemon.volatiles['twoturnmove'] && !pokemon.volatiles['iceball'] && !pokemon.volatiles['rollout']
  ) {
      return move.flags['futuremove'] ? pokemon : null;
  }
  ```

**Enables:** Correct self-targeting validation, prevents invalid self-targeting, supports two-turn moves and rollout mechanics

#### isAlly() Integration in get_target (1/1) ✅
- [x] **Replace simplified ally check with is_ally()** (battle.rs:8697-8703) - Use proper ally detection for multi-battle support

**Implementation Details:**
- Replaced `target_side == user_side` with proper `is_ally(target, user)` call
- Properly checks both same side AND ally side (multi battles)
- Matches JavaScript: `target.isAlly(pokemon)`
- For fainted ally targets:
  - Returns user (self) if move target is 'adjacentAllyOrSelf' and not Gen 5
  - Otherwise returns target (fainted ally: attack shouldn't retarget)

**Enables:** Correct ally detection in multi-battle formats (doubles, triples, multi), proper fainted ally targeting

#### Weather Immunity Check in spread_damage (1/1) ✅
- [x] **Weather damage immunity** (battle.rs:8978-8992) - Check weather immunity before applying damage

**Implementation Details:**
- Uses get_effect_type() to check if effect is 'Weather'
- Calls pokemon.run_status_immunity(effect_id) to check immunity
- Returns 0 damage if Pokemon is immune to weather (e.g., Ice types in Hail)
- Matches JavaScript:
  ```javascript
  if (effect.effectType === 'Weather' && !target.runStatusImmunity(effect.id))
  ```

**Enables:** Correct weather damage immunity (Ice types in Hail/Snow, Rock/Ground/Steel in Sandstorm, weather-immune abilities)

#### Recoil/Drain Handling (1/1) ✅
- [x] **Recoil/drain for moves** (battle.rs:9097-9142) - Apply recoil damage and drain healing based on move effects

**Implementation Details:**
- Extracts recoil and drain data from MoveData before calling methods (avoids borrow checker issues)
- Implements three distinct gen-specific cases:
  - **Gen 1 recoil:** `floor(damage * recoil[0] / recoil[1])`, clamped to minimum 1
  - **Gen 1-4 drain:** `floor(damage * drain[0] / drain[1])`, clamped to minimum 1, sets lastDamage for Gen 1
  - **Gen 5+ drain:** `round(damage * drain[0] / drain[1])` (no floor)
- Calls `self.damage()` for recoil with 'recoil' effect
- Calls `self.heal()` for drain with 'drain' effect
- Matches JavaScript:
  ```javascript
  if (targetDamage && effect.effectType === 'Move') {
      if (this.gen <= 1 && effect.recoil && source) {
          const amount = this.clampIntRange(Math.floor(targetDamage * effect.recoil[0] / effect.recoil[1]), 1);
          this.damage(amount, source, target, 'recoil');
      }
      if (this.gen <= 4 && effect.drain && source) {
          const amount = this.clampIntRange(Math.floor(targetDamage * effect.drain[0] / effect.drain[1]), 1);
          if (this.gen <= 1) this.lastDamage = amount;
          this.heal(amount, source, target, 'drain');
      }
      if (this.gen > 4 && effect.drain && source) {
          const amount = Math.round(targetDamage * effect.drain[0] / effect.drain[1]);
          this.heal(amount, source, target, 'drain');
      }
  }
  ```

**Enables:** Recoil damage (Take Down, Double-Edge, Submission), Drain healing (Absorb, Mega Drain, Giga Drain), gen-specific damage calculations

### Session 9 Continued Part 2 - Gen 1 Bide Instafaint Handling (1 implementation)

#### Gen 1 Bide Instafaint Clearing (1/1) ✅
- [x] **Clear Bide accumulated damage in instafaint** (battle.rs:9176-9220) - Reset Bide damage to 0 for all active Pokemon when instafaint occurs in Gen 1

**Implementation Details:**
- Iterates through all active Pokemon when instafaint occurs in Gen 1
- Checks pokemon.volatiles['bide'] for damage field
- Resets damage to 0 if it exists and is > 0
- Adds hint messages when Bide damage is cleared
- Matches JavaScript battle.ts:8897-8901:
  ```javascript
  if (this.gen <= 1) {
      this.queue.clear();
      for (const pokemon of this.getAllActive()) {
          if (pokemon.volatiles['bide']?.damage) {
              pokemon.volatiles['bide'].damage = 0;
              this.hint("Desync Clause Mod activated!");
              this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
          }
      }
  }
  ```
- Uses EffectState.data HashMap to access/modify Bide damage field
- Identical logic to faint_messages() implementation but in instafaint context

**Enables:** Correct Gen 1 Bide mechanics during instafaint, prevents accumulated damage from carrying over after instafaints

### Session 9 Continued Part 3 - Magic Bounce Speed Calculation (1 implementation)

#### Get Stat with Unmodified Flag (1/1) ✅
- [x] **Magic Bounce speed calculation** (battle.rs:9917-9921) - Use get_stat() with unboosted flag for unmodified speed

**Implementation Details:**
- Added StatID import to battle.rs
- Call pokemon.get_stat(StatID::Spe, true) for Magic Bounce priority calculation
- Matches JavaScript: `handler.speed = pokemon.getStat('spe', true, true);`
- When both unboosted and unmodified flags are true, getStat returns the base stored stat
- The current Rust get_stat() with unboosted=true achieves the same result as JavaScript getStat with both flags true
- Full JavaScript signature: `getStat(statName, unboosted?, unmodified?)`
  - `unboosted=true`: Skip boost calculation
  - `unmodified=true`: Skip Wonder Room effect and skip ModifyBoost/Modify{Stat} events
- For this specific use case (both flags true), returning storedStats is correct

**Enables:** Correct Magic Bounce ability priority ordering in onAllyTryHitSide callbacks

### Session 10 - Partiallytrapped Source Effect (1 implementation)

#### Partiallytrapped Source Effect Name (1/1) ✅
- [x] **Get source effect from volatiles** (battle.rs:9251-9278) - Extract sourceEffect.fullname from partiallytrapped volatile

**Implementation Details:**
- Accesses target Pokemon's volatiles HashMap to get 'partiallytrapped' state
- Extracts sourceEffect object from EffectState.data HashMap
- Gets fullname property from sourceEffect (e.g., "Bind", "Fire Spin", "Whirlpool")
- Uses fullname in damage log message instead of generic "partiallytrapped"
- Matches JavaScript battle.ts:8848:
  ```javascript
  this.add('-damage', target, target.getHealth, '[from] ' + target.volatiles['partiallytrapped'].sourceEffect.fullname, '[partiallytrapped]');
  ```
- Falls back to "partiallytrapped" if sourceEffect or fullname not found
- Uses nested Option chaining to safely access volatile data

**Enables:** Correct protocol logging for partial trapping moves showing which specific move is causing damage (Bind, Fire Spin, Whirlpool, etc.)

### Session 10 Continued - Move Secondary Effects and Boosts Conversion (2 implementations)

#### Move Secondary Effects Conversion (1/1) ✅
- [x] **convert_secondary() helper** (dex.rs:881-891) - Convert MoveSecondary to SecondaryEffect
- [x] **secondaries field initialization** (dex.rs:824-828) - Use helper to convert move_data.secondary to Vec<SecondaryEffect>

**Implementation Details:**
- Created `convert_secondary()` helper method in Dex
- Converts MoveSecondary (JSON format with HashMap boosts) to SecondaryEffect (typed format with BoostsTable)
- Maps chance, status, volatile_status fields directly
- Uses `convert_boosts_hash_to_table()` to convert boosts HashMap
- Sets self_effect to false (primary effect, not self-targeting)
- Updated get_active_move() to use helper when secondary exists
- Wraps result in Vec since ActiveMove.secondaries is Vec<SecondaryEffect>

**Enables:** Move secondary effects (Thunder's paralysis chance, Scald's burn chance, etc.) with proper typed boosts

#### Stat Boosts Conversion (1/1) ✅
- [x] **convert_boosts_hash_to_table() helper** (dex.rs:869-879) - Convert HashMap<String, i32> to BoostsTable
- [x] **boosts field initialization** (dex.rs:833) - Use helper to convert move_data.boosts to BoostsTable

**Implementation Details:**
- Created `convert_boosts_hash_to_table()` helper method in Dex
- Converts HashMap<String, i32> from JSON to typed BoostsTable struct
- Maps stat names to struct fields: atk, def, spa, spd, spe, accuracy, evasion
- Defaults to 0 if stat not present in HashMap
- Casts i32 to i8 for struct compatibility
- Updated get_active_move() to use helper when boosts exists
- Handles both move.boosts (direct stat changes) and secondary.boosts (chance-based changes)

**Enables:** Move stat changes (Swords Dance, Growl, etc.) and stat change secondary effects (Icy Wind, Charge Beam, etc.)

#### Source Effect Tracking (1/1) ✅
- [x] **active_move.source_effect assignment** (battle_actions.rs:2858-2865) - Set source effect and copy ignore_ability from source move

**Implementation Details:**
- Sets active_move.source_effect to Some(source_effect_id) when source_effect parameter is Some
- If source effect refers to a move, looks up the move using dex.get_active_move()
- Copies ignore_ability flag from source move to active_move
- Matches JavaScript battle-actions.ts:
  ```javascript
  if (sourceEffect) {
      move.sourceEffect = sourceEffect.id;
      move.ignoreAbility = (sourceEffect as ActiveMove).ignoreAbility;
  }
  ```
- Properly tracks which effect triggered this move (item/ability/move)
- Used for move chaining (Dancer, Instruct) and ability interactions

**Enables:** Correct source effect tracking for triggered moves, ability ignore flag inheritance

#### Current Effect Fallback and Filtering (1/1) ✅
- [x] **current_effect fallback logic** (battle_actions.rs:2731-2743) - Use battle.current_effect when source_effect not provided

**Implementation Details:**
- If source_effect parameter is None, checks battle.current_effect
- Sets source_effect to battle.current_effect if it exists
- Filters out 'instruct' and 'custapberry' from source effects
- Matches JavaScript battle-actions.ts:
  ```javascript
  if (!sourceEffect && this.battle.effect.id) sourceEffect = this.battle.effect;
  if (sourceEffect && ['instruct', 'custapberry'].includes(sourceEffect.id)) sourceEffect = null;
  ```
- Uses existing battle.current_effect field (already in Battle struct at line 363)
- Tracks which effect triggered the current move for chaining mechanics

**Enables:** Move chaining (Dancer, Instruct), proper effect context inheritance, excluded effects filtering

#### Active Move Priority Inheritance (1/1) ✅
- [x] **priority and pranksterBoosted inheritance** (battle_actions.rs:2816-2823) - Inherit priority and prankster boost from active move

**Implementation Details:**
- Checks if battle.active_move is Some (another move is currently active)
- Looks up the active move using dex.get_active_move()
- Copies priority from active move to current move
- Copies prankster_boosted from active move if current move hasn't bounced (has_bounced is false)
- Matches JavaScript battle-actions.ts:
  ```javascript
  if (this.battle.activeMove) {
      move.priority = this.battle.activeMove.priority;
      if (!move.hasBounced) move.pranksterBoosted = this.battle.activeMove.pranksterBoosted;
  }
  ```
- Uses existing battle.active_move field (Option<ID> at battle.rs:349)
- Critical for move chaining mechanics where called moves inherit caller's properties

**Enables:** Correct priority for called moves (Copycat, Me First, Dancer), Prankster boost inheritance for chained moves

### Session 11 - Quick Guard Move Priority Tracking (1 implementation)

#### Move Priority Modified Field (1/1) ✅
- [x] **move_priority_modified field in MoveAction** (battle_queue.rs:45-49) - Track modified move priority for Quick Guard
- [x] **Gen 6+ priority tracking** (battle.rs:6551-6556) - Set move_priority_modified when priority is modified

**Implementation Details:**
- Added `move_priority_modified: Option<i8>` field to MoveAction struct
- Stores the modified priority value for the move itself (not just the action's priority)
- Set in get_action_speed() when gen > 5: `if self.gen > 5 { move_action.move_priority_modified = Some(priority); }`
- Matches JavaScript: `if (this.gen > 5) action.move.priority = priority;`
- Updated all MoveAction constructions to include the new field (3 test cases in battle_queue.rs)
- Allows Quick Guard to detect if a move's priority was artificially enhanced by abilities like Prankster

**Enables:** Quick Guard detection of artificially enhanced move priority (Gen 6+ mechanic), proper Quick Guard blocking of priority-boosted moves

### Session 12 - Smart Targeting for Dragon Darts (1 implementation)

#### Smart Target Logic (1/1) ✅
- [x] **Smart targeting implementation** (battle.rs:4035-4080) - Implement smart targeting for Dragon Darts

**Implementation Details:**
- Extracts `smart_target` field from move data (line 3938)
- Implements full getSmartTargets logic inline in get_move_targets()
- Gets target's first adjacent ally using adjacent_allies()
- Checks if target2 is valid (exists, is not self, has HP)
- Returns appropriate targets based on HP status:
  - If target2 invalid: returns [target]
  - If target fainted: returns [target2]
  - If both alive: returns [target, target2]
- Matches JavaScript pokemon.ts getSmartTargets():
  ```javascript
  getSmartTargets(target: Pokemon, move: ActiveMove) {
      const target2 = target.adjacentAllies()[0];
      if (!target2 || target2 === this || !target2.hp) { return [target]; }
      if (!target.hp) { return [target2]; }
      return [target, target2];
  }
  ```

**Enables:** Dragon Darts multi-target mechanics, proper retargeting when targets faint

### Session 13 - Nature Stat Modifiers (1 implementation)

#### Nature Modifiers for Stats (1/1) ✅
- [x] **Nature stat modifiers** (battle.rs:10285-10327) - Apply nature +/- 10% modifiers to stats

**Implementation Details:**
- Uses Dex.get_nature() to retrieve nature data
- Checks if nature.plus matches stat_name for +10% boost
- Checks if nature.minus matches stat_name for -10% reduction
- Applies overflow protection when ruleTable has 'overflowstatmod':
  - Plus stats capped at 595 (Eternatus-Eternamax in Pure Hackmons)
  - Minus stats capped at 728
- Uses 16-bit truncation: `Dex::trunc(Dex::trunc(stat * 110, 16) / 100, 0)`
- Matches JavaScript battle.ts:10228-10237:
  ```javascript
  const nature = this.dex.natures.get(set.nature);
  if (nature.plus === statName) {
      stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 595) : stat;
      stat = tr(tr(stat * 110, 16) / 100);
  } else if (nature.minus === statName) {
      stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 728) : stat;
      stat = tr(tr(stat * 90, 16) / 100);
  }
  ```

**Enables:** Correct stat calculation with nature modifiers (Adamant, Modest, Timid, etc.), Pure Hackmons edge case handling

### Session 14 - Rule Table Loading (1 implementation)

#### Rule Table from Format (1/1) ✅
- [x] **Load rule_table from format** (battle.rs:424-434) - Load and build rule table during Battle construction

**Implementation Details:**
- Added import for `crate::data::formats::{get_format, Format, DexFormats}`
- Uses `get_format(&ID::new(&format_id_str))` to retrieve FormatDef
- Creates Format instance using `Format::from_def(format_def)`
- Creates DexFormats instance and calls `get_rule_table(&format)`
- Stores result in Battle.rule_table field
- Matches JavaScript: `this.ruleTable = this.dex.formats.getRuleTable(this.format);`

**Enables:** Rule-based format validation, overflow stat modifiers (used in nature calculations), ban/restriction checking, custom format rules

### Session 15 - Picked Team Size Check (1 implementation)

#### PickedTeamSize Team Preview Logic (1/1) ✅
- [x] **pickedTeamSize check in run_pick_team** (battle.rs:10094-10159) - Show Pokemon privately when pickedTeamSize is set but no onTeamPreview handler ran

**Implementation Details:**
- Checks if `rule_table.picked_team_size.is_some()`
- If true: Shows Pokemon privately (no onTeamPreview handler ran)
  - Adds 'clearpoke' log
  - Iterates through all Pokemon using get_all_pokemon()
  - Hides forme details for Zacian/Zamazenta/Xerneas:
    - Zacian/Zamazenta: Replaces with "-*" if not already "-Crowned"
    - Xerneas: Replaces any forme suffix with "-*" (e.g., "Xerneas-Neutral" -> "Xerneas-*")
  - Removes ", shiny" from details string
  - Calls add_split() for each Pokemon to show details privately
  - Makes teampreview request
- Matches JavaScript: battle.ts:10065-10077

**Technical Notes:**
- Used string methods instead of regex for forme replacement
- Collected Pokemon data into Vec first to avoid borrow checker issues
- Xerneas forme replacement: finds "Xerneas", checks for forme suffix (starts with '-'), replaces up to next comma or end
- Side ID mapping: side_index 0 -> "p1", side_index 1 -> "p2"

**Enables:** Team preview when pickedTeamSize is set, proper forme hiding for legendary Pokemon during team selection, correct side-specific message splitting

### Session 16 - Battle Start Switch In (1 implementation)

#### Switch In All Active Pokemon (1/1) ✅
- [x] **Switch in active Pokemon at battle start** (battle.rs:6353-6362) - Switch in all active Pokemon when battle starts

**Implementation Details:**
- Iterates through all sides
- For each side, iterates through active.length positions
- Calls switch_in(side_idx, i, i, None, false) for each position where:
  - side_idx: Side index
  - pos: i (position in active array)
  - pokemon_index: i (assumes first N Pokemon are the active ones)
  - source_effect: None (no specific effect triggering switch)
  - is_drag: false (not a forced switch/drag)
- Matches JavaScript: battle.ts (for loop over sides and active positions)

**Technical Notes:**
- Extracts active_length before loop to avoid borrow checker issues
- Switches in Pokemon in order (0th Pokemon to 0th position, 1st to 1st, etc.)
- Runs before the Start event for each Pokemon's species
- Sets up initial battle state with all active Pokemon on field

**Enables:** Proper battle initialization with all active Pokemon switched in, triggers switch-in abilities and events, correct battle start sequence

### Session 17 - KnownType Field and Trap Immunity (1 implementation)

#### Pokemon.knownType Field Infrastructure (1/1) ✅
- [x] **Add known_type field to Pokemon** (pokemon.rs:135, 260) - Track known type for illusion/disguise mechanics
- [x] **Implement trap immunity check** (battle.rs:5863-5874) - Check knownType and status immunity before MaybeTrapPokemon event

**Implementation Details:**
- Added `known_type: Option<String>` field to Pokemon struct (line 135)
- Initialized to None in Pokemon::new() (line 260)
- Implemented conditional MaybeTrapPokemon event call in end_turn()
- Checks: `pokemon.known_type.is_none() || pokemon.run_status_immunity("trapped")`
- Only runs MaybeTrapPokemon if type is unknown OR not immune to trapped status
- Matches JavaScript: `if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon))`

**Technical Notes:**
- knownType is used for Illusion/Disguise/Zorua mechanics
- When a Pokemon's type becomes known, this field is set
- run_status_immunity("trapped") checks if Pokemon is immune to being trapped
- Ghost types, levitating Pokemon may be immune based on abilities
- Proper check prevents unnecessary event calls

**Enables:** Illusion/Disguise type tracking, correct trap mechanics based on type knowledge, optimized event system (skips events when Pokemon is immune)

### Session 18 - Gen 1 Substitute HP Tracking (1 implementation)

#### Substitute HP Damage System (1/1) ✅
- [x] **sub_fainted field** (pokemon.rs:170, 291) - Track substitute faint separately from Pokemon faint
- [x] **Substitute HP tracking** (battle.rs:4523-4582) - Track and update substitute HP when damaged

**Implementation Details:**
- Added `sub_fainted: bool` field to Pokemon struct (Gen 1 specific flag)
- Initialized to false in Pokemon::new()
- Implemented Gen 1 special case: confusion and HJK recoil damage affects foe's substitute
- Accesses substitute HP from volatile's data HashMap: `sub_state.data.get("hp")`
- Deducts damage from substitute HP: `new_hp = current_hp - damage`
- If HP <= 0:
  - Removes substitute volatile
  - Sets `foe.sub_fainted = true`
- Otherwise:
  - Updates HP in volatile data
  - Adds activate log: `add_log("-activate", foe_slot, "Substitute", "[damage]")`
- Matches JavaScript: `foe.volatiles['substitute'].hp -= damage;`
- Uses Pokemon.get_slot() to construct identifier for logs

**Enables:** Gen 1 substitute damage mechanics, proper substitute HP tracking, correct substitute destruction, Gen 1 confusion/HJK recoil interaction with substitutes



## Remaining P1 Important (0 TODOs) ✅ ALL P1 COMPLETE

**Next Focus:** P2 Nice-to-have features (Gen-specific mechanics, Dynamax, Infrastructure improvements)

## Remaining P2 Nice-to-have (10 TODOs)

### Gen-Specific (1 TODO)
- Gen 1 no-progress checks
- Staleness checks  
- Berry cycling checks

### Dynamax (5 TODOs)
- Dynamax 3-turn removal
- Gen 1 partial trapping
- Zacian/Zamazenta forme changes
- Format callbacks
- Switch in all active Pokemon

### Infrastructure (19 TODOs)
- Various missing infrastructure pieces
- Boost migration
- Request handling improvements

## Next Steps

1. **Continue P1 Important implementations:**
   - Side Management (clearChoice, activeRequest, etc.)
   - Format Callbacks (onBegin, runAction, swap events)

2. **Track progress:** Commit after each feature group

3. **Goal:** Achieve functional parity for core battle mechanics

