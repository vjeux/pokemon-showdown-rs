# Seed 21 Turn Increment Issue

## Problem
After fixing Parting Shot and related issues, seed 21 now progresses to turn 32-33 but has a turn number desync:

**JavaScript (#35-#36):**
- #35: turn=32, Aggron faints (0/205)
- #36: turn=33, Chuggon switches in

**Rust (#35-#36):**
- #35: turn=32, Aggron faints (0/205)  
- #36: turn=32, Chuggon switches in (turn doesn't increment!)

## Analysis

In iteration 35 (both implementations):
1. A move faints Aggron
2. Side 1 needs to switch
3. `make_request(Switch)` is called
4. `request_state` is set to `Switch`
5. `turn_loop` returns early WITHOUT calling `end_turn()`

In iteration 36:
**JavaScript:**
- `turn_loop` clears `request_state`
- Processes the switch action
- Turn increments to 33 (somehow `end_turn()` is called)

**Rust:**
- `turn_loop` clears `request_state`  
- Processes the switch action
- But `request_state` is set to `Switch` AGAIN
- Returns without calling `end_turn()`
- Turn stays at 32

## Key Question
Why does JavaScript increment the turn in iteration 36, but Rust doesn't?

Possibilities:
1. JavaScript's `turn_loop` might call `end_turn()` after processing switches in mid-turn
2. There might be a second `turn_loop` call in JavaScript that we're not seeing
3. The switch processing might handle turn incrementing differently
4. Our Rust logic for when to call `end_turn()` might not match JavaScript

## Next Steps
1. Add detailed logging to JavaScript test to see EXACTLY when end_turn is called
2. Check if JavaScript processes switches differently when midTurn=true
3. Compare queue state between JS and Rust iterations 36-37
4. Verify that forced switches (from faints) should increment the turn

