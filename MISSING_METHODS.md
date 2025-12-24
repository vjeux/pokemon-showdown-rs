# Missing Methods - Pokemon Showdown Rust Port

This file tracks all methods from the TypeScript `sim/` files that are not yet implemented in the Rust port.

## Summary

| File | TS Methods | Missing in RS |
|------|------------|---------------|
| battle.ts | ~100+ | 25 |
| battle-actions.ts | 42 | 40 |
| pokemon.ts | ~80+ | 53 |
| side.ts | ~40+ | 33 |
| field.ts | 17 | 7 |
| battle-queue.ts | 20 | 8 |
| prng.ts | 22 | 3 |
| battle-stream.ts | ~20 | 12 |
| state.ts | 22 | 21 |
| teams.ts | ~10 | 8 |
| team-validator.ts | 27 | 25 |
| dex.ts | 26 | 24 |
| dex-data.ts | 20 | 6 |

---

## battle.ts → battle.rs

### Missing Methods:
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

### Missing Methods:
- [ ] switchIn (switch pokemon in)
- [ ] dragIn (force drag pokemon in)
- [ ] runSwitch (execute switch queue)
- [ ] runMove (execute move action)
- [ ] useMove (use a move)
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
- [ ] calcRecoilDamage (calculate recoil damage)
- [ ] getZMove (get Z-move variant)
- [ ] getActiveZMove (get active Z-move)
- [ ] canZMove (check if Z-move is possible)
- [ ] getMaxMove (get Max move variant)
- [ ] getActiveMaxMove (get active Max move)
- [ ] runZPower (execute Z-power effect)
- [ ] targetTypeChoices (determine valid targets)
- [ ] getDamage (calculate damage)
- [ ] modifyDamage (apply damage modifiers)
- [ ] getConfusionDamage (calculate confusion damage)
- [ ] canMegaEvo (check if Mega Evolution is possible)
- [ ] canUltraBurst (check if Ultra Burst is possible)
- [ ] runMegaEvo (execute Mega Evolution)
- [ ] canTerastallize (check if Terastallization is possible)
- [ ] terastallize (execute Terastallization)

---

## pokemon.ts → pokemon.rs

### Missing Methods:
- [ ] boostBy (Modify stat boosts with a specific amount)
- [ ] calculateStat (Calculate stat value with multipliers and modifications)
- [ ] clearAbility (Clear the Pokemon's ability)
- [ ] clearItem (Clear the Pokemon's held item)
- [ ] clearStatus (Clear status condition)
- [ ] clearVolatile (Clear a specific volatile condition)
- [ ] damage (Apply damage to Pokemon)
- [ ] deductPP (Deduct PP from move)
- [ ] destroy (Clean up and destroy Pokemon instance)
- [ ] eatItem (Handle Pokemon eating a held item)
- [ ] faint (Mark Pokemon as fainted)
- [ ] getAbility (Get the Pokemon's current ability)
- [ ] getAtLoc (Get Pokemon at a specific field location)
- [ ] getCappedBoost (Get a stat boost capped to legal limits)
- [ ] getCombatPower (Calculate combat power value)
- [ ] getDynamaxRequest (Get Dynamax request data for protocol)
- [ ] getItem (Get the Pokemon's current held item)
- [ ] getLastAttackedBy (Get last Pokemon that attacked this one)
- [ ] getLastDamagedBy (Get last Pokemon that damaged this one)
- [ ] getLocOf (Get field location of a Pokemon)
- [ ] getMoveData (Get data for a specific move slot)
- [ ] getMoveHitData (Get hit data for a move)
- [ ] getMoveRequestData (Get move request data for protocol)
- [ ] getNature (Get the Pokemon's nature)
- [ ] getSmartTargets (Get smart targeting for moves)
- [ ] getStatus (Get the Pokemon's status condition)
- [ ] getUndynamaxedHP (Get HP value as if Dynamaxed)
- [ ] getUpdatedDetails (Get updated protocol details string)
- [ ] gotAttacked (Record that Pokemon was attacked)
- [ ] hasType (Check if Pokemon has a specific type)
- [ ] ignoringAbility (Check if ability effects are ignored)
- [ ] ignoringItem (Check if item effects are ignored)
- [ ] isAdjacent (Check if two Pokemon are adjacent on field)
- [ ] isAlly (Check if another Pokemon is an ally)
- [ ] isLastActive (Check if Pokemon was last one active)
- [ ] isSkyDropped (Check if Pokemon is in Sky Drop)
- [ ] maxMoveDisabled (Check if Max Move is disabled)
- [ ] moveUsed (Record that a move was used)
- [ ] removeLinkedVolatiles (Remove linked volatile conditions)
- [ ] runEffectiveness (Run move type effectiveness check)
- [ ] runImmunity (Run type/effect immunity check)
- [ ] runStatusImmunity (Run status effect immunity check)
- [ ] setBoost (Set a stat boost to specific value)
- [ ] setItem (Set the Pokemon's held item)
- [ ] setSpecies (Change the Pokemon's species)
- [ ] sethp (Set HP directly)
- [ ] takeItem (Remove and return held item)
- [ ] toString (Get string representation of Pokemon)
- [ ] transformInto (Transform into another Pokemon)
- [ ] trySetStatus (Attempt to set status with immunity checks)
- [ ] tryTrap (Attempt to trap Pokemon with checks)
- [ ] updateMaxHp (Update max HP value)
- [ ] useItem (Use held item)

---

## side.ts → side.rs

### Missing Methods:
- [ ] canDynamaxNow (check if side can dynamax this turn in Gen 8)
- [ ] getChoice (convert a Choice into a choice string for the protocol)
- [ ] toString (return string representation of side)
- [ ] getRequestData (get side request data for protocol communication)
- [ ] randomFoe (get a random foe Pokemon)
- [ ] foeSidesWithConditions (iterate through all foe side conditions)
- [ ] foePokemonLeft (get total Pokemon left on foe side)
- [ ] allies (get allied Pokemon)
- [ ] foes (get foe Pokemon)
- [ ] activeTeam (get active team, handling multi battles)
- [ ] hasAlly (check if a Pokemon is an ally)
- [ ] addPokemon (add a Pokemon to the team)
- [ ] getSideCondition (get side condition by status)
- [ ] getSideConditionData (get side condition data)
- [ ] addSlotCondition (add condition to a slot)
- [ ] getSlotCondition (get slot condition by status)
- [ ] removeSlotCondition (remove slot condition)
- [ ] send (send message to clients)
- [ ] emitRequest (emit request to client)
- [ ] emitChoiceError (emit choice error to client)
- [ ] isChoiceDone (check if choice is complete)
- [ ] chooseMove (handle move choice action)
- [ ] updateDisabledRequest (update request with disabled moves)
- [ ] updateRequestForPokemon (update request for specific Pokemon)
- [ ] chooseSwitch (handle switch choice action)
- [ ] pickedTeamSize (get required team preview size)
- [ ] chooseTeam (handle team preview choice)
- [ ] chooseShift (handle shift choice in triples)
- [ ] clearChoice (reset choice data)
- [ ] choose (main choice parsing method)
- [ ] getChoiceIndex (get current choice action index)
- [ ] choosePass (handle pass action)
- [ ] autoChoose (auto-complete choices)
- [ ] destroy (cleanup/deallocate side)

---

## field.ts → field.rs

### Missing Methods:
- [ ] suppressingWeather (checks if any active Pokemon has an ability that suppresses weather)
- [ ] getWeather (retrieves the Condition/Effect object for the current weather)
- [ ] effectiveTerrain (returns terrain after checking TryTerrain event)
- [ ] getTerrain (retrieves the Condition/Effect object for the current terrain)
- [ ] getPseudoWeather (returns the condition object if pseudo-weather is active)
- [ ] toJSON (serializes the field to JSON)
- [ ] destroy (cleanup method for deallocating references)

---

## battle-queue.ts → battle_queue.rs

### Missing Methods:
- [ ] resolveAction (resolves action choices into full action objects)
- [ ] changeAction (changes a pokemon's action and reinserts it in priority order)
- [ ] addChoice (adds one or more action choices to the queue)
- [ ] insertChoice (inserts action into queue maintaining sort order)
- [ ] debug (debug output for queue state)
- [ ] entries (iterator over queue entries)

---

## prng.ts → prng.rs

### Missing Methods:
- [ ] get (static factory method - converts PRNG | PRNGSeed | null to PRNG)
- [ ] convertSeed (static - joins seed array/tuple to string format)
- [ ] setSeed (instance method - overwrite current seed)

---

## battle-stream.ts → battle_stream.rs

### Missing Methods:
- [ ] _writeLines (parsing incoming lines from stream format)
- [ ] pushMessage (pushing messages with replay mode handling)
- [ ] _writeLine (dispatching individual command types)
- [ ] _writeEnd (cleanup on stream end)
- [ ] _destroy (battle destruction)
- [ ] pushError (error handling mechanism)
- [ ] pushEnd (end of stream signaling)
- [ ] atEOF (property checking if at end of file)
- [ ] splitFirst (utility function for string splitting with limit)
- [ ] getPlayerStreams (function to split stream into omniscient/spectator/p1-p4 streams)
- [ ] BattlePlayer class (abstract class with start, receive, receiveLine, receiveRequest, receiveError, choose methods)
- [ ] BattleTextStream class (class with _listen, _write, _writeEnd methods)

---

## state.ts → state.rs

### Missing Methods:
- [ ] serializeBattle (serialize a Battle instance)
- [ ] deserializeBattle (deserialize a Battle instance)
- [ ] normalize (normalize state by processing log)
- [ ] normalizeLog (normalize log by removing timestamps)
- [ ] serializeField (serialize Field)
- [ ] deserializeField (deserialize Field)
- [ ] serializeSide (serialize Side)
- [ ] deserializeSide (deserialize Side)
- [ ] serializePokemon (serialize Pokemon)
- [ ] deserializePokemon (deserialize Pokemon)
- [ ] serializeChoice (serialize Choice)
- [ ] deserializeChoice (deserialize Choice)
- [ ] isActiveMove (check if object is ActiveMove)
- [ ] serializeActiveMove (serialize ActiveMove)
- [ ] deserializeActiveMove (deserialize ActiveMove)
- [ ] serializeWithRefs (serialize with reference handling)
- [ ] deserializeWithRefs (deserialize with reference handling)
- [ ] isReferable (check if object is Referable)
- [ ] toRef (convert object to reference string)
- [ ] fromRef (convert reference string back to object)
- [ ] serialize (generic serialize helper)
- [ ] deserialize (generic deserialize helper)

---

## teams.ts → teams.rs

### Missing Methods:
- [ ] packName (removes non-alphanumeric characters from names)
- [ ] unpackName (converts packed names back to readable format using dex table lookup)
- [ ] export (exports a full team to human-readable format with export options)
- [ ] exportSet (exports individual pokemon set with options to hide stats/remove nicknames)
- [ ] parseExportedTeamLine (parses individual lines of exported team format)
- [ ] import (accepts team in any format: JSON, packed, or exported text)
- [ ] getGenerator (gets the appropriate team generator based on format)
- [ ] generate (generates a random team for a given format)

---

## team-validator.ts → team_validator.rs

### Missing Methods:
- [ ] getEventOnlyData (retrieves event-only species data)
- [ ] getValidationSpecies (gets the out-of-battle and tier species)
- [ ] validateSet (validates individual Pokemon set with all its properties)
- [ ] validateStats (validates EV/IV spreads with detailed stat checks)
- [ ] possibleBottleCapHpType (checks if HP type is achievable with bottle caps)
- [ ] validateSource (validates if a Pokemon can be obtained from a specific source)
- [ ] validateEvent (validates if a Pokemon matches event data)
- [ ] validateForme (validates form-specific requirements and restrictions)
- [ ] checkSpecies (checks species bans, megas, gigantamax, etc.)
- [ ] checkItem (validates item bans and compatibility)
- [ ] checkMove (validates move bans and compatibility)
- [ ] checkAbility (validates ability bans and compatibility)
- [ ] checkNature (validates nature bans)
- [ ] findEggMoveFathers (finds valid parents for egg moves)
- [ ] fatherCanLearn (checks if a father can learn specific moves)
- [ ] motherCanLearn (checks if a mother can learn a specific move)
- [ ] allSources (generates all possible sources for a species)
- [ ] validateMoves (validates all moves in a set)
- [ ] validatePokemonGo (validates Pokemon GO specific constraints)
- [ ] omCheckCanLearn (OM-specific move legality check)
- [ ] checkCanLearn (main move legality checking logic)
- [ ] getExternalLearnsetData (retrieves learnset data from modded dex)
- [ ] fillStats (static: fills stat tables with default values)
- [ ] get (static: factory method for creating validators)

---

## dex.ts → dex.rs

### Missing Methods:
- [ ] data (getter)
- [ ] dexes (getter)
- [ ] mod (select mod/variant)
- [ ] forGen (get dex for specific generation)
- [ ] forFormat (get dex for specific format)
- [ ] modData (get modded data entry)
- [ ] effectToString (convert to string representation)
- [ ] getName (sanitize username/Pokemon nickname)
- [ ] getImmunity (check type immunity)
- [ ] getDescs (get descriptions from table)
- [ ] getActiveMove (get active move copy)
- [ ] getHiddenPower (calculate Hidden Power type/power)
- [ ] trunc (truncate to 32-bit unsigned integer)
- [ ] dataSearch (search dex for pokemon/moves/items)
- [ ] loadDataFile (load data file from path)
- [ ] loadTextFile (load text file from path)
- [ ] includeMods (load mod list)
- [ ] includeModData (load all mod data)
- [ ] includeData (load data)
- [ ] loadTextData (load text descriptions)
- [ ] getAlias (get alias for id)
- [ ] loadAliases (load and build aliases map)
- [ ] loadData (main data loading)
- [ ] includeFormats (load formats)

---

## dex-data.ts → dex_data.rs

### Missing Methods:
- [ ] assignMissingFields (utility function for assigning missing object fields)
- [ ] DexNatures class (get, getByID, all methods)
- [ ] DexTypes class (get, getByID, names, isName, all methods)
- [ ] DexStats class (getID, ids methods)
- [ ] BasicEffect.toString method
- [ ] TypeInfo.toString method

---

## Files Incorrectly ported

The following TS files contain mostly type definitions and data classes that are incorrectly moved in other places. You need to move them back here to be 1-1.

- [ ] dex-abilities.ts - Ability class (ported to data/abilities.rs)
- [ ] dex-conditions.ts - EventMethods, ConditionData (ported to event.rs, data/conditions.rs)
- [ ] dex-formats.ts - RuleTable, Format classes (ported to data/formats.rs)
- [ ] dex-items.ts - Item class (ported to data/items.rs)
- [ ] dex-moves.ts - Move class (ported to data/moves.rs)
- [ ] dex-species.ts - Species class (ported to data/species.rs)


## Files that don't need to be ported

- global-types.ts - Type definitions only (distributed across Rust modules)
- index.ts - Re-exports only (handled by lib.rs)

---

## Implementation Priority

### High Priority (Core Battle Logic):
1. battle-actions.ts methods - Essential for battle execution
2. pokemon.ts methods - Pokemon state management
3. side.ts methods - Side/player management
4. battle.ts remaining methods - Battle orchestration

### Medium Priority (Supporting Systems):
5. battle-queue.ts methods - Action queue management
6. field.ts methods - Field conditions
7. state.ts methods - Serialization/deserialization

### Lower Priority (Utilities):
8. teams.ts methods - Team parsing/export
9. team-validator.ts methods - Team validation
10. dex.ts methods - Data loading
11. prng.ts methods - RNG utilities
12. battle-stream.ts methods - Stream handling
13. dex-data.ts methods - Data utilities
