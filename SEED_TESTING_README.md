# Battle Testing - Seeds 1-5

This directory contains 5 random gen9 battle seeds ready for JavaScript vs Rust comparison testing.

## Quick Test Summary

| Seed | Turn | PRNG | P1 Team | P2 Team | Status |
|------|------|------|---------|---------|--------|
| 1 | 31 | 147 | Grubbin, Genesect-Shock, Glimmora, Golem-Alola, Vivillon-Savanna, Grovyle | Slowbro-Mega, Latias-Mega, Mantine, Cinderace-Gmax, Zacian, Cutiefly | ✅ Rust passes |
| 2 | 52 | 199 | Sandaconda, Poliwag, Slowbro, Houndoom, Smoguana, Sinistea-Antique | Metang, Beheeyem, Lechonk, Mudkip, Politoed, Kyurem | ✅ Rust passes |
| 3 | 30 | 140 | Braviary-Hisui, Remoraid, Vulpix-Alola, Snorunt, Swirlix, Seedot | Dedenne, Silcoon, Kirlia, Sawk, Vivillon-Ocean, Urshifu-Rapid-Strike-Gmax | ✅ Rust passes |
| 4 | ? | ? | Lickitung, Nacli, Tepig, Aggron-Mega, Ninjask, Rebble | Celesteela, Flabébé, Cubone, Maushold-Four, Dachsbun, Sandslash-Alola | ⏳ Not tested |
| 5 | ? | ? | Spoink, Clawitzer, Jumpluff, Tarountula, Delcatty, Starly | Smokomodo, Urshifu-Rapid-Strike-Gmax, Saharascal, Foongus, Spiritomb, Dubwool | ⏳ Not tested |

## How to Verify JavaScript Matches Rust

### Run Individual Seed Tests

**Rust:**
```bash
cargo run --example compare_seed1 2>&1 | grep "^#" > rust-seed1.txt
```

**JavaScript (outside Docker, in parent directory):**
```bash
cd /Users/vjeux/random/showdown/pokemon-showdown-rs
node compare-seed1.js > js-seed1.txt
```

**Compare:**
```bash
diff rust-seed1.txt js-seed1.txt
```

If the diff is empty, the implementations match perfectly!

### Run All Seeds

**Rust:**
```bash
./test-all-seeds.sh
```

**JavaScript:**
```bash
for seed in 1 2 3 4 5; do
    echo "=== Testing seed $seed ==="
    node compare-seed${seed}.js 2>&1 | tail -4
    echo ""
done
```

## Files Structure

For each seed N:
- `teams-seedN-rust.json` - Team data for Rust
- `teams-seedN-js.json` - Team data for JavaScript (same content)
- `examples/compare_seedN.rs` - Rust battle runner
- `compare-seedN.js` - JavaScript battle runner

## Expected Behavior

When JavaScript and Rust match exactly, you should see:
1. **Same turn count** for battle end
2. **Same PRNG call count** at each turn
3. **Same HP values** for all Pokemon at each turn
4. **Same winner**

Any divergence means there's a difference in implementation that needs to be fixed.

## Fixed Issues

### Shield Dust Divergence (Fixed in commit 8d9b9bb4)
- **Problem:** Rust was making extra PRNG calls for secondary effects that Shield Dust should block
- **Fix:** Implemented `EventResult::Null` handling and Shield Dust `onModifySecondaries` callback
- **Impact:** All tested battles now complete without divergences

## Next Steps

1. Run JavaScript tests for all 5 seeds
2. If any divergences found, investigate and fix (1-to-1 port from JavaScript)
3. Generate and test additional random seeds
4. Continue until multiple battles all pass without divergences
