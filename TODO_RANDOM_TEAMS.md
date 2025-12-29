# TODO: Port Random Battle Team Generator

## Status
The current `src/random_teams.rs` is a simplified stub that doesn't match the JavaScript implementation.

## What needs to be done
Port the JavaScript random battle team generator 1-to-1 from:
- Source: `/Users/vjeux/random/showdown/pokemon-showdown-ts/dist/data/random-battles/gen9/teams.js` (105KB)
- Target: `src/random_teams.rs`

## Key differences
The JavaScript implementation includes:
- Weighted species selection based on ratings
- Complex movepool logic with move scoring
- Type coverage analysis
- Ability/item synergy with moves
- Level calculation based on BST tiers
- EV spread optimization
- Nature selection based on stats
- Much more sophisticated logic

The current Rust stub just:
- Randomly picks from all fully-evolved non-Uber Pokemon
- Random ability/move/item selection
- Basic BST-based level calculation

## Why this matters
Without matching team generation, the battle comparison tests will generate completely different teams even with the same seed, making it impossible to compare battle outcomes.

## Workaround for testing
For now, we can:
1. Generate teams in JavaScript
2. Serialize them to JSON
3. Load the same teams in both JavaScript and Rust tests
4. Test that battles progress identically with identical starting conditions

This lets us test the battle engine logic without requiring the team generator to match first.
