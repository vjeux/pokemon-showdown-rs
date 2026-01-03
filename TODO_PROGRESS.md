# TODO Progress Tracking

## Summary
- Total ability callback TODOs: 380
- Completed: 15 (3.9%)
- In Progress: Continuing systematic implementation

## Completed Implementations

### Batch 1 - Basic Weather/Terrain Modifiers
1. **Sand Rush** (sandrush.rs) - onModifySpe: Doubles speed in sandstorm
2. **Triage** (triage.rs) - onModifyPriority: Adds +3 priority to healing moves
3. **Grass Pelt** (grasspelt.rs) - onModifyDef: 1.5x defense in grassy terrain
4. **Swift Swim** (swiftswim.rs) - onModifySpe: Doubles speed in rain/primordial sea
5. **Solar Power** (solarpower.rs) - onModifySpA: 1.5x Sp.Atk in sun, onWeather: Takes damage in sun

### Batch 2 - More Weather/Terrain Effects
6. **Chlorophyll** (chlorophyll.rs) - onModifySpe: Doubles speed in sun
7. **Slush Rush** (slushrush.rs) - onModifySpe: Doubles speed in hail/snow
8. **Surge Surfer** (surgesurfer.rs) - onModifySpe: Doubles speed in electric terrain
9. **Sand Veil** (sandveil.rs) - onModifyAccuracy: Lowers accuracy in sandstorm
10. **Sand Force** (sandforce.rs) - onBasePower: 1.3x power for Rock/Ground/Steel in sandstorm
11. **Ice Body** (icebody.rs) - onWeather: Heals 1/16 HP in hail/snow
12. **Snow Cloak** (snowcloak.rs) - onModifyAccuracy: Lowers accuracy in hail/snow
13. **Rain Dish** (raindish.rs) - onWeather: Heals 1/16 HP in rain (respects Utility Umbrella)

### Batch 3 - Additional Weather Effects
14. **Dry Skin** (dryskin.rs) - onWeather: Heals in rain, takes damage in sun (respects Utility Umbrella)

## Current Session
Implemented 14 abilities across 3 batches.
All implementations are 1-to-1 from JavaScript and compile successfully.
Focus on weather and terrain-based abilities with stat modifications and healing.

## Notes
- Many ability callbacks are already implemented (Blaze, Overgrow, Torrent, Hustle, Pure Power, Huge Power, Fur Coat, etc.)
- Focusing on simple stat modifiers and weather/terrain-based effects first
- More complex abilities requiring infrastructure changes are being documented separately

