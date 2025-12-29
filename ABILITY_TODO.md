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
