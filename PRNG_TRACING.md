# PRNG Tracing Documentation

This document explains how to use the PRNG tracing feature to debug divergences between JavaScript and Rust battle implementations.

## Overview

PRNG tracing shows stack traces for specific PRNG calls, making it easy to identify where JavaScript and Rust diverge in their random number generation.

## Usage

### Quick Trace Script

The easiest way to trace PRNG calls:

```bash
./tests/trace-prng.sh <seed> <trace_spec> [js|rust|both]
```

**Examples:**

```bash
# Trace PRNG call #1 in both JS and Rust for seed 100
./tests/trace-prng.sh 100 1

# Trace calls 1-5 in both languages
./tests/trace-prng.sh 100 1-5

# Trace specific calls (1, 5, and 10)
./tests/trace-prng.sh 100 1,5,10

# Trace only JavaScript
./tests/trace-prng.sh 100 1-5 js

# Trace only Rust
./tests/trace-prng.sh 100 1-5 rust
```

### Manual Tracing

You can also set the `TRACE_PRNG` environment variable directly:

**JavaScript:**
```bash
TRACE_PRNG=1-5 node tests/test-battle-js.js 100
```

**Rust:**
```bash
docker exec -e TRACE_PRNG="1-5" pokemon-rust-dev bash -c \
  "cd /home/builder/workspace && cargo run --example test_battle_rust 100"
```

### Integrated with Compare Script

The comparison script also supports tracing:

```bash
./tests/compare-battles.sh 100 1-5
```

This will run the full comparison while showing stack traces for PRNG calls 1-5.

## Trace Spec Format

The `TRACE_PRNG` environment variable accepts:

- **Single call:** `1` - Trace only call #1
- **Range:** `1-5` - Trace calls 1 through 5
- **Multiple:** `1,5,10` - Trace calls 1, 5, and 10
- **Combined:** `1-5,10,15-20` - Trace calls 1-5, 10, and 15-20

## Output Format

### JavaScript Output

```
[JS PRNG #1] Stack trace:
  at PRNG.random (prng.js:86:29)
  at PRNG.shuffle (prng.js:142:30)
  at Battle.speedSort (battle.js:285:19)
  at Battle.fieldEvent (battle.js:333:10)
  ...
```

### Rust Output

```
[Rust PRNG #1] value=1989571486
  pokemon_showdown::battle::speed_sort::{{closure}}
    at src/battle/speed_sort.rs:99
  pokemon_showdown::battle::shuffle_range
    at src/battle/shuffle_range.rs:30
  ...
```

## Debugging Workflow

### 1. Find the Divergence Point

Run a comparison to identify where PRNG counts diverge:

```bash
./tests/compare-battles.sh 100
```

Example output:
```
JavaScript:
#1: turn=2, prng=0->1, ...

Rust:
#1: turn=2, prng=0->0, ...
```

This shows JS makes 1 PRNG call on turn 2, Rust makes 0 calls.

### 2. Trace the Divergent Call

Trace the first PRNG call to see where it happens:

```bash
./tests/trace-prng.sh 100 1
```

### 3. Compare Stack Traces

Look at the JavaScript stack trace to see what triggered the call:

```
at PRNG.shuffle (prng.js:142:30)
at Battle.speedSort (battle.js:285:19)
```

Then check if Rust has the same code path and why it's not calling PRNG.

### 4. Investigate Rust Implementation

Read the corresponding Rust files:

```bash
# Read Rust's speed_sort implementation
less src/battle/speed_sort.rs

# Read Rust's shuffle implementation
less src/battle/shuffle_range.rs
```

### 5. Fix and Verify

After fixing the issue:

```bash
# Compile
docker exec pokemon-rust-dev bash -c \
  "cd /home/builder/workspace && cargo build"

# Retest
./tests/compare-battles.sh 100
```

## Common Divergence Patterns

### Missing PRNG Call in Rust

**Symptom:** Rust's PRNG call count is lower than JavaScript

**Causes:**
- Missing random chance check (e.g., `if battle.random_chance(...)`)
- Missing shuffle when there are ties
- Callback not implemented or not called

**Example:**
```
JS: #1: turn=2, prng=0->1
Rust: #1: turn=2, prng=0->0
```

### Extra PRNG Call in Rust

**Symptom:** Rust's PRNG call count is higher than JavaScript

**Causes:**
- Extra random call somewhere
- Calling RNG twice for the same operation
- Not checking a condition before calling RNG

**Example:**
```
JS: #1: turn=2, prng=0->1
Rust: #1: turn=2, prng=0->2
```

### Different Call Order

**Symptom:** PRNG counts match but values diverge

**Causes:**
- Different event execution order
- Different sorting/iteration order
- Speed tie resolution happening at different times

**Example:**
```
JS: #1: turn=2, prng=0->3, P1=[A(100/100)], P2=[B(50/100)]
Rust: #1: turn=2, prng=0->3, P1=[A(50/100)], P2=[B(100/100)]
```

## Advanced Usage

### Trace Across Multiple Turns

To trace calls across a range that spans multiple turns:

```bash
./tests/trace-prng.sh 100 1-50
```

### Compare Specific Calls Side-by-Side

```bash
# Get JS trace
TRACE_PRNG=1-5 node tests/test-battle-js.js 100 2>&1 > /tmp/js-trace.txt

# Get Rust trace
docker exec -e TRACE_PRNG="1-5" pokemon-rust-dev bash -c \
  "cd /home/builder/workspace && cargo run --example test_battle_rust 100 2>&1" \
  > /tmp/rust-trace.txt

# Compare
diff /tmp/js-trace.txt /tmp/rust-trace.txt
```

### Focus on Specific Functions

Use grep to filter stack traces to specific functions:

```bash
./tests/trace-prng.sh 100 1-10 | grep -A 10 "speedSort"
```

## Tips

- Start with tracing just the first divergent call (e.g., `TRACE_PRNG=1`)
- Use ranges to see patterns (e.g., `TRACE_PRNG=1-10`)
- Compare the call sites between JS and Rust
- Check if the conditions for making the call match
- Verify the order of operations is the same
- Look for early returns or guards that might skip calls

## Examples

### Example 1: Seed 100 Investigation

```bash
$ ./tests/compare-battles.sh 100
...
JavaScript:
#1: turn=2, prng=0->1, P1=[Linoone(153/153)], P2=[Raging Bolt(225/225)]

Rust:
#1: turn=2, prng=0->0, P1=[Linoone(153/153)], P2=[Raging Bolt(225/225)]
```

Trace call #1:

```bash
$ ./tests/trace-prng.sh 100 1 js

[JS PRNG #1] Stack trace:
  at PRNG.shuffle (prng.js:142:30)
  at Battle.speedSort (battle.js:285:19)
  at Battle.fieldEvent (battle.js:333:10)
```

**Analysis:** JavaScript is calling shuffle during speedSort in a field event on turn 2. Rust is not making this call. Need to check if Rust's speedSort is being called and if it's handling ties correctly.

### Example 2: Seed 123 Investigation

If seed 123 shows matching PRNG counts but different damage, trace the damage-related calls:

```bash
./tests/trace-prng.sh 123 10-20
```

Then look for calls in damage calculation code.

## Limitations

- Stack traces are limited to the most relevant frames
- Very high call counts (>1000) may impact performance
- Docker environment variable passing has size limits
- Rust stack traces may be less readable than JavaScript due to name mangling

## Troubleshooting

**Issue:** No stack traces appear

**Solution:** Make sure `TRACE_PRNG` is set and the call numbers are valid

**Issue:** Rust stack traces are garbled

**Solution:** This is normal. Look for function names and file paths

**Issue:** Too much output

**Solution:** Use more specific ranges or pipe to `head`/`tail`

## Future Improvements

Potential enhancements:
- Filter stack traces to show only specific functions
- Colorized output for easier reading
- Side-by-side comparison mode
- Automatic divergence detection with stack trace
