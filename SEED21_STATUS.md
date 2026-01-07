# Seed 21 Status

## Progress
- ✅ Fixed: Turn increment issue (turn 32->33 mismatch)
- ✅ Fixed: switch_flag not being cleared when Pokemon switches out  
- ⚠️ Remaining: PRNG divergence at iteration 41

## Fixes Applied
1. **switch_flag clearing** (commit 9143ec7d)
   - When a Pokemon switches out, its switch_flag is now cleared
   - This prevents false positives when the Pokemon switches back in later
   - Fixes the issue where Chuggon's old "partingshot" switch_flag from iteration 8 was still set in iteration 36

## Current Divergence
Iteration 41 (Turn 35->36):
- JavaScript: prng=78->79 (1 call)
- Rust: prng=78->80 (2 calls)
- Same outcome: Weepinbell uses Baneful Bunker (gains stall), Grovyle uses Petal Dance (takes damage, gets poisoned)
- One extra PRNG call in Rust

## Next Steps
1. Add PRNG call logging to identify where the extra call happens
2. Compare Baneful Bunker and Petal Dance implementations
3. Check stall volatile and poison application logic
