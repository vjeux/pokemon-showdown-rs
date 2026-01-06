# Systematic Bug Patterns and Solutions

This document analyzes bug patterns from the last 50 commits to identify systematic issues and propose comprehensive solutions.

**Current Pass Rate**: 37% (16/100 seeds passing)

---

## Most Frequently Changed Files (Infrastructure)

Analysis of the last 50 commits reveals the following hotspots:

| Changes | File | Purpose |
|---------|------|---------|
| 10 | `src/data/condition_callbacks/mod.rs` | Condition callback dispatcher |
| 10 | `src/battle.rs` | Core battle logic |
| 8 | `src/battle/run_event.rs` | Event execution |
| 8 | `src/battle/handle_condition_event.rs` | Condition event routing |
| 7 | `src/battle_actions/run_move_effects.rs` | Move effect application |
| 6 | `src/data/move_callbacks/mod.rs` | Move callback dispatcher |
| 6 | `src/battle/dispatch_single_event.rs` | Event dispatching |
| 6 | `src/battle/handle_move_event.rs` | Move event handling |
| 6 | `src/event.rs` | Event type definitions |
| 6 | `src/battle/single_event.rs` | Single event execution |

**These files are the infrastructure layer where most bugs originate.**

---

## Bug Categories (By Frequency)

### 1. Parameter Order Bugs ⚠️ **MOST COMMON**

**Affected Commits**:
- `ba9ab35b` - gmaxcannonade self.onHit parameter order
- `2c4bc5e0` - Parameter order in move callbacks and effectiveness handlers
- `bf5b5ecd` - Parameter order in move dispatchers and self callbacks
- `683781ee` - Move onAfterHit callback parameter order
- `03b29f3c` - Soak parameter order
- `60c58c15` - Tarshot effectiveness event parameter passing
- `14b79045` - TryAddVolatile callback parameter passing

**Root Cause**:
Dispatcher functions have misleading parameter names that don't match the actual data being passed. For example, in `dispatch_self_on_hit`, the parameter named `target_pos` actually receives the source position, and `source_pos` receives the target position.

**Example**:
```rust
// In src/data/move_callbacks/mod.rs
pub fn dispatch_self_on_hit(
    battle: &mut Battle,
    move_id: &str,
    target_pos: (usize, usize),         // MISLEADING: receives SOURCE
    source_pos: Option<(usize, usize)>, // MISLEADING: receives TARGET
) -> EventResult
```

Called from `src/battle/handle_move_event.rs:117`:
```rust
move_callbacks::dispatch_self_on_hit(self, move_str, src, target_pos)
//                                                     ^^^  ^^^^^^^^^^
//                                                   source   target
```

**Impact**: 7+ bugs, countless hours debugging, high risk of future regressions.

---

### 2. Data Not Copied to ActiveMove

**Affected Commits**:
- `6c8d3f7c` - Baton Pass selfSwitch not copied from MoveData
- `36a2b1f9` - self_effect not copied from MoveData

**Root Cause**:
`src/dex/get_active_move.rs` doesn't copy all necessary fields from `MoveData` to `ActiveMove`, causing move effects to be missing during execution.

**Impact**: Move effects silently fail because the data isn't available at runtime.

---

### 3. Event Routing/Infrastructure

**Affected Commits**:
- `6f2f8759` - Side condition event routing
- `939b87c7` - TryMove event routing
- `0e407e16` - ModifyType event prioritization
- `9f164cb4` - Side condition infrastructure
- `412f4808` - Move self effects infrastructure

**Root Cause**:
The event dispatch system doesn't match JavaScript's routing logic, causing callbacks to not be called or called with incorrect parameters.

**Impact**: Callbacks don't fire when they should, or fire with wrong context.

---

### 4. Move-Embedded vs Standalone Conditions

**Affected Commits**:
- `9cad5eca` - Move-embedded vs standalone separation
- `416cc639` - Fallbacks for move-embedded callbacks
- `355555d9` - has_callback infrastructure

**Root Cause**:
JavaScript stores condition callbacks in two places:
1. Standalone condition files (e.g., `data/conditions.ts`)
2. Embedded in move data (e.g., `gmaxcannonade.condition.onSideStart`)

Rust initially only checked standalone conditions.

**Impact**: Move-embedded callbacks were never called.

---

### 5. Self vs Target Confusion

**Affected Commits**:
- `e097e6bf` - Self-targeting secondary effects
- `7879adce` - self.onHit being called recursively
- `da27333c` - gmaxbefuddle incorrect move-level on_hit

**Root Cause**:
Confusion about when effects target the move user (self) vs the target, and which callbacks handle which case.

**JavaScript Convention**:
- `onHit` - affects the TARGET of the move
- `self.onHit` - affects the USER of the move
- Both can exist on the same move and both should be called

**Impact**: Effects applied to wrong Pokemon, or called recursively.

---

## Systematic Solutions

### Solution 1: Parameter Order Audit & Rename 🎯 **HIGHEST PRIORITY**

**Goal**: Eliminate parameter order confusion by making parameter names match actual data.

**Action Items**:

1. **Audit all dispatcher functions** in:
   - `src/data/move_callbacks/mod.rs`
   - `src/data/condition_callbacks/mod.rs`
   - `src/battle/handle_move_event.rs`
   - `src/battle/handle_condition_event.rs`
   - `src/battle/handle_ability_event.rs`

2. **For each dispatcher, document**:
   - What data each parameter ACTUALLY receives (not what it's named)
   - Who calls the function and in what order
   - Create a mapping table

3. **Rename parameters** to match actual data:
   ```rust
   // BEFORE (misleading)
   pub fn dispatch_self_on_hit(
       battle: &mut Battle,
       move_id: &str,
       target_pos: (usize, usize),         // Actually receives source!
       source_pos: Option<(usize, usize)>, // Actually receives target!
   )

   // AFTER (accurate)
   pub fn dispatch_self_on_hit(
       battle: &mut Battle,
       move_id: &str,
       user_pos: (usize, usize),           // Move user (source)
       target_pos: Option<(usize, usize)>, // Move target
   )
   ```

4. **Update all call sites** to use new parameter names

5. **Add documentation** explaining the convention:
   - For `onHit` callbacks: first param is target, second is source
   - For `self.onHit` callbacks: first param is user, second is target
   - etc.

**Expected Impact**: Prevent entire class of parameter order bugs (7+ bugs fixed, future bugs prevented).

---

### Solution 2: ActiveMove Field Completeness Audit

**Goal**: Ensure all necessary MoveData fields are copied to ActiveMove.

**Action Items**:

1. **Create a checklist** of all MoveData fields in JavaScript's Move class

2. **Audit `src/dex/get_active_move.rs`** against the checklist:
   - ✅ Fields that are copied
   - ❌ Fields that are NOT copied
   - 📝 Explain why any field is intentionally omitted

3. **Add missing fields** to ActiveMove struct and copy logic

4. **Add comments** in `get_active_move.rs` explaining each field:
   ```rust
   // Copy selfSwitch - needed for Baton Pass, U-turn, etc.
   self_switch: move_data.self_switch.clone(),

   // Copy self_effect - needed for stat boosts on user
   self_effect: move_data.self_effect.clone(),
   ```

**Expected Impact**: Prevent "data not copied" bugs (2+ bugs fixed).

---

### Solution 3: Event Dispatch Flow Documentation

**Goal**: Document the complete event dispatch flow to match JavaScript.

**Action Items**:

1. **Map JavaScript event flow**:
   - Which events exist (Hit, Try, TryMove, ModifyType, etc.)
   - Which events route to which handlers
   - Callback priority/ordering
   - Event propagation rules

2. **Compare to Rust implementation**:
   - `src/battle/dispatch_single_event.rs` - event routing logic
   - `src/battle/handle_*_event.rs` - specialized handlers
   - Identify gaps

3. **Create a flow diagram** showing:
   ```
   single_event(event_id, effect_id, target, source)
     |
     +--> dispatch_single_event (routes by effect type)
           |
           +--> Move events -> handle_move_event
           |     |
           |     +--> "Hit" -> dispatch_on_hit + dispatch_self_on_hit
           |     +--> "Try" -> dispatch_try
           |     etc.
           |
           +--> Condition events -> handle_condition_event
           |
           +--> Ability events -> handle_ability_event
   ```

4. **Add missing event types** to routing logic

5. **Document event priorities** (which events fire first)

**Expected Impact**: Prevent event routing bugs (5+ bugs fixed).

---

### Solution 4: Dual Callback Location Check

**Goal**: Ensure callbacks are found in both standalone and move-embedded locations.

**Action Items**:

1. **Audit `src/battle/has_callback.rs`**:
   - Verify it checks standalone condition files
   - Verify it checks move-embedded condition data
   - Verify it checks both for all callback types

2. **Add tests** to verify callback detection:
   ```rust
   // Test standalone condition callback
   assert!(battle.has_callback("gmaxvolcalith", "onSideStart"));

   // Test move-embedded condition callback
   assert!(battle.has_callback("gmaxcannonade", "onSideStart"));
   ```

3. **Document the two-location pattern** in code comments

**Expected Impact**: Prevent move-embedded callback bugs (3+ bugs fixed).

---

### Solution 5: Self vs Target Convention Documentation

**Goal**: Clarify when to use `onHit` vs `self.onHit` and document conventions.

**Action Items**:

1. **Document the convention** in `src/data/move_callbacks/mod.rs`:
   ```rust
   /// # Move Callback Conventions
   ///
   /// - `onHit` - Effect applies to the TARGET of the move
   ///   - First parameter: target position
   ///   - Second parameter: source position
   ///
   /// - `self.onHit` - Effect applies to the USER of the move
   ///   - First parameter: user position (move user)
   ///   - Second parameter: target position (move target)
   ///
   /// - Both can exist on the same move:
   ///   - Regular onHit fires for each target hit
   ///   - self.onHit fires once after all hits, affects user
   ///
   /// - Never call self.onHit recursively from within itself
   ```

2. **Add validation** in move callback implementations:
   ```rust
   // In self.onHit callbacks, add assertion:
   debug_assert_ne!(user_pos, target_pos, "self.onHit should not be called recursively");
   ```

3. **Audit all move callbacks** for correct self vs target usage

**Expected Impact**: Prevent self/target confusion bugs (3+ bugs fixed).

---

## Implementation Priority

1. **🔴 Critical**: Solution 1 (Parameter Order Audit) - Most bugs stem from this
2. **🟠 High**: Solution 2 (ActiveMove Completeness) - Prevents silent failures
3. **🟡 Medium**: Solution 3 (Event Dispatch Flow) - Prevents infrastructure bugs
4. **🟢 Low**: Solution 4 (Dual Callback Check) - Already mostly fixed
5. **🟢 Low**: Solution 5 (Self vs Target Docs) - Prevents future bugs

---

## Success Metrics

- **Before**: 37% pass rate (16/100 seeds)
- **Target after Solution 1**: 50%+ pass rate
- **Target after all solutions**: 70%+ pass rate

---

## Notes

- This document should be updated as solutions are implemented
- Mark solutions as ✅ DONE when completed
- Add new bug patterns as they're discovered
- Track pass rate improvements after each solution
