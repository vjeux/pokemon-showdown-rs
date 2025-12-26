# Battle.ts vs Battle.rs - Detailed Comparison Notes

## Architectural Differences

### 1. Object References vs Index Tuples

**JavaScript:**
```typescript
damage(damage: number, target: Pokemon | null, source: Pokemon | null, effect: Effect | null)
```

**Rust:**
```rust
pub fn damage(&mut self, damage: u32, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&str>)
```

**Decision:** This is a fundamental Rust design pattern. Rust uses indices instead of object references to avoid borrow checker issues. This is ACCEPTABLE as long as the internal logic matches.

### 2. Event Context Defaults

**JavaScript:** Uses `this.event` to provide default values for target/source/effect
```typescript
if (this.event) {
    target ||= this.event.target;
    source ||= this.event.source;
    effect ||= this.effect;
}
```

**Rust:** Does not have this pattern yet
**TODO:** Consider adding event context support if needed for callbacks

### 3. Delegation Patterns

**JavaScript:** `damage()` delegates to `spreadDamage()`
**Rust:** `spread_damage()` delegates to `damage()`

Both achieve the same result, just inverted delegation.

## Methods Compared

### ✅ RNG Methods (4/4) - COMPLETE

| Method | JS Line | Rust Line | Status | Notes |
|--------|---------|-----------|--------|-------|
| `random` | 346 | 634 | ✅ MATCH | Both delegate to PRNG |
| `randomChance` | 350 | 639 | ✅ MATCH | Added `force_random_chance` support |
| `sample` | 355 | 647 | ✅ MATCH | Both delegate to PRNG.sample |
| `resetRNG` | 360 | 3974 | ✅ MATCH | Added missing log message |

### 🔍 Damage Methods (3/6) - IN PROGRESS

| Method | JS Line | Rust Line | Status | Notes |
|--------|---------|-----------|--------|-------|
| `damage` | 2165 | 3176 | ⚠️ DIFFERENT | JS delegates to spreadDamage, Rust is direct |
| `directDamage` | 2177 | 3218 | ❌ MISMATCH | Missing: clampIntRange, Gen1 handling, faint() call |
| `spreadDamage` | 2045 | 4853 | ✅ MATCH | Logic matches |
| `heal` | 2231 | 3227 | 🔍 TODO | Need to compare |
| `boost` | 1974 | 3294 | 🔍 TODO | Need to compare |
| `setBoost` | 2068 | ? | ⚠️ MISSING | Not found in Rust |

### Issues Found in `damage/directDamage`:

1. **Missing `clampIntRange`** - JS clamps damage to minimum 1
2. **Missing Gen 1 Substitute handling** - Special case for confusion/recoil
3. **Missing `faint()` call** - JS calls `this.faint(target)` after damage
4. **Missing effect-specific log messages** - confusion, recoil, etc.

## Next Steps

1. ✅ Fix RNG methods
2. 🔄 Fix damage/heal/boost methods to match JS logic
3. ⏳ Compare event system methods
4. ⏳ Compare turn flow methods
5. ⏳ Check battle-actions.ts for delegated methods
6. ⏳ Remove unnecessary extra Rust methods

## Testing Strategy

- Run tests after each category of fixes
- Currently: 43/46 tests passing (3 pre-existing failures)
- Goal: Maintain or improve test pass rate

## Refactoring Priorities

1. **HIGH**: Core battle mechanics (damage, heal, boost, events)
2. **MEDIUM**: Turn flow, requests, logging
3. **LOW**: Utilities, accessors (as long as they work)

