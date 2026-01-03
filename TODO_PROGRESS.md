# TODO Progress Tracking

## Summary
- Total ability callback TODOs: 380
- Completed: 163 (42.9%)
- Infrastructure: Major getMoveHitData refactor completed, onModifySTAB infrastructure updated, EffectState.source field added, Volatile status system fully functional, Ability state system (EffectState.data HashMap) confirmed working
- In Progress: Continuing systematic implementation with abilities using existing infrastructure

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

### Batch 33 - Ally Protection & Rage Boost (3 abilities)
118. **Telepathy** (telepathy.rs) - onTryHit: Prevents hitting allies with non-status moves (returns Null to block)
119. **Anger Point** (angerpoint.rs) - onHit: Maximizes Attack (+12 stages) when hit by a critical hit (uses getMoveHitData)
120. **Pressure** (pressure.rs) - onDeductPP: Causes moves to use 1 extra PP when targeting this Pokemon

### Batch 34 - Team Support & Sun Protection (3 abilities)
121. **Friend Guard** (friendguard.rs) - onAnyModifyDamage: Reduces damage to allies by 0.75x (uses effectState.target)
122. **Soul-Heart** (soulheart.rs) - onAnyFaint: Boosts Special Attack by 1 when any Pokemon faints
123. **Leaf Guard** (leafguard.rs) - onSetStatus/onTryAddVolatile: Prevents status and yawn in harsh sunlight (uses Pokemon::effective_weather)

### Batch 35 - STAB Boost & Damage Modification (3 abilities + infrastructure)

**Infrastructure Change:**
Updated onModifySTAB dispatcher infrastructure to properly handle STAB modification:
1. **Dispatcher Signature Update**: Added `stab: f64` parameter to `dispatch_on_modify_s_t_a_b` and all alias functions (`dispatch_on_modify_s_t_a_b_priority`, `dispatch_on_modify_s_t_a_b_order`, `dispatch_on_modify_s_t_a_b_sub_order`)
2. **Event Wiring**: Updated `handle_ability_event.rs` to extract STAB value from `event.relay_var_float` and wire through source/target positions and move ID from event
3. **ActiveMove Access**: Abilities now access `battle.active_move.force_stab` and `battle.active_move.move_type` for STAB calculations

**New Ability Implementations:**
124. **Poison Heal** (poisonheal.rs) - onDamage: Heals 1/8 max HP from poison/toxic instead of taking damage (checks effect.id === "psn" || "tox")
125. **Neuroforce** (neuroforce.rs) - onModifyDamage: 1.25x damage (5120/4096) on super-effective hits (checks move hit data type_mod > 0)
126. **Adaptability** (adaptability.rs) - onModifySTAB: Increases STAB bonus from 1.5x to 2x, or 2x to 2.25x (checks move.forceSTAB || source.hasType(move.type))

### Batch 36 - OHKO Protection & Item Blocking (3 abilities)
127. **Sturdy** (sturdy.rs) - onTryHit: Prevents OHKO moves with immunity message; onDamage: Survives lethal damage at full HP with 1 HP remaining (checks effect.effectType === 'Move')
128. **Unnerve** (unnerve.rs) - onStart: Shows ability activation (one-time via effectState.unnerved flag); onEnd: Resets unnerved flag; onFoeTryEatItem: Prevents foes from eating berries
129. **Wonder Guard** (wonderguard.rs) - onTryHit: Prevents non-super-effective moves from hitting (checks runEffectiveness <= 0, handles skydrop/struggle/status exceptions, respects smartTarget flag)

### Batch 37 - Stat Boost Reversal & Ally Support (3 abilities)
130. **Battery** (battery.rs) - onAllyBasePower: Boosts ally Special moves by 1.3x (5325/4096) when attacker ≠ ability holder (uses effectState.target and active_move.category)
131. **Contrary** (contrary.rs) - onChangeBoost: Reverses all stat changes by multiplying boosts by -1 (skips Z-Power boosts, modifies battle.current_event.relay_var_boost in place)
132. **Intimidate** (intimidate.rs) - onStart: Lowers Attack of all adjacent foes by 1 (shows immunity for Substitute, uses Pokemon::adjacent_foes)

### Batch 38 - Ability Ignoring & Item Protection (3 abilities)
133. **Mold Breaker** (moldbreaker.rs) - onModifyMove: Sets ignore_ability flag to bypass target abilities (already had onStart)
134. **Sticky Hold** (stickyhold.rs) - onTakeItem: Prevents item removal by opponents or Knock Off (allows Sticky Barb removal)
135. **Unaware** (unaware.rs) - onAnyModifyBoost: Zeroes out stat boosts based on active Pokemon/target positions (def/spd/evasion when target is attacking, atk/def/spa/accuracy when target is defending)

### Batch 39 - Accuracy Modification & Stat Doubling (3 abilities)
136. **No Guard** (noguard.rs) - onAnyInvulnerability: Returns 0 when No Guard user is involved (makes Pokemon always hittable); onAnyAccuracy: Returns true when No Guard user is involved (makes moves always hit)
137. **Simple** (simple.rs) - onChangeBoost: Doubles all stat changes by multiplying boosts by 2 (skips Z-Power boosts, modifies battle.current_event.relay_var_boost in place)
138. **Unseen Fist** (unseenfist.rs) - onModifyMove: Removes protect flag from contact moves (sets flags.protect to false)

### Batch 40 - Ability Ignoring Variants (2 abilities)
139. **Turboblaze** (turboblaze.rs) - onModifyMove: Sets ignore_ability flag to bypass target abilities (already had onStart, identical to Mold Breaker)
140. **Teravolt** (teravolt.rs) - onModifyMove: Sets ignore_ability flag to bypass target abilities (already had onStart, identical to Mold Breaker and Turboblaze)

### Batch 41 - Aura & Type Modification (2 abilities)
141. **Aura Break** (aurabreak.rs) - onAnyTryPrimaryHit: Sets has_aura_break flag on move to reverse Dark Aura and Fairy Aura effects (skips self-targeting and Status moves)
142. **Liquid Voice** (liquidvoice.rs) - onModifyType: Changes sound moves to Water type (skips Dynamax Pokemon, checks flags.sound)

### Batch 42 - Ally Support & One-Time Boosts (3 abilities)
143. **Hospitality** (hospitality.rs) - onStart: Heals adjacent allies for 1/4 of their max HP when switching in (uses Pokemon::adjacent_allies)
144. **Dauntless Shield** (dauntlessshield.rs) - onStart: Boosts Defense by 1 once per battle (uses ability_state.data to track with "shieldBoost" flag)
145. **Curious Medicine** (curiousmedicine.rs) - onStart: Clears all stat changes from adjacent allies when switching in (uses Pokemon::clear_boosts)

### Batch 43 - Item Reveal & Ally Power Boost (2 abilities)
146. **Frisk** (frisk.rs) - onStart: Reveals all opposing Pokemon's held items (uses get_all_active and filters by opposing side)
147. **Power Spot** (powerspot.rs) - onAllyBasePower: Boosts ally moves by 1.3x (5325/4096) when attacker ≠ ability holder

### Batch 44 - Type-Changing Abilities (4 abilities)
148. **Refrigerate** (refrigerate.rs) - onModifyType: Changes Normal-type moves to Ice type (1.2x power with typeChangerBoosted tracking); onBasePower: Boosts changed moves by 1.2x (4915/4096)
149. **Aerilate** (aerilate.rs) - onModifyType: Changes Normal-type moves to Flying type (1.2x power with typeChangerBoosted tracking); onBasePower: Boosts changed moves by 1.2x (4915/4096)
150. **Normalize** (normalize.rs) - onModifyType: Changes all moves to Normal type (1.2x power with typeChangerBoosted tracking); onBasePower: Boosts changed moves by 1.2x (4915/4096)
151. **Galvanize** (galvanize.rs) - onModifyType: Changes Normal-type moves to Electric type (1.2x power with typeChangerBoosted tracking); onBasePower: Boosts changed moves by 1.2x (4915/4096)

### Batch 45 - Aura Abilities & Strong Weathers (4 abilities + infrastructure)
152. **Dark Aura** (darkaura.rs) - onStart: Shows ability with suppressingAbility check; onAnyBasePower: Boosts Dark-type moves by 1.33x (5448/4096) or 0.75x (3072/4096) with Aura Break (uses aura_booster tracking)
153. **Fairy Aura** (fairyaura.rs) - onStart: Shows ability with suppressingAbility check; onAnyBasePower: Boosts Fairy-type moves by 1.33x (5448/4096) or 0.75x (3072/4096) with Aura Break (uses aura_booster tracking)
154. **Delta Stream** (deltastream.rs) - onStart: Sets deltastream weather; onAnySetWeather: Prevents non-strong weathers from replacing deltastream; onEnd: Transfers weather source to another Pokemon with Delta Stream or clears weather (uses EffectState.source)
155. **Desolate Land** (desolateland.rs) - onStart: Sets desolateland weather; onAnySetWeather: Prevents non-strong weathers from replacing desolateland; onEnd: Transfers weather source to another Pokemon with Desolate Land or clears weather (uses EffectState.source)

**EffectState.source Infrastructure (Major Change):**
Added `source: Option<(usize, usize)>` field to EffectState struct in src/dex_data.rs to track which Pokemon created an effect. This matches JavaScript's `EffectState.source` which stores the Pokemon that set the weather/terrain. Required for proper strong weather source tracking and hand-off mechanics.

### Batch 46 - Stat Randomization & Type Immunity (2 abilities)
156. **Moody** (moody.rs) - onResidual: Randomly boosts one stat (excluding accuracy/evasion) by 2 and lowers a different stat by 1 (uses BoostID::stats_only and battle.sample for random selection)
157. **Mountaineer** (mountaineer.rs) - onTryHit: Grants immunity to Rock-type moves when Pokemon just switched in (activeTurns == 0), shows immunity message and returns Null to block hit

### Batch 47 - Partial Implementations (1 ability - partial)
158. **Gorilla Tactics** (gorillatactics.rs) - PARTIAL: onModifyAtk: 1.5x Attack boost when not Dynamaxed (remaining handlers need abilityState infrastructure: onStart, onBeforeMove, onModifyMove, onDisableMove, onEnd)

### Batch 48 - Volatile Status Abilities (1 ability)
159. **Flash Fire** (flashfire.rs) - onTryHit: Grants immunity to Fire moves, adds flashfire volatile; onEnd: Removes flashfire volatile; Volatile condition handlers: onStart (shows message), onModifyAtk/onModifySpA (1.5x boost for Fire moves), onEnd (shows silent end message)

### Batch 49 - Fainted Pokemon Tracking (1 ability)
160. **Supreme Overlord** (supremeoverlord.rs) - onStart: Tracks number of fainted teammates (up to 5) in ability_state.data["fallen"]; onBasePower: Boosts power based on fallen count (10%/20%/30%/40%/50% for 1-5 fallen); onEnd: Shows fallen count in end message

### Batch 50 - Secondary Effect Modification (3 abilities)
161. **Serene Grace** (serenegrace.rs) - onModifyMove: Doubles chance of all secondary effects (modifies ActiveMove.secondaries[].chance *= 2)
162. **Skill Link** (skilllink.rs) - onModifyMove: Multi-hit moves always hit maximum times (parses multi_hit_type "2-5" to set multi_hit to 5)
163. **Stench** (stench.rs) - onModifyMove: Adds 10% flinch chance to damaging moves (pushes new SecondaryEffect with volatile_status="flinch" and chance=10)

### Batch 51 - Stat-Based Conditional Boost (1 ability)
164. **Download** (download.rs) - onStart: Compares total Defense vs Special Defense of all foes using battle.get_pokemon_stat(); boosts Special Attack if Defense ≥ Special Defense, otherwise boosts Attack

## Current Session
Completed Flash Fire (Batch 48) using volatile status infrastructure.
Completed Supreme Overlord (Batch 49) using ability_state.data and side.total_fainted.
Completed Serene Grace, Skill Link, and Stench (Batch 50) using ActiveMove.secondaries modification.
Completed Download (Batch 51) using pokemon.foes() and battle.get_pokemon_stat().
Progress: 163/380 abilities (42.9%).
All implementations are 1-to-1 from JavaScript and compile successfully.

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
Based on systematic analysis of remaining TODOs, the following infrastructure is needed (in priority order):

### Critical Infrastructure Needed
1. **Volatile Status System** (37 references) - Highest priority
   - Pokemon.addVolatile(id, source, sourceEffect)
   - Pokemon.removeVolatile(id)
   - Pokemon.hasVolatile(id) / Pokemon.volatiles[id] checking
   - Volatile status effects (Flash Fire boost, Unburden speed, etc.)

2. **Ability State System** (24 references)
   - Pokemon.abilityState.data HashMap for per-ability state
   - Used by: Gorilla Tactics (choiceLock), Berserk (checkedBerserk), etc.

3. **Side Condition System** (10 references)
   - Side.addSideCondition(id, source, sourceEffect)
   - Side.removeSideCondition(id)
   - Side.getSideCondition(id)
   - Conditions: reflect, lightscreen, tailwind, toxicspikes, stealthrock, etc.

4. **Queue/Turn Order System**
   - this.queue.willMove(pokemon) - check if Pokemon will move this turn
   - Used by: Analytic, Stall, and other speed-based abilities

5. **Battle Utility Methods**
   - Pokemon.getLastAttackedBy() - track damage sources
   - Pokemon.isAlly(other) - check if Pokemon are allies
   - this.attrLastMove('[still]') - animation control
   - Pokemon.ignoringAbility() - check if abilities are suppressed

### Implementation Strategy
- Implement volatile status system first (unblocks 37+ abilities)
- Then ability state (unblocks 24+ abilities including completing Gorilla Tactics)
- Then side conditions (unblocks 10+ abilities)
- Queue and utility methods as needed

All infrastructure must be 1-to-1 with JavaScript implementation.

