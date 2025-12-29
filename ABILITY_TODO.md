# Ability Implementation TODOs

This file tracks abilities that require infrastructure not yet implemented in Rust.

## Missing Infrastructure

### Screen Cleaner (screencleaner.rs)
- **Missing**: Need infrastructure for looping through sides and removing side conditions with battle log messages
- **JavaScript code**: Loops through ['reflect', 'lightscreen', 'auroraveil'], checks both own side and foe sides, removes conditions
- **Required**:
  - Easy way to loop through all sides (own + foe sides with conditions)
  - Battle log message with pokemon identifier: `this.add('-activate', pokemon, 'ability: Screen Cleaner')`
  - Need to understand Arg enum and how to construct pokemon identifier for add()

### Leaf Guard (leafguard.rs)
- **Missing**: Battle log messages with pokemon identifier
- **JavaScript code**:
  - `this.add('-immune', target, '[from] ability: Leaf Guard')` in on_set_status
  - `this.add('-immune', target, '[from] ability: Leaf Guard')` in on_try_add_volatile
- **Required**:
  - Need to understand Arg enum and how to construct pokemon identifier for add()
  - Otherwise implementation is straightforward (check weather, return false/null)

### Filter / Solid Rock / Prism Armor (filter.rs, solidrock.rs, prismarmor.rs)
- **Missing**: getMoveHitData to check type effectiveness
- **JavaScript code**: `if (target.getMoveHitData(move).typeMod > 0)` - checks if super effective
- **Required**:
  - Need getMoveHitData(move) function that returns {typeMod, crit, ...}
  - typeMod > 0 means super effective
  - Then reduce damage with chainModify(0.75)

### Sniper (sniper.rs)
- **Missing**: getMoveHitData to check critical hits
- **JavaScript code**: `if (target.getMoveHitData(move).crit)` - checks if move was a crit
- **Required**:
  - Need getMoveHitData(move) function that returns {crit, ...}
  - crit is boolean indicating if the move critically hit
  - Then boost damage with chainModify(1.5)

### Reckless (reckless.rs)
- **Missing**: hasCrashDamage field on MoveData
- **JavaScript code**: `if (move.recoil || move.hasCrashDamage)`
- **Required**:
  - MoveData already has `recoil: Option<(i32, i32)>`
  - Need `has_crash_damage: bool` or similar field for moves like Jump Kick that damage on miss
  - Then boost with chain_modify_fraction(4915, 4096)

### Sheer Force (sheerforce.rs)
- **Missing**: Move modification infrastructure
- **JavaScript code**: Modifies move by deleting secondaries, setting hasSheerForce flag
- **Required**:
  - Need ability to modify move state during battle
  - Need hasSheerForce flag on move state
  - onModifyMove needs to be able to alter move properties
  - onBasePower checks move.hasSheerForce and boosts with chain_modify_fraction(5325, 4096)

### Technician (technician.rs)
- **Missing**: this.modify() function and this.event.modifier
- **JavaScript code**: `this.modify(basePower, this.event.modifier)` to get final base power
- **Required**:
  - Need modify function that applies event modifiers to base power
  - Need access to event.modifier value
  - Then check if <= 60 and boost with chain_modify(1.5)

### Adaptability (adaptability.rs)
- **Missing**: STAB calculation and hasType checking
- **JavaScript code**: `if (move.forceSTAB || source.hasType(move.type))`
- **Required**:
  - Need source.hasType(type_string) method
  - Need move.forceSTAB field
  - Need to receive stab parameter value and modify it
  - Return 2.25 if stab === 2, else return 2

### Aerilate (aerilate.rs) and similar -ate abilities
- **Missing**: Move type modification infrastructure
- **JavaScript code**: Changes move.type, sets move.typeChangerBoosted
- **Required**:
  - Need ability to modify move type during battle
  - Need typeChangerBoosted flag on move state
  - Need checks for move.isZ, pokemon.terastallized, activeMove.isMax
  - Boost with chain_modify_fraction(4915, 4096) if typeChangerBoosted

### Analytic (analytic.rs)
- **Missing**: Queue system and getAllActive
- **JavaScript code**: `this.queue.willMove(target)` and `this.getAllActive()`
- **Required**:
  - Need getAllActive() to get all active pokemon
  - Need queue.willMove(pokemon) to check if pokemon will move this turn
  - Boost with chain_modify_fraction(5325, 4096) if moving last

### Flash Fire (flashfire.rs)
- **Missing**: Volatile status system and hasAbility checking
- **JavaScript code**: `target.addVolatile('flashfire')`, `attacker.hasAbility('flashfire')`
- **Required**:
  - Need addVolatile/removeVolatile for temporary status
  - Need hasAbility checking
  - Need battle log messages
  - Flash Fire condition boosts Fire moves by 1.5x

### Weather-based speed abilities (Chlorophyll, Swift Swim, Sand Rush, Slush Rush)
- **Missing**: pokemon.effectiveWeather() method
- **JavaScript code**: `if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather()))`
- **Required**:
  - Need pokemon.effectiveWeather() that returns current weather ID
  - Different abilities check different weather conditions
  - All double speed in their respective weather

### Priority modification abilities (Gale Wings, Prankster, Triage)
- **Missing**: Ability to modify priority and set move state flags
- **JavaScript code**: `move.pranksterBoosted = true; return priority + 1;`
- **Required**:
  - Need ability to return modified priority value
  - Need ability to set flags on move state (pranksterBoosted, etc.)
  - onModifyPriority needs to actually modify the priority

### Wonder Guard (wonderguard.rs)
- **Missing**: Type effectiveness and immunity checking
- **JavaScript code**: `target.runEffectiveness(move)`, `target.runImmunity(move)`
- **Required**:
  - Need runEffectiveness to calculate type matchup
  - Need runImmunity to check immunities
  - Only allows super-effective hits to damage

### Contact-based abilities (Flame Body, Static, Poison Touch, Aftermath, Cute Charm, etc.)
- **Missing**: checkMoveMakesContact, trySetStatus, damage functions
- **JavaScript code**: `this.checkMoveMakesContact(move, source, target)`, `target.trySetStatus('brn', source)`
- **Required**:
  - Need checkMoveMakesContact to determine if move makes contact
  - Need trySetStatus to attempt inflicting status
  - Need damage() function to deal direct damage
  - Many use randomChance for 30% probability

### Boost-modifying abilities (Moxie, Simple, Clear Body, White Smoke, Keen Eye, etc.)
- **Missing**: Boost system
- **JavaScript code**: `this.boost({atk: 1}, source)`, `delete boost.atk`, `boosts['atk'] = 0`
- **Required**:
  - Need boost() to change stat stages
  - Need way to modify boost parameter object
  - Need onTryBoost, onChangeBoost, onAnyModifyBoost infrastructure

### Move modification abilities (Infiltrator, Long Reach, Keen Eye, Unseen Fist, etc.)
- **Missing**: Ability to modify move state during battle
- **JavaScript code**: `move.infiltrates = true`, `delete move.flags['contact']`, `move.ignoreEvasion = true`
- **Required**:
  - Need mutable move state that can be modified by abilities
  - Different abilities set different flags/properties

### Item-based abilities (Frisk, Pickup, Unburden, etc.)
- **Missing**: Item system
- **JavaScript code**: `target.item`, `pokemon.setItem(item)`, `target.getItem()`, `pokemon.hasItem('utilityumbrella')`
- **Required**:
  - Need item field on Pokemon
  - Need setItem, getItem, hasItem methods
  - Need lastItem tracking

### Weather/Terrain setting abilities (Sand Stream, Electric Surge, Grassy Surge, etc.)
- **Missing**: field.setWeather() and field.setTerrain() functions
- **JavaScript code**: `this.field.setWeather('sandstorm')`, `this.field.setTerrain('electricterrain')`
- **Required**:
  - Need field.setWeather(weather_id) to set weather
  - Need field.setTerrain(terrain_id) to set terrain
  - Examples: Sand Stream, Snow Warning, Electric Surge, Grassy Surge, Misty Surge, Psychic Surge

### Healing abilities (Regenerator, Earth Eater, etc.)
- **Missing**: heal() and damage() functions
- **JavaScript code**: `pokemon.heal(pokemon.baseMaxhp / 3)`, `this.heal(target.baseMaxhp / 4)`
- **Required**:
  - Need pokemon.heal(amount) to restore HP
  - Need this.heal(amount) that returns boolean (false if already at full HP)
  - Need pokemon.baseMaxhp or maxhp field
  - Examples: Regenerator (heals 1/3 on switch), Earth Eater (heals 1/4 when hit by Ground)

### Priority blocking abilities (Queenly Majesty, Armor Tail, Dazzling)
- **Missing**: Move priority checking and target checking
- **JavaScript code**: `move.priority > 0.1`, `move.target === 'foeSide'`, `source.isAlly(holder)`
- **Required**:
  - Need access to move.priority field
  - Need access to move.target field
  - Need source.isAlly(target) checking
  - Need this.attrLastMove('[still]')
  - Examples: Queenly Majesty, Armor Tail, Dazzling

### Trapping abilities (Arena Trap, Shadow Tag, Magnet Pull)
- **Missing**: Trapping system
- **JavaScript code**: `pokemon.tryTrap(true)`, `pokemon.maybeTrapped = true`, `pokemon.isGrounded()`, `pokemon.isAdjacent(target)`
- **Required**:
  - Need pokemon.tryTrap() to trap a pokemon
  - Need pokemon.maybeTrapped field
  - Need pokemon.isGrounded() to check if grounded
  - Need pokemon.isAdjacent(target) to check adjacency
  - Examples: Arena Trap, Shadow Tag, Magnet Pull

### effectState.target access (Soundproof, No Guard, many others)
- **Missing**: effectState system to track ability state and get holder
- **JavaScript code**: `this.effectState.target` to get the pokemon with the ability
- **Required**:
  - Need effectState.target to identify which pokemon has the ability in onAny* callbacks
  - Examples: soundproof.rs on_ally_try_hit_side, noguard.rs on_any_invulnerability
  - Pattern: `this.add('-immune', this.effectState.target, '[from] ability: Soundproof')`

### getImmunity checking (Overcoat, etc.)
- **Missing**: Type/effect immunity checking
- **JavaScript code**: `this.dex.getImmunity('powder', target)`
- **Required**:
  - Need dex.getImmunity(type_or_effect, pokemon) to check immunities
  - Returns true if pokemon can be affected by the type/effect
  - Used in overcoat.rs on_try_hit: checks powder immunity before blocking

### Pokemon HP tracking (Sturdy, Multiscale, etc.)
- **Missing**: pokemon.hp and pokemon.maxhp fields
- **JavaScript code**: `target.hp === target.maxhp`, `damage >= target.hp`, `target.hp - 1`
- **Required**:
  - Need pokemon.hp field to track current HP
  - Need pokemon.maxhp or baseMaxhp field for maximum HP
  - Used in sturdy.rs to survive OHKO at full HP
  - Used in many abilities to check HP thresholds

### Effect type checking (Sturdy, Disguise, etc.)
- **Missing**: effect.effectType property
- **JavaScript code**: `effect.effectType === 'Move'`
- **Required**:
  - Need way to check if effect is a Move vs Ability vs Item vs Status
  - Used to distinguish between different damage sources
  - Used in sturdy.rs on_damage, disguise.rs on_damage

### Move OHKO field (Sturdy)
- **Missing**: MoveData ohko field
- **JavaScript code**: `move.ohko` checks if move is a one-hit-KO move
- **Required**:
  - Need MoveData to expose ohko: bool field
  - OHKO moves: Fissure, Guillotine, Horn Drill, Sheer Cold
  - Used in sturdy.rs on_try_hit to block OHKO moves

### activeMove access (Rock Head, many others)
- **Missing**: battle.activeMove to get currently executing move
- **JavaScript code**: `this.activeMove`, `this.activeMove.id`
- **Required**:
  - Need battle.active_move or similar to access current move being executed
  - Used to check move ID in context where move_id parameter isn't provided
  - Used in rockhead.rs to check if recoil damage is from Struggle
