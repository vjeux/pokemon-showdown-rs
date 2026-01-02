# Pokemon Module Divergences from JavaScript

This document tracks divergences between the JavaScript and Rust implementations in the `src/pokemon/` folder.

## Overview
- Total TODOs/NOTEs found: 120
- Goal: Achieve 1:1 line-by-line equivalence with JavaScript

## Status Legend
- ✅ Fixed - Fully implemented matching JavaScript
- 🔧 In Progress - Currently being worked on
- ❌ Not Started - Needs implementation
- 📝 Documented - Divergence documented but intentional

## Files to Fix

### High Priority - Missing Core Methods

#### adjacentAllies.rs
- Status: ✅ Fixed
- Issue: Empty TODO stub, needs full implementation
- JS Source: Available in file
- Fix: Implemented proper adjacency filtering with multi-battle support

#### adjacentFoes.rs
- Status: ✅ Fixed
- Issue: Empty TODO stub, needs full implementation
- JS Source: Available in file
- Fix: Implemented with activePerHalf check for singles/doubles optimization

#### allies.rs
- Status: ✅ Fixed
- Issue: Empty TODO stub, needs full implementation
- JS Source: Available in file
- Fix: Implemented to return all active teammates except self

#### alliesAndSelf.rs
- Status: ✅ Fixed
- Issue: Empty TODO stub, needs full implementation
- JS Source: Available in file
- Fix: Implemented to return all active teammates including self

#### clearVolatile.rs
- Status: ❌ Not Started
- Issue: Empty TODO stub, needs full implementation
- JS Source: Available in file

#### foes.rs
- Status: ✅ Fixed
- Issue: Empty TODO stub, needs full implementation
- JS Source: Available in file
- Fix: Implemented with FreeForAll and standard game type support

### Medium Priority - Stub Implementations

#### adjacent_allies_stub.rs
- Status: ✅ Fixed (Deleted)
- Issue: Uses simplified logic, should use battle.side.allies()
- Note: Consolidated into adjacent_allies.rs

#### adjacent_foes_stub.rs
- Status: ✅ Fixed (Deleted)
- Issue: Uses simplified logic, should use battle.side.foes()
- Note: Consolidated into adjacent_foes.rs

#### allies_stub.rs
- Status: ✅ Fixed (Deleted)
- Issue: Stub implementation exists, needs proper method
- Note: Consolidated into allies.rs

#### allies_and_self_stub.rs
- Status: ✅ Fixed (Deleted)
- Issue: Stub implementation exists, needs proper method
- Note: Consolidated into allies_and_self.rs

#### foes_stub.rs
- Status: ✅ Fixed (Deleted)
- Issue: Stub implementation exists, needs proper method
- Note: Consolidated into foes.rs

### Low Priority - Partial Implementations

#### add_volatile.rs
- Status: ❌ Not Started
- Issue: "TODO: Double check that the entire logic is mapped 1-1 with JavaScript"
- Action: Review and verify against JS source

#### boost_by.rs
- Status: ❌ Not Started
- Issue: Multiple "TODO: implement the same logic as JavaScript"
- Action: Implement missing logic

#### calculate_stat.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete stat calculation logic

#### clear_boosts.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Implement boost clearing

#### clear_status.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Implement status clearing

#### clear_volatile_full.rs
- Status: ❌ Not Started
- Issues:
  - Should be moved to clear_volatile.rs
  - Missing hpType and hpPower implementation
  - attacked_by field doesn't exist in Rust Pokemon struct
- Action: Consolidate with clear_volatile.rs and implement missing fields

#### copy_volatile_from.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete volatile copying logic

#### copy_volatile_from_full.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete volatile copying logic

#### cure_status.rs
- Status: ❌ Not Started
- Issue: "TODO: have the callsite pass in the Battle object"
- Note: Due to Rust borrow checker limitations
- Action: Refactor to match JS pattern

#### deduct_pp.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete PP deduction logic

#### disable_move.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete move disabling logic

#### eat_item.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete item eating logic

#### effective_weather.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete weather logic

#### faint.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete fainting logic

#### get_ability.rs
- Status: ❌ Not Started
- Issue: "TODO: implement the same logic as JavaScript"
- Action: Complete ability retrieval logic

### Rust-Specific Helpers (May be intentional)

The following are marked as "NOTE: This method is NOT in JavaScript - Rust-specific implementation":
- boost.rs
- can_switch.rs
- can_tera.rs
- clear_switch_state.rs
- clear_turn_state.rs
- clear_volatile_full.rs
- clear_volatiles.rs
- copy_volatile_from_full.rs
- details.rs
- fullname.rs
- get_base_moves.rs

**Action**: Verify these are truly Rust-specific helpers or if they should match JS equivalents.

## Progress Tracking

### Session 1 - 2026-01-01
- Created tracking document
- Identified 120 TODOs/NOTEs
- Categorized by priority
- **Completed**:
  - ✅ Implemented allies.rs (returns teammates except self)
  - ✅ Implemented allies_and_self.rs (returns all teammates including self)
  - ✅ Implemented adjacent_allies.rs (returns adjacent teammates with multi-battle support)
  - ✅ Implemented adjacent_foes.rs (returns adjacent foes with activePerHalf optimization)
  - ✅ Implemented foes.rs (returns enemy pokemon with FreeForAll support)
  - ✅ Deleted 5 _stub files that were consolidated
  - ✅ Fixed src/dex/mod.rs compilation conflict (sync script error)
  - ✅ Updated pokemon.rs mod declarations
  - ✅ Project compiles successfully (0 errors, 0 warnings)
  - ✅ Committed and pushed: "Consolidate pokemon helper methods and fix dex/mod.rs conflict"
- **Next**: Continue with Low Priority partial implementations (add_volatile, boost_by, calculate_stat, etc.)

### Session 2 - [Date]
- TBD

## Notes
- Must compile after each fix
- Commit and push after each successful implementation
- May require significant refactoring of Pokemon struct to add missing fields
