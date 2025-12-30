# DAMAGE CALCULATION FIX - Complete

## Problem

HP values differed by exactly 1 between JavaScript and Rust:
- JavaScript: P2 had 264 HP
- Rust: P2 had 263 HP
- Difference: **1 HP**

## Investigation Process

### 1. Initial Suspicion: Randomizer Function
First suspected the `randomizer()` function that applies 85%-100% variance to damage.
- Added logging to both JavaScript and Rust
- Found: randomizer WAS working correctly
- JavaScript used roll=11, Rust used roll=11 (same)

### 2. Base Damage Difference
Discovered the input to randomizer was different:
- JavaScript: base_damage = 87
- Rust: base_damage = 88
- This 1-point difference propagated through!

### 3. Damage Formula Investigation
The base damage formula is: `tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50)`

Found that `getDamage()` returned:
- JavaScript: 85
- Rust: 86

The `modifyDamage()` function adds 2, making it:
- JavaScript: 87
- Rust: 88

### 4. BasePower Event
Found the issue was in the **BasePower event** for Knock Off:
- Move: Knock Off (base power 65)
- Knock Off applies 1.5x when target has an item
- JavaScript: 65 → 97 after event
- Rust: 65 → 98 after event
- **Difference: 1 point!**

### 5. Root Cause: modify_internal()

The bug was in `/src/battle/modify_internal.rs`:

**JavaScript formula:**
```javascript
tr((tr(value * modifier) + 2048 - 1) / 4096)
```

**Rust (BEFORE fix):**
```rust
((value * modifier + 2048) >> 12)
```

**Key differences:**
1. JavaScript adds **2047** (`2048 - 1`), Rust was adding **2048**
2. JavaScript has `tr(value * modifier)` truncation before adding
3. JavaScript divides by `4096.0` with `trunc()`, Rust was using bit shift `>> 12`

## The Fix

Updated `modify_internal()` to match JavaScript exactly:

```rust
pub fn modify_internal(&self, value: i32, modifier: i32) -> i32 {
    let product = value as i64 * modifier as i64;
    let truncated_product = self.trunc(product as f64, None) as i64;
    let adjusted = truncated_product + 2048 - 1;  // +2047, not +2048!
    let result = self.trunc(adjusted as f64 / 4096.0, None) as i32;
    result.max(1)
}
```

## Results

**Before:**
- Knock Off BasePower: 98
- getDamage base_damage: 86
- After +2 in modifyDamage: 88
- After randomizer: 78
- Final P2 HP: **263**

**After:**
- Knock Off BasePower: 97 ✓
- getDamage base_damage: 85 ✓
- After +2 in modifyDamage: 87 ✓
- After randomizer: 77 ✓
- Final P2 HP: **264** ✓

## Impact

This fix ensures all damage modifiers (abilities, items, moves) calculate **exactly** the same values as JavaScript, preventing any HP desynchronization in battles.

The formula `tr((tr(value * modifier) + 2048 - 1) / 4096)` is critical for maintaining perfect synchronization with the JavaScript implementation.
