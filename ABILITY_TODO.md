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
