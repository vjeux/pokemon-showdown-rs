# Turn Counter Bug Analysis - Seed 100 Turn 35 Divergence

## Problem Statement

After fixing the Present move, seed 100 now matches through turn 34 but diverges at turn 35.

**Symptom:**
- JavaScript: #38 executes at turn=35
- Rust: #38 executes at turn=34
- Turn counter is off by 1

## Root Cause

When a Pokemon faints during move execution (causing a switch request), and then the switch completes, **Rust does not call `end_turn()` before requesting new moves**.

## Detailed Analysis

### JavaScript Behavior (Correct)

```
#37: turn=34, prng=93->93, P1=[Embirch(132/218)], P2=[Clefable(293/293)]
     (Clefable switches in, no moves yet - just the switch)

#38: turn=35, prng=93->97, P1=[Embirch(132/218)], P2=[Clefable(269/293)]
     (Moves execute: hiddenpowerground vs shadowforce)
     (Embirch takes damage and faints)
```

**Turn Flow:**
1. Turn 34: Clefable switches in (#37)
2. **`end_turn()` called → turn increments to 35**
3. Turn 35: New moves requested and executed (#38)

### Rust Behavior (Bug)

```
#37: turn=34, prng=93->93, P1=[Embirch(132/218)], P2=[Clefable(293/293)]
     (Clefable switches in)

#38: turn=34, prng=93->99, P1=[Embirch(0/218)], P2=[Clefable(269/293)]
     (Moves execute AT TURN 34 - WRONG!)
```

**Turn Flow:**
1. Turn 34: Clefable switches in (#37)
2. **`end_turn()` NOT called → turn stays at 34**
3. Turn 34: New moves requested and executed at wrong turn (#38)

## Execution Trace

### Iteration #36 (Beedrill Faints)
```
#36: turn=33, prng=86->93, P1=[Embirch(132/218)], P2=[Beedrill(0/200)]
[MAKE_CHOICES] request_state=Switch (Beedrill fainted)
[TURN_LOOP] processes switch action queue
[END_TURN] Called, turn 33 -> 34  ✓ CORRECT
[MAKE_REQUEST] Setting request_state to Move
```

### Iteration #37 (Clefable Switches In)
```
#37: turn=34, prng=93->93, P1=[Embirch(132/218)], P2=[Clefable(293/293)]
[MAKE_CHOICES] request_state=Move, turn=34
[TURN_LOOP] Starting with 2 actions (BeforeTurn + Residual)
  - BeforeTurn action
  - Move: hiddenpowerground
  - Move: shadowforce
  - Residual action
[MAKE_REQUEST] Setting request_state to Switch (Embirch fainted during moves!)
[TURN_LOOP] Request state is Switch, exiting WITHOUT calling end_turn  ❌ BUG!
[MAKE_CHOICES] Returning, turn=34, request_state=Switch
```

### Iteration #38 (Moves Execute - Wrong Turn!)
```
#38: turn=34, prng=93->99, P1=[Embirch(0/218)], P2=[Clefable(269/293)]
                ^^^^^ SHOULD BE turn=35!
[MAKE_CHOICES] request_state=Move, turn=34  ❌ STILL TURN 34!
```

## Turn Loop Logic

From `src/battle/turn_loop.rs`:

```rust
pub fn turn_loop(&mut self) {
    // ... setup ...

    while let Some(action) = self.queue.shift() {
        self.run_action(&action);

        if self.ended {
            return;
        }

        if self.request_state != BattleRequestState::None {
            // BUG: Exits WITHOUT calling end_turn()!
            eprintln!("[TURN_LOOP] Request state is {:?}, exiting WITHOUT calling end_turn", self.request_state);
            return;  ❌
        }
    }

    // Only reached if queue is empty AND request_state is None
    eprintln!("[TURN_LOOP] Queue empty, calling end_turn");
    self.end_turn();  ✓ But this is skipped when request_state is set!
}
```

## The Issue

When `request_state` is set during action processing (e.g., a Pokemon faints):
1. `turn_loop()` exits early at line 91
2. `end_turn()` is NOT called
3. Turn counter does NOT increment
4. Next `turn_loop()` starts at the same turn number

## Solution Options

### Option 1: Call `end_turn()` After Switch Completes

After all switch actions are processed and the switch request is fulfilled, call `end_turn()` before requesting new moves.

**Pros:**
- Matches JavaScript behavior
- Minimal changes

**Cons:**
- Need to track when switches are "complete" vs "in progress"

### Option 2: Defer `end_turn()` Call

Track whether `end_turn()` needs to be called at the start of the next `turn_loop()`.

**Pros:**
- Handles all request_state cases uniformly

**Cons:**
- More complex state tracking

### Option 3: Call `end_turn()` in `make_request()` for Switch→Move Transitions

When transitioning from Switch request to Move request, call `end_turn()` first.

**Pros:**
- Centralized in make_request()
- Explicit transition handling

**Cons:**
- Make_request() is called from many places
- May cause issues with other request types

## Recommended Solution

**Option 1** - Call `end_turn()` after switches complete.

In `turn_loop()`, after processing a Switch request and before requesting new moves:
1. Check if all required switches are complete
2. Call `end_turn()` to increment turn
3. Request new moves

## Implementation Plan

1. Modify `turn_loop()` or `commit_choices()` to call `end_turn()` after switch completion
2. Add test to verify turn counter increments correctly after switches
3. Verify seed 100 now matches through turn 35+

## Files To Modify

- `src/battle/turn_loop.rs` - Add end_turn() call after switch completion
- `src/battle/make_request.rs` - May need to track switch completion state
- OR `src/battle/commit_choices.rs` - Alternative location for the fix

## Testing

After fix, verify:
- Seed 100 matches through turn 35+
- Seed 1 still passes
- Other seeds don't regress
