# TODO Progress Tracking

## Summary
- Total ability callback TODOs: 380
- Completed: 117 (30.8%)
- Infrastructure: Major getMoveHitData refactor completed
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

### Batch 5 - Terrain Setting Abilities (4 abilities)
22. **Electric Surge** (electricsurge.rs) - onStart: Sets electric terrain
23. **Grassy Surge** (grassysurge.rs) - onStart: Sets grassy terrain
24. **Psychic Surge** (psychicsurge.rs) - onStart: Sets psychic terrain
25. **Misty Surge** (mistysurge.rs) - onStart: Sets misty terrain

### Batch 6 - Damaging Hit Abilities (6 abilities)
26. **Seed Sower** (seedsower.rs) - onDamagingHit: Sets grassy terrain when hit
27. **Rock Head** (rockhead.rs) - onDamage: Prevents recoil damage except for Struggle
28. **Sand Spit** (sandspit.rs) - onDamagingHit: Sets sandstorm when hit
29. **Tangling Hair** (tanglinghair.rs) - onDamagingHit: Lowers attacker's Speed on contact moves
30. **Aftermath** (aftermath.rs) - onDamagingHit: Damages attacker 1/4 max HP on contact if target faints
31. **Cotton Down** (cottondown.rs) - onDamagingHit: Lowers Speed of all other active Pokemon when hit

### Batch 7 - Contact & Residual Abilities (6 abilities)
32. **Speed Boost** (speedboost.rs) - onResidual: Boosts Speed by 1 every turn after the first
33. **Steadfast** (steadfast.rs) - onFlinch: Boosts Speed by 1 when Pokemon flinches
34. **Poison Point** (poisonpoint.rs) - onDamagingHit: 30% chance to poison attacker on contact
35. **Static** (static.rs) - onDamagingHit: 30% chance to paralyze attacker on contact
36. **Rough Skin** (roughskin.rs) - onDamagingHit: Damages attacker 1/8 max HP on contact
37. **Iron Barbs** (ironbarbs.rs) - onDamagingHit: Damages attacker 1/8 max HP on contact

### Batch 8 - Type-Based Boost Abilities (6 abilities)
38. **Stamina** (stamina.rs) - onDamagingHit: Boosts Defense by 1 when hit by any damaging move
39. **Regenerator** (regenerator.rs) - onSwitchOut: Heals 1/3 max HP when switching out
40. **Justified** (justified.rs) - onDamagingHit: Boosts Attack by 1 when hit by Dark-type move
41. **Water Compaction** (watercompaction.rs) - onDamagingHit: Boosts Defense by 2 when hit by Water-type move
42. **Steam Engine** (steamengine.rs) - onDamagingHit: Boosts Speed by 6 when hit by Water or Fire-type move
43. **Motor Drive** (motordrive.rs) - onTryHit: Immune to Electric-type moves and boosts Speed by 1

### Batch 9 - Type Immunity & Absorb Abilities (6 abilities)
44. **Sap Sipper** (sapsipper.rs) - onTryHit: Immune to Grass-type moves and boosts Attack by 1
45. **Well-Baked Body** (wellbakedbody.rs) - onTryHit: Immune to Fire-type moves and boosts Defense by 2
46. **Weak Armor** (weakarmor.rs) - onDamagingHit: Lowers Defense by 1 and boosts Speed by 2 when hit by Physical move
47. **Innards Out** (innardsout.rs) - onDamagingHit: Damages attacker by damage taken if target faints
48. **Volt Absorb** (voltabsorb.rs) - onTryHit: Immune to Electric-type moves and heals 1/4 max HP
49. **Water Absorb** (waterabsorb.rs) - onTryHit: Immune to Water-type moves and heals 1/4 max HP

### Batch 10 - More Absorb & Boost Abilities (4 abilities)
50. **Earth Eater** (eartheater.rs) - onTryHit: Immune to Ground-type moves and heals 1/4 max HP
51. **Storm Drain** (stormdrain.rs) - onTryHit: Immune to Water-type moves and boosts Special Attack by 1
52. **Lightning Rod** (lightningrod.rs) - onTryHit: Immune to Electric-type moves and boosts Special Attack by 1
53. **Cheek Pouch** (cheekpouch.rs) - onEatItem: Heals 1/3 max HP when eating an item

### Batch 11 - Contact Poison & Infrastructure Refactor (18 abilities)

**New Ability Implementations:**
54. **Toxic Chain** (toxicchain.rs) - onSourceDamagingHit: 30% chance to badly poison target after damaging them
55. **Poison Touch** (poisontouch.rs) - onSourceDamagingHit: 30% chance to poison target on contact
56. **Stakeout** (stakeout.rs) - onModifyAtk/onModifySpA: Doubles Attack and Special Attack against Pokemon that just switched in (activeTurns == 0)
57. **Liquid Ooze** (liquidooze.rs) - onSourceTryHeal: Reverses drain/leechseed/strengthsap healing into damage

**onTryBoost Infrastructure Refactor (14 abilities fixed):**
This batch included a major infrastructure refactor for `onTryBoost` callbacks. The refactor changed the signature from:
```rust
pub fn on_try_boost(battle: &mut Battle, boost: &str, target_pos, source_pos, effect_id) -> EventResult
```
to:
```rust
pub fn on_try_boost(battle: &mut Battle, target_pos, boost: Option<&mut BoostsTable>) -> EventResult
```

This allows abilities to properly modify boost tables before they're applied, matching the JavaScript implementation exactly.

**Abilities Fixed by Refactor:**
58. **Big Pecks** (bigpecks.rs) - onTryBoost: Prevents Defense drops (except self-inflicted and secondaries/octolock)
59. **Hyper Cutter** (hypercutter.rs) - onTryBoost: Prevents Attack drops (except self-inflicted and secondaries)
60. **Clear Body** (clearbody.rs) - onTryBoost: Prevents all negative boosts (except self-inflicted and secondaries/octolock)
61. **White Smoke** (whitesmoke.rs) - onTryBoost: Prevents all negative boosts (except self-inflicted and secondaries/octolock)
62. **Full Metal Body** (fullmetalbody.rs) - onTryBoost: Prevents all negative boosts (except self-inflicted and secondaries/octolock)
63. **Keen Eye** (keeneye.rs) - onTryBoost: Prevents accuracy drops (except self-inflicted and secondaries)
64. **Illuminate** (illuminate.rs) - onTryBoost: Prevents accuracy drops (except self-inflicted and secondaries)
65. **Mind's Eye** (mindseye.rs) - onTryBoost: Prevents accuracy drops (except self-inflicted and secondaries)
66. **Inner Focus** (innerfocus.rs) - onTryBoost: Prevents Attack drops from Intimidate
67. **Oblivious** (oblivious.rs) - onTryBoost: Prevents Attack drops from Intimidate
68. **Own Tempo** (owntempo.rs) - onTryBoost: Prevents Attack drops from Intimidate
69. **Scrappy** (scrappy.rs) - onTryBoost: Prevents Attack drops from Intimidate
70. **Guard Dog** (guarddog.rs) - onTryBoost: Prevents Attack drops from Intimidate and boosts Attack by 1 instead
71. **Mirror Armor** (mirrorarmor.rs) - onTryBoost: Reflects all negative boosts back to the source (skips if target boost at -6 or source has no HP)

### Batch 12 - Contact Speed Reduction (1 ability)
72. **Gooey** (gooey.rs) - onDamagingHit: Lowers attacker's Speed by 1 on contact moves

### Batch 13 - Contact Burn (1 ability)
73. **Flame Body** (flamebody.rs) - onDamagingHit: 30% chance to burn attacker on contact

### Batch 14 - Type-Based Boosts & Immunities (5 abilities)
74. **Rattled** (rattled.rs) - onDamagingHit: Boosts Speed by 1 when hit by Dark/Bug/Ghost-type move
75. **Thermal Exchange** (thermalexchange.rs) - onDamagingHit: Boosts Attack by 1 when hit by Fire-type move
76. **Dry Skin** (dryskin.rs) - onTryHit: Immune to Water-type moves and heals 1/4 max HP (shows immune message if already at full HP)
77. **Soundproof** (soundproof.rs) - onAllyTryHitSide: Protects allies from sound moves by showing immune message
78. **Overcoat** (overcoat.rs) - onTryHit: Immune to powder moves when target ≠ source

### Batch 15 - Intimidate Response (1 ability)
79. **Rattled** (rattled.rs) - onAfterBoost: Boosts Speed by 1 when Intimidated (completes Rattled implementation)

### Batch 16 - Volatile Status Handling (3 abilities)
80. **Electromorphosis** (electromorphosis.rs) - onDamagingHit: Adds charge volatile when hit
81. **Tangled Feet** (tangledfeet.rs) - onModifyAccuracy: Halves opponent's accuracy when Pokemon is confused
82. **Cute Charm** (cutecharm.rs) - onDamagingHit: 30% chance to attract attacker on contact

### Batch 17 - Advanced Volatile & Boost Handling (2 abilities)
83. **Truant** (truant.rs) - onBeforeMove: Prevents Pokemon from moving every other turn via volatile status
84. **Defiant** (defiant.rs) - onAfterEachBoost: Boosts Attack by 2 when any stat is lowered by an opponent

### Batch 18 - Residual Damage & Move Modification (4 abilities)
85. **Bad Dreams** (baddreams.rs) - onResidual: Damages sleeping foes for 1/8 max HP each turn
86. **Long Reach** (longreach.rs) - onModifyMove: Removes contact flag from all moves
87. **Propeller Tail** (propellertail.rs) - onModifyMove: Sets tracksTarget to ignore redirection
88. **Illuminate** (illuminate.rs) - onModifyMove: Sets ignoreEvasion to bypass evasion boosts

### Batch 19 - Ignore Immunity Infrastructure & Abilities (3 abilities + major refactor)

**Infrastructure Change:**
Created new `IgnoreImmunity` enum in `src/battle_actions.rs` to properly represent JavaScript's union type `boolean | { [k: string]: boolean }`:
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IgnoreImmunity {
    /// Ignore all type immunities (true)
    All,
    /// Ignore specific type immunities ({ Type: true, ... })
    Specific(HashMap<String, bool>),
}
```
Changed `ActiveMove.ignore_immunity` field from `Option<bool>` to `Option<IgnoreImmunity>`.
Updated `hit_step_type_immunity.rs` to use the new enum.

**New Ability Implementations:**
89. **Keen Eye** (keeneye.rs) - onModifyMove: Sets ignoreEvasion to bypass evasion boosts
90. **Scrappy** (scrappy.rs) - onModifyMove: Sets ignoreImmunity for Fighting and Normal types to hit Ghost types
91. **Mind's Eye** (mindseye.rs) - onModifyMove: Sets ignoreEvasion and ignoreImmunity for Fighting/Normal types

### Batch 20 - Move Targeting & Infiltration (2 abilities)
92. **Stalwart** (stalwart.rs) - onModifyMove: Sets tracksTarget based on move.target (ignores redirection unless 'scripted')
93. **Infiltrator** (infiltrator.rs) - onModifyMove: Sets infiltrates flag to bypass Substitute/screens

### Batch 21 - Priority Modification (2 abilities)
94. **Gale Wings** (galewings.rs) - onModifyPriority: Adds +1 priority to Flying-type moves when at full HP
95. **Prankster** (prankster.rs) - onModifyPriority: Adds +1 priority to Status moves and sets pranksterBoosted flag

## Infrastructure Batch - getMoveHitData System

**Major Infrastructure Refactor:**
Implemented proper getMoveHitData infrastructure to match JavaScript's per-target hit data tracking system.

**Changes Made:**
1. **ActiveMove.move_hit_data**: Changed from `Option<MoveHitData>` to `HashMap<String, MoveHitData>` to store per-slot hit data
2. **Battle::get_move_hit_data()**: Added immutable accessor method
3. **Battle::get_move_hit_data_mut()**: Added mutable accessor method that creates entries on-demand
4. **modify_damage.rs**: Updated to store type_mod in MoveHitData after calculating type effectiveness
5. **get_active_move.rs**: Updated initialization to use HashMap::new()

**Why This Change Was Needed:**
JavaScript's getMoveHitData() stores hit data (crit, typeMod, zBrokeProtect) per target slot. This is essential for:
- Spread moves that hit multiple targets (each target has different type effectiveness)
- Abilities that check type effectiveness (Filter, Solid Rock, Prism Armor reduce super-effective damage)
- Tracking per-target battle state during move execution

**Files Modified:**
- `src/battle_actions.rs` - Changed move_hit_data field type
- `src/battle/get_move_hit_data.rs` - New Battle methods for accessing hit data
- `src/battle.rs` - Added module declaration
- `src/battle_actions/modify_damage.rs` - Store typeMod in hit data, fix borrow checker issues
- `src/dex/get_active_move.rs` - Initialize with HashMap::new()
- `src/pokemon/get_move_hit_data.rs` - Updated documentation

This infrastructure enables implementation of damage-reduction abilities (Filter, Solid Rock, Prism Armor) and many other abilities that need access to move hit data.

### Batch 22 - Super-Effective Damage Reduction (3 abilities)
96. **Filter** (filter.rs) - onSourceModifyDamage: Reduces super-effective damage by 25% (0.75x modifier)
97. **Solid Rock** (solidrock.rs) - onSourceModifyDamage: Reduces super-effective damage by 25% (0.75x modifier)
98. **Prism Armor** (prismarmor.rs) - onSourceModifyDamage: Reduces super-effective damage by 25% (0.75x modifier)

### Batch 23 - Ally Synergy (2 abilities)
99. **Plus** (plus.rs) - onModifySpA: 1.5x Special Attack when ally has Plus or Minus ability
100. **Minus** (minus.rs) - onModifySpA: 1.5x Special Attack when ally has Plus or Minus ability

### Batch 24 - Base Power Modification (2 abilities)
101. **Technician** (technician.rs) - onBasePower: 1.5x boost when base power (after modifier) ≤ 60
102. **Reckless** (reckless.rs) - onBasePower: 1.2x boost (4915/4096) for moves with recoil or crash damage

### Batch 25 - Electric Terrain Synergy (1 ability)
103. **Hadron Engine** (hadronengine.rs) - onStart: Sets electric terrain or shows activate message if already active; onModifySpA: 1.33x Special Attack in electric terrain (5461/4096)

### Batch 26 - Burn Immunity (1 ability)
104. **Water Bubble** (waterbubble.rs) - Already had stat modifiers implemented; Added onUpdate: Cures burn status automatically; onSetStatus: Prevents burn with immunity message

### Batch 27 - Stat Debuff Aura (1 ability)
105. **Sword of Ruin** (swordofruin.rs) - onStart: Shows ability activation (respects suppressingAbility); onAnyModifyDef: Reduces opponents' Defense by 0.75x (uses effectState.target and ActiveMove.ruined_def tracking)

### Batch 28 - Stat Debuff Aura Family (3 abilities)
106. **Tablets of Ruin** (tabletsofruin.rs) - onStart: Shows ability activation; onAnyModifyAtk: Reduces opponents' Attack by 0.75x
107. **Vessel of Ruin** (vesselofruin.rs) - onStart: Shows ability activation; onAnyModifySpA: Reduces opponents' Special Attack by 0.75x
108. **Beads of Ruin** (beadsofruin.rs) - onStart: Shows ability activation; onAnyModifySpD: Reduces opponents' Special Defense by 0.75x

### Batch 29 - Best Stat Boost (1 ability)
109. **Beast Boost** (beastboost.rs) - onSourceAfterFaint: Boosts highest stat by 1 when KOing with a move (uses battle.get_pokemon_stat to calculate best stat)

### Batch 30 - Damage Modifiers & Competitive Spirit (3 abilities)
110. **Sniper** (sniper.rs) - onModifyDamage: 1.5x damage on critical hits (checks move hit data crit flag)
111. **Tinted Lens** (tintedlens.rs) - onModifyDamage: 2x damage on not-very-effective hits (checks move hit data type_mod < 0)
112. **Competitive** (competitive.rs) - onAfterEachBoost: Boosts Special Attack by 2 when any stat is lowered by an opponent

### Batch 31 - Status Cure Abilities (2 abilities)
113. **Hydration** (hydration.rs) - onResidual: Cures status in rain/primordial sea (uses Pokemon::effective_weather and Pokemon::cure_status)
114. **Healer** (healer.rs) - onResidual: 30% chance to cure adjacent ally's status (uses Pokemon::adjacent_allies)

### Batch 32 - Damage Prevention & Team Support (3 abilities)
115. **Magic Guard** (magicguard.rs) - onDamage: Prevents all non-move damage (checks if effect is a move by looking up in dex)
116. **Victory Star** (victorystar.rs) - onAnyModifyAccuracy: Boosts ally accuracy by 1.1x (4506/4096)
117. **Gluttony** (gluttony.rs) - onStart/onDamage: Sets gluttony flag in ability state for early berry consumption

## Current Session
Completed major getMoveHitData infrastructure refactor.
Implemented 22 abilities (batches 22-32).
Progress: 117/380 abilities (30.8%).
All implementations are 1-to-1 from JavaScript and compile successfully.
Completed entire Ruin ability family using battle.effect_state.target and ActiveMove.ruined_* fields for proper multi-ability coordination.
Completed Beast Boost using inline stat calculation to avoid borrow checker issues.
Completed damage modifier abilities using battle.get_move_hit_data() to access critical hit and type effectiveness data.
Completed residual status cure abilities using Pokemon::effective_weather and Pokemon::adjacent_allies for weather checking and ally iteration.
Completed Magic Guard using effect type detection via dex lookups.

## Implementation Notes
- Using `battle.boost()` for stat boosts (Attack, Special Attack, Speed, Defense, etc.)
- Using `battle.field.set_weather()` for weather-setting abilities
- Using `battle.field.set_terrain()` for terrain-setting abilities
- Using `battle.check_move_makes_contact()` to check for contact moves
- Using `battle.get_all_active()` to iterate through all active Pokemon
- Using `battle.damage()` to deal damage to Pokemon
- Using `battle.heal()` to heal Pokemon
- Using `battle.random_chance()` for probability checks
- Using `Pokemon::try_set_status()` to apply status conditions
- Using `move_data.move_type` to check move types for type-based abilities
- Using `move_data.category` to check move category (Physical/Special/Status)
- Using `EventResult::Null` to prevent moves from hitting (immunity)
- Properly handling special cases (Kyogre/Blue Orb, Groudon/Red Orb)
- All implementations follow Rust borrow checker patterns with two-phase access

## Next Steps
- Continue with more boost-based abilities (Beast Boost, Soul-Heart, etc.)
- Implement type-changing abilities (Aerilate, Refrigerate, etc.)
- Implement abilities requiring complex infrastructure (move modification, forme changes)

