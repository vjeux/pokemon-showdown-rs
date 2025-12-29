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
