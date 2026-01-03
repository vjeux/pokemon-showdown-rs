# TODO Progress Tracking

## Summary
- Total ability callback TODOs: 380
- Completed: 21 (5.5%)
- In Progress: Continuing systematic implementation

## Completed Implementations

### Batch 1 - Basic Weather/Terrain Modifiers (5 abilities)
1. **Sand Rush** (sandrush.rs) - onModifySpe: Doubles speed in sandstorm
2. **Triage** (triage.rs) - onModifyPriority: Adds +3 priority to healing moves
3. **Grass Pelt** (grasspelt.rs) - onModifyDef: 1.5x defense in grassy terrain
4. **Swift Swim** (swiftswim.rs) - onModifySpe: Doubles speed in rain/primordial sea
5. **Solar Power** (solarpower.rs) - onModifySpA: 1.5x Sp.Atk in sun, onWeather: Takes damage in sun

### Batch 2 - More Weather/Terrain Effects (8 abilities)
6. **Chlorophyll** (chlorophyll.rs) - onModifySpe: Doubles speed in sun
7. **Slush Rush** (slushrush.rs) - onModifySpe: Doubles speed in hail/snow
8. **Surge Surfer** (surgesurfer.rs) - onModifySpe: Doubles speed in electric terrain
9. **Sand Veil** (sandveil.rs) - onModifyAccuracy: Lowers opponent accuracy in sandstorm
10. **Sand Force** (sandforce.rs) - onBasePower: 1.3x power for Rock/Ground/Steel in sandstorm
11. **Ice Body** (icebody.rs) - onWeather: Heals 1/16 HP in hail/snow
12. **Snow Cloak** (snowcloak.rs) - onModifyAccuracy: Lowers opponent accuracy in hail/snow
13. **Rain Dish** (raindish.rs) - onWeather: Heals 1/16 HP in rain (respects Utility Umbrella)

### Batch 3 - Additional Weather Effects (1 ability)
14. **Dry Skin** (dryskin.rs) - onWeather: Heals in rain, takes damage in sun (respects Utility Umbrella)

### Batch 4 - Boost & Weather Setting Abilities (7 abilities)
15. **Moxie** (moxie.rs) - onSourceAfterFaint: Boosts Attack by 1 when KOing with a move
16. **Chilling Neigh** (chillingneigh.rs) - onSourceAfterFaint: Boosts Attack by 1 when KOing with a move
17. **Grim Neigh** (grimneigh.rs) - onSourceAfterFaint: Boosts Special Attack by 1 when KOing with a move
18. **Drizzle** (drizzle.rs) - onStart: Sets rain (except Kyogre with Blue Orb)
19. **Drought** (drought.rs) - onStart: Sets sun (except Groudon with Red Orb)
20. **Snow Warning** (snowwarning.rs) - onStart: Sets snow
21. **Sand Stream** (sandstream.rs) - onStart: Sets sandstorm

## Current Session
Implemented 21 abilities across 4 batches.
All implementations are 1-to-1 from JavaScript and compile successfully.

## Implementation Notes
- Using `battle.boost()` for stat boosts (Attack, Special Attack, etc.)
- Using `battle.field.set_weather()` for weather-setting abilities
- Properly handling special cases (Kyogre/Blue Orb, Groudon/Red Orb)
- All implementations follow Rust borrow checker patterns with two-phase access

## Next Steps
- Continue with more boost-based abilities (Beast Boost, Soul-Heart, etc.)
- Implement type-changing abilities (Aerilate, Refrigerate, etc.)
- Implement abilities requiring complex infrastructure (move modification, forme changes)

