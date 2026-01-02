# JavaScript Type Definitions from Pokemon Showdown (sim/ folder)

Generated: 2026-01-02T15:42:31.272Z
Source: ../pokemon-showdown-ts/sim/
Total types: 217

## Table of Contents

- [Classes](#classes) (35)
- [Interfaces](#interfaces) (95)
- [Type Aliases](#type-aliases) (87)

---

## Classes

### BasicEffect

**File:** `sim/dex-data.ts`

**Implements:** EffectData

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| id | `ID` |  |  | public |
| name | `string` |  |  | public |
| fullname | `string` |  |  | public |
| effectType | `EffectType` |  |  | public |
| exists | `boolean` |  |  | public |
| num | `number` |  |  | public |
| gen | `number` |  |  | public |
| shortDesc | `string` |  |  | public |
| desc | `string` |  |  | public |
| isNonstandard | `Nonstandard | null` |  |  | public |
| duration | `number` | ✓ |  | public |
| noCopy | `boolean` |  |  | public |
| affectsFainted | `boolean` |  |  | public |
| status | `ID` | ✓ |  | public |
| weather | `ID` | ✓ |  | public |
| sourceEffect | `string` |  |  | public |

---

### Battle

**File:** `sim/battle.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| id | `ID` |  | ✓ | public |
| debugMode | `boolean` |  | ✓ | public |
| forceRandomChance | `boolean | null` |  | ✓ | public |
| deserialized | `boolean` |  | ✓ | public |
| strictChoices | `boolean` |  | ✓ | public |
| format | `Format` |  | ✓ | public |
| formatData | `EffectState` |  | ✓ | public |
| gameType | `GameType` |  | ✓ | public |
| activePerHalf | `1 | 2 | 3` |  | ✓ | public |
| field | `Field` |  | ✓ | public |
| sides | `[Side, Side] | [Side, Side, Side, Side]` |  | ✓ | public |
| prngSeed | `PRNGSeed` |  | ✓ | public |
| dex | `ModdedDex` |  |  | public |
| gen | `number` |  |  | public |
| ruleTable | `Dex.RuleTable` |  |  | public |
| prng | `PRNG` |  |  | public |
| rated | `boolean | string` |  |  | public |
| reportExactHP | `boolean` |  |  | public |
| reportPercentages | `boolean` |  |  | public |
| supportCancel | `boolean` |  |  | public |
| actions | `BattleActions` |  |  | public |
| queue | `BattleQueue` |  |  | public |
| faintQueue | `{` |  | ✓ | public |
| target | `Pokemon,` |  |  | public |
| source | `Pokemon | null,` |  |  | public |
| effect | `Effect | null,` |  |  | public |
| log | `string[]` |  | ✓ | public |
| inputLog | `string[]` |  | ✓ | public |
| messageLog | `string[]` |  | ✓ | public |
| sentLogPos | `number` |  |  | public |
| sentEnd | `boolean` |  |  | public |
| requestState | `RequestState` |  |  | public |
| turn | `number` |  |  | public |
| midTurn | `boolean` |  |  | public |
| started | `boolean` |  |  | public |
| ended | `boolean` |  |  | public |
| winner | `string` | ✓ |  | public |
| effect | `Effect` |  |  | public |
| effectState | `EffectState` |  |  | public |
| event | `AnyObject` |  |  | public |
| events | `AnyObject | null` |  |  | public |
| eventDepth | `number` |  |  | public |
| activeMove | `ActiveMove | null` |  |  | public |
| activePokemon | `Pokemon | null` |  |  | public |
| activeTarget | `Pokemon | null` |  |  | public |
| lastMove | `ActiveMove | null` |  |  | public |
| lastSuccessfulMoveThisTurn | `ID | null` |  |  | public |
| lastMoveLine | `number` |  |  | public |
| lastDamage | `number` |  |  | public |
| effectOrder | `number` |  |  | public |
| quickClawRoll | `boolean` |  |  | public |
| speedOrder | `number[]` |  |  | public |
| teamGenerator | `ReturnType<typeof Teams.getGenerator> | null` |  |  | public |
| hints | `Set<string>` |  | ✓ | public |
| NOT_FAIL | `''` |  | ✓ | public |
| HIT_SUBSTITUTE | `0` |  | ✓ | public |
| FAIL | `false` |  | ✓ | public |
| SILENT_FAIL | `null` |  | ✓ | public |
| formatid | `options.formatid, seed: this.prngSeed,` |  |  | public |
| eventid | `string, effect: Effect, state: EffectState | Record<string, never> | null,` |  |  | public |
| target | `string | Pokemon | Side | Field | Battle | null, source?: string | Pokemon | Effect | false | null,` |  |  | public |
| sourceEffect | `Effect | string | null, relayVar?: any, customCallback?: unknown` | ✓ |  | public |
| eventid | `string, target?: Pokemon | Pokemon[] | Side | Battle | null, source?: string | Pokemon | false | null,` |  |  | public |
| sourceEffect | `Effect | null, relayVar?: any, onEffect?: boolean, fastExit?: boolean` | ✓ |  | public |
| BeforeMove | `1,` |  |  | public |
| BasePower | `1,` |  |  | public |
| Immunity | `1,` |  |  | public |
| RedirectTarget | `1,` |  |  | public |
| Heal | `1,` |  |  | public |
| SetStatus | `1,` |  |  | public |
| CriticalHit | `1,` |  |  | public |
| ModifyAtk | `1, ModifyDef: 1, ModifySpA: 1, ModifySpD: 1, ModifySpe: 1, ModifyAccuracy: 1,` |  |  | public |
| ModifyBoost | `1,` |  |  | public |
| ModifyDamage | `1,` |  |  | public |
| ModifySecondaries | `1,` |  |  | public |
| ModifyWeight | `1,` |  |  | public |
| TryAddVolatile | `1,` |  |  | public |
| TryHit | `1,` |  |  | public |
| TryHitSide | `1,` |  |  | public |
| TryMove | `1,` |  |  | public |
| Boost | `1,` |  |  | public |
| DragOut | `1,` |  |  | public |
| Effectiveness | `1,` |  |  | public |
| eventid | `string, target: Pokemon | Side | Battle, source?: Pokemon | null,` |  |  | public |
| effect | `Effect, relayVar?: any, onEffect?: boolean` | ✓ |  | public |
| Condition | `2,` |  |  | public |
| Weather | `5,` |  |  | public |
| Format | `5,` |  |  | public |
| Rule | `5,` |  |  | public |
| Ruleset | `5,` |  |  | public |
| Ability | `7,` |  |  | public |
| Item | `8,` |  |  | public |
| effect | `status, callback, state: pokemon.statusState, end: pokemon.clearStatus, effectHolder: pokemon,` |  |  | public |
| effect | `volatile, callback, state: volatileState, end: pokemon.removeVolatile, effectHolder: pokemon,` |  |  | public |
| effect | `ability, callback, state: pokemon.abilityState, end: pokemon.clearAbility, effectHolder: pokemon,` |  |  | public |
| effect | `item, callback, state: pokemon.itemState, end: pokemon.clearItem, effectHolder: pokemon,` |  |  | public |
| effect | `slotCondition,` |  |  | public |
| state | `slotConditionState,` |  |  | public |
| end | `side.removeSlotCondition,` |  |  | public |
| endCallArgs | `[side, pokemon, slotCondition.id],` |  |  | public |
| effectHolder | `pokemon,` |  |  | public |
| effect | `format, callback, state: this.formatData, end: null, effectHolder: customHolder || this,` |  |  | public |
| effect | `handler.target, callback: handler.callback, state, end: null, effectHolder: customHolder || this,` |  |  | public |
| priority | `handler.priority, order: handler.order, subOrder: handler.subOrder,` |  |  | public |
| effect | `pseudoWeather, callback, state: pseudoWeatherState,` |  |  | public |
| end | `customHolder ? null : field.removePseudoWeather, effectHolder: customHolder || field,` |  |  | public |
| effect | `weather, callback, state: this.field.weatherState,` |  |  | public |
| end | `customHolder ? null : field.clearWeather, effectHolder: customHolder || field,` |  |  | public |
| effect | `terrain, callback, state: field.terrainState,` |  |  | public |
| end | `customHolder ? null : field.clearTerrain, effectHolder: customHolder || field,` |  |  | public |
| effect | `sideCondition, callback, state: sideConditionData,` |  |  | public |
| end | `customHolder ? null : side.removeSideCondition, effectHolder: customHolder || side,` |  |  | public |
| boost | `SparseBoostsTable, target: Pokemon | null` |  |  | public |
| effect | `Effect | null` |  |  | public |
| source | `Pokemon | null` |  |  | public |
| damage | `number, target: Pokemon | null` |  |  | public |
| effect | `'drain' | 'recoil' | Effect | null` |  |  | public |
| move | `move.name,` |  |  | public |
| id | `move.id,` |  |  | public |
| pp | `move.noPPBoosts ? move.pp : move.pp * 8 / 5,` |  |  | public |
| maxpp | `move.noPPBoosts ? move.pp : move.pp * 8 / 5,` |  |  | public |
| target | `move.target,` |  |  | public |
| disabled | `false,` |  |  | public |
| disabledSource | `'',` |  |  | public |
| used | `false,` |  |  | public |
| sourceEffect | `action.sourceEffect, zMove: action.zmove,` |  |  | public |
| maxMove | `action.maxMove, originalTarget: action.originalTarget,` |  |  | public |
| choice | `'instaswitch',` |  |  | public |
| pokemon | `action.target,` |  |  | public |
| target | `action.target,` |  |  | public |
| name | `'',` |  |  | public |
| species | `set.species,` |  |  | public |
| item | `set.item,` |  |  | public |
| ability | `set.ability,` |  |  | public |
| moves | `set.moves,` |  |  | public |
| nature | `'',` |  |  | public |
| gender | `pokemon.gender,` |  |  | public |
| evs | `null!,` |  |  | public |
| ivs | `null!,` |  |  | public |
| level | `set.level,` |  |  | public |
| winner | `this.winner,` |  |  | public |
| seed | `this.prngSeed,` |  |  | public |
| turns | `this.turn,` |  |  | public |
| p1 | `this.sides[0].name,` |  |  | public |
| p2 | `this.sides[1].name,` |  |  | public |
| p3 | `this.sides[2]?.name,` |  |  | public |
| p4 | `this.sides[3]?.name,` |  |  | public |
| p1team | `this.sides[0].team,` |  |  | public |
| p2team | `this.sides[1].team,` |  |  | public |
| p3team | `this.sides[2]?.team,` |  |  | public |
| p4team | `this.sides[3]?.team,` |  |  | public |
| score | `[this.sides[0].pokemonLeft, this.sides[1].pokemonLeft],` |  |  | public |
| inputLog | `this.inputLog,` |  |  | public |

---

### BattleActions

**File:** `sim/battle-actions.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| battle | `Battle` |  |  | public |
| dex | `ModdedDex` |  |  | public |
| MAX_MOVES | `{ readonly [k: string]: string }` |  | ✓ | public |
| Flying | `'Max Airstream',` |  |  | public |
| Dark | `'Max Darkness',` |  |  | public |
| Fire | `'Max Flare',` |  |  | public |
| Bug | `'Max Flutterby',` |  |  | public |
| Water | `'Max Geyser',` |  |  | public |
| Status | `'Max Guard',` |  |  | public |
| Ice | `'Max Hailstorm',` |  |  | public |
| Fighting | `'Max Knuckle',` |  |  | public |
| Electric | `'Max Lightning',` |  |  | public |
| Psychic | `'Max Mindstorm',` |  |  | public |
| Poison | `'Max Ooze',` |  |  | public |
| Grass | `'Max Overgrowth',` |  |  | public |
| Ghost | `'Max Phantasm',` |  |  | public |
| Ground | `'Max Quake',` |  |  | public |
| Rock | `'Max Rockfall',` |  |  | public |
| Fairy | `'Max Starfall',` |  |  | public |
| Steel | `'Max Steelspike',` |  |  | public |
| Normal | `'Max Strike',` |  |  | public |
| Dragon | `'Max Wyrmwind',` |  |  | public |
| Z_MOVES | `{ readonly [k: string]: string }` |  | ✓ | public |
| Poison | `"Acid Downpour",` |  |  | public |
| Fighting | `"All-Out Pummeling",` |  |  | public |
| Dark | `"Black Hole Eclipse",` |  |  | public |
| Grass | `"Bloom Doom",` |  |  | public |
| Normal | `"Breakneck Blitz",` |  |  | public |
| Rock | `"Continental Crush",` |  |  | public |
| Steel | `"Corkscrew Crash",` |  |  | public |
| Dragon | `"Devastating Drake",` |  |  | public |
| Electric | `"Gigavolt Havoc",` |  |  | public |
| Water | `"Hydro Vortex",` |  |  | public |
| Fire | `"Inferno Overdrive",` |  |  | public |
| Ghost | `"Never-Ending Nightmare",` |  |  | public |
| Bug | `"Savage Spin-Out",` |  |  | public |
| Psychic | `"Shattered Psyche",` |  |  | public |
| Ice | `"Subzero Slammer",` |  |  | public |
| Flying | `"Supersonic Skystrike",` |  |  | public |
| Ground | `"Tectonic Rage",` |  |  | public |
| Fairy | `"Twinkle Tackle",` |  |  | public |
| moveOrMoveName | `Move | string, pokemon: Pokemon, targetLoc: number,` |  |  | public |
| options | `{` | ✓ |  | public |
| sourceEffect | `Effect | null, zMove?: string, externalMove?: boolean,` | ✓ |  | public |
| maxMove | `string, originalTarget?: Pokemon,` | ✓ |  | public |
| move | `Move | string, pokemon: Pokemon, options?: {` |  |  | public |
| target | `Pokemon | null, sourceEffect?: Effect | null,` | ✓ |  | public |
| zMove | `string, maxMove?: string,` | ✓ |  | public |
| moveOrMoveName | `Move | string, pokemon: Pokemon, options?: {` |  |  | public |
| target | `Pokemon | null, sourceEffect?: Effect | null,` | ✓ |  | public |
| zMove | `string, maxMove?: string,` | ✓ |  | public |
| targets | `SpreadMoveTargets, pokemon: Pokemon, moveOrMoveName: ActiveMove,` |  |  | public |
| hitEffect | `Dex.HitEffect, isSecondary?: boolean, isSelf?: boolean` | ✓ |  | public |
| damage | `SpreadMoveDamage, targets: SpreadMoveTargets, pokemon: Pokemon,` |  |  | public |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean` |  |  | public |
| damage | `SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,` |  |  | public |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean` |  |  | public |
| damage | `SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,` |  |  | public |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean` |  |  | public |
| targets | `SpreadMoveTargets, source: Pokemon,` |  |  | public |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean` |  |  | public |
| damage | `SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove` |  |  | public |
| moveData | `Dex.HitEffect, isSecondary?: boolean, isSelf?: boolean` | ✓ |  | public |
| left | `T, right: U` |  |  | public |
| source | `Pokemon, target: Pokemon, move: string | number | ActiveMove,` |  |  | public |
| type | `'???',` |  |  | public |
| category | `'Physical',` |  |  | public |
| willCrit | `false,` |  |  | public |
| baseDamage | `number, pokemon: Pokemon, target: Pokemon, move: ActiveMove, suppressMessages` |  |  | public |

---

### BattlePlayer

**File:** `sim/battle-stream.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| stream | `Streams.ObjectReadWriteStream<string>` |  | ✓ | public |
| log | `string[]` |  | ✓ | public |
| debug | `boolean` |  | ✓ | public |

---

### BattleQueue

**File:** `sim/battle-queue.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| battle | `Battle` |  |  | public |
| list | `Action[]` |  |  | public |
| team | `1,` |  |  | public |
| start | `2,` |  |  | public |
| instaswitch | `3,` |  |  | public |
| beforeTurn | `4,` |  |  | public |
| beforeTurnMove | `5,` |  |  | public |
| revivalblessing | `6,` |  |  | public |
| runSwitch | `101,` |  |  | public |
| switch | `103,` |  |  | public |
| megaEvo | `104,` |  |  | public |
| megaEvoX | `104,` |  |  | public |
| megaEvoY | `104,` |  |  | public |
| runDynamax | `105,` |  |  | public |
| terastallize | `106,` |  |  | public |
| priorityChargeMove | `107,` |  |  | public |
| shift | `200,` |  |  | public |
| residual | `300,` |  |  | public |
| choice | `'beforeTurnMove', pokemon: action.pokemon, move: action.move, targetLoc: action.targetLoc,` |  |  | public |
| choice | `'megaEvo',` |  |  | public |
| pokemon | `action.pokemon,` |  |  | public |
| choice | `'megaEvoX',` |  |  | public |
| pokemon | `action.pokemon,` |  |  | public |
| choice | `'megaEvoY',` |  |  | public |
| pokemon | `action.pokemon,` |  |  | public |
| choice | `'terastallize',` |  |  | public |
| pokemon | `action.pokemon,` |  |  | public |
| choice | `'runDynamax',` |  |  | public |
| pokemon | `action.pokemon,` |  |  | public |
| choice | `'priorityChargeMove',` |  |  | public |
| pokemon | `action.pokemon,` |  |  | public |
| move | `action.move,` |  |  | public |

---

### BattleTextStream

**File:** `sim/battle-stream.ts`

**Extends:** Streams.ReadWriteStream

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| battleStream | `BattleStream` |  | ✓ | public |
| currentMessage | `string` |  |  | public |

---

### CoordinatedPlayerAI

**File:** `sim/tools/exhaustive-runner.ts`

**Extends:** RandomPlayerAI

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| active | `AnyObject | undefined, switches: { slot: number, pokemon: AnyObject }[]` |  |  | public |

---

### DexAbilities

**File:** `sim/dex-abilities.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| allCache | `readonly Ability[] | null` |  |  | public |
| name | `id,` |  |  | public |

---

### DexConditions

**File:** `sim/dex-conditions.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |

---

### DexFormats

**File:** `sim/dex-formats.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| formatsListCache | `readonly Format[] | null` |  |  | public |
| searchShow | `false,` |  |  | public |
| default | `const [ruleName, value]` |  |  | public |

---

### DexItems

**File:** `sim/dex-items.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| allCache | `readonly Item[] | null` |  |  | public |
| name | `id,` |  |  | public |

---

### DexMoves

**File:** `sim/dex-moves.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| allCache | `readonly Move[] | null` |  |  | public |
| name | `id,` |  |  | public |
| name | `id, exists: false,` |  |  | public |

---

### DexNatures

**File:** `sim/dex-data.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| allCache | `readonly Nature[] | null` |  |  | public |

---

### DexSpecies

**File:** `sim/dex-species.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| allCache | `readonly Species[] | null` |  |  | public |
| name | `id,` |  |  | public |
| name | `species.baseSpecies + '-' + cosmeticForme.forme!, // Forme always exists on cosmetic forme entries` |  |  | public |
| baseForme | `"",` |  |  | public |
| otherFormes | `null,` |  |  | public |
| cosmeticFormes | `null,` |  |  | public |
| name | `forme,` |  |  | public |
| baseSpecies | `species.name,` |  |  | public |
| baseForme | `"",` |  |  | public |
| isCosmeticForme | `true,` |  |  | public |
| otherFormes | `null,` |  |  | public |
| cosmeticFormes | `null,` |  |  | public |
| alola | `['a', 'alola', 'alolan'],` |  |  | public |
| galar | `['g', 'galar', 'galarian'],` |  |  | public |
| hisui | `['h', 'hisui', 'hisuian'],` |  |  | public |
| paldea | `['p', 'paldea', 'paldean'],` |  |  | public |
| mega | `['m', 'mega'],` |  |  | public |
| primal | `['p', 'primal'],` |  |  | public |
| tags | `baseSpeciesTags,` |  |  | public |
| exists | `false, tier: 'Illegal', doublesTier: 'Illegal', natDexTier: 'Illegal', isNonstandard: 'Custom',` |  |  | public |

---

### DexStats

**File:** `sim/dex-data.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| shortNames | `{ readonly [k in StatID]: string }` |  | ✓ | public |
| mediumNames | `{ readonly [k in StatID]: string }` |  | ✓ | public |
| names | `{ readonly [k in StatID]: string }` |  | ✓ | public |
| __proto__ | `null, hp: "HP", atk: "Atk", def: "Def", spa: "SpA", spd: "SpD", spe: "Spe",` |  |  | public |
| __proto__ | `null, hp: "HP", atk: "Attack", def: "Defense", spa: "Sp. Atk", spd: "Sp. Def", spe: "Speed",` |  |  | public |
| __proto__ | `null, hp: "HP", atk: "Attack", def: "Defense", spa: "Special Attack", spd: "Special Defense", spe: "Speed",` |  |  | public |
| __proto__ | `null, hp: "HP", atk: "Atk", def: "Def", spa: "Spc", spd: "[SpD]", spe: "Spe",` |  |  | public |
| __proto__ | `null, hp: "HP", atk: "Attack", def: "Defense", spa: "Special", spd: "[Sp. Def]", spe: "Speed",` |  |  | public |
| __proto__ | `null, hp: "HP", atk: "Attack", def: "Defense", spa: "Special", spd: "[Special Defense]", spe: "Speed",` |  |  | public |

---

### DexTypes

**File:** `sim/dex-data.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| allCache | `readonly TypeInfo[] | null` |  |  | public |
| namesCache | `readonly string[] | null` |  |  | public |

---

### DualStream

**File:** `sim/tools/runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| debug | `boolean` |  |  | private |
| test | `RawBattleStream` |  |  | private |

---

### ExhaustiveRunner

**File:** `sim/tools/exhaustive-runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| failures | `number` |  |  | private |
| games | `number` |  |  | private |
| prng | `this.prng,` |  |  | public |
| format | `this.format,` |  |  | public |
| dual | `this.dual,` |  |  | public |
| error | `true,` |  |  | public |

---

### Field

**File:** `sim/field.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| battle | `Battle` |  | ✓ | public |
| id | `ID` |  | ✓ | public |
| weather | `ID` |  |  | public |
| weatherState | `EffectState` |  |  | public |
| terrain | `ID` |  |  | public |
| terrainState | `EffectState` |  |  | public |
| pseudoWeather | `{ [id: string]: EffectState }` |  |  | public |
| id | `status.id,` |  |  | public |
| duration | `status.duration,` |  |  | public |
| status | `string | Condition,` |  |  | public |
| source | `Pokemon | 'debug' | null` |  |  | public |
| sourceEffect | `Effect | null` |  |  | public |
| id | `status.id,` |  |  | public |
| duration | `status.duration,` |  |  | public |

---

### Gen3RNG

**File:** `sim/prng.ts`

**Implements:** RNG

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| seed | `number` |  |  | public |

---

### Gen5RNG

**File:** `sim/prng.ts`

**Implements:** RNG

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| seed | `Gen5RNGSeed` |  |  | public |

---

### Learnset

**File:** `sim/dex-species.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| effectType | `'Learnset'` |  | ✓ | public |
| learnset | `{ [moveid: string]: MoveSource[] }` | ✓ | ✓ | public |
| eventOnly | `boolean` |  | ✓ | public |
| eventData | `EventInfo[]` | ✓ | ✓ | public |
| encounters | `EventInfo[]` | ✓ | ✓ | public |
| exists | `boolean` |  | ✓ | public |
| species | `Species` |  | ✓ | public |

---

### ModdedDex

**File:** `sim/dex.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| isBase | `boolean` |  | ✓ | public |
| currentMod | `string` |  | ✓ | public |
| dataDir | `string` |  | ✓ | public |
| dataCache | `DexTableData | null` |  |  | public |
| textCache | `TextTableData | null` |  |  | public |
| formats | `DexFormats` |  | ✓ | public |
| abilities | `DexAbilities` |  | ✓ | public |
| items | `DexItems` |  | ✓ | public |
| moves | `DexMoves` |  | ✓ | public |
| species | `DexSpecies` |  | ✓ | public |
| conditions | `DexConditions` |  | ✓ | public |
| natures | `Data.DexNatures` |  | ✓ | public |
| types | `Data.DexTypes` |  | ✓ | public |
| stats | `Data.DexStats` |  | ✓ | public |
| aliases | `Map<ID, ID> | null` |  | ✓ | public |
| fuzzyAliases | `Map<ID, ID[]> | null` |  | ✓ | public |
| source | `{ type: string } | string,` |  |  | public |
| source | `{ type: string } | string,` |  |  | public |
| default | `return 0` |  |  | public |
| desc | `dataEntry.desc,` |  |  | public |
| shortDesc | `dataEntry.shortDesc,` |  |  | public |
| desc | `'',` |  |  | public |
| shortDesc | `'',` |  |  | public |
| target | `string,` |  |  | public |
| isInexact | `boolean` | ✓ |  | public |
| Pokedex | `'species', Moves: 'moves', Abilities: 'abilities', Items: 'items', Natures: 'natures', TypeChart: 'types',` |  |  | public |
| Pokedex | `'pokemon', Moves: 'move', Abilities: 'ability', Items: 'item', Natures: 'nature', TypeChart: 'type',` |  |  | public |
| searchType | `searchTypes[table],` |  |  | public |
| name | `res.name,` |  |  | public |
| isInexact | `true,` |  |  | public |
| searchType | `searchTypes[table],` |  |  | public |
| name | `res.name,` |  |  | public |
| name | `string, exportName: string` |  |  | public |

---

### MultiRandomRunner

**File:** `sim/tools/multi-random-runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| formatIndex | `number` |  |  | private |
| numGames | `number` |  |  | private |

---

### Pokemon

**File:** `sim/pokemon.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| side | `Side` |  | ✓ | public |
| battle | `Battle` |  | ✓ | public |
| set | `PokemonSet` |  | ✓ | public |
| name | `string` |  | ✓ | public |
| fullname | `string` |  | ✓ | public |
| level | `number` |  | ✓ | public |
| gender | `GenderName` |  | ✓ | public |
| happiness | `number` |  | ✓ | public |
| pokeball | `ID` |  | ✓ | public |
| dynamaxLevel | `number` |  | ✓ | public |
| gigantamax | `boolean` |  | ✓ | public |
| baseHpType | `string` |  | ✓ | public |
| baseHpPower | `number` |  | ✓ | public |
| baseMoveSlots | `MoveSlot[]` |  | ✓ | public |
| moveSlots | `MoveSlot[]` |  |  | public |
| hpType | `string` |  |  | public |
| hpPower | `number` |  |  | public |
| position | `number` |  |  | public |
| details | `string` |  |  | public |
| baseSpecies | `Species` |  |  | public |
| species | `Species` |  |  | public |
| speciesState | `EffectState` |  |  | public |
| status | `ID` |  |  | public |
| statusState | `EffectState` |  |  | public |
| volatiles | `{ [id: string]: EffectState }` |  |  | public |
| showCure | `boolean` | ✓ |  | public |
| baseStoredStats | `StatsTable` |  |  | public |
| storedStats | `StatsExceptHPTable` |  |  | public |
| boosts | `BoostsTable` |  |  | public |
| baseAbility | `ID` |  |  | public |
| ability | `ID` |  |  | public |
| abilityState | `EffectState` |  |  | public |
| item | `ID` |  |  | public |
| itemState | `EffectState` |  |  | public |
| lastItem | `ID` |  |  | public |
| usedItemThisTurn | `boolean` |  |  | public |
| ateBerry | `boolean` |  |  | public |
| itemKnockedOff | `boolean` |  |  | public |
| trapped | `boolean | "hidden"` |  |  | public |
| maybeTrapped | `boolean` |  |  | public |
| maybeDisabled | `boolean` |  |  | public |
| maybeLocked | `boolean | null` |  |  | public |
| illusion | `Pokemon | null` |  |  | public |
| transformed | `boolean` |  |  | public |
| maxhp | `number` |  |  | public |
| baseMaxhp | `number` |  |  | public |
| hp | `number` |  |  | public |
| fainted | `boolean` |  |  | public |
| faintQueued | `boolean` |  |  | public |
| subFainted | `boolean | null` |  |  | public |
| formeRegression | `boolean` |  |  | public |
| types | `string[]` |  |  | public |
| addedType | `string` |  |  | public |
| knownType | `boolean` |  |  | public |
| apparentType | `string` |  |  | public |
| switchFlag | `ID | boolean` |  |  | public |
| forceSwitchFlag | `boolean` |  |  | public |
| skipBeforeSwitchOutEventFlag | `boolean` |  |  | public |
| draggedIn | `number | null` |  |  | public |
| newlySwitched | `boolean` |  |  | public |
| beingCalledBack | `boolean` |  |  | public |
| lastMove | `ActiveMove | null` |  |  | public |
| lastMoveEncore | `ActiveMove | null` | ✓ |  | public |
| lastMoveUsed | `ActiveMove | null` |  |  | public |
| lastMoveTargetLoc | `number` | ✓ |  | public |
| moveThisTurn | `string | boolean` |  |  | public |
| statsRaisedThisTurn | `boolean` |  |  | public |
| statsLoweredThisTurn | `boolean` |  |  | public |
| moveLastTurnResult | `boolean | null | undefined` |  |  | public |
| moveThisTurnResult | `boolean | null | undefined` |  |  | public |
| hurtThisTurn | `number | null` |  |  | public |
| lastDamage | `number` |  |  | public |
| attackedBy | `Attacker[]` |  |  | public |
| timesAttacked | `number` |  |  | public |
| isActive | `boolean` |  |  | public |
| activeTurns | `number` |  |  | public |
| activeMoveActions | `number` |  |  | public |
| previouslySwitchedIn | `number` |  |  | public |
| truantTurn | `boolean` |  |  | public |
| bondTriggered | `boolean` |  |  | public |
| heroMessageDisplayed | `boolean` |  |  | public |
| swordBoost | `boolean` |  |  | public |
| shieldBoost | `boolean` |  |  | public |
| syrupTriggered | `boolean` |  |  | public |
| stellarBoostedTypes | `string[]` |  |  | public |
| isStarted | `boolean` |  |  | public |
| duringMove | `boolean` |  |  | public |
| weighthg | `number` |  |  | public |
| speed | `number` |  |  | public |
| canMegaEvo | `string | false | null | undefined` |  |  | public |
| canMegaEvoX | `string | false | null | undefined` |  |  | public |
| canMegaEvoY | `string | false | null | undefined` |  |  | public |
| canUltraBurst | `string | null | undefined` |  |  | public |
| canGigantamax | `string | null` |  | ✓ | public |
| canTerastallize | `string | false | null` |  |  | public |
| teraType | `string` |  |  | public |
| baseTypes | `string[]` |  |  | public |
| terastallized | `string` | ✓ |  | public |
| staleness | `'internal' | 'external'` | ✓ |  | public |
| pendingStaleness | `'internal' | 'external'` | ✓ |  | public |
| volatileStaleness | `'external'` | ✓ |  | public |
| modifiedStats | `StatsExceptHPTable` | ✓ |  | public |
| m | `{` |  |  | public |
| innate | `string, // Partners in Crime` | ✓ |  | public |
| originalSpecies | `string, // Mix and Mega` | ✓ |  | public |
| move | `move.name,` |  |  | public |
| id | `move.id,` |  |  | public |
| pp | `basepp,` |  |  | public |
| maxpp | `basepp,` |  |  | public |
| target | `move.target,` |  |  | public |
| disabled | `false,` |  |  | public |
| disabledSource | `'',` |  |  | public |
| used | `false,` |  |  | public |
| crit | `false,` |  |  | public |
| typeMod | `0,` |  |  | public |
| zBrokeProtect | `false,` |  |  | public |
| default | `const selectedTarget` |  |  | public |
| damage | `damageNumber,` |  |  | public |
| move | `move.id,` |  |  | public |
| thisTurn | `true,` |  |  | public |
| damageValue | `damage,` |  |  | public |
| move | `string, id: ID, disabled?: string | boolean, disabledSource?: string,` |  |  | public |
| target | `string, pp?: number, maxpp?: number,` | ✓ |  | public |
| move | `'Recharge',` |  |  | public |
| id | `'recharge' as ID,` |  |  | public |
| move | `moveSlot.move,` |  |  | public |
| id | `moveSlot.id,` |  |  | public |
| id | `lockedMove,` |  |  | public |
| move | `moveName,` |  |  | public |
| id | `moveSlot.id,` |  |  | public |
| pp | `moveSlot.pp,` |  |  | public |
| maxpp | `moveSlot.maxpp,` |  |  | public |
| ident | `this.fullname,` |  |  | public |
| details | `this.details,` |  |  | public |
| stats | `{` |  |  | public |
| atk | `this.baseStoredStats['atk'],` |  |  | public |
| def | `this.baseStoredStats['def'],` |  |  | public |
| spa | `this.baseStoredStats['spa'],` |  |  | public |
| spd | `this.baseStoredStats['spd'],` |  |  | public |
| spe | `this.baseStoredStats['spe'],` |  |  | public |
| baseAbility | `this.baseAbility,` |  |  | public |
| item | `this.item,` |  |  | public |
| pokeball | `this.pokeball,` |  |  | public |
| move | `moveName,` |  |  | public |
| id | `moveSlot.id,` |  |  | public |
| pp | `moveSlot.maxpp` |  |  | public |
| maxpp | `this.battle.gen >` |  |  | public |
| target | `moveSlot.target,` |  |  | public |
| disabled | `false,` |  |  | public |
| used | `false,` |  |  | public |
| virtual | `true,` |  |  | public |
| speciesId | `string | Species, source: Effect | null` |  |  | public |
| isPermanent | `boolean, abilitySlot` | ✓ |  | public |
| atk | `0,` |  |  | public |
| def | `0,` |  |  | public |
| spa | `0,` |  |  | public |
| spd | `0,` |  |  | public |
| spe | `0,` |  |  | public |
| accuracy | `0,` |  |  | public |
| evasion | `0,` |  |  | public |
| target | `this,` |  |  | public |
| status | `string | Condition,` |  |  | public |
| source | `Pokemon | null` |  |  | public |
| sourceEffect | `Effect | null` |  |  | public |
| ability | `string | Ability, source?: Pokemon | null, sourceEffect?: Effect | null,` |  |  | public |
| status | `string | Condition, source: Pokemon | null` |  |  | public |
| linkedStatus | `string | Condition | null` |  |  | public |

---

### PokemonSources

**File:** `sim/team-validator.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| sources | `PokemonSource[]` |  |  | public |
| sourcesBefore | `number` |  |  | public |
| sourcesAfter | `number` |  |  | public |
| isHidden | `boolean | null` |  |  | public |
| limitedEggMoves | `ID[] | null` | ✓ |  | public |
| possiblyLimitedEggMoves | `ID[] | null` | ✓ |  | public |
| tradebackLimitedEggMoves | `ID[] | null` | ✓ |  | public |
| levelUpEggMoves | `ID[] | null` | ✓ |  | public |
| pomegEggMoves | `ID[] | null` | ✓ |  | public |
| pomegEventEgg | `string | null` | ✓ |  | public |
| eventOnlyMinSourceGen | `number` | ✓ |  | public |
| learnsetDomain | `string[] | null` | ✓ |  | public |
| moveEvoCarryCount | `number` |  |  | public |
| babyOnly | `string` | ✓ |  | public |
| sketchMove | `string` | ✓ |  | public |
| dreamWorldMoveCount | `number` |  |  | public |
| hm | `string` | ✓ |  | public |
| isFromPokemonGo | `boolean` | ✓ |  | public |
| pokemonGoSource | `string` | ✓ |  | public |
| restrictiveMoves | `string[]` | ✓ |  | public |
| restrictedMove | `ID` | ✓ |  | public |

---

### Pool

**File:** `sim/tools/exhaustive-runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| possible | `string[]` |  | ✓ | public |
| unused | `Set<string>` |  |  | private |
| filled | `Set<string> | undefined` |  |  | private |
| filler | `string[] | undefined` |  |  | private |
| exhausted | `number` |  |  | public |

---

### PRNG

**File:** `sim/prng.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| startingSeed | `PRNGSeed` |  | ✓ | public |

---

### RandomPlayerAI

**File:** `sim/tools/random-player-ai.ts`

**Extends:** BattlePlayer

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| playerStream | `ObjectReadWriteStream<string>,` |  |  | public |
| options | `{ move?: number, mega?: number, seed?: PRNG | PRNGSeed | null }` |  |  | public |
| slot | `j,` |  |  | public |
| move | `possibleMoves[j - 1].move,` |  |  | public |
| target | `possibleMoves[j - 1].target,` |  |  | public |
| zMove | `false,` |  |  | public |
| slot | `j,` |  |  | public |
| move | `active.canZMove[j - 1].move,` |  |  | public |
| target | `active.canZMove[j - 1].target,` |  |  | public |
| zMove | `true,` |  |  | public |

---

### RawBattleStream

**File:** `sim/tools/runner.ts`

**Extends:** BattleStreams.BattleStream

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| rawInputLog | `string[]` |  | ✓ | public |

---

### Runner

**File:** `sim/tools/runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| move | `0.7,` |  |  | public |
| mega | `0.6,` |  |  | public |

---

### Side

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| battle | `Battle` |  | ✓ | public |
| id | `SideID` |  | ✓ | public |
| n | `number` |  | ✓ | public |
| name | `string` |  |  | public |
| avatar | `string` |  |  | public |
| foe | `Side` |  |  | public |
| allySide | `Side | null` |  |  | public |
| team | `PokemonSet[]` |  |  | public |
| pokemon | `Pokemon[]` |  |  | public |
| active | `Pokemon[]` |  |  | public |
| pokemonLeft | `number` |  |  | public |
| zMoveUsed | `boolean` |  |  | public |
| dynamaxUsed | `boolean` |  |  | public |
| faintedLastTurn | `Pokemon | null` |  |  | public |
| faintedThisTurn | `Pokemon | null` |  |  | public |
| totalFainted | `number` |  |  | public |
| lastSelectedMove | `ID` |  |  | public |
| sideConditions | `{ [id: string]: EffectState }` |  |  | public |
| slotConditions | `{ [id: string]: EffectState }[]` |  |  | public |
| activeRequest | `ChoiceRequest | null` |  |  | public |
| choice | `Choice` |  |  | public |
| lastMove | `Move | null` |  |  | public |
| default | `this.active` |  |  | public |
| cantUndo | `false,` |  |  | public |
| error | ```,` |  |  | public |
| actions | `[],` |  |  | public |
| forcedSwitchesLeft | `0,` |  |  | public |
| forcedPassesLeft | `0,` |  |  | public |
| zMove | `false,` |  |  | public |
| mega | `false,` |  |  | public |
| ultra | `false,` |  |  | public |
| dynamax | `false,` |  |  | public |
| terastallize | `false,` |  |  | public |
| default | `return action.choice` |  |  | public |
| name | `this.name,` |  |  | public |
| id | `this.id,` |  |  | public |
| pokemon | `[] as PokemonSwitchRequestData[],` |  |  | public |
| status | `string | Condition, source: Pokemon | 'debug' | null` |  |  | public |
| id | `status.id,` |  |  | public |
| target | `this,` |  |  | public |
| duration | `status.duration,` |  |  | public |
| target | `Pokemon | number, status: string | Condition, source: Pokemon | 'debug' | null` |  |  | public |
| sourceEffect | `Effect | null` |  |  | public |
| id | `status.id,` |  |  | public |
| target | `this,` |  |  | public |
| isSlotCondition | `true,` |  |  | public |
| duration | `status.duration,` |  |  | public |
| moveText | `string | number,` | ✓ |  | public |
| event | `'mega' | 'megax' | 'megay' | 'zmove' | 'ultra' | 'dynamax' | 'terastallize' | ''` |  |  | public |
| choice | `'move',` |  |  | public |
| targetLoc | `lockedMoveTargetLoc,` |  |  | public |
| moveid | `lockedMoveID,` |  |  | public |
| choice | `'move',` |  |  | public |
| moveid | `'struggle',` |  |  | public |
| choice | `'move',` |  |  | public |
| mega | `mega || ultra,` |  |  | public |
| zmove | `zMove,` |  |  | public |
| maxMove | `maxMove ? maxMove.id : undefined,` |  |  | public |
| terastallize | `terastallize ? pokemon.teraType : undefined,` |  |  | public |
| choice | `'revivalblessing',` |  |  | public |
| target | `targetPokemon,` |  |  | public |
| target | `targetPokemon,` |  |  | public |
| choice | `'team',` |  |  | public |
| pokemon | `this.pokemon[pos],` |  |  | public |
| priority | `-index,` |  |  | public |
| choice | `'shift',` |  |  | public |
| cantUndo | `false,` |  |  | public |
| error | ```,` |  |  | public |
| actions | `[],` |  |  | public |
| forcedSwitchesLeft | `forcedSwitches,` |  |  | public |
| forcedPassesLeft | `forcedPasses,` |  |  | public |
| zMove | `false,` |  |  | public |
| mega | `false,` |  |  | public |
| ultra | `false,` |  |  | public |
| dynamax | `false,` |  |  | public |
| terastallize | `false,` |  |  | public |
| choice | `'pass',` |  |  | public |

---

### TeamGenerator

**File:** `sim/tools/exhaustive-runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `typeof Dex, prng: PRNG | PRNGSeed | null, pools: Pools,` |  |  | public |
| signatures | `Map<string, { item: string, move?: string }[]>` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| gender | `species.gender,` |  |  | public |
| ability | `this.dex.gen >` |  |  | public |
| evs | `{` |  |  | public |
| ivs | `{` |  |  | public |

---

### Teams

**File:** `sim/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| hp | `ivs[0]` |  |  | public |
| atk | `ivs[1]` |  |  | public |
| def | `ivs[2]` |  |  | public |
| spa | `ivs[3]` |  |  | public |
| spd | `ivs[4]` |  |  | public |
| spe | `ivs[5]` |  |  | public |
| name | `'', species: '', item: '', ability: '', gender: '',` |  |  | public |
| nature | `'',` |  |  | public |
| evs | `{ hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 },` |  |  | public |
| ivs | `{ hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },` |  |  | public |
| level | `100,` |  |  | public |
| moves | `[],` |  |  | public |

---

### TeamValidator

**File:** `sim/team-validator.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| format | `Format` |  | ✓ | public |
| dex | `ModdedDex` |  | ✓ | public |
| gen | `number` |  | ✓ | public |
| ruleTable | `RuleTable` |  | ✓ | public |
| minSourceGen | `number` |  | ✓ | public |
| team | `PokemonSet[] | null,` |  |  | public |
| options | `{` |  |  | public |
| removeNicknames | `boolean,` | ✓ |  | public |
| skipSets | `{ [name: string]: { [key: string]: boolean } },` | ✓ |  | public |
| team | `PokemonSet[] | null,` |  |  | public |
| options | `{` |  |  | public |
| removeNicknames | `boolean,` | ✓ |  | public |
| skipSets | `{ [name: string]: { [key: string]: boolean } },` | ✓ |  | public |
| set | `PokemonSet, source: PokemonSource, setSources: PokemonSources, species: Species, because: string` |  |  | public |
| set | `PokemonSet, source: PokemonSource, setSources: PokemonSources, species: Species` |  |  | public |
| set | `PokemonSet, source: PokemonSource, setSources: PokemonSources, species: Species, because?: string` |  |  | public |
| generation | `2,` |  |  | public |
| level | `isMew ? 5 : isCelebi ? 30 : 3, // Level 1/2 Pokémon can't be obtained by transfer from RBY/GSC` |  |  | public |
| perfectIVs | `isMew || isCelebi ? 5 : 3,` |  |  | public |
| shiny | `isMew ? undefined : 1,` |  |  | public |
| pokeball | `'pokeball',` |  |  | public |
| from | `'Gen 1-2 Virtual Console transfer',` |  |  | public |
| generation | `8,` |  |  | public |
| perfectIVs | `isMew ? 3 : undefined,` |  |  | public |
| shiny | `isMew ? undefined : 1,` |  |  | public |
| from | `'Gen 7 Let\'s Go! HOME transfer',` |  |  | public |
| generation | `5,` |  |  | public |
| level | `10,` |  |  | public |
| from | `'Gen 5 Dream World',` |  |  | public |
| getAll | `false, pokemonBlacklist?: ID[], noRecurse?: true): boolean` | ✓ |  | public |
| getAll | `boolean, pokemonBlacklist?: ID[], noRecurse?: boolean) {` | ✓ |  | public |
| noRecurse | `boolean | undefined) {` |  |  | public |
| meteormash | `'Pikachu-Rock-Star', iciclecrash: 'Pikachu-Belle', drainingkiss: 'Pikachu-Pop-Star',` |  |  | public |
| electricterrain | `'Pikachu-PhD', flyingpress: 'Pikachu-Libre',` |  |  | public |
| set | `PokemonSet, setSources: PokemonSources, eventData: EventInfo, eventSpecies: Species` |  |  | public |
| set | `PokemonSet, setSources: PokemonSources, eventData: EventInfo, eventSpecies: Species,` |  |  | public |
| because | `string, from?: string` |  |  | public |
| set | `PokemonSet, setSources: PokemonSources, eventData: EventInfo, eventSpecies: Species,` |  |  | public |
| species | `Species, moves: string[], setSources: PokemonSources, set?: Partial<PokemonSet>,` |  |  | public |
| name | `string` |  |  | public |
| species | `Species, set: PokemonSet, setSources: PokemonSources, name: string` |  |  | public |
| move | `Move,` |  |  | public |
| s | `Species,` |  |  | public |
| set | `Partial<PokemonSet>` |  |  | public |
| move | `Move,` |  |  | public |
| originalSpecies | `Species,` |  |  | public |
| set | `Partial<PokemonSet>` |  |  | public |

---

## Interfaces

### AbilityDataTable

**File:** `sim/dex-abilities.ts`

---

### AbilityEventMethods

**File:** `sim/dex-abilities.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| onCheckShow | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onEnd | `(this: Battle, target: Pokemon & Side & Field)` | ✓ |  |
| onStart | `(this: Battle, target: Pokemon)` | ✓ |  |

---

### AbilityFlags

**File:** `sim/dex-abilities.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| breakable | `1` | ✓ |  |
| cantsuppress | `1` | ✓ |  |
| failroleplay | `1` | ✓ |  |
| failskillswap | `1` | ✓ |  |
| noentrain | `1` | ✓ |  |
| noreceiver | `1` | ✓ |  |
| notrace | `1` | ✓ |  |
| notransform | `1` | ✓ |  |

---

### ActionChoice

**File:** `sim/battle-queue.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choice | `string` |  |  |

---

### ActiveMove

**File:** `sim/dex-moves.ts`

**Extends:** MutableMove

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  | ✓ |
| effectType | `'Move'` |  | ✓ |
| id | `ID` |  | ✓ |
| num | `number` |  |  |
| weather | `ID` | ✓ |  |
| status | `ID` | ✓ |  |
| hit | `number` |  |  |
| moveHitData | `MoveHitData` | ✓ |  |
| hitTargets | `Pokemon[]` | ✓ |  |
| ability | `Ability` | ✓ |  |
| allies | `Pokemon[]` | ✓ |  |
| auraBooster | `Pokemon` | ✓ |  |
| causedCrashDamage | `boolean` | ✓ |  |
| forceStatus | `ID` | ✓ |  |
| hasAuraBreak | `boolean` | ✓ |  |
| hasBounced | `boolean` | ✓ |  |
| hasSheerForce | `boolean` | ✓ |  |
| isExternal | `boolean` | ✓ |  |
| lastHit | `boolean` | ✓ |  |
| magnitude | `number` | ✓ |  |
| pranksterBoosted | `boolean` | ✓ |  |
| selfDropped | `boolean` | ✓ |  |
| selfSwitch | `'copyvolatile' | 'shedtail' | boolean` | ✓ |  |
| spreadHit | `boolean` | ✓ |  |
| statusRoll | `string` | ✓ |  |
| stellarBoosted | `boolean` | ✓ |  |
| totalDamage | `number | false` | ✓ |  |
| typeChangerBoosted | `Effect` | ✓ |  |
| willChangeForme | `boolean` | ✓ |  |
| infiltrates | `boolean` | ✓ |  |
| ruinedAtk | `Pokemon` | ✓ |  |
| ruinedDef | `Pokemon` | ✓ |  |
| ruinedSpA | `Pokemon` | ✓ |  |
| ruinedSpD | `Pokemon` | ✓ |  |
| isZOrMaxPowered | `boolean` | ✓ |  |

---

### AIOptions

**File:** `sim/tools/runner.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| createAI | `(stream: ObjectReadWriteStream<string>, options: AIOptions)` |  |  |
| move | `number` | ✓ |  |
| mega | `number` | ✓ |  |
| seed | `PRNG | PRNGSeed | null` | ✓ |  |
| team | `PokemonSet[]` | ✓ |  |

---

### AliasesTable

**File:** `sim/dex.ts`

---

### AnyObject

**File:** `sim/global-types.ts`

---

### Attacker

**File:** `sim/pokemon.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| source | `Pokemon` |  |  |
| damage | `number` |  |  |
| thisTurn | `boolean` |  |  |
| move | `ID` | ✓ |  |
| slot | `PokemonSlot` |  |  |
| damageValue | `(number | boolean | undefined)` | ✓ |  |

---

### BasicEffect

**File:** `sim/global-types.ts`

**Extends:** EffectData

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `ID` |  |  |
| effectType | `EffectType` |  |  |
| exists | `boolean` |  |  |
| fullname | `string` |  |  |
| gen | `number` |  |  |
| sourceEffect | `string` |  |  |
| toString | `()` |  |  |

---

### BasicTextData

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| desc | `string` | ✓ |  |
| shortDesc | `string` | ✓ |  |

---

### BattleOptions

**File:** `sim/battle.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| format | `Format` | ✓ |  |
| formatid | `ID` |  |  |
| send | `(type: string, data: string | string[])` | ✓ |  |
| prng | `PRNG` | ✓ |  |
| seed | `PRNGSeed` | ✓ |  |
| rated | `boolean | string` | ✓ |  |
| p1 | `PlayerOptions` | ✓ |  |
| p2 | `PlayerOptions` | ✓ |  |
| p3 | `PlayerOptions` | ✓ |  |
| p4 | `PlayerOptions` | ✓ |  |
| debug | `boolean` | ✓ |  |
| forceRandomChance | `boolean` | ✓ |  |
| deserialized | `boolean` | ✓ |  |
| strictChoices | `boolean` | ✓ |  |

---

### BattleScriptsData

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| gen | `number` |  |  |

---

### Choice

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| cantUndo | `boolean` |  |  |
| error | `string` |  |  |
| actions | `ChosenAction[]` |  |  |
| forcedSwitchesLeft | `number` |  |  |
| forcedPassesLeft | `number` |  |  |
| switchIns | `Set<number>` |  |  |
| zMove | `boolean` |  |  |
| mega | `boolean` |  |  |
| ultra | `boolean` |  |  |
| dynamax | `boolean` |  |  |
| terastallize | `boolean` |  |  |

---

### ChosenAction

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choice | `'move' | 'switch' | 'instaswitch' | 'revivalblessing' | 'team' | 'shift' | 'pass'` |  |  |
| pokemon | `Pokemon` | ✓ |  |
| targetLoc | `number` | ✓ |  |
| moveid | `string` |  |  |
| move | `ActiveMove` | ✓ |  |
| target | `Pokemon` | ✓ |  |
| index | `number` | ✓ |  |
| side | `Side` | ✓ |  |
| mega | `boolean | null` | ✓ |  |
| megax | `boolean | null` | ✓ |  |
| megay | `boolean | null` | ✓ |  |
| zmove | `string` | ✓ |  |
| maxMove | `string` | ✓ |  |
| terastallize | `string` | ✓ |  |
| priority | `number` | ✓ |  |

---

### CommonHandlers

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| ModifierEffect | `(this: Battle, relayVar: number, target: Pokemon, source: Pokemon, effect: Effect)` |  |  |
| ModifierMove | `(this: Battle, relayVar: number, target: Pokemon, source: Pokemon, move: ActiveMove)` |  |  |
| ResultMove | `boolean | (` |  |  |
| ExtResultMove | `boolean | (` |  |  |
| VoidEffect | `(this: Battle, target: Pokemon, source: Pokemon, effect: Effect)` |  |  |
| VoidMove | `(this: Battle, target: Pokemon, source: Pokemon, move: ActiveMove)` |  |  |
| ModifierSourceEffect | `(` |  |  |
| this | `Battle, relayVar: number, source: Pokemon, target: Pokemon, effect: Effect` |  |  |
| ModifierSourceMove | `(` |  |  |
| this | `Battle, relayVar: number, source: Pokemon, target: Pokemon, move: ActiveMove` |  |  |
| ResultSourceMove | `boolean | (` |  |  |
| ExtResultSourceMove | `boolean | (` |  |  |
| VoidSourceEffect | `(this: Battle, source: Pokemon, target: Pokemon, effect: Effect)` |  |  |
| VoidSourceMove | `(this: Battle, source: Pokemon, target: Pokemon, move: ActiveMove)` |  |  |

---

### ConditionDataTable

**File:** `sim/dex-conditions.ts`

---

### ConditionTextData

**File:** `sim/global-types.ts`

**Extends:** BasicTextData

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| activate | `string` | ✓ |  |
| addItem | `string` | ✓ |  |
| block | `string` | ✓ |  |
| boost | `string` | ✓ |  |
| cant | `string` | ✓ |  |
| changeAbility | `string` | ✓ |  |
| damage | `string` | ✓ |  |
| end | `string` | ✓ |  |
| heal | `string` | ✓ |  |
| move | `string` | ✓ |  |
| start | `string` | ✓ |  |
| transform | `string` | ✓ |  |

---

### CosmeticFormeData

**File:** `sim/dex-species.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| isCosmeticForme | `boolean` |  |  |
| name | `string` |  |  |
| baseSpecies | `string` |  |  |
| forme | `string` |  |  |
| color | `string` |  |  |

---

### DexTableData

**File:** `sim/dex.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| Abilities | `DexTable<import('./dex-abilities').AbilityData>` |  |  |
| Rulesets | `DexTable<import('./dex-formats').FormatData>` |  |  |
| Items | `DexTable<import('./dex-items').ItemData>` |  |  |
| Learnsets | `DexTable<import('./dex-species').LearnsetData>` |  |  |
| Moves | `DexTable<import('./dex-moves').MoveData>` |  |  |
| Natures | `DexTable<import('./dex-data').NatureData>` |  |  |
| Pokedex | `DexTable<import('./dex-species').SpeciesData>` |  |  |
| FormatsData | `DexTable<import('./dex-species').SpeciesFormatsData>` |  |  |
| PokemonGoData | `DexTable<import('./dex-species').PokemonGoData>` |  |  |
| Scripts | `DexTable<AnyObject>` |  |  |
| Conditions | `DexTable<import('./dex-conditions').ConditionData>` |  |  |
| TypeChart | `DexTable<import('./dex-data').TypeData>` |  |  |

---

### DynamaxOptions

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| maxMoves | `({ move: string, target: MoveTarget, disabled?: boolean })[]` |  |  |
| gigantamax | `string` | ✓ |  |

---

### EffectData

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` | ✓ |  |
| desc | `string` | ✓ |  |
| duration | `number` | ✓ |  |
| durationCallback | `(this: Battle, target: Pokemon, source: Pokemon, effect: Effect | null)` | ✓ |  |
| effectType | `string` | ✓ |  |
| infiltrates | `boolean` | ✓ |  |
| isNonstandard | `Nonstandard | null` | ✓ |  |
| shortDesc | `string` | ✓ |  |

---

### EffectState

**File:** `sim/pokemon.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `string` |  |  |
| effectOrder | `number` |  |  |
| duration | `number` | ✓ |  |

---

### EventInfo

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| generation | `number` |  |  |
| level | `number` | ✓ |  |
| shiny | `boolean | 1` | ✓ |  |
| gender | `GenderName` | ✓ |  |
| nature | `string` | ✓ |  |
| ivs | `SparseStatsTable` | ✓ |  |
| perfectIVs | `number` | ✓ |  |
| isHidden | `boolean` | ✓ |  |
| abilities | `IDEntry[]` | ✓ |  |
| maxEggMoves | `number` | ✓ |  |
| moves | `IDEntry[]` | ✓ |  |
| pokeball | `IDEntry` | ✓ |  |
| from | `string` | ✓ |  |
| japan | `boolean` | ✓ |  |
| emeraldEventEgg | `boolean` | ✓ |  |
| source | `string` | ✓ |  |

---

### EventListener

**File:** `sim/battle.ts`

**Extends:** EventListenerWithoutPriority

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| order | `number | false` |  |  |
| priority | `number` |  |  |
| subOrder | `number` |  |  |
| effectOrder | `number` | ✓ |  |
| speed | `number` | ✓ |  |

---

### EventListenerWithoutPriority

**File:** `sim/battle.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| effect | `Effect` |  |  |
| target | `Pokemon` | ✓ |  |
| index | `number` | ✓ |  |
| callback | `Function` | ✓ |  |
| state | `EffectState | null` |  |  |
| end | `Function | null` |  |  |
| endCallArgs | `any[]` | ✓ |  |
| effectHolder | `Pokemon | Side | Field | Battle` |  |  |

---

### EventMethods

**File:** `sim/dex-conditions.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| onDamagingHit | `(this: Battle, damage: number, target: Pokemon, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onEmergencyExit | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAfterEachBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAfterHit | `MoveEventMethods['onAfterHit']` | ✓ |  |
| onAfterMega | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAfterSetStatus | `(this: Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAfterSubDamage | `MoveEventMethods['onAfterSubDamage']` | ✓ |  |
| onAfterSwitchInSelf | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAfterTerastallization | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAfterUseItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAfterTakeItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAfterBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAfterFaint | `(this: Battle, length: number, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAfterMoveSecondarySelf | `MoveEventMethods['onAfterMoveSecondarySelf']` | ✓ |  |
| onAfterMoveSecondary | `MoveEventMethods['onAfterMoveSecondary']` | ✓ |  |
| onAfterMove | `MoveEventMethods['onAfterMove']` | ✓ |  |
| onAfterMoveSelf | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAttract | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onAccuracy | `(` | ✓ |  |
| this | `Battle, accuracy: number, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onBasePower | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onBeforeFaint | `(this: Battle, pokemon: Pokemon, effect: Effect)` | ✓ |  |
| onBeforeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onBeforeSwitchIn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onBeforeSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onBeforeTurn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onChangeBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onTryBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onChargeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onCriticalHit | `((this: Battle, pokemon: Pokemon, source: null, move: ActiveMove)` | ✓ |  |
| onDamage | `(` | ✓ |  |
| this | `Battle, damage: number, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onDeductPP | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onDisableMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onDragOut | `(this: Battle, pokemon: Pokemon, source?: Pokemon, move?: ActiveMove)` | ✓ |  |
| onEatItem | `(this: Battle, item: Item, pokemon: Pokemon, source?: Pokemon, effect?: Effect)` | ✓ |  |
| onEffectiveness | `MoveEventMethods['onEffectiveness']` | ✓ |  |
| onEntryHazard | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFaint | `CommonHandlers['VoidEffect']` | ✓ |  |
| onFlinch | `((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFractionalPriority | `CommonHandlers['ModifierSourceMove'] | -0.1` | ✓ |  |
| onHit | `MoveEventMethods['onHit']` | ✓ |  |
| onImmunity | `(this: Battle, type: string, pokemon: Pokemon)` | ✓ |  |
| onLockMove | `string | ((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onMaybeTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onModifyAccuracy | `CommonHandlers['ModifierMove']` | ✓ |  |
| onModifyAtk | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifyBoost | `(this: Battle, boosts: SparseBoostsTable, pokemon: Pokemon)` | ✓ |  |
| onModifyCritRatio | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifyDef | `CommonHandlers['ModifierMove']` | ✓ |  |
| onModifyMove | `MoveEventMethods['onModifyMove']` | ✓ |  |
| onModifyPriority | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifySecondaries | `(` | ✓ |  |
| this | `Battle, secondaries: SecondaryEffect[], target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onModifyType | `MoveEventMethods['onModifyType']` | ✓ |  |
| onModifyTarget | `MoveEventMethods['onModifyTarget']` | ✓ |  |
| onModifySpA | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifySpD | `CommonHandlers['ModifierMove']` | ✓ |  |
| onModifySpe | `(this: Battle, spe: number, pokemon: Pokemon)` | ✓ |  |
| onModifySTAB | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifyWeight | `(this: Battle, weighthg: number, pokemon: Pokemon)` | ✓ |  |
| onMoveAborted | `CommonHandlers['VoidMove']` | ✓ |  |
| onNegateImmunity | `((this: Battle, pokemon: Pokemon, type: string)` | ✓ |  |
| onOverrideAction | `(this: Battle, pokemon: Pokemon, target: Pokemon, move: ActiveMove)` | ✓ |  |
| onPrepareHit | `CommonHandlers['ResultSourceMove']` | ✓ |  |
| onPseudoWeatherChange | `(this: Battle, target: Pokemon, source: Pokemon, pseudoWeather: Condition)` | ✓ |  |
| onRedirectTarget | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, source2: Effect, move: ActiveMove` |  |  |
| onResidual | `(this: Battle, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onSetAbility | `(` | ✓ |  |
| this | `Battle, ability: string, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onSetStatus | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onSetWeather | `(this: Battle, target: Pokemon, source: Pokemon, weather: Condition)` | ✓ |  |
| onSideConditionStart | `(this: Battle, target: Side, source: Pokemon, sideCondition: Condition)` | ✓ |  |
| onStallMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSwitchIn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSwap | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onTakeItem | `(` | ✓ |  |
| onWeatherChange | `(this: Battle, target: Pokemon, source: Pokemon, sourceEffect: Effect)` | ✓ |  |
| onTerrainChange | `(this: Battle, target: Pokemon, source: Pokemon, sourceEffect: Effect)` | ✓ |  |
| onTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onTryAddVolatile | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, sourceEffect: Effect` |  |  |
| onTryEatItem | `boolean | ((this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onTryHeal | `(` | ✓ |  |
| onTryHit | `MoveEventMethods['onTryHit']` | ✓ |  |
| onTryHitField | `MoveEventMethods['onTryHitField']` | ✓ |  |
| onTryHitSide | `CommonHandlers['ResultMove']` | ✓ |  |
| onInvulnerability | `CommonHandlers['ExtResultMove']` | ✓ |  |
| onTryMove | `MoveEventMethods['onTryMove']` | ✓ |  |
| onTryPrimaryHit | `(this: Battle, target: Pokemon, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onType | `(this: Battle, types: string[], pokemon: Pokemon)` | ✓ |  |
| onUseItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onUpdate | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onWeather | `(this: Battle, target: Pokemon, source: null, effect: Condition)` | ✓ |  |
| onWeatherModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifyDamagePhase1 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onModifyDamagePhase2 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeDamagingHit | `(this: Battle, damage: number, target: Pokemon, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onFoeAfterEachBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon)` | ✓ |  |
| onFoeAfterHit | `MoveEventMethods['onAfterHit']` | ✓ |  |
| onFoeAfterSetStatus | `(this: Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onFoeAfterSubDamage | `MoveEventMethods['onAfterSubDamage']` | ✓ |  |
| onFoeAfterSwitchInSelf | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeAfterUseItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onFoeAfterBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onFoeAfterFaint | `(this: Battle, length: number, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onFoeAfterMoveSecondarySelf | `MoveEventMethods['onAfterMoveSecondarySelf']` | ✓ |  |
| onFoeAfterMoveSecondary | `MoveEventMethods['onAfterMoveSecondary']` | ✓ |  |
| onFoeAfterMove | `MoveEventMethods['onAfterMove']` | ✓ |  |
| onFoeAfterMoveSelf | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onFoeAttract | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onFoeAccuracy | `(` | ✓ |  |
| this | `Battle, accuracy: number, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onFoeBasePower | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeBeforeFaint | `(this: Battle, pokemon: Pokemon, effect: Effect)` | ✓ |  |
| onFoeBeforeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onFoeBeforeSwitchIn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeBeforeSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeTryBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onFoeChargeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onFoeCriticalHit | `((this: Battle, pokemon: Pokemon, source: null, move: ActiveMove)` | ✓ |  |
| onFoeDamage | `(` | ✓ |  |
| this | `Battle, damage: number, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onFoeDeductPP | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onFoeDisableMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeDragOut | `(this: Battle, pokemon: Pokemon, source?: Pokemon, move?: ActiveMove)` | ✓ |  |
| onFoeEatItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onFoeEffectiveness | `MoveEventMethods['onEffectiveness']` | ✓ |  |
| onFoeFaint | `CommonHandlers['VoidEffect']` | ✓ |  |
| onFoeFlinch | `((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeHit | `MoveEventMethods['onHit']` | ✓ |  |
| onFoeImmunity | `(this: Battle, type: string, pokemon: Pokemon)` | ✓ |  |
| onFoeLockMove | `string | ((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeMaybeTrapPokemon | `(this: Battle, pokemon: Pokemon, source?: Pokemon)` | ✓ |  |
| onFoeModifyAccuracy | `CommonHandlers['ModifierMove']` | ✓ |  |
| onFoeModifyAtk | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifyBoost | `(this: Battle, boosts: SparseBoostsTable, pokemon: Pokemon)` | ✓ |  |
| onFoeModifyCritRatio | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifyDef | `CommonHandlers['ModifierMove']` | ✓ |  |
| onFoeModifyMove | `MoveEventMethods['onModifyMove']` | ✓ |  |
| onFoeModifyPriority | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifySecondaries | `(` | ✓ |  |
| this | `Battle, secondaries: SecondaryEffect[], target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onFoeModifySpA | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifySpD | `CommonHandlers['ModifierMove']` | ✓ |  |
| onFoeModifySpe | `(this: Battle, spe: number, pokemon: Pokemon)` | ✓ |  |
| onFoeModifySTAB | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifyType | `MoveEventMethods['onModifyType']` | ✓ |  |
| onFoeModifyTarget | `MoveEventMethods['onModifyTarget']` | ✓ |  |
| onFoeModifyWeight | `(this: Battle, weighthg: number, pokemon: Pokemon)` | ✓ |  |
| onFoeMoveAborted | `CommonHandlers['VoidMove']` | ✓ |  |
| onFoeNegateImmunity | `((this: Battle, pokemon: Pokemon, type: string)` | ✓ |  |
| onFoeOverrideAction | `(this: Battle, pokemon: Pokemon, target: Pokemon, move: ActiveMove)` | ✓ |  |
| onFoePrepareHit | `CommonHandlers['ResultSourceMove']` | ✓ |  |
| onFoeRedirectTarget | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, source2: Effect, move: ActiveMove` |  |  |
| onFoeResidual | `(this: Battle, target: Pokemon & Side, source: Pokemon, effect: Effect)` | ✓ |  |
| onFoeSetAbility | `(this: Battle, ability: string, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onFoeSetStatus | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onFoeSetWeather | `(this: Battle, target: Pokemon, source: Pokemon, weather: Condition)` | ✓ |  |
| onFoeStallMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeTakeItem | `(` | ✓ |  |
| onFoeTerrain | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onFoeTryAddVolatile | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, sourceEffect: Effect` |  |  |
| onFoeTryEatItem | `boolean | ((this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onFoeTryHeal | `(` | ✓ |  |
| onFoeTryHit | `MoveEventMethods['onTryHit']` | ✓ |  |
| onFoeTryHitField | `MoveEventMethods['onTryHitField']` | ✓ |  |
| onFoeTryHitSide | `CommonHandlers['ResultMove']` | ✓ |  |
| onFoeInvulnerability | `CommonHandlers['ExtResultMove']` | ✓ |  |
| onFoeTryMove | `MoveEventMethods['onTryMove']` | ✓ |  |
| onFoeTryPrimaryHit | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onFoeType | `(this: Battle, types: string[], pokemon: Pokemon)` | ✓ |  |
| onFoeWeatherModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifyDamagePhase1 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onFoeModifyDamagePhase2 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceDamagingHit | `(this: Battle, damage: number, target: Pokemon, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onSourceAfterEachBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon)` | ✓ |  |
| onSourceAfterHit | `MoveEventMethods['onAfterHit']` | ✓ |  |
| onSourceAfterSetStatus | `(this: Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onSourceAfterSubDamage | `MoveEventMethods['onAfterSubDamage']` | ✓ |  |
| onSourceAfterSwitchInSelf | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceAfterUseItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onSourceAfterBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onSourceAfterFaint | `(this: Battle, length: number, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onSourceAfterMoveSecondarySelf | `MoveEventMethods['onAfterMoveSecondarySelf']` | ✓ |  |
| onSourceAfterMoveSecondary | `MoveEventMethods['onAfterMoveSecondary']` | ✓ |  |
| onSourceAfterMove | `MoveEventMethods['onAfterMove']` | ✓ |  |
| onSourceAfterMoveSelf | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onSourceAttract | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onSourceAccuracy | `(` | ✓ |  |
| this | `Battle, accuracy: number, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onSourceBasePower | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceBeforeFaint | `(this: Battle, pokemon: Pokemon, effect: Effect)` | ✓ |  |
| onSourceBeforeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onSourceBeforeSwitchIn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceBeforeSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceTryBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onSourceChargeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onSourceCriticalHit | `((this: Battle, pokemon: Pokemon, source: null, move: ActiveMove)` | ✓ |  |
| onSourceDamage | `(` | ✓ |  |
| this | `Battle, damage: number, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onSourceDeductPP | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onSourceDisableMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceDragOut | `(this: Battle, pokemon: Pokemon, source?: Pokemon, move?: ActiveMove)` | ✓ |  |
| onSourceEatItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onSourceEffectiveness | `MoveEventMethods['onEffectiveness']` | ✓ |  |
| onSourceFaint | `CommonHandlers['VoidEffect']` | ✓ |  |
| onSourceFlinch | `((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceHit | `MoveEventMethods['onHit']` | ✓ |  |
| onSourceImmunity | `(this: Battle, type: string, pokemon: Pokemon)` | ✓ |  |
| onSourceLockMove | `string | ((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceMaybeTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceModifyAccuracy | `CommonHandlers['ModifierMove']` | ✓ |  |
| onSourceModifyAtk | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifyBoost | `(this: Battle, boosts: SparseBoostsTable, pokemon: Pokemon)` | ✓ |  |
| onSourceModifyCritRatio | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifyDef | `CommonHandlers['ModifierMove']` | ✓ |  |
| onSourceModifyMove | `MoveEventMethods['onModifyMove']` | ✓ |  |
| onSourceModifyPriority | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifySecondaries | `(` | ✓ |  |
| this | `Battle, secondaries: SecondaryEffect[], target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onSourceModifySpA | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifySpD | `CommonHandlers['ModifierMove']` | ✓ |  |
| onSourceModifySpe | `(this: Battle, spe: number, pokemon: Pokemon)` | ✓ |  |
| onSourceModifySTAB | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifyType | `MoveEventMethods['onModifyType']` | ✓ |  |
| onSourceModifyTarget | `MoveEventMethods['onModifyTarget']` | ✓ |  |
| onSourceModifyWeight | `(this: Battle, weighthg: number, pokemon: Pokemon)` | ✓ |  |
| onSourceMoveAborted | `CommonHandlers['VoidMove']` | ✓ |  |
| onSourceNegateImmunity | `((this: Battle, pokemon: Pokemon, type: string)` | ✓ |  |
| onSourceOverrideAction | `(this: Battle, pokemon: Pokemon, target: Pokemon, move: ActiveMove)` | ✓ |  |
| onSourcePrepareHit | `CommonHandlers['ResultSourceMove']` | ✓ |  |
| onSourceRedirectTarget | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, source2: Effect, move: ActiveMove` |  |  |
| onSourceResidual | `(this: Battle, target: Pokemon & Side, source: Pokemon, effect: Effect)` | ✓ |  |
| onSourceSetAbility | `(` | ✓ |  |
| this | `Battle, ability: string, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onSourceSetStatus | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onSourceSetWeather | `(this: Battle, target: Pokemon, source: Pokemon, weather: Condition)` | ✓ |  |
| onSourceStallMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceTakeItem | `(` | ✓ |  |
| onSourceTerrain | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onSourceTryAddVolatile | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, sourceEffect: Effect` |  |  |
| onSourceTryEatItem | `boolean | ((this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onSourceTryHeal | `(` | ✓ |  |
| onSourceTryHit | `MoveEventMethods['onTryHit']` | ✓ |  |
| onSourceTryHitField | `MoveEventMethods['onTryHitField']` | ✓ |  |
| onSourceTryHitSide | `CommonHandlers['ResultMove']` | ✓ |  |
| onSourceInvulnerability | `CommonHandlers['ExtResultMove']` | ✓ |  |
| onSourceTryMove | `MoveEventMethods['onTryMove']` | ✓ |  |
| onSourceTryPrimaryHit | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onSourceType | `(this: Battle, types: string[], pokemon: Pokemon)` | ✓ |  |
| onSourceWeatherModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifyDamagePhase1 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onSourceModifyDamagePhase2 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyDamagingHit | `(this: Battle, damage: number, target: Pokemon, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onAnyAfterEachBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon)` | ✓ |  |
| onAnyAfterHit | `MoveEventMethods['onAfterHit']` | ✓ |  |
| onAnyAfterSetStatus | `(this: Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAnyAfterSubDamage | `MoveEventMethods['onAfterSubDamage']` | ✓ |  |
| onAnyAfterSwitchInSelf | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyAfterUseItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAnyAfterBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAnyAfterFaint | `(this: Battle, length: number, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAnyAfterMega | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyAfterMoveSecondarySelf | `MoveEventMethods['onAfterMoveSecondarySelf']` | ✓ |  |
| onAnyAfterMoveSecondary | `MoveEventMethods['onAfterMoveSecondary']` | ✓ |  |
| onAnyAfterMove | `MoveEventMethods['onAfterMove']` | ✓ |  |
| onAnyAfterMoveSelf | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAnyAfterTerastallization | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyAttract | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onAnyAccuracy | `(` | ✓ |  |
| this | `Battle, accuracy: number, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onAnyBasePower | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyBeforeFaint | `(this: Battle, pokemon: Pokemon, effect: Effect)` | ✓ |  |
| onAnyBeforeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAnyBeforeSwitchIn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyBeforeSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyTryBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAnyChargeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAnyCriticalHit | `((this: Battle, pokemon: Pokemon, source: null, move: ActiveMove)` | ✓ |  |
| onAnyDamage | `(` | ✓ |  |
| this | `Battle, damage: number, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onAnyDeductPP | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onAnyDisableMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyDragOut | `(this: Battle, pokemon: Pokemon, source?: Pokemon, move?: ActiveMove)` | ✓ |  |
| onAnyEatItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAnyEffectiveness | `MoveEventMethods['onEffectiveness']` | ✓ |  |
| onAnyFaint | `CommonHandlers['VoidEffect']` | ✓ |  |
| onAnyFlinch | `((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyHit | `MoveEventMethods['onHit']` | ✓ |  |
| onAnyImmunity | `(this: Battle, type: string, pokemon: Pokemon)` | ✓ |  |
| onAnyLockMove | `string | ((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyMaybeTrapPokemon | `(this: Battle, pokemon: Pokemon, source?: Pokemon)` | ✓ |  |
| onAnyModifyAccuracy | `CommonHandlers['ModifierMove']` | ✓ |  |
| onAnyModifyAtk | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifyBoost | `(this: Battle, boosts: SparseBoostsTable, pokemon: Pokemon)` | ✓ |  |
| onAnyModifyCritRatio | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifyDef | `CommonHandlers['ModifierMove']` | ✓ |  |
| onAnyModifyMove | `MoveEventMethods['onModifyMove']` | ✓ |  |
| onAnyModifyPriority | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifySecondaries | `(` | ✓ |  |
| this | `Battle, secondaries: SecondaryEffect[], target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onAnyModifySpA | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifySpD | `CommonHandlers['ModifierMove']` | ✓ |  |
| onAnyModifySpe | `(this: Battle, spe: number, pokemon: Pokemon)` | ✓ |  |
| onAnyModifySTAB | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifyType | `MoveEventMethods['onModifyType']` | ✓ |  |
| onAnyModifyTarget | `MoveEventMethods['onModifyTarget']` | ✓ |  |
| onAnyModifyWeight | `(this: Battle, weighthg: number, pokemon: Pokemon)` | ✓ |  |
| onAnyMoveAborted | `CommonHandlers['VoidMove']` | ✓ |  |
| onAnyNegateImmunity | `((this: Battle, pokemon: Pokemon, type: string)` | ✓ |  |
| onAnyOverrideAction | `(this: Battle, pokemon: Pokemon, target: Pokemon, move: ActiveMove)` | ✓ |  |
| onAnyPrepareHit | `CommonHandlers['ResultSourceMove']` | ✓ |  |
| onAnyPseudoWeatherChange | `(this: Battle, target: Pokemon, source: Pokemon, pseudoWeather: Condition)` | ✓ |  |
| onAnyRedirectTarget | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, source2: Effect, move: ActiveMove` |  |  |
| onAnyResidual | `(this: Battle, target: Pokemon & Side, source: Pokemon, effect: Effect)` | ✓ |  |
| onAnySetAbility | `(this: Battle, ability: string, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAnySetStatus | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onAnySetWeather | `(this: Battle, target: Pokemon, source: Pokemon, weather: Condition)` | ✓ |  |
| onAnyStallMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnySwitchIn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnySwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyTakeItem | `(` | ✓ |  |
| onAnyTerrain | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAnyTryAddVolatile | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, sourceEffect: Effect` |  |  |
| onAnyTryEatItem | `boolean | ((this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAnyTryHeal | `(` | ✓ |  |
| onAnyTryHit | `MoveEventMethods['onTryHit']` | ✓ |  |
| onAnyTryHitField | `MoveEventMethods['onTryHitField']` | ✓ |  |
| onAnyTryHitSide | `CommonHandlers['ResultMove']` | ✓ |  |
| onAnyInvulnerability | `CommonHandlers['ExtResultMove']` | ✓ |  |
| onAnyTryMove | `MoveEventMethods['onTryMove']` | ✓ |  |
| onAnyTryPrimaryHit | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onAnyType | `(this: Battle, types: string[], pokemon: Pokemon)` | ✓ |  |
| onAnyWeatherModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifyDamagePhase1 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAnyModifyDamagePhase2 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAccuracyPriority | `number` | ✓ |  |
| onDamagingHitOrder | `number` | ✓ |  |
| onAfterMoveSecondaryPriority | `number` | ✓ |  |
| onAfterMoveSecondarySelfPriority | `number` | ✓ |  |
| onAfterMoveSelfPriority | `number` | ✓ |  |
| onAfterSetStatusPriority | `number` | ✓ |  |
| onAnyBasePowerPriority | `number` | ✓ |  |
| onAnyInvulnerabilityPriority | `number` | ✓ |  |
| onAnyModifyAccuracyPriority | `number` | ✓ |  |
| onAnyFaintPriority | `number` | ✓ |  |
| onAnyPrepareHitPriority | `number` | ✓ |  |
| onAnySwitchInPriority | `number` | ✓ |  |
| onAnySwitchInSubOrder | `number` | ✓ |  |
| onAllyBasePowerPriority | `number` | ✓ |  |
| onAllyModifyAtkPriority | `number` | ✓ |  |
| onAllyModifySpAPriority | `number` | ✓ |  |
| onAllyModifySpDPriority | `number` | ✓ |  |
| onAttractPriority | `number` | ✓ |  |
| onBasePowerPriority | `number` | ✓ |  |
| onBeforeMovePriority | `number` | ✓ |  |
| onBeforeSwitchOutPriority | `number` | ✓ |  |
| onChangeBoostPriority | `number` | ✓ |  |
| onDamagePriority | `number` | ✓ |  |
| onDragOutPriority | `number` | ✓ |  |
| onEffectivenessPriority | `number` | ✓ |  |
| onFoeBasePowerPriority | `number` | ✓ |  |
| onFoeBeforeMovePriority | `number` | ✓ |  |
| onFoeModifyDefPriority | `number` | ✓ |  |
| onFoeModifySpDPriority | `number` | ✓ |  |
| onFoeRedirectTargetPriority | `number` | ✓ |  |
| onFoeTrapPokemonPriority | `number` | ✓ |  |
| onFractionalPriorityPriority | `number` | ✓ |  |
| onHitPriority | `number` | ✓ |  |
| onInvulnerabilityPriority | `number` | ✓ |  |
| onMaybeTrapPokemonPriority | `number` | ✓ |  |
| onModifyAccuracyPriority | `number` | ✓ |  |
| onModifyAtkPriority | `number` | ✓ |  |
| onModifyCritRatioPriority | `number` | ✓ |  |
| onModifyDefPriority | `number` | ✓ |  |
| onModifyMovePriority | `number` | ✓ |  |
| onModifyPriorityPriority | `number` | ✓ |  |
| onModifySpAPriority | `number` | ✓ |  |
| onModifySpDPriority | `number` | ✓ |  |
| onModifySpePriority | `number` | ✓ |  |
| onModifySTABPriority | `number` | ✓ |  |
| onModifyTypePriority | `number` | ✓ |  |
| onModifyWeightPriority | `number` | ✓ |  |
| onRedirectTargetPriority | `number` | ✓ |  |
| onResidualOrder | `number` | ✓ |  |
| onResidualPriority | `number` | ✓ |  |
| onResidualSubOrder | `number` | ✓ |  |
| onSourceBasePowerPriority | `number` | ✓ |  |
| onSourceInvulnerabilityPriority | `number` | ✓ |  |
| onSourceModifyAccuracyPriority | `number` | ✓ |  |
| onSourceModifyAtkPriority | `number` | ✓ |  |
| onSourceModifyDamagePriority | `number` | ✓ |  |
| onSourceModifySpAPriority | `number` | ✓ |  |
| onSwitchInPriority | `number` | ✓ |  |
| onSwitchInSubOrder | `number` | ✓ |  |
| onTrapPokemonPriority | `number` | ✓ |  |
| onTryBoostPriority | `number` | ✓ |  |
| onTryEatItemPriority | `number` | ✓ |  |
| onTryHealPriority | `number` | ✓ |  |
| onTryHitPriority | `number` | ✓ |  |
| onTryMovePriority | `number` | ✓ |  |
| onTryPrimaryHitPriority | `number` | ✓ |  |
| onTypePriority | `number` | ✓ |  |

---

### ExhaustiveRunnerOptions

**File:** `sim/tools/exhaustive-runner.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| format | `string` |  |  |
| cycles | `number` | ✓ |  |
| prng | `PRNG | PRNGSeed | null` | ✓ |  |
| log | `boolean` | ✓ |  |
| maxGames | `number` | ✓ |  |
| maxFailures | `number` | ✓ |  |
| dual | `boolean | 'debug'` | ✓ |  |

---

### ExportOptions

**File:** `sim/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| hideStats | `boolean` | ✓ |  |
| removeNicknames | `boolean | ((nickname: string)` | ✓ |  |

---

### FactoryTeamDetails

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| megaCount | `number` | ✓ |  |
| zCount | `number` | ✓ |  |
| wantsTeraCount | `number` | ✓ |  |
| forceResult | `boolean` |  |  |
| weather | `string` | ✓ |  |
| terrain | `string[]` | ✓ |  |
| typeCount | `{ [k: string]: number }` |  |  |
| typeComboCount | `{ [k: string]: number }` |  |  |
| baseFormes | `{ [k: string]: number }` |  |  |
| has | `{ [k: string]: number }` |  |  |
| weaknesses | `{ [k: string]: number }` |  |  |
| resistances | `{ [k: string]: number }` |  |  |
| gigantamax | `boolean` | ✓ |  |

---

### FieldAction

**File:** `sim/battle-queue.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choice | `'start' | 'residual' | 'pass' | 'beforeTurn'` |  |  |
| priority | `number` |  |  |
| speed | `1` |  |  |
| pokemon | `null` |  |  |

---

### FieldEventMethods

**File:** `sim/dex-conditions.ts`

**Extends:** EventMethods

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| onFieldStart | `(this: Battle, target: Field, source: Pokemon, sourceEffect: Effect)` | ✓ |  |
| onFieldRestart | `(this: Battle, target: Field, source: Pokemon, sourceEffect: Effect)` | ✓ |  |
| onFieldResidual | `(this: Battle, target: Field, source: Pokemon, effect: Effect)` | ✓ |  |
| onFieldEnd | `(this: Battle, target: Field)` | ✓ |  |
| onFieldResidualOrder | `number` | ✓ |  |
| onFieldResidualPriority | `number` | ✓ |  |
| onFieldResidualSubOrder | `number` | ✓ |  |

---

### FlingData

**File:** `sim/dex-items.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| basePower | `number` |  |  |
| status | `string` | ✓ |  |
| volatileStatus | `string` | ✓ |  |
| effect | `CommonHandlers['ResultMove']` | ✓ |  |

---

### FormatDataTable

**File:** `sim/dex-formats.ts`

---

### FormatSection

**File:** `sim/dex-formats.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| section | `string` |  |  |
| column | `number` | ✓ |  |
| formats | `FormatData[]` |  |  |

---

### GameTimerSettings

**File:** `sim/dex-formats.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| dcTimer | `boolean` |  |  |
| dcTimerBank | `boolean` |  |  |
| starting | `number` |  |  |
| grace | `number` |  |  |
| addPerTurn | `number` |  |  |
| maxPerTurn | `number` |  |  |
| maxFirstTurn | `number` |  |  |
| timeoutAutoChoose | `boolean` |  |  |
| accelerate | `boolean` |  |  |

---

### HitEffect

**File:** `sim/dex-moves.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| onHit | `MoveEventMethods['onHit']` | ✓ |  |
| boosts | `SparseBoostsTable | null` | ✓ |  |
| status | `string` | ✓ |  |
| volatileStatus | `string` | ✓ |  |
| sideCondition | `string` | ✓ |  |
| slotCondition | `string` | ✓ |  |
| pseudoWeather | `string` | ✓ |  |
| terrain | `string` | ✓ |  |
| weather | `string` | ✓ |  |

---

### ItemDataTable

**File:** `sim/dex-items.ts`

---

### LearnsetData

**File:** `sim/dex-species.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| learnset | `{ [moveid: IDEntry]: MoveSource[] }` | ✓ |  |
| eventData | `EventInfo[]` | ✓ |  |
| eventOnly | `boolean` | ✓ |  |
| encounters | `EventInfo[]` | ✓ |  |
| exists | `boolean` | ✓ |  |

---

### LearnsetDataTable

**File:** `sim/dex-species.ts`

---

### ModdedAbilityDataTable

**File:** `sim/dex-abilities.ts`

---

### ModdedBattleActions

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| inherit | `true` | ✓ |  |
| afterMoveSecondaryEvent | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| calcRecoilDamage | `(this: BattleActions, damageDealt: number, move: Move, pokemon: Pokemon)` | ✓ |  |
| canMegaEvo | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| canMegaEvoX | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| canMegaEvoY | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| canTerastallize | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| canUltraBurst | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| canZMove | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| forceSwitch | `(` | ✓ |  |
| this | `BattleActions, damage: SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,` |  |  |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean` |  |  |
| getActiveMaxMove | `(this: BattleActions, move: Move, pokemon: Pokemon)` | ✓ |  |
| getActiveZMove | `(this: BattleActions, move: Move, pokemon: Pokemon)` | ✓ |  |
| getMaxMove | `(this: BattleActions, move: Move, pokemon: Pokemon)` | ✓ |  |
| getSpreadDamage | `(` | ✓ |  |
| this | `BattleActions, damage: SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,` |  |  |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean` |  |  |
| getZMove | `(this: BattleActions, move: Move, pokemon: Pokemon, skipChecks?: boolean)` | ✓ |  |
| hitStepAccuracy | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| hitStepBreakProtect | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| hitStepMoveHitLoop | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| hitStepTryImmunity | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| hitStepStealBoosts | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| hitStepTryHitEvent | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| hitStepInvulnerabilityEvent | `(` | ✓ |  |
| this | `BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove` |  |  |
| hitStepTypeImmunity | `(this: BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove)` | ✓ |  |
| moveHit | `(` | ✓ |  |
| this | `BattleActions, target: Pokemon | null, pokemon: Pokemon, move: ActiveMove,` |  |  |
| moveData | `ActiveMove, isSecondary?: boolean, isSelf?: boolean` | ✓ |  |
| runAction | `(this: BattleActions, action: Action)` | ✓ |  |
| runMegaEvo | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| runMegaEvoX | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| runMegaEvoY | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| runMove | `(` | ✓ |  |
| this | `BattleActions, moveOrMoveName: Move | string, pokemon: Pokemon, targetLoc: number, options?: {` |  |  |
| sourceEffect | `Effect | null, zMove?: string, externalMove?: boolean,` | ✓ |  |
| maxMove | `string, originalTarget?: Pokemon,` | ✓ |  |
| runMoveEffects | `(` | ✓ |  |
| this | `BattleActions, damage: SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,` |  |  |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean` |  |  |
| runSwitch | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| runZPower | `(this: BattleActions, move: ActiveMove, pokemon: Pokemon)` | ✓ |  |
| secondaries | `(` | ✓ |  |
| this | `BattleActions, targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove,` |  |  |
| moveData | `ActiveMove, isSelf?: boolean` |  |  |
| selfDrops | `(` | ✓ |  |
| this | `BattleActions, targets: SpreadMoveTargets, source: Pokemon,` |  |  |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean` |  |  |
| spreadMoveHit | `(` | ✓ |  |
| this | `BattleActions, targets: SpreadMoveTargets, pokemon: Pokemon, move: ActiveMove,` |  |  |
| moveData | `ActiveMove, isSecondary?: boolean, isSelf?: boolean` | ✓ |  |
| switchIn | `(` | ✓ |  |
| this | `BattleActions, pokemon: Pokemon, pos: number, sourceEffect: Effect | null, isDrag?: boolean` |  |  |
| targetTypeChoices | `(this: BattleActions, targetType: string)` | ✓ |  |
| terastallize | `(this: BattleActions, pokemon: Pokemon)` | ✓ |  |
| tryMoveHit | `(` | ✓ |  |
| this | `BattleActions, target: Pokemon, pokemon: Pokemon, move: ActiveMove` |  |  |
| tryPrimaryHitEvent | `(` | ✓ |  |
| this | `BattleActions, damage: SpreadMoveDamage, targets: SpreadMoveTargets, pokemon: Pokemon,` |  |  |
| move | `ActiveMove, moveData: ActiveMove, isSecondary?: boolean` |  |  |
| trySpreadMoveHit | `(` | ✓ |  |
| this | `BattleActions, targets: Pokemon[], pokemon: Pokemon, move: ActiveMove, notActive?: boolean` |  |  |
| useMove | `(` | ✓ |  |
| this | `BattleActions, move: Move, pokemon: Pokemon, options?: {` |  |  |
| target | `Pokemon | null, sourceEffect?: Effect | null,` | ✓ |  |
| zMove | `string, maxMove?: string,` | ✓ |  |
| useMoveInner | `(` | ✓ |  |
| this | `BattleActions, move: Move, pokemon: Pokemon, options?: {` |  |  |
| target | `Pokemon | null, sourceEffect?: Effect | null,` | ✓ |  |
| zMove | `string, maxMove?: string,` | ✓ |  |
| getDamage | `(` | ✓ |  |
| this | `BattleActions, pokemon: Pokemon, target: Pokemon, move: string | number | ActiveMove, suppressMessages: boolean` |  |  |
| modifyDamage | `(` | ✓ |  |
| this | `BattleActions, baseDamage: number, pokemon: Pokemon, target: Pokemon, move: ActiveMove, suppressMessages?: boolean` |  |  |
| mutateOriginalSpecies | `(this: BattleActions, species: Species, deltas: AnyObject)` | ✓ |  |
| getFormeChangeDeltas | `(this: BattleActions, formeChangeSpecies: Species, pokemon?: Pokemon)` | ✓ |  |
| getMixedSpecies | `(this: BattleActions, originalName: string, megaName: string, pokemon?: Pokemon)` | ✓ |  |

---

### ModdedBattlePokemon

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| inherit | `true` | ✓ |  |
| lostItemForDelibird | `Item | null` | ✓ |  |
| boostBy | `(this: Pokemon, boost: SparseBoostsTable)` | ✓ |  |
| clearBoosts | `(this: Pokemon)` | ✓ |  |
| calculateStat | `(this: Pokemon, statName: StatIDExceptHP, boost: number, modifier?: number)` | ✓ |  |
| cureStatus | `(this: Pokemon, silent?: boolean)` | ✓ |  |
| deductPP | `(` | ✓ |  |
| this | `Pokemon, move: string | Move, amount?: number | null, target?: Pokemon | null | false` |  |  |
| eatItem | `(this: Pokemon, force?: boolean, source?: Pokemon, sourceEffect?: Effect)` | ✓ |  |
| effectiveWeather | `(this: Pokemon)` | ✓ |  |
| formeChange | `(` | ✓ |  |
| this | `Pokemon, speciesId: string | Species, source: Effect, isPermanent?: boolean, message?: string` |  |  |
| hasType | `(this: Pokemon, type: string | string[])` | ✓ |  |
| getAbility | `(this: Pokemon)` | ✓ |  |
| getActionSpeed | `(this: Pokemon)` | ✓ |  |
| getItem | `(this: Pokemon)` | ✓ |  |
| getMoveRequestData | `(this: Pokemon)` | ✓ |  |
| moves | `{ move: string, id: ID, target?: string, disabled?: boolean }[],` |  |  |
| maybeDisabled | `boolean, trapped?: boolean, maybeTrapped?: boolean,` | ✓ |  |
| canMegaEvo | `boolean, canUltraBurst?: boolean, canZMove?: ZMoveOptions,` | ✓ |  |
| getMoves | `(this: Pokemon, lockedMove?: string | null, restrictData?: boolean)` | ✓ |  |
| move | `string, id: string, disabled?: string | boolean, disabledSource?: string,` |  |  |
| target | `string, pp?: number, maxpp?: number,` | ✓ |  |
| getMoveTargets | `(this: Pokemon, move: ActiveMove, target: Pokemon)` | ✓ |  |
| targets | `Pokemon[], pressureTargets: Pokemon[],` |  |  |
| getStat | `(` | ✓ |  |
| this | `Pokemon, statName: StatIDExceptHP, unboosted?: boolean, unmodified?: boolean, fastReturn?: boolean` |  |  |
| getTypes | `(this: Pokemon, excludeAdded?: boolean, preterastallized?: boolean)` | ✓ |  |
| getWeight | `(this: Pokemon)` | ✓ |  |
| hasAbility | `(this: Pokemon, ability: string | string[])` | ✓ |  |
| hasItem | `(this: Pokemon, item: string | string[])` | ✓ |  |
| isGrounded | `(this: Pokemon, negateImmunity: boolean | undefined)` | ✓ |  |
| modifyStat | `(this: Pokemon, statName: StatIDExceptHP, modifier: number)` | ✓ |  |
| moveUsed | `(this: Pokemon, move: ActiveMove, targetLoc?: number)` | ✓ |  |
| recalculateStats | `(this: Pokemon)` | ✓ |  |
| runEffectiveness | `(this: Pokemon, move: ActiveMove)` | ✓ |  |
| runImmunity | `(this: Pokemon, source: ActiveMove | string, message?: string | boolean)` | ✓ |  |
| setAbility | `(` | ✓ |  |
| this | `Pokemon, ability: string | Ability, source?: Pokemon | null, sourceEffect?: Effect | null,` |  |  |
| isFromFormeChange | `boolean, isTransform?: boolean` | ✓ |  |
| setItem | `(this: Pokemon, item: string | Item, source?: Pokemon, effect?: Effect)` | ✓ |  |
| setStatus | `(` | ✓ |  |
| this | `Pokemon, status: string | Condition, source: Pokemon | null,` |  |  |
| sourceEffect | `Effect | null, ignoreImmunities: boolean` |  |  |
| takeItem | `(this: Pokemon, source: Pokemon | undefined)` | ✓ |  |
| transformInto | `(this: Pokemon, pokemon: Pokemon, effect: Effect | null)` | ✓ |  |
| useItem | `(this: Pokemon, source?: Pokemon, sourceEffect?: Effect)` | ✓ |  |
| ignoringAbility | `(this: Pokemon)` | ✓ |  |
| ignoringItem | `(this: Pokemon)` | ✓ |  |
| getLinkedMoves | `(this: Pokemon, ignoreDisabled?: boolean)` | ✓ |  |
| hasLinkedMove | `(this: Pokemon, moveid: string)` | ✓ |  |

---

### ModdedBattleSide

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| inherit | `true` | ✓ |  |
| addSideCondition | `(` | ✓ |  |
| this | `Side, status: string | Condition, source: Pokemon | 'debug' | null, sourceEffect: Effect | null` |  |  |
| allies | `(this: Side, all?: boolean)` | ✓ |  |
| canDynamaxNow | `(this: Side)` | ✓ |  |
| chooseSwitch | `(this: Side, slotText?: string)` | ✓ |  |
| getChoice | `(this: Side)` | ✓ |  |
| getRequestData | `(this: Side, forAlly?: boolean)` | ✓ |  |

---

### ModdedConditionDataTable

**File:** `sim/dex-conditions.ts`

---

### ModdedFormatDataTable

**File:** `sim/dex-formats.ts`

---

### ModdedItemDataTable

**File:** `sim/dex-items.ts`

---

### ModdedLearnsetDataTable

**File:** `sim/dex-species.ts`

---

### ModdedMoveDataTable

**File:** `sim/dex-moves.ts`

---

### ModdedSpeciesDataTable

**File:** `sim/dex-species.ts`

---

### ModdedSpeciesFormatsDataTable

**File:** `sim/dex-species.ts`

---

### ModdedTypeDataTable

**File:** `sim/dex-data.ts`

---

### MoveAction

**File:** `sim/battle-queue.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choice | `'move' | 'beforeTurnMove' | 'priorityChargeMove'` |  |  |
| order | `3 | 5 | 200 | 201 | 199 | 106` |  |  |
| priority | `number` |  |  |
| fractionalPriority | `number` |  |  |
| speed | `number` |  |  |
| pokemon | `Pokemon` |  |  |
| targetLoc | `number` |  |  |
| originalTarget | `Pokemon` |  |  |
| moveid | `ID` |  |  |
| move | `Move` |  |  |
| mega | `boolean | 'done'` |  |  |
| zmove | `string` | ✓ |  |
| maxMove | `string` | ✓ |  |
| sourceEffect | `Effect | null` | ✓ |  |

---

### MoveData

**File:** `sim/dex-moves.ts`

**Extends:** EffectData, MoveEventMethods, HitEffect

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| num | `number` | ✓ |  |
| condition | `ConditionData` | ✓ |  |
| basePower | `number` |  |  |
| accuracy | `true | number` |  |  |
| pp | `number` |  |  |
| category | `'Physical' | 'Special' | 'Status'` |  |  |
| type | `string` |  |  |
| priority | `number` |  |  |
| target | `MoveTarget` |  |  |
| flags | `MoveFlags` |  |  |
| realMove | `string` | ✓ |  |
| damage | `number | 'level' | false | null` | ✓ |  |
| contestType | `string` | ✓ |  |
| noPPBoosts | `boolean` | ✓ |  |
| isZ | `boolean | IDEntry` | ✓ |  |
| zMove | `{` | ✓ |  |
| basePower | `number,` | ✓ |  |
| effect | `IDEntry,` | ✓ |  |
| boost | `SparseBoostsTable,` | ✓ |  |
| isMax | `boolean | string` | ✓ |  |
| maxMove | `{` | ✓ |  |
| basePower | `number,` |  |  |
| ohko | `boolean | 'Ice'` | ✓ |  |
| thawsTarget | `boolean` | ✓ |  |
| heal | `number[] | null` | ✓ |  |
| forceSwitch | `boolean` | ✓ |  |
| selfSwitch | `'copyvolatile' | 'shedtail' | boolean` | ✓ |  |
| selfBoost | `{ boosts?: SparseBoostsTable }` | ✓ |  |
| selfdestruct | `'always' | 'ifHit' | boolean` | ✓ |  |
| breaksProtect | `boolean` | ✓ |  |
| recoil | `[number, number]` | ✓ |  |
| drain | `[number, number]` | ✓ |  |
| mindBlownRecoil | `boolean` | ✓ |  |
| stealsBoosts | `boolean` | ✓ |  |
| struggleRecoil | `boolean` | ✓ |  |
| secondary | `SecondaryEffect | null` | ✓ |  |
| secondaries | `SecondaryEffect[] | null` | ✓ |  |
| self | `SecondaryEffect | null` | ✓ |  |
| hasSheerForce | `boolean` | ✓ |  |
| alwaysHit | `boolean` | ✓ |  |
| baseMoveType | `string` | ✓ |  |
| basePowerModifier | `number` | ✓ |  |
| critModifier | `number` | ✓ |  |
| critRatio | `number` | ✓ |  |
| overrideOffensivePokemon | `'target' | 'source'` | ✓ |  |
| overrideOffensiveStat | `StatIDExceptHP` | ✓ |  |
| overrideDefensivePokemon | `'target' | 'source'` | ✓ |  |
| overrideDefensiveStat | `StatIDExceptHP` | ✓ |  |
| forceSTAB | `boolean` | ✓ |  |
| ignoreAbility | `boolean` | ✓ |  |
| ignoreAccuracy | `boolean` | ✓ |  |
| ignoreDefensive | `boolean` | ✓ |  |
| ignoreEvasion | `boolean` | ✓ |  |
| ignoreImmunity | `boolean | { [typeName: string]: boolean }` | ✓ |  |
| ignoreNegativeOffensive | `boolean` | ✓ |  |
| ignoreOffensive | `boolean` | ✓ |  |
| ignorePositiveDefensive | `boolean` | ✓ |  |
| ignorePositiveEvasion | `boolean` | ✓ |  |
| multiaccuracy | `boolean` | ✓ |  |
| multihit | `number | number[]` | ✓ |  |
| multihitType | `'parentalbond'` | ✓ |  |
| noDamageVariance | `boolean` | ✓ |  |
| nonGhostTarget | `MoveTarget` | ✓ |  |
| spreadModifier | `number` | ✓ |  |
| sleepUsable | `boolean` | ✓ |  |
| smartTarget | `boolean` | ✓ |  |
| tracksTarget | `boolean` | ✓ |  |
| willCrit | `boolean` | ✓ |  |
| callsMove | `boolean` | ✓ |  |
| hasCrashDamage | `boolean` | ✓ |  |
| isConfusionSelfHit | `boolean` | ✓ |  |
| stallingMove | `boolean` | ✓ |  |
| baseMove | `ID` | ✓ |  |

---

### MoveDataTable

**File:** `sim/dex-moves.ts`

---

### MoveEventMethods

**File:** `sim/dex-moves.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| basePowerCallback | `(this: Battle, pokemon: Pokemon, target: Pokemon, move: ActiveMove)` | ✓ |  |
| beforeMoveCallback | `(this: Battle, pokemon: Pokemon, target: Pokemon | null, move: ActiveMove)` | ✓ |  |
| beforeTurnCallback | `(this: Battle, pokemon: Pokemon, target: Pokemon)` | ✓ |  |
| damageCallback | `(this: Battle, pokemon: Pokemon, target: Pokemon)` | ✓ |  |
| priorityChargeCallback | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onDisableMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAfterHit | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAfterSubDamage | `(this: Battle, damage: number, target: Pokemon, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onAfterMoveSecondarySelf | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAfterMoveSecondary | `CommonHandlers['VoidMove']` | ✓ |  |
| onAfterMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onDamagePriority | `number` | ✓ |  |
| onDamage | `(` | ✓ |  |
| this | `Battle, damage: number, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onBasePower | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onEffectiveness | `(` | ✓ |  |
| this | `Battle, typeMod: number, target: Pokemon | null, type: string, move: ActiveMove` |  |  |
| onHit | `CommonHandlers['ResultMove']` | ✓ |  |
| onHitField | `CommonHandlers['ResultMove']` | ✓ |  |
| onHitSide | `(this: Battle, side: Side, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onModifyMove | `(this: Battle, move: ActiveMove, pokemon: Pokemon, target: Pokemon | null)` | ✓ |  |
| onModifyPriority | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onMoveFail | `CommonHandlers['VoidMove']` | ✓ |  |
| onModifyType | `(this: Battle, move: ActiveMove, pokemon: Pokemon, target: Pokemon)` | ✓ |  |
| onModifyTarget | `(` | ✓ |  |
| this | `Battle, relayVar: { target: Pokemon }, pokemon: Pokemon, target: Pokemon, move: ActiveMove` |  |  |
| onPrepareHit | `CommonHandlers['ResultMove']` | ✓ |  |
| onTry | `CommonHandlers['ResultSourceMove']` | ✓ |  |
| onTryHit | `CommonHandlers['ExtResultSourceMove']` | ✓ |  |
| onTryHitField | `CommonHandlers['ResultMove']` | ✓ |  |
| onTryHitSide | `(this: Battle, side: Side, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onTryImmunity | `CommonHandlers['ResultMove']` | ✓ |  |
| onTryMove | `CommonHandlers['ResultSourceMove']` | ✓ |  |
| onUseMoveMessage | `CommonHandlers['VoidSourceMove']` | ✓ |  |

---

### MoveFlags

**File:** `sim/dex-moves.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| allyanim | `1` | ✓ |  |
| bypasssub | `1` | ✓ |  |
| bite | `1` | ✓ |  |
| bullet | `1` | ✓ |  |
| cantusetwice | `1` | ✓ |  |
| charge | `1` | ✓ |  |
| contact | `1` | ✓ |  |
| dance | `1` | ✓ |  |
| defrost | `1` | ✓ |  |
| distance | `1` | ✓ |  |
| failcopycat | `1` | ✓ |  |
| failencore | `1` | ✓ |  |
| failinstruct | `1` | ✓ |  |
| failmefirst | `1` | ✓ |  |
| failmimic | `1` | ✓ |  |
| futuremove | `1` | ✓ |  |
| gravity | `1` | ✓ |  |
| heal | `1` | ✓ |  |
| metronome | `1` | ✓ |  |
| mirror | `1` | ✓ |  |
| mustpressure | `1` | ✓ |  |
| noassist | `1` | ✓ |  |
| nonsky | `1` | ✓ |  |
| noparentalbond | `1` | ✓ |  |
| nosketch | `1` | ✓ |  |
| nosleeptalk | `1` | ✓ |  |
| pledgecombo | `1` | ✓ |  |
| powder | `1` | ✓ |  |
| protect | `1` | ✓ |  |
| pulse | `1` | ✓ |  |
| punch | `1` | ✓ |  |
| recharge | `1` | ✓ |  |
| reflectable | `1` | ✓ |  |
| slicing | `1` | ✓ |  |
| snatch | `1` | ✓ |  |
| sound | `1` | ✓ |  |
| wind | `1` | ✓ |  |

---

### MoveHitData

**File:** `sim/dex-moves.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| crit | `boolean,` |  |  |
| typeMod | `number,` |  |  |
| zBrokeProtect | `boolean,` |  |  |

---

### MoveRequest

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| wait | `undefined` | ✓ |  |
| teamPreview | `undefined` | ✓ |  |
| forceSwitch | `undefined` | ✓ |  |
| active | `PokemonMoveRequestData[]` |  |  |
| side | `SideRequestData` |  |  |
| ally | `SideRequestData` | ✓ |  |
| noCancel | `boolean` | ✓ |  |
| update | `boolean` | ✓ |  |

---

### MoveSlot

**File:** `sim/pokemon.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `ID` |  |  |
| move | `string` |  |  |
| pp | `number` |  |  |
| maxpp | `number` |  |  |
| target | `string` | ✓ |  |
| disabled | `boolean | 'hidden'` |  |  |
| disabledSource | `string` | ✓ |  |
| used | `boolean` |  |  |
| virtual | `boolean` | ✓ |  |

---

### MoveTextData

**File:** `sim/global-types.ts`

**Extends:** ConditionTextData

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| alreadyStarted | `string` | ✓ |  |
| blockSelf | `string` | ✓ |  |
| clearBoost | `string` | ✓ |  |
| endFromItem | `string` | ✓ |  |
| fail | `string` | ✓ |  |
| failSelect | `string` | ✓ |  |
| failTooHeavy | `string` | ✓ |  |
| failWrongForme | `string` | ✓ |  |
| megaNoItem | `string` | ✓ |  |
| prepare | `string` | ✓ |  |
| removeItem | `string` | ✓ |  |
| startFromItem | `string` | ✓ |  |
| startFromZEffect | `string` | ✓ |  |
| switchOut | `string` | ✓ |  |
| takeItem | `string` | ✓ |  |
| typeChange | `string` | ✓ |  |
| upkeep | `string` | ✓ |  |

---

### NatureData

**File:** `sim/dex-data.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| plus | `StatIDExceptHP` | ✓ |  |
| minus | `StatIDExceptHP` | ✓ |  |

---

### NatureDataTable

**File:** `sim/dex-data.ts`

---

### PlayerOptions

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` | ✓ |  |
| avatar | `string` | ✓ |  |
| rating | `number` | ✓ |  |
| team | `PokemonSet[] | string | null` | ✓ |  |
| seed | `PRNGSeed` | ✓ |  |

---

### PokemonAction

**File:** `sim/battle-queue.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choice | `'megaEvo' | 'megaEvoX' | 'megaEvoY' | 'shift' | 'runSwitch' | 'event' | 'runDynamax' | 'terastallize'` |  |  |
| priority | `number` |  |  |
| speed | `number` |  |  |
| pokemon | `Pokemon` |  |  |
| dragger | `Pokemon` | ✓ |  |
| event | `string` | ✓ |  |

---

### PokemonEventMethods

**File:** `sim/dex-conditions.ts`

**Extends:** EventMethods

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| onAllyDamagingHit | `(this: Battle, damage: number, target: Pokemon, source: Pokemon, move: ActiveMove)` | ✓ |  |
| onAllyAfterEachBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon)` | ✓ |  |
| onAllyAfterHit | `MoveEventMethods['onAfterHit']` | ✓ |  |
| onAllyAfterSetStatus | `(this: Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAllyAfterSubDamage | `MoveEventMethods['onAfterSubDamage']` | ✓ |  |
| onAllyAfterSwitchInSelf | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyAfterUseItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAllyAfterBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAllyAfterFaint | `(this: Battle, length: number, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAllyAfterMoveSecondarySelf | `MoveEventMethods['onAfterMoveSecondarySelf']` | ✓ |  |
| onAllyAfterMoveSecondary | `MoveEventMethods['onAfterMoveSecondary']` | ✓ |  |
| onAllyAfterMove | `MoveEventMethods['onAfterMove']` | ✓ |  |
| onAllyAfterMoveSelf | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAllyAttract | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onAllyAccuracy | `(` | ✓ |  |
| this | `Battle, accuracy: number, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onAllyBasePower | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyBeforeFaint | `(this: Battle, pokemon: Pokemon, effect: Effect)` | ✓ |  |
| onAllyBeforeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAllyBeforeSwitchIn | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyBeforeSwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyTryBoost | `(this: Battle, boost: SparseBoostsTable, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAllyChargeMove | `CommonHandlers['VoidSourceMove']` | ✓ |  |
| onAllyCriticalHit | `((this: Battle, pokemon: Pokemon, source: null, move: ActiveMove)` | ✓ |  |
| onAllyDamage | `(` | ✓ |  |
| this | `Battle, damage: number, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onAllyDeductPP | `(this: Battle, target: Pokemon, source: Pokemon)` | ✓ |  |
| onAllyDisableMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyDragOut | `(this: Battle, pokemon: Pokemon, source?: Pokemon, move?: ActiveMove)` | ✓ |  |
| onAllyEatItem | `(this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAllyEffectiveness | `MoveEventMethods['onEffectiveness']` | ✓ |  |
| onAllyFaint | `CommonHandlers['VoidEffect']` | ✓ |  |
| onAllyFlinch | `((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyHit | `MoveEventMethods['onHit']` | ✓ |  |
| onAllyImmunity | `(this: Battle, type: string, pokemon: Pokemon)` | ✓ |  |
| onAllyLockMove | `string | ((this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyMaybeTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyModifyAccuracy | `CommonHandlers['ModifierMove']` | ✓ |  |
| onAllyModifyAtk | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifyBoost | `(this: Battle, boosts: SparseBoostsTable, pokemon: Pokemon)` | ✓ |  |
| onAllyModifyCritRatio | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifyDef | `CommonHandlers['ModifierMove']` | ✓ |  |
| onAllyModifyMove | `MoveEventMethods['onModifyMove']` | ✓ |  |
| onAllyModifyPriority | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifySecondaries | `(` | ✓ |  |
| this | `Battle, secondaries: SecondaryEffect[], target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onAllyModifySpA | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifySpD | `CommonHandlers['ModifierMove']` | ✓ |  |
| onAllyModifySpe | `(this: Battle, spe: number, pokemon: Pokemon)` | ✓ |  |
| onAllyModifySTAB | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifyType | `MoveEventMethods['onModifyType']` | ✓ |  |
| onAllyModifyTarget | `MoveEventMethods['onModifyTarget']` | ✓ |  |
| onAllyModifyWeight | `(this: Battle, weighthg: number, pokemon: Pokemon)` | ✓ |  |
| onAllyMoveAborted | `CommonHandlers['VoidMove']` | ✓ |  |
| onAllyNegateImmunity | `((this: Battle, pokemon: Pokemon, type: string)` | ✓ |  |
| onAllyOverrideAction | `(this: Battle, pokemon: Pokemon, target: Pokemon, move: ActiveMove)` | ✓ |  |
| onAllyPrepareHit | `CommonHandlers['ResultSourceMove']` | ✓ |  |
| onAllyRedirectTarget | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, source2: Effect, move: ActiveMove` |  |  |
| onAllyResidual | `(this: Battle, target: Pokemon & Side, source: Pokemon, effect: Effect)` | ✓ |  |
| onAllySetAbility | `(this: Battle, ability: string, target: Pokemon, source: Pokemon, effect: Effect)` | ✓ |  |
| onAllySetStatus | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, effect: Effect` |  |  |
| onAllySetWeather | `(this: Battle, target: Pokemon, source: Pokemon, weather: Condition)` | ✓ |  |
| onAllyStallMove | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllySwitchOut | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyTakeItem | `(` | ✓ |  |
| onAllyTerrain | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyTrapPokemon | `(this: Battle, pokemon: Pokemon)` | ✓ |  |
| onAllyTryAddVolatile | `(` | ✓ |  |
| this | `Battle, status: Condition, target: Pokemon, source: Pokemon, sourceEffect: Effect` |  |  |
| onAllyTryEatItem | `boolean | ((this: Battle, item: Item, pokemon: Pokemon)` | ✓ |  |
| onAllyTryHeal | `(` | ✓ |  |
| onAllyTryHit | `MoveEventMethods['onTryHit']` | ✓ |  |
| onAllyTryHitField | `MoveEventMethods['onTryHitField']` | ✓ |  |
| onAllyTryHitSide | `CommonHandlers['ResultMove']` | ✓ |  |
| onAllyInvulnerability | `CommonHandlers['ExtResultMove']` | ✓ |  |
| onAllyTryMove | `MoveEventMethods['onTryMove']` | ✓ |  |
| onAllyTryPrimaryHit | `(` | ✓ |  |
| this | `Battle, target: Pokemon, source: Pokemon, move: ActiveMove` |  |  |
| onAllyType | `(this: Battle, types: string[], pokemon: Pokemon)` | ✓ |  |
| onAllyWeatherModifyDamage | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifyDamagePhase1 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |
| onAllyModifyDamagePhase2 | `CommonHandlers['ModifierSourceMove']` | ✓ |  |

---

### PokemonGoData

**File:** `sim/dex-species.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| encounters | `string[]` | ✓ |  |
| LGPERestrictiveMoves | `{ [moveid: string]: number | null }` | ✓ |  |

---

### PokemonGoDataTable

**File:** `sim/dex-species.ts`

---

### PokemonMoveRequestData

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| moves | `{ move: string, id: ID, target?: string, disabled?: string | boolean, disabledSource?: string }[]` |  |  |
| maybeDisabled | `boolean` | ✓ |  |
| maybeLocked | `boolean` | ✓ |  |
| trapped | `boolean` | ✓ |  |
| maybeTrapped | `boolean` | ✓ |  |
| canMegaEvo | `boolean` | ✓ |  |
| canMegaEvoX | `boolean` | ✓ |  |
| canMegaEvoY | `boolean` | ✓ |  |
| canUltraBurst | `boolean` | ✓ |  |
| canZMove | `AnyObject | null` | ✓ |  |
| canDynamax | `boolean` | ✓ |  |
| maxMoves | `DynamaxOptions` | ✓ |  |
| canTerastallize | `string` | ✓ |  |

---

### PokemonSet

**File:** `sim/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| species | `string` |  |  |
| item | `string` |  |  |
| ability | `string` |  |  |
| moves | `string[]` |  |  |
| nature | `string` |  |  |
| gender | `string` |  |  |
| evs | `StatsTable` |  |  |
| ivs | `StatsTable` |  |  |
| level | `number` |  |  |
| shiny | `boolean` | ✓ |  |
| happiness | `number` | ✓ |  |
| pokeball | `string` | ✓ |  |
| hpType | `string` | ✓ |  |
| dynamaxLevel | `number` | ✓ |  |
| gigantamax | `boolean` | ✓ |  |
| teraType | `string` | ✓ |  |

---

### PokemonSwitchRequestData

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| ident | `string` |  |  |
| details | `string` |  |  |
| condition | `string` |  |  |
| active | `boolean` |  |  |
| stats | `StatsExceptHPTable` |  |  |
| moves | `ID[]` |  |  |
| baseAbility | `ID` |  |  |
| item | `ID` |  |  |
| pokeball | `ID` |  |  |
| ability | `ID` | ✓ |  |
| commanding | `boolean` | ✓ |  |
| reviving | `boolean` | ✓ |  |
| teraType | `string` | ✓ |  |
| terastallized | `string` | ✓ |  |

---

### Pools

**File:** `sim/tools/exhaustive-runner.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| pokemon | `Pool` |  |  |
| items | `Pool` |  |  |
| abilities | `Pool` |  |  |
| moves | `Pool` |  |  |

---

### RandomDraftFactorySet

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| species | `string` |  |  |
| gender | `string` |  |  |
| moves | `string[]` |  |  |
| ability | `string` |  |  |
| evs | `SparseStatsTable` |  |  |
| ivs | `SparseStatsTable` |  |  |
| item | `string` |  |  |
| level | `number` |  |  |
| shiny | `boolean` |  |  |
| nature | `string` | ✓ |  |
| happiness | `number` | ✓ |  |
| dynamaxLevel | `number` | ✓ |  |
| gigantamax | `boolean` | ✓ |  |
| teraType | `string` | ✓ |  |
| teraCaptain | `boolean` | ✓ |  |

---

### RandomFactorySet

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| species | `string` |  |  |
| gender | `string` |  |  |
| item | `string` |  |  |
| ability | `string` |  |  |
| shiny | `boolean` |  |  |
| level | `number` |  |  |
| happiness | `number` |  |  |
| evs | `SparseStatsTable` |  |  |
| ivs | `SparseStatsTable` |  |  |
| nature | `string` |  |  |
| moves | `string[]` |  |  |
| dynamaxLevel | `number` | ✓ |  |
| gigantamax | `boolean` | ✓ |  |
| wantsTera | `boolean` | ✓ |  |
| teraType | `string` | ✓ |  |

---

### RandomSet

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| species | `string` |  |  |
| gender | `string | boolean` |  |  |
| moves | `string[]` |  |  |
| ability | `string` |  |  |
| evs | `SparseStatsTable` |  |  |
| ivs | `SparseStatsTable` |  |  |
| item | `string` |  |  |
| level | `number` |  |  |
| shiny | `boolean` |  |  |
| nature | `string` | ✓ |  |
| happiness | `number` | ✓ |  |
| dynamaxLevel | `number` | ✓ |  |
| gigantamax | `boolean` | ✓ |  |
| teraType | `string` | ✓ |  |
| role | `Role` | ✓ |  |

---

### RandomSetData

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| role | `Role` |  |  |
| movepool | `string[]` |  |  |
| abilities | `string[]` | ✓ |  |
| teraTypes | `string[]` | ✓ |  |
| preferredTypes | `string[]` | ✓ |  |

---

### RandomSpeciesData

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| level | `number` | ✓ |  |
| sets | `RandomSetData[]` |  |  |

---

### RNG

**File:** `sim/prng.ts`

---

### RunnerOptions

**File:** `sim/tools/runner.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| format | `string` |  |  |
| prng | `PRNG | PRNGSeed | null` | ✓ |  |
| p1options | `AIOptions` | ✓ |  |
| p2options | `AIOptions` | ✓ |  |
| p3options | `AIOptions` | ✓ |  |
| p4options | `AIOptions` | ✓ |  |
| input | `boolean` | ✓ |  |
| output | `boolean` | ✓ |  |
| error | `boolean` | ✓ |  |
| dual | `boolean | 'debug'` | ✓ |  |

---

### SecondaryEffect

**File:** `sim/dex-moves.ts`

**Extends:** HitEffect

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| chance | `number` | ✓ |  |
| ability | `Ability` | ✓ |  |
| kingsrock | `boolean` | ✓ |  |
| self | `HitEffect` | ✓ |  |

---

### SideEventMethods

**File:** `sim/dex-conditions.ts`

**Extends:** EventMethods

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| onSideStart | `(this: Battle, target: Side, source: Pokemon, sourceEffect: Effect)` | ✓ |  |
| onSideRestart | `(this: Battle, target: Side, source: Pokemon, sourceEffect: Effect)` | ✓ |  |
| onSideResidual | `(this: Battle, target: Side, source: Pokemon, effect: Effect)` | ✓ |  |
| onSideEnd | `(this: Battle, target: Side)` | ✓ |  |
| onSideResidualOrder | `number` | ✓ |  |
| onSideResidualPriority | `number` | ✓ |  |
| onSideResidualSubOrder | `number` | ✓ |  |

---

### SideRequestData

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| id | `SideID` |  |  |
| pokemon | `PokemonSwitchRequestData[]` |  |  |
| noCancel | `boolean` | ✓ |  |

---

### SpeciesAbility

**File:** `sim/dex-species.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| 0 | `string` |  |  |
| 1 | `string` | ✓ |  |
| H | `string` | ✓ |  |
| S | `string` | ✓ |  |

---

### SpeciesDataTable

**File:** `sim/dex-species.ts`

---

### SpeciesFormatsData

**File:** `sim/dex-species.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| doublesTier | `TierTypes.Doubles | TierTypes.Other` | ✓ |  |
| gmaxUnreleased | `boolean` | ✓ |  |
| isNonstandard | `Nonstandard | null` | ✓ |  |
| natDexTier | `TierTypes.Singles | TierTypes.Other` | ✓ |  |
| tier | `TierTypes.Singles | TierTypes.Other` | ✓ |  |

---

### SpeciesFormatsDataTable

**File:** `sim/dex-species.ts`

---

### SwitchAction

**File:** `sim/battle-queue.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choice | `'switch' | 'instaswitch' | 'revivalblessing'` |  |  |
| order | `3 | 6 | 103` |  |  |
| priority | `number` |  |  |
| speed | `number` |  |  |
| pokemon | `Pokemon` |  |  |
| target | `Pokemon` |  |  |
| sourceEffect | `Effect | null` |  |  |

---

### SwitchRequest

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| wait | `undefined` | ✓ |  |
| teamPreview | `undefined` | ✓ |  |
| forceSwitch | `boolean[]` |  |  |
| side | `SideRequestData` |  |  |
| noCancel | `boolean` | ✓ |  |
| update | `boolean` | ✓ |  |

---

### TeamAction

**File:** `sim/battle-queue.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choice | `'team'` |  |  |
| priority | `number` |  |  |
| speed | `1` |  |  |
| pokemon | `Pokemon` |  |  |
| index | `number` |  |  |

---

### TeamDetails

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| megaStone | `number` | ✓ |  |
| zMove | `number` | ✓ |  |
| snow | `number` | ✓ |  |
| hail | `number` | ✓ |  |
| rain | `number` | ✓ |  |
| sand | `number` | ✓ |  |
| sun | `number` | ✓ |  |
| stealthRock | `number` | ✓ |  |
| spikes | `number` | ✓ |  |
| toxicSpikes | `number` | ✓ |  |
| stickyWeb | `number` | ✓ |  |
| rapidSpin | `number` | ✓ |  |
| defog | `number` | ✓ |  |
| screens | `number` | ✓ |  |
| illusion | `number` | ✓ |  |
| statusCure | `number` | ✓ |  |
| teraBlast | `number` | ✓ |  |
| imprison | `number` | ✓ |  |

---

### TeamPreviewRequest

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| wait | `undefined` | ✓ |  |
| teamPreview | `true` |  |  |
| forceSwitch | `undefined` | ✓ |  |
| maxChosenTeamSize | `number` | ✓ |  |
| side | `SideRequestData` |  |  |
| noCancel | `boolean` | ✓ |  |

---

### TextTableData

**File:** `sim/dex.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| Abilities | `DexTable<AbilityText>` |  |  |
| Items | `DexTable<ItemText>` |  |  |
| Moves | `DexTable<MoveText>` |  |  |
| Pokedex | `DexTable<PokedexText>` |  |  |
| Default | `DexTable<DefaultText>` |  |  |

---

### TypeData

**File:** `sim/dex-data.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| damageTaken | `{ [attackingTypeNameOrEffectid: string]: number }` |  |  |
| HPdvs | `SparseStatsTable` | ✓ |  |
| HPivs | `SparseStatsTable` | ✓ |  |
| isNonstandard | `Nonstandard | null` | ✓ |  |

---

### TypeDataTable

**File:** `sim/dex-data.ts`

---

### WaitRequest

**File:** `sim/side.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| wait | `true` |  |  |
| teamPreview | `undefined` | ✓ |  |
| forceSwitch | `undefined` | ✓ |  |
| side | `SideRequestData` |  |  |
| noCancel | `boolean` | ✓ |  |

---

## Type Aliases

### Ability

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-abilities').Ability
```

---

### AbilityText

**File:** `sim/global-types.ts`

**Definition:**
```typescript
TextFile<ConditionTextData & {
	activateFromItem?: string,
	activateNoTarget?: string,
	copyBoost?: string,
	transformEnd?: string,
}>
```

---

### Action

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./battle-queue').Action
```

---

### ActionChoice

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./battle-queue').ActionChoice
```

---

### ActiveMove

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-moves').ActiveMove
```

---

### Battle

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./battle').Battle
```

---

### BattleActions

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./battle-actions').BattleActions
```

---

### BattleQueue

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./battle-queue').BattleQueue
```

---

### BoostID

**File:** `sim/global-types.ts`

**Definition:**
```typescript
StatIDExceptHP | 'accuracy' | 'evasion'
```

---

### BoostsTable

**File:** `sim/global-types.ts`

**Definition:**
```typescript
{ [boost in BoostID]: number }
```

---

### ChannelID

**File:** `sim/battle.ts`

**Definition:**
```typescript
0 | 1 | 2 | 3 | 4
```

---

### ChoiceRequest

**File:** `sim/side.ts`

**Definition:**
```typescript
SwitchRequest | TeamPreviewRequest | MoveRequest | WaitRequest
```

---

### ComplexBan

**File:** `sim/dex-formats.ts`

**Definition:**
```typescript
[string, string, number, string[]]
```

---

### ComplexTeamBan

**File:** `sim/dex-formats.ts`

**Definition:**
```typescript
ComplexBan
```

---

### Condition

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-conditions').Condition
```

---

### ConditionData

**File:** `sim/dex-conditions.ts`

**Definition:**
```typescript
PokemonConditionData | SideConditionData | FieldConditionData
```

---

### DataType

**File:** `sim/dex.ts`

**Definition:**
```typescript
'Abilities' | 'Rulesets' | 'FormatsData' | 'Items' | 'Learnsets' | 'Moves' |
	'Natures' | 'Pokedex' | 'Scripts' | 'Conditions' | 'TypeChart' | 'PokemonGoData'
```

---

### DefaultText

**File:** `sim/global-types.ts`

**Definition:**
```typescript
AnyObject
```

---

### Doubles

**File:** `sim/global-types.ts`

**Definition:**
```typescript
"DUber" | "(DUber)" | "DOU" | "(DOU)" | "DBL" | "DUU" | "(DUU)" | "NFE" | "LC"
```

---

### Effect

**File:** `sim/global-types.ts`

**Definition:**
```typescript
Ability | Item | ActiveMove | Species | Condition | Format
```

---

### EffectType

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'Condition' | 'Pokemon' | 'Move' | 'Item' | 'Ability' | 'Format' |
	'Nature' | 'Ruleset' | 'Weather' | 'Status' | 'Terrain' | 'Rule' | 'ValidatorRule'
```

---

### Field

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./field').Field
```

---

### Format

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-formats').Format
```

---

### FormatEffectType

**File:** `sim/dex-formats.ts`

**Definition:**
```typescript
'Format' | 'Ruleset' | 'Rule' | 'ValidatorRule'
```

---

### FormatList

**File:** `sim/dex-formats.ts`

**Definition:**
```typescript
(FormatData | { section: string, column?: number })[]
```

---

### GameType

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'singles' | 'doubles' | 'triples' | 'rotation' | 'multi' | 'freeforall'
```

---

### Gen3RNGSeed

**File:** `sim/prng.ts`

**Definition:**
```typescript
['gen3', number]
```

---

### Gen5RNGSeed

**File:** `sim/prng.ts`

**Definition:**
```typescript
[number, number, number, number]
```

---

### GenderName

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'M' | 'F' | 'N' | ''
```

---

### HitEffect

**File:** `sim/dex.ts`

**Definition:**
```typescript
import('./dex-moves').HitEffect
```

---

### ID

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'' | Lowercase<string> & { __isID: true }
```

---

### IDEntry

**File:** `sim/global-types.ts`

**Definition:**
```typescript
Lowercase<string>
```

---

### Item

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-items').Item
```

---

### ItemText

**File:** `sim/global-types.ts`

**Definition:**
```typescript
TextFile<ConditionTextData>
```

---

### ModdedAbilityData

**File:** `sim/dex-abilities.ts`

**Definition:**
```typescript
AbilityData | Partial<AbilityData> & { inherit: true }
```

---

### ModdedConditionData

**File:** `sim/dex-conditions.ts`

**Definition:**
```typescript
ConditionData & { inherit?: true }
```

---

### ModdedDex

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex').ModdedDex
```

---

### ModdedEffectData

**File:** `sim/global-types.ts`

**Definition:**
```typescript
EffectData | Partial<EffectData> & { inherit: true }
```

---

### ModdedFormatData

**File:** `sim/dex-formats.ts`

**Definition:**
```typescript
FormatData | Omit<FormatData, 'name'> & { inherit: true }
```

---

### ModdedItemData

**File:** `sim/dex-items.ts`

**Definition:**
```typescript
ItemData | Partial<Omit<ItemData, 'name'>> & {
	inherit: true,
	onCustap?: (this: Battle, pokemon: Pokemon) => void,
}
```

---

### ModdedLearnsetData

**File:** `sim/dex-species.ts`

**Definition:**
```typescript
LearnsetData & { inherit?: true }
```

---

### ModdedMoveData

**File:** `sim/dex-moves.ts`

**Definition:**
```typescript
MoveData | Partial<Omit<MoveData, 'name'>> & {
	inherit: true,
	igniteBoosted?: boolean,
	settleBoosted?: boolean,
	bodyofwaterBoosted?: boolean,
	longWhipBoost?: boolean,
	gen?: number,
}
```

---

### ModdedNatureData

**File:** `sim/dex-data.ts`

**Definition:**
```typescript
NatureData | Partial<Omit<NatureData, 'name'>> & { inherit: true }
```

---

### ModdedSpeciesData

**File:** `sim/dex-species.ts`

**Definition:**
```typescript
SpeciesData | CosmeticFormeData |
	Partial<Omit<SpeciesData, 'name'>> & { inherit: true } |
	Partial<Omit<CosmeticFormeData, 'isCosmeticForme'>> & { inherit: true }
```

---

### ModdedSpeciesFormatsData

**File:** `sim/dex-species.ts`

**Definition:**
```typescript
SpeciesFormatsData & { inherit?: true }
```

---

### ModdedTypeData

**File:** `sim/dex-data.ts`

**Definition:**
```typescript
TypeData | Partial<Omit<TypeData, 'name'>> & { inherit: true }
```

---

### Move

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-moves').Move
```

---

### MoveAction

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./battle-queue').MoveAction
```

---

### MoveCategory

**File:** `sim/dex-moves.ts`

**Definition:**
```typescript
'Physical' | 'Special' | 'Status'
```

---

### MoveSource

**File:** `sim/dex-species.ts`

**Definition:**
```typescript
`${
	1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
}${
	'M' | 'T' | 'L' | 'R' | 'E' | 'D' | 'S' | 'V' | 'C'
}${string}`
```

---

### MoveTarget

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-moves').MoveTarget
```

---

### MoveText

**File:** `sim/global-types.ts`

**Definition:**
```typescript
TextFile<MoveTextData>
```

---

### MutableMove

**File:** `sim/dex-moves.ts`

**Definition:**
```typescript
BasicEffect & MoveData
```

---

### Nature

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-data').Nature
```

---

### Nonstandard

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'Past' | 'Future' | 'Unobtainable' | 'CAP' | 'LGPE' | 'Custom' | 'Gigantamax'
```

---

### Other

**File:** `sim/global-types.ts`

**Definition:**
```typescript
"Unreleased" | "Illegal" | "CAP" | "CAP NFE" | "CAP LC"
```

---

### Part

**File:** `sim/battle.ts`

**Definition:**
```typescript
string | number | boolean | Pokemon | Side | Effect | Move | null | undefined
```

---

### PokedexText

**File:** `sim/global-types.ts`

**Definition:**
```typescript
TextFile<BasicTextData>
```

---

### Pokemon

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./pokemon').Pokemon
```

---

### PokemonSet

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./teams').PokemonSet
```

---

### PokemonSlot

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'' | IDEntry & { __isSlot: true }
```

---

### PokemonSource

**File:** `sim/team-validator.ts`

**Definition:**
```typescript
string
```

---

### PokemonSources

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./team-validator').PokemonSources
```

---

### PRNGSeed

**File:** `sim/prng.ts`

**Definition:**
```typescript
`${'sodium' | 'gen5' | number},${string}`
```

---

### Referable

**File:** `sim/state.ts`

**Definition:**
```typescript
Battle | Field | Side | Pokemon | Condition | Ability | Item | Move | Species
```

---

### RequestState

**File:** `sim/battle.ts`

**Definition:**
```typescript
'teampreview' | 'move' | 'switch' | ''
```

---

### Role

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'' | 'Fast Attacker' | 'Setup Sweeper' | 'Wallbreaker' | 'Tera Blast user' |
		'Bulky Attacker' | 'Bulky Setup' | 'Fast Bulky Setup' | 'Bulky Support' | 'Fast Support' | 'AV Pivot' |
		'Doubles Fast Attacker' | 'Doubles Setup Sweeper' | 'Doubles Wallbreaker' | 'Doubles Bulky Attacker' |
		'Doubles Bulky Setup' | 'Offensive Protect' | 'Bulky Protect' | 'Doubles Support' | 'Choice Item user' |
		'Z-Move user' | 'Staller' | 'Spinner' | 'Generalist' | 'Berry Sweeper' | 'Thief user' | 'Imprisoner'
```

---

### RuleTable

**File:** `sim/dex.ts`

**Definition:**
```typescript
import('./dex-formats').RuleTable
```

---

### SecondaryEffect

**File:** `sim/dex.ts`

**Definition:**
```typescript
import('./dex-moves').SecondaryEffect
```

---

### Side

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./side').Side
```

---

### SideID

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'p1' | 'p2' | 'p3' | 'p4'
```

---

### Singles

**File:** `sim/global-types.ts`

**Definition:**
```typescript
"AG" | "Uber" | "(Uber)" | "OU" | "(OU)" | "UUBL" | "UU" | "RUBL" | "RU" | "NUBL" | "NU" |
		"(NU)" | "PUBL" | "PU" | "(PU)" | "ZUBL" | "ZU" | "NFE" | "LC"
```

---

### SodiumRNGSeed

**File:** `sim/prng.ts`

**Definition:**
```typescript
['sodium', string]
```

---

### SparseBoostsTable

**File:** `sim/global-types.ts`

**Definition:**
```typescript
Partial<BoostsTable>
```

---

### SparseStatsTable

**File:** `sim/global-types.ts`

**Definition:**
```typescript
Partial<StatsTable>
```

---

### Species

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-species').Species
```

---

### SpeciesTag

**File:** `sim/dex-species.ts`

**Definition:**
```typescript
"Mythical" | "Restricted Legendary" | "Sub-Legendary" | "Ultra Beast" | "Paradox"
```

---

### SpreadMoveDamage

**File:** `sim/global-types.ts`

**Definition:**
```typescript
(number | boolean | undefined)[]
```

---

### SpreadMoveTargets

**File:** `sim/global-types.ts`

**Definition:**
```typescript
(Pokemon | false | null)[]
```

---

### StatID

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'hp' | StatIDExceptHP
```

---

### StatIDExceptHP

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'atk' | 'def' | 'spa' | 'spd' | 'spe'
```

---

### StatsExceptHPTable

**File:** `sim/global-types.ts`

**Definition:**
```typescript
{ [stat in StatIDExceptHP]: number }
```

---

### StatsTable

**File:** `sim/global-types.ts`

**Definition:**
```typescript
{ [stat in StatID]: number }
```

---

### TeamValidator

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./team-validator').TeamValidator
```

---

### TypeInfo

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-data').TypeInfo
```

---

### TypeInfoEffectType

**File:** `sim/dex-data.ts`

**Definition:**
```typescript
'Type' | 'EffectType'
```

---

### ZMoveOptions

**File:** `sim/global-types.ts`

**Definition:**
```typescript
({ move: string, target: MoveTarget } | null)[]
```

---

