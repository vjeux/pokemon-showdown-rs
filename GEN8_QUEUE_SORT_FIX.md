# PRNG Fix - Gen 8 Queue Sorting Fixed

## Issue Found

In `src/battle/run_action.rs`, the Gen 8 queue sorting code was NOT matching JavaScript 1-to-1:

### JavaScript (lines 558-567 in battle.js):
```javascript
if (this.gen >= 8 && (this.queue.peek()?.choice === "move" || this.queue.peek()?.choice === "runDynamax")) {
    this.updateSpeed();
    for (const queueAction of this.queue.list) {
        if (queueAction.pokemon) this.getActionSpeed(queueAction);
    }
    this.queue.sort();
}
```

### Rust (BEFORE fix):
```rust
let mut list = std::mem::take(&mut self.queue.list);
for action in &mut list {
    self.get_action_speed(action);  // ❌ Called on ALL actions
}
self.speed_sort(&mut list, |action| { ... });  // ❌ Manually extracted PriorityItem
```

## Problems

1. **Missing pokemon check**: Rust called `get_action_speed()` on ALL actions, but JavaScript only calls it when `queueAction.pokemon` exists (Field actions don't have pokemon)
2. **Manual PriorityItem extraction**: Rust manually created PriorityItem with a closure, but JavaScript simply calls `this.queue.sort()` which internally uses the default comparator

## Fix Applied

### Changes Made:

1. **Added `has_pokemon()` method** to Action enum in `src/battle_queue.rs`:
   ```rust
   pub fn has_pokemon(&self) -> bool {
       !matches!(self, Action::Field(_))
   }
   ```

2. **Created `sort_action_queue()` method** in `src/battle/sort_action_queue.rs`:
   ```rust
   pub fn sort_action_queue(&mut self) {
       let mut list = std::mem::take(&mut self.queue.list);
       self.speed_sort(&mut list, |action| { ... });
       self.queue.list = list;
   }
   ```

3. **Updated Gen 8 sorting code** in `src/battle/run_action.rs`:
   ```rust
   let mut list = std::mem::take(&mut self.queue.list);
   for action in &mut list {
       if action.has_pokemon() {  // ✅ Only for actions with pokemon
           self.get_action_speed(action);
       }
   }
   self.queue.list = list;
   self.sort_action_queue();  // ✅ Direct method call like JavaScript
   ```

## Result

Now the Rust code matches JavaScript exactly:

| JavaScript | Rust |
|------------|------|
| `if (queueAction.pokemon)` | `if action.has_pokemon()` |
| `this.queue.sort()` | `self.sort_action_queue()` |

This ensures PRNG calls happen in the exact same order and quantity.
