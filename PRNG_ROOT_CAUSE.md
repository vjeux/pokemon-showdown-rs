# PRNG Desync Root Cause - FOUND

## Issue
Seed [0,0,0,2] has PRNG desynchronization causing P2 HP to be off by 1.

## Root Cause: Gender Randomization

### JavaScript Logic
```typescript
this.gender = genders[set.gender] || this.species.gender || this.battle.sample(['M', 'F']);
```

Only calls `sample(['M', 'F'])` (which calls `random(2)`) if:
1. set.gender is not specified AND
2. species.gender is not specified

### Rust Logic (BUGGY)
```rust
// Calls random(2) for EVERY Pokemon if set.gender is None
for poke_idx in 0..pokemon_count {
    if self.sides[slot_num].pokemon[poke_idx].gender == Gender::None {
        // ... calls self.random(2) here
    }
}
```

Rust doesn't check species.gender before calling random!

### Test Team Pokemon
1. **Passimian** - No species.gender → Should random ✓
2. **Cryogonal** - gender: "N" → Should NOT random ❌ RUST BUG
3. **Great Tusk** - Need to check
4. **Rotom-Fan** - gender: "N" → Should NOT random ❌ RUST BUG
5. **Mew** - gender: "N" → Should NOT random ❌ RUST BUG
6. **Ho-Oh** - gender: "N" → Should NOT random ❌ RUST BUG

### PRNG Calls
**JavaScript** (correct):
- P1: random(2) once for Passimian only
- P2: random(2) once for Passimian only

**Rust** (buggy):
- P1: random(2) SIX times (for all 6 Pokemon)
- P2: random(2) SIX times (for all 6 Pokemon)

This causes the PRNG to be out of sync by 10 extra calls!

## The Fix

Update `src/battle/set_player.rs` to:
1. Check species.gender from Dex
2. Only call random(2) if species.gender is None or empty
3. Respect species.gender if it's "N", "M", or "F"

File: `src/battle/set_player.rs` lines 100-136
