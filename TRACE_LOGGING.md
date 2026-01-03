# Trace Logging for Battle Debugging

This document describes the trace logging system added to help debug battle divergences between Rust and JavaScript implementations.

## Overview

Trace logging provides detailed, human-readable output showing what happens during battle execution:
- Which events fire and which handlers respond
- When abilities activate and what they return
- When items trigger
- Volatile effects being added/removed
- Stat boosts being applied

## Environment Variables

Enable trace logging via environment variables:

```bash
# Individual traces
TRACE_EVENTS=1      # Log all event firing and handler calls
TRACE_ABILITIES=1   # Log ability activations
TRACE_ITEMS=1       # Log item activations
TRACE_VOLATILES=1   # Log volatile add/remove
TRACE_BOOSTS=1      # Log stat boost changes

# Enable all traces
TRACE_ALL=1         # Shortcut for all of the above
```

## Usage Examples

### Basic Usage

```bash
# Run battle test with ability tracing
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && \
  TRACE_ABILITIES=1 cargo run --example test_battle_rust 1 2>&1" | less
```

### Debugging Shield Dust

Example output showing Shield Dust filtering secondary effects:

```
[ABILITY] turn=30, ability='shielddust' on Vivillon-Savanna, event='ModifySecondaries'
[ABILITY]   ← Ability 'shielddust' on Vivillon-Savanna returned: Continue
[SPREAD_MOVE_HIT T30] No secondaries left after ModifySecondaries (filtered by Shield Dust or similar), skipping PRNG call
```

This immediately shows:
1. Shield Dust ability activated on Vivillon-Savanna
2. It handled the ModifySecondaries event
3. All secondaries were filtered out
4. PRNG call was skipped (preventing divergence)

### Event Tracing

With `TRACE_EVENTS=1`, see all events firing:

```
[EVENT] turn=30, event='ModifySecondaries', target=Some((0, 4)), source=Some((1, 4)), handlers=1
  → Calling handler: effect='shielddust' on Some((0, 4))
  ← Handler returned: Continue
```

### Boost Tracing

With `TRACE_BOOSTS=1`, see stat changes:

```
[BOOST] turn=18, target=Zacian, boosts=[atk:+1], effect=Some("intrepidsword")
```

### Volatile Tracing

With `TRACE_VOLATILES=1`, see volatile effects:

```
[VOLATILE] turn=18, ADD volatile='swordboost' to Zacian, source_effect=None
[VOLATILE] turn=29, REMOVE volatile='trapped' from Golem-Alola
```

## Common Debugging Workflows

### Find Which Ability is Causing Divergence

1. Enable ability tracing:
   ```bash
   TRACE_ABILITIES=1 cargo run --example test_battle_rust 1 2>&1 | grep ABILITY
   ```

2. Compare with JavaScript to find differences

3. Focus on the divergent turn

### Find Missing/Extra PRNG Calls

1. Run with events enabled:
   ```bash
   TRACE_EVENTS=1 cargo run --example test_battle_rust 1 2>&1
   ```

2. Look for events firing with different handlers counts

3. Check if abilities/items are triggering when they shouldn't (or vice versa)

### Debug Specific Turn

1. Enable all traces:
   ```bash
   TRACE_ALL=1 cargo run --example test_battle_rust 1 2>&1 | \
     grep "turn=30" | less
   ```

2. See complete picture of what happened that turn

## Output Format

### Events
```
[EVENT] turn=<turn>, event='<event_name>', target=<pos>, source=<pos>, handlers=<count>
  → Calling handler: effect='<effect_id>' on <pos>
  ← Handler returned: <result>
```

### Abilities
```
[ABILITY] turn=<turn>, ability='<ability_id>' on <pokemon_name>, event='<event_name>'
  ← Ability '<ability_id>' on <pokemon_name> returned: <result>
```

### Items
```
[ITEM] turn=<turn>, item='<item_id>' on <pokemon_name>, event='<event_name>'
  ← Item '<item_id>' on <pokemon_name> returned: <result>
```

### Boosts
```
[BOOST] turn=<turn>, target=<pokemon_name>, boosts=[<stat>:<value>, ...], effect=<effect>
```

### Volatiles
```
[VOLATILE] turn=<turn>, ADD volatile='<volatile_id>' to <pokemon_name>, source_effect=<effect>
[VOLATILE] turn=<turn>, REMOVE volatile='<volatile_id>' from <pokemon_name>
```

## Performance Impact

Trace logging has minimal performance impact when disabled (just env var checks).

When enabled, output can be verbose. Recommended approaches:

1. **Grep for specific patterns:**
   ```bash
   TRACE_ALL=1 cargo run --example test_battle_rust 1 2>&1 | grep "turn=30"
   ```

2. **Filter to specific traces:**
   ```bash
   # Only abilities, not all events
   TRACE_ABILITIES=1 cargo run --example test_battle_rust 1 2>&1
   ```

3. **Save to file for analysis:**
   ```bash
   TRACE_ALL=1 cargo run --example test_battle_rust 1 2>&1 > /tmp/trace.log
   ```

## Tips

1. **Start narrow:** Begin with `TRACE_ABILITIES=1` or `TRACE_EVENTS=1` rather than `TRACE_ALL=1`

2. **Grep is your friend:** Use grep to filter output to specific turns or events

3. **Compare side-by-side:** Run both Rust and JS, save logs, use `diff` to compare

4. **Look for asymmetry:** If Rust logs show an ability firing but JS doesn't (or vice versa), that's your bug

5. **PRNG correlation:** Extra/missing PRNG calls usually correlate with extra/missing event handlers

## Future Enhancements

Potential additions:
- TRACE_DAMAGE: Log damage calculation steps
- TRACE_MOVES: Log move execution phases
- TRACE_PRNG: Log each PRNG call with context
- TRACE_ACCURACY: Log accuracy checks
- Trace filtering by Pokemon name or turn range
