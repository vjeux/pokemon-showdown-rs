# JavaScript Type Definitions from Pokemon Showdown

Generated: 2026-01-02T15:40:26.471Z
Source: ../pokemon-showdown-ts
Total types: 577

## Table of Contents

- [Classes](#classes) (121)
- [Interfaces](#interfaces) (247)
- [Type Aliases](#type-aliases) (209)

---

## Classes

### AbstractChallenge

**File:** `server/ladders-challenges.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| from | `ID` |  |  | public |
| to | `ID` |  |  | public |
| ready | `BattleReady | null` |  |  | public |
| format | `string` |  |  | public |
| acceptCommand | `string | null` |  |  | public |
| message | `string` |  |  | public |
| acceptButton | `string` |  |  | public |
| rejectButton | `string` |  |  | public |
| roomid | `RoomID` |  |  | public |
| acceptCommand | `string, rejectCommand?: string, roomid?: RoomID,` | ✓ |  | public |
| message | `string, acceptButton?: string, rejectButton?: string,` | ✓ |  | public |

---

### Announcement

**File:** `server/chat-plugins/announcements.ts`

**Extends:** Rooms.MinorActivity

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| activityNumber | `number` |  |  | public |
| source | `string` |  |  | public |
| source | `this.source,` |  |  | public |
| activityNumber | `this.activityNumber,` |  |  | public |
| timeoutMins | `this.timeoutMins,` |  |  | public |
| timerEnd | `this.timerEnd,` |  |  | public |
| activityid | `'announcement',` |  |  | public |

---

### Auction

**File:** `server/chat-plugins/auction.ts`

**Extends:** Rooms.SimpleRoomGame

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| startingCredits | `number` |  |  | public |
| type | `'auction' | 'blind' | 'snake'` |  |  | public |
| lastQueue | `Team[] | null` |  |  | public |
| queue | `Team[]` |  |  | public |
| nomTimer | `NodeJS.Timeout` |  |  | public |
| bidTimer | `NodeJS.Timeout` |  |  | public |
| nominatingTeam | `Team` |  |  | public |
| nominatedPlayer | `Player` |  |  | public |
| highestBidder | `Team` |  |  | public |
| state | `'setup' | 'nom' | 'bid'` |  |  | public |
| price | `0,` |  |  | public |
| price | `0,` |  |  | public |

---

### AutoResponder

**File:** `server/chat-plugins/responder.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| data | `PluginData` |  |  | public |
| room | `Room` |  |  | public |
| room | `roomid,` |  |  | public |
| message | `entry,` |  |  | public |
| faqName | `faq,` |  |  | public |
| regex | `expression,` |  |  | public |
| date | `day,` |  |  | public |

---

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

### BasicRoom

**File:** `server/rooms.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| roomid | `RoomID` |  | ✓ | public |
| title | `string` |  |  | public |
| type | `'chat' | 'battle'` |  | ✓ | public |
| users | `UserTable` |  | ✓ | public |
| log | `Roomlog` |  | ✓ | public |
| game | `RoomGame | null` |  |  | public |
| subGame | `RoomGame | null` |  |  | public |
| battle | `RoomBattle | null` |  |  | public |
| bestOf | `BestOfGame | null` |  |  | public |
| tour | `Tournament | null` |  |  | public |
| auth | `RoomAuth` |  |  | public |
| parent | `Room | null` |  | ✓ | public |
| subRooms | `ReadonlyMap<string, Room> | null` |  | ✓ | public |
| muteQueue | `MuteEntry[]` |  | ✓ | public |
| userCount | `number` |  |  | public |
| active | `boolean` |  |  | public |
| muteTimer | `NodeJS.Timeout | null` |  |  | public |
| modchatTimer | `NodeJS.Timeout | null` |  |  | public |
| lastUpdate | `number` |  |  | public |
| lastBroadcast | `string` |  |  | public |
| lastBroadcastTime | `number` |  |  | public |
| settings | `RoomSettings` |  |  | public |
| persist | `boolean` |  |  | public |
| scavgame | `ScavengerGameTemplate | null` |  |  | public |
| scavLeaderboard | `AnyObject` |  |  | public |
| responder | `AutoResponder | null` | ✓ |  | public |
| privacySetter | `Set<ID> | null` | ✓ |  | public |
| hideReplay | `boolean` |  |  | public |
| reportJoins | `boolean` |  |  | public |
| batchJoins | `number` |  |  | public |
| reportJoinsInterval | `NodeJS.Timeout | null` |  |  | public |
| minorActivity | `MinorActivity | null` |  |  | public |
| minorActivityQueue | `MinorActivityData[] | null` |  |  | public |
| banwordRegex | `RegExp | true | null` |  |  | public |
| logUserStatsInterval | `NodeJS.Timeout | null` |  |  | public |
| expireTimer | `NodeJS.Timeout | null` |  |  | public |
| userList | `string` |  |  | public |
| pendingApprovals | `Map<string, ShowRequest> | null` |  |  | public |
| messagesSent | `number` |  |  | public |
| nthMessageHandlers | `Map<MessageHandler, number>` |  |  | public |
| title | `this.title,` |  |  | public |
| guestNum | `user.guestNum,` |  |  | public |
| autoconfirmed | `user.autoconfirmed,` |  |  | public |
| action | `'AUTOMODCHAT ACTIVATE',` |  |  | public |

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

### BattleReady

**File:** `server/ladders-challenges.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| userid | `ID` |  | ✓ | public |
| formatid | `string` |  | ✓ | public |
| settings | `User['battleSettings']` |  | ✓ | public |
| rating | `number` |  | ✓ | public |
| challengeType | `ChallengeType` |  | ✓ | public |
| time | `number` |  | ✓ | public |
| userid | `ID,` |  |  | public |
| formatid | `string,` |  |  | public |
| settings | `User['battleSettings'],` |  |  | public |
| challengeType | `ChallengeType` |  |  | public |

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

### CommandContext

**File:** `server/chat.ts`

**Extends:** MessageContext

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| message | `string` |  |  | public |
| pmTarget | `User | null` |  |  | public |
| room | `Room | null` |  |  | public |
| connection | `Connection` |  |  | public |
| cmd | `string` |  |  | public |
| cmdToken | `string` |  |  | public |
| target | `string` |  |  | public |
| fullCmd | `string` |  |  | public |
| handler | `AnnotatedChatHandler | null` |  |  | public |
| isQuiet | `boolean` |  |  | public |
| bypassRoomCheck | `boolean` | ✓ |  | public |
| broadcasting | `boolean` |  |  | public |
| broadcastToRoom | `boolean` |  |  | public |
| broadcastPrefix | `string` |  |  | public |
| broadcastMessage | `string` |  |  | public |
| message | `string, user: User, connection: Connection,` |  |  | public |
| room | `Room | null, pmTarget?: User | null, cmd?: string, cmdToken?: string, target?: string, fullCmd?: string,` | ✓ |  | public |
| recursionDepth | `number, isQuiet?: boolean, broadcastPrefix?: string, bypassRoomCheck?: boolean,` | ✓ |  | public |
| msg | `string,` | ✓ |  | public |
| options | `Partial<{ isQuiet: boolean, broadcastPrefix: string, bypassRoomCheck: boolean }>` |  |  | public |
| message | `msg,` |  |  | public |
| user | `this.user,` |  |  | public |
| connection | `this.connection,` |  |  | public |
| room | `this.room,` |  |  | public |
| pmTarget | `this.pmTarget,` |  |  | public |
| recursionDepth | `this.recursionDepth + 1,` |  |  | public |
| bypassRoomCheck | `this.bypassRoomCheck,` |  |  | public |
| user | `this.user.name,` |  |  | public |
| room | `this.room?.roomid,` |  |  | public |
| pmTarget | `this.pmTarget?.name,` |  |  | public |
| message | `this.message,` |  |  | public |
| user | `this.user.name,` |  |  | public |
| room | `this.room?.roomid,` |  |  | public |
| pmTarget | `this.pmTarget?.name,` |  |  | public |
| message | `this.message,` |  |  | public |
| isGlobal | `true,` |  |  | public |
| loggedBy | `this.user.id,` |  |  | public |
| action | `string,` |  |  | public |
| user | `string | User | null` |  |  | public |
| note | `string | null` |  |  | public |
| options | `Partial<{ noalts: any, noip: any }>` |  |  | public |
| loggedBy | `this.user.id,` |  |  | public |

---

### Connection

**File:** `server/users.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| id | `string` |  | ✓ | public |
| socketid | `string` |  | ✓ | public |
| worker | `ProcessManager.StreamWorker` |  | ✓ | public |
| inRooms | `Set<RoomID>` |  | ✓ | public |
| ip | `string` |  | ✓ | public |
| protocol | `string` |  | ✓ | public |
| connectedAt | `number` |  | ✓ | public |
| user | `User` |  |  | public |
| challenge | `string` |  |  | public |
| autojoins | `string` |  |  | public |
| lastRequestedPage | `string | null` |  |  | public |
| lastActiveTime | `number` |  |  | public |
| openPages | `null | Set<string>` |  |  | public |
| id | `string,` |  |  | public |
| worker | `ProcessManager.StreamWorker,` |  |  | public |
| socketid | `string,` |  |  | public |
| user | `User | null,` |  |  | public |
| ip | `string | null,` |  |  | public |
| protocol | `string | null` |  |  | public |

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

### Elimination

**File:** `server/tournaments/generator-elimination.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| name | `string` |  | ✓ | public |
| isDrawingSupported | `boolean` |  | ✓ | public |
| isBracketFrozen | `boolean` |  |  | public |
| players | `TournamentPlayer[]` |  |  | public |
| maxSubtrees | `number` |  |  | public |
| treeRoot | `ElimNode` |  |  | public |
| type | `'tree',` |  |  | public |
| rootNode | `null,` |  |  | public |
| type | `'tree',` |  |  | public |
| currentLayerLeafNodes | `[],` |  |  | public |
| nextLayerLeafNodes | `[],` |  |  | public |
| currentLayerLeafNodes | `[],` |  |  | public |
| nextLayerLeafNodes | `[],` |  |  | public |
| match | `[user, node.children[1].user],` |  |  | public |
| result | `'loss',` |  |  | public |
| score | `[0, 1],` |  |  | public |
| match | `[node.children[0].user, user],` |  |  | public |
| result | `'win',` |  |  | public |
| score | `[1, 0],` |  |  | public |

---

### ElimNode

**File:** `server/tournaments/generator-elimination.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| children | `[ElimNode, ElimNode] | null` |  |  | public |
| user | `TournamentPlayer | null` |  |  | public |
| state | `'available' | 'finished' | ''` |  |  | public |
| result | `'win' | 'loss' | ''` |  |  | public |
| score | `number[] | null` |  |  | public |
| losersBracketNode | `ElimNode | null` |  |  | public |
| losersBracketIndex | `number` |  |  | public |
| parent | `ElimNode | null` |  |  | public |
| fromNode | `ElimNode | null` |  |  | public |

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

### FileReadStream

**File:** `lib/fs.ts`

**Extends:** ReadStream

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| fd | `Promise<number>` |  |  | public |

---

### FriendsDatabase

**File:** `server/friends.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| file | `string` |  |  | public |

---

### FSLogSearcher

**File:** `server/chat-plugins/chatlog.ts`

**Extends:** Searcher

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| results | `number` |  |  | public |
| deadTime | `0,` |  |  | public |
| deadPercent | `0,` |  |  | public |
| lines | `{},` |  |  | public |
| users | `{},` |  |  | public |
| days | `1, // irrelevant` |  |  | public |
| linesPerUser | `0,` |  |  | public |
| totalLines | `0,` |  |  | public |
| averagePresent | `0,` |  |  | public |
| deadTime | `0,` |  |  | public |
| deadPercent | `0,` |  |  | public |
| lines | `{},` |  |  | public |
| users | `{},` |  |  | public |
| days | `0,` |  |  | public |
| linesPerUser | `0,` |  |  | public |
| totalLines | `0,` |  |  | public |
| averagePresent | `0,` |  |  | public |

---

### FSPath

**File:** `lib/fs.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| path | `string` |  |  | public |
| isWriting | `false,` |  |  | public |
| pendingDataFetcher | `dataFetcher,` |  |  | public |
| pendingOptions | `options,` |  |  | public |
| isWriting | `true,` |  |  | public |
| pendingDataFetcher | `null,` |  |  | public |
| pendingOptions | `null,` |  |  | public |
| throttleTimer | `null,` |  |  | public |

---

### GameRoom

**File:** `server/rooms.ts`

**Extends:** BasicRoom

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| format | `string` |  | ✓ | public |
| p1 | `User | null` |  |  | public |
| p2 | `User | null` |  |  | public |
| p3 | `User | null` |  |  | public |
| p4 | `User | null` |  |  | public |
| rated | `number` |  |  | public |
| modchatUser | `string` |  |  | public |
| id | `idWithServer,` |  |  | public |
| format | `format.name,` |  |  | public |
| private | `hidden,` |  |  | public |
| format | `format.name,` |  |  | public |
| hidden | `hidden` |  |  | public |

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

### GlobalRoomState

**File:** `server/rooms.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| settingsList | `RoomSettings[]` |  | ✓ | public |
| chatRooms | `ChatRoom[]` |  | ✓ | public |
| autojoinList | `RoomID[]` |  | ✓ | public |
| modjoinedAutojoinList | `RoomID[]` |  | ✓ | public |
| ladderIpLog | `Streams.WriteStream` |  | ✓ | public |
| reportUserStatsInterval | `NodeJS.Timeout` |  | ✓ | public |
| lockdown | `boolean | 'pre' | 'ddos'` |  |  | public |
| battleCount | `number` |  |  | public |
| lastReportedCrash | `number` |  |  | public |
| lastBattle | `number` |  |  | public |
| lastWrittenBattle | `number` |  |  | public |
| maxUsers | `number` |  |  | public |
| maxUsersDate | `number` |  |  | public |
| formatList | `string` |  |  | public |
| title | `'Lobby',` |  |  | public |
| auth | `{},` |  |  | public |
| autojoin | `true,` |  |  | public |
| section | `'official',` |  |  | public |
| title | `'Staff',` |  |  | public |
| auth | `{},` |  |  | public |
| isPrivate | `'hidden',` |  |  | public |
| modjoin | `'%',` |  |  | public |
| autojoin | `true,` |  |  | public |
| roomid | `room.roomid,` |  |  | public |
| title | `room.title,` |  |  | public |
| rated | `room.battle.rated,` |  |  | public |
| timer | `{` |  |  | public |
| active | `!!room.battle.timer.timer || false,` |  |  | public |
| format | `formatid,` |  |  | public |
| players | `[],` |  |  | public |
| delayedTimer | `timer.active,` |  |  | public |
| isBestOfSubBattle | `true, // not technically true but prevents a crash` |  |  | public |
| date | `this.maxUsersDate,` |  |  | public |
| users | `this.maxUsers,` |  |  | public |
| users | `Users.onlineCount,` |  |  | public |
| symbol | `rank,` |  |  | public |
| type | `groupType })` |  |  | public |
| chat | `ChatRoomTable[],` |  |  | public |
| sectionTitles | `string[],` |  |  | public |
| userCount | `number,` |  |  | public |
| battleCount | `number,` |  |  | public |
| chat | `[],` |  |  | public |
| userCount | `Users.onlineCount,` |  |  | public |
| battleCount | `this.battleCount,` |  |  | public |
| title | `room.title,` |  |  | public |
| desc | `room.settings.desc || '',` |  |  | public |
| userCount | `room.userCount,` |  |  | public |
| section | `room.settings.section ?` |  |  | public |
| privacy | `!room.settings.isPrivate ? undefined : room.settings.isPrivate,` |  |  | public |
| auth | `{},` |  |  | public |
| format | `string` |  | ✓ | public |
| p1 | `User | null` |  |  | public |
| p2 | `User | null` |  |  | public |
| p3 | `User | null` |  |  | public |
| p4 | `User | null` |  |  | public |
| rated | `number` |  |  | public |
| modchatUser | `string` |  |  | public |
| id | `idWithServer,` |  |  | public |
| format | `format.name,` |  |  | public |
| private | `hidden,` |  |  | public |
| format | `format.name,` |  |  | public |
| hidden | `hidden` |  |  | public |
| Modlog | `mainModlog,` |  |  | public |
| get | `getRoom,` |  |  | public |
| global | `null! as GlobalRoomState,` |  |  | public |
| lobby | `null as ChatRoom | null,` |  |  | public |
| ChatRoom | `BasicRoom as typeof ChatRoom,` |  |  | public |
| PM | `RoomBattlePM,` |  |  | public |

---

### GroupWatch

**File:** `server/chat-plugins/youtube.ts`

**Extends:** Rooms.SimpleRoomGame

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| url | `string` |  |  | public |
| info | `GroupwatchData` |  |  | public |
| started | `number | null` |  |  | public |
| id | `string` |  |  | public |

---

### Hangman

**File:** `server/chat-plugins/hangman.ts`

**Extends:** Rooms.SimpleRoomGame

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| gameNumber | `number` |  |  | public |
| creator | `ID` |  |  | public |
| word | `string` |  |  | public |
| hint | `string` |  |  | public |
| incorrectGuesses | `number` |  |  | public |
| options | `HangmanOptions` |  |  | public |
| guesses | `string[]` |  |  | public |
| letterGuesses | `string[]` |  |  | public |
| lastGuesser | `string` |  |  | public |
| wordSoFar | `string[]` |  |  | public |
| room | `Room,` |  |  | public |
| user | `User,` |  |  | public |
| word | `string,` |  |  | public |
| gameOptions | `HangmanOptions` |  |  | public |
| question | `shuffled,` |  |  | public |

---

### HelpTicket

**File:** `server/chat-plugins/helptickets.ts`

**Extends:** Rooms.SimpleRoomGame

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| ticket | `TicketState` |  |  | public |
| claimQueue | `string[]` |  |  | public |
| createTime | `number` |  |  | public |
| activationTime | `number` |  |  | public |
| lastUnclaimedStart | `number` |  |  | public |
| resolution | `'unknown' | 'dead' | 'unresolved' | 'resolved'` |  |  | public |
| result | `TicketResult | null` |  |  | public |
| default | `this.result` |  |  | public |
| text | `ticket.text,` |  |  | public |
| resolved | `ticket.resolved,` |  |  | public |
| meta | `ticket.meta,` |  |  | public |
| created | `ticket.created,` |  |  | public |
| userid | `ticket.userid,` |  |  | public |
| type | `ticket.type,` |  |  | public |
| claimed | `ticket.claimed,` |  |  | public |
| state | `ticket.state || {},` |  |  | public |
| recommended | `ticket.recommended,` |  |  | public |
| reportUserid | `ID,` |  |  | public |
| proofString | `string,` |  |  | public |
| ticket | `TicketState,` |  |  | public |
| title | `string,` | ✓ |  | public |
| inner | `string,` | ✓ |  | public |
| type | `'TICKETBAN',` |  |  | public |
| id | `userid,` |  |  | public |
| expireTime | `duration,` |  |  | public |

---

### HttpError

**File:** `lib/net.ts`

**Extends:** Error

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| statusCode | `number` | ✓ |  | public |
| body | `string` |  |  | public |

---

### Ladder

**File:** `server/ladders.ts`

**Extends:** LadderStore

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| team | `ready.settings.team,` |  |  | public |
| rating | `ready.rating,` |  |  | public |
| hidden | `ready.settings.hidden,` |  |  | public |
| inviteOnly | `ready.settings.inviteOnly,` |  |  | public |
| format | `formatid,` |  |  | public |
| rated | `minRating,` |  |  | public |
| challengeType | `readies[0].challengeType,` |  |  | public |

---

### LadderStore

**File:** `server/ladders-remote.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| formatid | `string` |  |  | public |
| format | `formatid,` |  |  | public |
| user | `userid,` |  |  | public |
| p1 | `p1name,` |  |  | public |
| p2 | `p2name,` |  |  | public |
| score | `p1score,` |  |  | public |
| format | `formatid,` |  |  | public |

---

### LadderTracker

**File:** `server/chat-plugins/laddertours.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| config | `TrackerConfig` |  | ✓ | public |
| format | `ID` |  |  | public |
| prefix | `ID` |  |  | public |
| deadline | `Date` | ✓ |  | private |
| rating | `number` |  |  | public |
| users | `Set<ID>` |  |  | public |
| lastid | `string` | ✓ |  | private |
| showdiffs | `boolean` | ✓ |  | public |
| started | `NodeJS.Timeout` | ✓ |  | private |
| final | `NodeJS.Timeout` | ✓ |  | private |
| leaderboard | `Leaderboard` |  |  | public |
| cooldown | `Date` | ✓ |  | public |
| changed | `boolean` | ✓ |  | private |
| lines | `{ them: number, total: number }` |  |  | public |
| room | `Room` |  |  | public |
| name | `data.username,` |  |  | public |
| gxe | `data.gxe,` |  |  | public |

---

### LastFMInterface

**File:** `server/chat-plugins/the-studio.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| query | `{` |  |  | public |
| method | `'user.getRecentTracks', user: accountName,` |  |  | public |
| limit | `1, api_key: Config.lastfmkey, format: 'json',` |  |  | public |
| method | `'track.search', limit: 1, api_key: Config.lastfmkey, track, format: 'json',` |  |  | public |

---

### Leaderboard

**File:** `server/chat-plugins/scavenger-games.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| data | `{ [userid: string]: AnyObject }` |  |  | public |
| rank | `lastPlacement,` |  |  | public |

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

### Limiter

**File:** `server/artemis/remote.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| max | `number` |  | ✓ | public |

---

### LocalClassifier

**File:** `server/artemis/local.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| sexual_explicit | `{},` |  |  | public |
| severe_toxicity | `{},` |  |  | public |
| toxicity | `{},` |  |  | public |
| obscene | `{},` |  |  | public |
| identity_attack | `{},` |  |  | public |
| insult | `{},` |  |  | public |
| threat | `{},` |  |  | public |
| stream | `Streams.ObjectReadWriteStream<string>` | ✓ |  | public |
| readyPromise | `Promise<boolean> | null` |  |  | public |

---

### LoginServerInstance

**File:** `server/loginserver.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| uri | `string` |  | ✓ | public |
| requestTimer | `NodeJS.Timeout | null` |  |  | public |
| requestLog | `string` |  |  | public |
| lastRequest | `number` |  |  | public |
| openRequests | `number` |  |  | public |
| disabled | `false` |  |  | public |
| query | `{` |  |  | public |
| act | `action,` |  |  | public |
| serverid | `Config.serverid,` |  |  | public |
| servertoken | `Config.servertoken,` |  |  | public |
| body | `{` |  |  | public |
| serverid | `Config.serverid,` |  |  | public |
| servertoken | `Config.servertoken,` |  |  | public |
| timeout | `LOGIN_SERVER_TIMEOUT,` |  |  | public |

---

### LogReaderRoom

**File:** `server/chat-plugins/chatlog.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| roomid | `RoomID` |  |  | public |

---

### Mastermind

**File:** `server/chat-plugins/trivia/trivia.ts`

**Extends:** Rooms.SimpleRoomGame

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| leaderboard | `Map<ID, { score: number, hasLeft?: boolean }>` |  |  | public |
| phase | `string` |  |  | public |
| currentRound | `MastermindRound | MastermindFinals | null` |  |  | public |
| numFinalists | `number` |  |  | public |

---

### MessageContext

**File:** `server/chat.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| user | `User` |  | ✓ | public |
| language | `ID | null` |  |  | public |
| recursionDepth | `number` |  |  | public |
| targetUsername | `targetUser ? targetUser.name : inputUsername,` |  |  | public |

---

### MinorActivity

**File:** `server/room-minor-activity.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| timeout | `NodeJS.Timeout | null` |  |  | public |
| timeoutMins | `number` |  |  | public |
| timerEnd | `number` |  |  | public |
| roomid | `RoomID` |  |  | public |
| room | `Room` |  |  | public |
| supportHTML | `boolean` |  |  | public |
| action | `'POLL',` |  |  | public |

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

### Modlog

**File:** `server/modlog/index.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| database | `SQL.DatabaseManager` |  | ✓ | public |
| readyPromise | `null | Promise<void>` |  |  | public |
| databaseReady | `boolean` |  |  | public |
| queuedEntries | `ModlogEntry[]` |  |  | public |
| modlogInsertionQuery | `SQL.Statement | null` |  |  | public |
| altsInsertionQuery | `SQL.Statement | null` |  |  | public |
| renameQuery | `SQL.Statement | null` |  |  | public |
| globalPunishmentsSearchQuery | `SQL.Statement | null` |  |  | public |
| action | `entry.action,` |  |  | public |
| visualRoomID | `overrideID || entry.visualRoomID || '',` |  |  | public |
| userid | `entry.userid || null,` |  |  | public |
| autoconfirmedID | `entry.autoconfirmedID || null,` |  |  | public |
| ip | `entry.ip || null,` |  |  | public |
| isGlobal | `entry.isGlobal || roomID` |  |  | public |
| loggedBy | `entry.loggedBy || null,` |  |  | public |
| note | `entry.note || '',` |  |  | public |
| roomid | `ModlogID` |  |  | public |
| search | `ModlogSearch` |  |  | public |
| entryID | `row.modlog_id,` |  |  | public |
| action | `row.action,` |  |  | public |
| roomID | `row.roomid,` |  |  | public |
| visualRoomID | `row.visual_roomid,` |  |  | public |
| userid | `row.userid,` |  |  | public |
| autoconfirmedID | `row.autoconfirmed_userid,` |  |  | public |
| ip | `row.ip || null,` |  |  | public |
| loggedBy | `row.action_taker_userid,` |  |  | public |
| note | `row.note,` |  |  | public |
| time | `row.timestamp,` |  |  | public |
| select | `string,` |  |  | public |
| ors | `SQLQuery[],` |  |  | public |
| ands | `SQLQuery[],` |  |  | public |
| sortAndLimit | `SQLQuery` |  |  | public |
| queryText | `query,` |  |  | public |
| rooms | `ModlogID[] | 'all',` |  |  | public |
| maxLines | `number,` |  |  | public |
| onlyPunishments | `boolean,` |  |  | public |
| search | `ModlogSearch` |  |  | public |
| args | `[param],` |  |  | public |

---

### ModlogConverterSQLite

**File:** `tools/modlog/converter.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| databaseFile | `string` |  | ✓ | public |
| textLogDir | `string` |  | ✓ | public |
| isTesting | `{ files: Map<string, string>, db: DatabaseType.Database } | null` |  | ✓ | public |
| newestAllowedTimestamp | `number` | ✓ | ✓ | public |
| databaseFile | `string, textLogDir: string,` |  |  | public |
| isTesting | `DatabaseType.Database, newestAllowedTimestamp?: number` | ✓ |  | public |
| action | `result.action,` |  |  | public |
| visualRoomID | `result.visual_roomid,` |  |  | public |
| userid | `result.userid,` |  |  | public |
| autoconfirmedID | `result.autoconfirmed_userid,` |  |  | public |
| ip | `result.ip,` |  |  | public |
| loggedBy | `result.action_taker_userid,` |  |  | public |
| note | `result.note,` |  |  | public |
| time | `result.timestamp,` |  |  | public |

---

### ModlogConverterTest

**File:** `tools/modlog/converter.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| inputDir | `string` |  | ✓ | public |
| outputDir | `string` |  | ✓ | public |

---

### ModlogConverterTxt

**File:** `tools/modlog/converter.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| databaseFile | `string` |  | ✓ | public |
| modlog | `typeof Modlog` |  | ✓ | public |
| newestAllowedTimestamp | `number` | ✓ | ✓ | public |
| textLogDir | `string` |  | ✓ | public |
| isTesting | `{ files: Map<string, string>, ml?: typeof Modlog } | null` |  | ✓ | public |
| databaseFile | `string,` |  |  | public |
| textLogDir | `string,` |  |  | public |
| isTesting | `Map<string, string>,` | ✓ |  | public |
| newestAllowedTimestamp | `number` | ✓ |  | public |

---

### MultiRandomRunner

**File:** `sim/tools/multi-random-runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| formatIndex | `number` |  |  | private |
| numGames | `number` |  |  | private |

---

### NetRequest

**File:** `lib/net.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| uri | `string` |  |  | public |
| response | `http.IncomingMessage` | ✓ |  | public |
| method | `'POST',` |  |  | public |

---

### NetStream

**File:** `lib/net.ts`

**Extends:** Streams.ReadWriteStream

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| opts | `NetRequestOptions | null` |  |  | public |
| uri | `string` |  |  | public |
| request | `http.ClientRequest` |  |  | public |
| response | `Promise<http.IncomingMessage | null> | http.IncomingMessage | null` |  |  | public |
| statusCode | `number | null` |  |  | public |
| headers | `http.IncomingHttpHeaders | null` |  |  | public |
| state | `'pending' | 'open' | 'timeout' | 'success' | 'error'` |  |  | public |

---

### OtdHandler

**File:** `server/chat-plugins/thing-of-the-day.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| id | `string` |  |  | public |
| name | `string` |  |  | public |
| room | `Room` |  |  | public |
| nominations | `Map<string, AnyObject>` |  |  | public |
| removedNominations | `Map<string, AnyObject>` |  |  | public |
| voting | `boolean` |  |  | public |
| timer | `NodeJS.Timeout | null` |  |  | public |
| autoStartTimer | `NodeJS.Timeout | null` |  |  | public |
| keys | `string[]` |  |  | public |
| keyLabels | `string[]` |  |  | public |
| timeLabel | `string` |  |  | public |
| settings | `OtdSettings` |  |  | public |
| lastPrenom | `number` |  |  | public |
| winners | `AnyObject[]` |  |  | public |
| id | `string, room: Room, settings: OtdSettings` |  |  | public |
| name | `user.name,` |  |  | public |
| settings | `this.settings,` |  |  | public |
| winners | `this.winners,` |  |  | public |

---

### PageContext

**File:** `server/chat.ts`

**Extends:** MessageContext

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| connection | `Connection` |  | ✓ | public |
| room | `Room | null` |  |  | public |
| pageid | `string` |  |  | public |
| initialized | `boolean` |  |  | public |
| title | `string` |  |  | public |
| args | `string[]` |  |  | public |
| user | `this.user.name,` |  |  | public |
| room | `this.room && this.room.roomid,` |  |  | public |
| pageid | `this.pageid,` |  |  | public |

---

### PatternTester

**File:** `server/chat.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| elements | `string[]` |  | ✓ | public |
| fastElements | `Set<string>` |  | ✓ | public |
| regexp | `RegExp | null` |  |  | public |

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

### Poll

**File:** `server/chat-plugins/poll.ts`

**Extends:** Rooms.MinorActivity

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| activityNumber | `number` |  |  | public |
| question | `string` |  |  | public |
| multiPoll | `boolean` |  |  | public |
| pendingVotes | `{ [userid: string]: number[] }` |  |  | public |
| voters | `{ [k: string]: number[] }` |  |  | public |
| voterIps | `{ [k: string]: number[] }` |  |  | public |
| totalVotes | `number` |  |  | public |
| isQuiz | `boolean` |  |  | public |
| maxVotes | `number` |  |  | public |
| answers | `Map<number, PollAnswer>` |  |  | public |
| options | `Rooms.MinorActivityData, room: Room,` |  |  | public |
| activityid | `'poll',` |  |  | public |
| activityNumber | `this.activityNumber,` |  |  | public |
| question | `this.question,` |  |  | public |
| supportHTML | `this.supportHTML,` |  |  | public |
| multiPoll | `this.multiPoll,` |  |  | public |
| pendingVotes | `this.pendingVotes,` |  |  | public |
| voters | `this.voters,` |  |  | public |
| voterIps | `this.voterIps,` |  |  | public |
| totalVotes | `this.totalVotes,` |  |  | public |
| timeoutMins | `this.timeoutMins,` |  |  | public |
| timerEnd | `this.timerEnd,` |  |  | public |
| isQuiz | `this.isQuiz,` |  |  | public |
| votes | `0,` |  |  | public |

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

### RandomBabyTeams

**File:** `data/random-battles/gen9baby/teams.ts`

**Extends:** RandomTeams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `Species,` |  |  | public |
| s | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.gender,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |

---

### RandomBDSPTeams

**File:** `data/random-battles/gen8bdsp/teams.ts`

**Extends:** RandomGen8Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean` |  |  | public |
| ability | `string,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean` |  |  | public |
| move | `Move,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |

---

### RandomCAPTeams

**File:** `data/random-battles/gen9cap/teams.ts`

**Extends:** RandomTeams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `Species,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| s | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.baseSpecies` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |
| randomCAPSets | `{ [species: string]: RandomTeamsTypes.RandomSpeciesData }` |  |  | public |

---

### RandomChatBatsTeams

**File:** `data/random-battles/chatbats/teams.ts`

**Extends:** RandomTeams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| s | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.baseSpecies` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |

---

### RandomFFATeams

**File:** `data/random-battles/gen9ffa/teams.ts`

**Extends:** RandomTeams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `Species,` |  |  | public |
| s | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.gender,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |

---

### RandomGen1Teams

**File:** `data/random-battles/gen1/teams.ts`

**Extends:** RandomGen2Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| hp | `0,` |  |  | public |
| spd | `0,` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| gender | `false,` |  |  | public |
| ability | `'No Ability',` |  |  | public |
| item | `'',` |  |  | public |
| happiness | `0,` |  |  | public |
| shiny | `false,` |  |  | public |
| nature | `'Serious',` |  |  | public |
| name | `species.name,` |  |  | public |
| species | `species.name,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |
| ability | `'No Ability',` |  |  | public |
| item | `'',` |  |  | public |
| shiny | `false,` |  |  | public |
| gender | `false,` |  |  | public |
| baseStats | `{` |  |  | public |
| spd | `0,` |  |  | public |
| spd | `0,` |  |  | public |
| hp | `0,` |  |  | public |
| spd | `0,` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| gender | `species.gender,` |  |  | public |
| item | `'',` |  |  | public |
| ability | `'No Ability',` |  |  | public |
| nature | `'',` |  |  | public |
| shiny | `false,` |  |  | public |
| hc | `hackmonsCup[species.id],` |  |  | public |

---

### RandomGen2Teams

**File:** `data/random-battles/gen2/teams.ts`

**Extends:** RandomGen3Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| dragon | `{ def: 28 },` |  |  | public |
| ice | `{ def: 26 },` |  |  | public |
| psychic | `{ def: 24 },` |  |  | public |
| electric | `{ atk: 28 },` |  |  | public |
| grass | `{ atk: 28, def: 28 },` |  |  | public |
| water | `{ atk: 28, def: 26 },` |  |  | public |
| fire | `{ atk: 28, def: 24 },` |  |  | public |
| steel | `{ atk: 26 },` |  |  | public |
| ghost | `{ atk: 26, def: 28 },` |  |  | public |
| bug | `{ atk: 26, def: 26 },` |  |  | public |
| rock | `{ atk: 26, def: 24 },` |  |  | public |
| ground | `{ atk: 24 },` |  |  | public |
| poison | `{ atk: 24, def: 28 },` |  |  | public |
| flying | `{ atk: 24, def: 26 },` |  |  | public |
| fighting | `{ atk: 24, def: 24 },` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |
| ability | `'No Ability',` |  |  | public |
| shiny | `false,` |  |  | public |
| gender | `species.gender ? species.gender : 'M',` |  |  | public |

---

### RandomGen3Teams

**File:** `data/random-battles/gen3/teams.ts`

**Extends:** RandomGen4Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| battleHasDitto | `boolean` |  |  | public |
| battleHasWobbuffet | `boolean` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.gender,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |

---

### RandomGen4Teams

**File:** `data/random-battles/gen4/teams.ts`

**Extends:** RandomGen5Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.gender,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |

---

### RandomGen5Teams

**File:** `data/random-battles/gen5/teams.ts`

**Extends:** RandomGen6Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.gender,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |

---

### RandomGen6Teams

**File:** `data/random-battles/gen6/teams.ts`

**Extends:** RandomGen7Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| Normal | `movePool` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |
| species | `Species,` |  |  | public |
| teamData | `RandomTeamsTypes.FactoryTeamDetails,` |  |  | public |
| tier | `string` |  |  | public |
| rapidspin | `1, batonpass: 1, stealthrock: 1, defog: 1, spikes: 1, toxicspikes: 1,` |  |  | public |
| stealthrock | `'hazardSet', rapidspin: 'hazardClear', defog: 'hazardClear',` |  |  | public |
| hydration | `'raindance', swiftswim: 'raindance',` |  |  | public |
| leafguard | `'sunnyday', solarpower: 'sunnyday', chlorophyll: 'sunnyday',` |  |  | public |
| sandforce | `'sandstorm', sandrush: 'sandstorm', sandveil: 'sandstorm',` |  |  | public |
| snowcloak | `'hail',` |  |  | public |
| name | `setData.set.name || species.baseSpecies,` |  |  | public |
| species | `setData.set.species,` |  |  | public |
| item | `setData.set.item || '',` |  |  | public |
| ability | `setData.set.ability || species.abilities['0'],` |  |  | public |
| shiny | `typeof setData.set.shiny` |  |  | public |
| level | `this.adjustLevel || 100,` |  |  | public |
| happiness | `typeof setData.set.happiness` |  |  | public |
| evs | `setData.set.evs || { hp: 84, atk: 84, def: 84, spa: 84, spd: 84, spe: 84 },` |  |  | public |
| ivs | `setData.set.ivs || { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },` |  |  | public |
| nature | `setData.set.nature || 'Serious',` |  |  | public |
| typeCount | `{}, typeComboCount: {}, baseFormes: {}, megaCount: 0, has: {}, forceResult,` |  |  | public |
| weaknesses | `{}, resistances: {},` |  |  | public |
| stealthrock | `'hazardSet', rapidspin: 'hazardClear', defog: 'hazardClear',` |  |  | public |
| drizzle | `'raindance', drought: 'sunnyday', snowwarning: 'hail', sandstream: 'sandstorm',` |  |  | public |
| dryskin | `['Water'], waterabsorb: ['Water'], stormdrain: ['Water'],` |  |  | public |
| flashfire | `['Fire'], heatproof: ['Fire'],` |  |  | public |
| lightningrod | `['Electric'], motordrive: ['Electric'], voltabsorb: ['Electric'],` |  |  | public |
| sapsipper | `['Grass'],` |  |  | public |
| thickfat | `['Ice', 'Fire'],` |  |  | public |
| levitate | `['Ground'],` |  |  | public |

---

### RandomGen7Teams

**File:** `data/random-battles/gen7/teams.ts`

**Extends:** RandomGen8Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| randomSets | `{ [species: string]: RandomTeamsTypes.RandomSpeciesData }` |  |  | public |
| cachedStatusMoves | `ID[]` |  |  | protected |
| Normal | `movePool` |  |  | public |
| moves | `Set<string> | null,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| abilities | `string[],` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| movePool | `string[],` |  |  | public |
| movesA | `string | string[],` |  |  | public |
| movesB | `string | string[],` |  |  | public |
| move | `string,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ZU | `81,` |  |  | public |
| ZUBL | `79,` |  |  | public |
| PU | `77,` |  |  | public |
| PUBL | `75,` |  |  | public |
| NU | `73,` |  |  | public |
| NUBL | `71,` |  |  | public |
| UU | `69,` |  |  | public |
| UUBL | `67,` |  |  | public |
| OU | `65,` |  |  | public |
| Uber | `61,` |  |  | public |
| species | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.baseSpecies` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |
| species | `Species, teamData: RandomTeamsTypes.FactoryTeamDetails, tier: string` |  |  | public |
| choicespecs | `1,` |  |  | public |
| choiceband | `1,` |  |  | public |
| choicescarf | `1,` |  |  | public |
| rapidspin | `1,` |  |  | public |
| batonpass | `1,` |  |  | public |
| stealthrock | `1,` |  |  | public |
| defog | `1,` |  |  | public |
| spikes | `1,` |  |  | public |
| toxicspikes | `1,` |  |  | public |
| stealthrock | `'hazardSet',` |  |  | public |
| rapidspin | `'hazardClear',` |  |  | public |
| defog | `'hazardClear',` |  |  | public |
| hydration | `'raindance', swiftswim: 'raindance',` |  |  | public |
| leafguard | `'sunnyday', solarpower: 'sunnyday', chlorophyll: 'sunnyday',` |  |  | public |
| sandforce | `'sandstorm', sandrush: 'sandstorm', sandveil: 'sandstorm',` |  |  | public |
| slushrush | `'hail', snowcloak: 'hail',` |  |  | public |
| name | `setData.set.name || species.baseSpecies,` |  |  | public |
| species | `setData.set.species,` |  |  | public |
| item | `item || '',` |  |  | public |
| ability | `ability || species.abilities['0'],` |  |  | public |
| shiny | `typeof setData.set.shiny` |  |  | public |
| happiness | `typeof setData.set.happiness` |  |  | public |
| evs | `{ hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0, ...setData.set.evs },` |  |  | public |
| ivs | `{ hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31, ...setData.set.ivs },` |  |  | public |
| nature | `nature || 'Serious',` |  |  | public |
| Uber | `5,` |  |  | public |
| OU | `4, UUBL: 4,` |  |  | public |
| UU | `3, RUBL: 3,` |  |  | public |
| RU | `2, NUBL: 2,` |  |  | public |
| NU | `1, PUBL: 1,` |  |  | public |
| PU | `0,` |  |  | public |
| typeCount | `{}, typeComboCount: {}, baseFormes: {}, megaCount: 0, zCount: 0,` |  |  | public |
| has | `{}, forceResult, weaknesses: {}, resistances: {},` |  |  | public |
| stealthrock | `'hazardSet',` |  |  | public |
| rapidspin | `'hazardClear',` |  |  | public |
| defog | `'hazardClear',` |  |  | public |
| drizzle | `'raindance',` |  |  | public |
| drought | `'sunnyday',` |  |  | public |
| snowwarning | `'hail',` |  |  | public |
| sandstream | `'sandstorm',` |  |  | public |
| dryskin | `['Water'], waterabsorb: ['Water'], stormdrain: ['Water'],` |  |  | public |
| flashfire | `['Fire'], heatproof: ['Fire'],` |  |  | public |
| lightningrod | `['Electric'], motordrive: ['Electric'], voltabsorb: ['Electric'],` |  |  | public |
| sapsipper | `['Grass'],` |  |  | public |
| thickfat | `['Ice', 'Fire'],` |  |  | public |
| levitate | `['Ground'],` |  |  | public |
| species | `Species, teamData: RandomTeamsTypes.FactoryTeamDetails` |  |  | public |
| batonpass | `1,` |  |  | public |
| stealthrock | `1,` |  |  | public |
| spikes | `1,` |  |  | public |
| toxicspikes | `1,` |  |  | public |
| doubleedge | `1,` |  |  | public |
| trickroom | `1,` |  |  | public |
| swiftswim | `'raindance',` |  |  | public |
| sandrush | `'sandstorm', sandveil: 'sandstorm',` |  |  | public |
| name | `setData.set.nickname || setData.set.name || species.baseSpecies,` |  |  | public |
| species | `setData.set.species,` |  |  | public |
| ability | `setData.set.ability || species.abilities['0'],` |  |  | public |
| shiny | `typeof setData.set.shiny` |  |  | public |
| level | `setData.set.level || 50,` |  |  | public |
| happiness | `typeof setData.set.happiness` |  |  | public |
| evs | `{ hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0, ...setData.set.evs },` |  |  | public |
| ivs | `{ hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31, ...setData.set.ivs },` |  |  | public |
| nature | `setData.set.nature || 'Serious',` |  |  | public |
| typeCount | `{}, typeComboCount: {}, baseFormes: {}, megaCount: 0, zCount: 0,` |  |  | public |
| eeveeLimCount | `0, has: {}, forceResult, weaknesses: {}, resistances: {},` |  |  | public |
| drizzle | `'raindance',` |  |  | public |
| drought | `'sunnyday',` |  |  | public |
| snowwarning | `'hail',` |  |  | public |
| sandstream | `'sandstorm',` |  |  | public |
| waterabsorb | `['Water'],` |  |  | public |
| flashfire | `['Fire'],` |  |  | public |
| lightningrod | `['Electric'], voltabsorb: ['Electric'],` |  |  | public |
| thickfat | `['Ice', 'Fire'],` |  |  | public |
| levitate | `['Ground'],` |  |  | public |

---

### RandomGen8Teams

**File:** `data/random-battles/gen8/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| gen | `number` |  |  | public |
| factoryTier | `string` |  |  | public |
| format | `Format` |  |  | public |
| prng | `PRNG` |  |  | public |
| noStab | `string[]` |  |  | public |
| priorityPokemon | `string[]` |  |  | public |
| maxTeamSize | `number` |  | ✓ | public |
| adjustLevel | `number | null` |  | ✓ | public |
| maxMoveCount | `number` |  | ✓ | public |
| forceMonotype | `string | undefined` |  | ✓ | public |
| randomData | `{ [species: string]: OldRandomBattleSpecies }` |  |  | public |
| moveEnforcementCheckers | `{ [k: string]: MoveEnforcementChecker }` |  |  | public |
| poolsCacheKey | `[string | undefined, number | undefined, RuleTable | undefined, boolean] | undefined` |  |  | private |
| cachedPool | `number[] | undefined` |  |  | private |
| cachedSpeciesPool | `Species[] | undefined` |  |  | private |
| Bug | `movePool` |  |  | public |
| effectTypeName | `string,` |  |  | public |
| basicEffectPool | `BasicEffect[],` |  |  | public |
| requiredCount | `number,` |  |  | public |
| requiredCountExplanation | `string` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| moves | `m,` |  |  | public |
| moves | `Set<string> | null,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| movePool | `string[]` |  |  | public |
| move | `Move,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| isNoDynamax | `boolean,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| isNoDynamax | `boolean` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| preferredType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| isNoDynamax | `boolean` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| ability | `string,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| isNoDynamax | `boolean` |  |  | public |
| ability | `string,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| isNoDynamax | `boolean` |  |  | public |
| species | `Species,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| isNoDynamax | `boolean,` |  |  | public |
| Uber | `76,` |  |  | public |
| OU | `80,` |  |  | public |
| UUBL | `81,` |  |  | public |
| UU | `82,` |  |  | public |
| RUBL | `83,` |  |  | public |
| RU | `84,` |  |  | public |
| NUBL | `85,` |  |  | public |
| NU | `86,` |  |  | public |
| PUBL | `87,` |  |  | public |
| zaciancrowned | `65, calyrexshadow: 68, xerneas: 70, necrozmaduskmane: 72, zacian: 72, kyogre: 73, eternatus: 73,` |  |  | public |
| zekrom | `74, marshadow: 75, urshifurapidstrike: 79, haxorus: 80, inteleon: 80,` |  |  | public |
| cresselia | `83, jolteon: 84, swoobat: 84, dugtrio: 84, slurpuff: 84, polteageist: 84,` |  |  | public |
| wobbuffet | `86, scrafty: 86,` |  |  | public |
| delibird | `100, vespiquen: 96, pikachu: 92, shedinja: 92, solrock: 90, arctozolt: 88, reuniclus: 87,` |  |  | public |
| decidueye | `87, noivern: 85, magnezone: 82, slowking: 81,` |  |  | public |
| Uber | `76, Unreleased: 76,` |  |  | public |
| OU | `80,` |  |  | public |
| UUBL | `81,` |  |  | public |
| UU | `82,` |  |  | public |
| RUBL | `83,` |  |  | public |
| RU | `84,` |  |  | public |
| NUBL | `85,` |  |  | public |
| NU | `86,` |  |  | public |
| PUBL | `87,` |  |  | public |
| delibird | `100, dugtrio: 76, glalie: 76, luvdisc: 100, spinda: 100, unown: 100,` |  |  | public |
| species | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gigantamax | `gmax,` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |
| type | `string,` |  |  | public |
| pokemonToExclude | `RandomTeamsTypes.RandomSet[]` |  |  | public |
| pokemonList | `string[]` |  |  | public |
| randomCAP1v1Sets | `AnyObject` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| level | `this.adjustLevel || 100,` |  |  | public |
| evs | `{ hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0, ...setData.evs },` |  |  | public |
| nature | `setData.nature,` |  |  | public |
| ivs | `{ hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31, ...setData.ivs },` |  |  | public |
| randomFactorySets | `{ [format: string]: { [species: string]: BattleFactorySpecies } }` |  |  | public |
| species | `Species, teamData: RandomTeamsTypes.FactoryTeamDetails, tier: string` |  |  | public |
| choicespecs | `1,` |  |  | public |
| choiceband | `1,` |  |  | public |
| choicescarf | `1,` |  |  | public |
| rapidspin | `1,` |  |  | public |
| batonpass | `1,` |  |  | public |
| stealthrock | `1,` |  |  | public |
| defog | `1,` |  |  | public |
| spikes | `1,` |  |  | public |
| toxicspikes | `1,` |  |  | public |
| stealthrock | `'hazardSet',` |  |  | public |
| rapidspin | `'hazardClear',` |  |  | public |
| defog | `'hazardClear',` |  |  | public |
| name | `setData.set.name || species.baseSpecies,` |  |  | public |
| species | `setData.set.species,` |  |  | public |
| item | `item || '',` |  |  | public |
| ability | `ability || species.abilities['0'],` |  |  | public |
| shiny | `typeof setData.set.shiny` |  |  | public |
| happiness | `typeof setData.set.happiness` |  |  | public |
| evs | `{ hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0, ...setData.set.evs },` |  |  | public |
| ivs | `{ hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31, ...setData.set.ivs },` |  |  | public |
| nature | `nature || 'Serious',` |  |  | public |
| Uber | `5,` |  |  | public |
| OU | `4, UUBL: 4,` |  |  | public |
| UU | `3, RUBL: 3,` |  |  | public |
| RU | `2, NUBL: 2,` |  |  | public |
| NU | `1, PUBL: 1,` |  |  | public |
| PU | `0,` |  |  | public |
| typeCount | `{}, typeComboCount: {}, baseFormes: {},` |  |  | public |
| has | `{}, forceResult, weaknesses: {}, resistances: {},` |  |  | public |
| stealthrock | `'hazardSet',` |  |  | public |
| rapidspin | `'hazardClear',` |  |  | public |
| defog | `'hazardClear',` |  |  | public |
| drizzle | `'raindance',` |  |  | public |
| drought | `'sunnyday',` |  |  | public |
| snowwarning | `'hail',` |  |  | public |
| sandstream | `'sandstorm',` |  |  | public |
| dryskin | `['Water'], waterabsorb: ['Water'], stormdrain: ['Water'],` |  |  | public |
| flashfire | `['Fire'], heatproof: ['Fire'],` |  |  | public |
| lightningrod | `['Electric'], motordrive: ['Electric'], voltabsorb: ['Electric'],` |  |  | public |
| sapsipper | `['Grass'],` |  |  | public |
| thickfat | `['Ice', 'Fire'],` |  |  | public |
| levitate | `['Ground'],` |  |  | public |

---

### RandomLetsGoTeams

**File:** `data/random-battles/gen7letsgo/teams.ts`

**Extends:** RandomGen8Teams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| move | `Move,` |  |  | public |
| types | `Set<string>,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| isSetup | `true,` |  |  | public |
| isSetup | `true,` |  |  | public |
| isSetup | `true,` |  |  | public |
| cull | `counter.damagingMoves.size < 2 && !counter.setupType,` |  |  | public |
| isSetup | `!counter.setupType,` |  |  | public |
| species | `string | Species, teamDetails: RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| level | `this.adjustLevel || 100,` |  |  | public |
| happiness | `70,` |  |  | public |
| ability | `'No Ability',` |  |  | public |
| evs | `{ hp: 20, atk: 20, def: 20, spa: 20, spd: 20, spe: 20 },` |  |  | public |

---

### RandomMHSTeams

**File:** `data/random-battles/monsterhunter/teams.ts`

**Extends:** RandomTeams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| s | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.baseSpecies` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |

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

### RandomStaffBrosTeams

**File:** `data/mods/gen9ssb/random-teams.ts`

**Extends:** RandomTeams

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| species | `ssbSet.species,` |  |  | public |
| evs | `ssbSet.evs ? { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0, ...ssbSet.evs } :` |  |  | public |
| level | `this.adjustLevel || ssbSet.level || 100,` |  |  | public |
| happiness | `typeof ssbSet.happiness` |  |  | public |
| shiny | `typeof ssbSet.shiny` |  |  | public |

---

### RandomTeams

**File:** `data/random-battles/gen9/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| dex | `ModdedDex` |  | ✓ | public |
| gen | `number` |  |  | public |
| factoryTier | `string` |  |  | public |
| format | `Format` |  |  | public |
| prng | `PRNG` |  |  | public |
| noStab | `string[]` |  |  | public |
| maxTeamSize | `number` |  | ✓ | public |
| adjustLevel | `number | null` |  | ✓ | public |
| maxMoveCount | `number` |  | ✓ | public |
| forceMonotype | `string | undefined` |  | ✓ | public |
| forceTeraType | `string | undefined` |  | ✓ | public |
| moveEnforcementCheckers | `{ [k: string]: MoveEnforcementChecker }` |  |  | public |
| poolsCacheKey | `[string | undefined, number | undefined, RuleTable | undefined, boolean] | undefined` |  |  | private |
| cachedPool | `number[] | undefined` |  |  | private |
| cachedSpeciesPool | `Species[] | undefined` |  |  | private |
| cachedStatusMoves | `ID[]` |  |  | protected |
| effectTypeName | `string,` |  |  | public |
| basicEffectPool | `BasicEffect[],` |  |  | public |
| requiredCount | `number,` |  |  | public |
| requiredCountExplanation | `string` |  |  | public |
| moves | `Set<string> | null,` |  |  | public |
| species | `Species,` |  |  | public |
| teraType | `string,` |  |  | public |
| abilities | `string[],` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| movePool | `string[],` |  |  | public |
| movesA | `string | string[],` |  |  | public |
| movesB | `string | string[],` |  |  | public |
| move | `string,` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| abilities | `string[],` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| movePool | `string[],` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| abilities | `string[],` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| ability | `string,` |  |  | public |
| types | `string[],` |  |  | public |
| moves | `Set<string>,` |  |  | public |
| counter | `MoveCounter,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails,` |  |  | public |
| species | `Species,` |  |  | public |
| isLead | `boolean,` |  |  | public |
| teraType | `string,` |  |  | public |
| role | `RandomTeamsTypes.Role,` |  |  | public |
| species | `Species,` |  |  | public |
| isDoubles | `boolean,` |  |  | public |
| Uber | `76,` |  |  | public |
| OU | `80,` |  |  | public |
| UUBL | `81,` |  |  | public |
| UU | `82,` |  |  | public |
| RUBL | `83,` |  |  | public |
| RU | `84,` |  |  | public |
| NUBL | `85,` |  |  | public |
| NU | `86,` |  |  | public |
| PUBL | `87,` |  |  | public |
| s | `string | Species,` |  |  | public |
| teamDetails | `RandomTeamsTypes.TeamDetails` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `forme,` |  |  | public |
| gender | `species.baseSpecies` |  |  | public |
| moves | `shuffledMoves,` |  |  | public |
| type | `string,` |  |  | public |
| pokemonToExclude | `RandomTeamsTypes.RandomSet[]` |  |  | public |
| pokemonList | `string[]` |  |  | public |
| randomSets | `{ [species: string]: RandomTeamsTypes.RandomSpeciesData }` |  |  | public |
| randomDoublesSets | `{ [species: string]: RandomTeamsTypes.RandomSpeciesData }` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| moves | `m,` |  |  | public |
| randomFactorySets | `{ [format: string]: { [species: string]: BattleFactorySpecies } }` |  |  | public |
| species | `Species, teamData: RandomTeamsTypes.FactoryTeamDetails, tier: string` |  |  | public |
| stealthrock | `'stealthRock',` |  |  | public |
| stoneaxe | `'stealthRock',` |  |  | public |
| spikes | `'spikes',` |  |  | public |
| ceaselessedge | `'spikes',` |  |  | public |
| toxicspikes | `'toxicSpikes',` |  |  | public |
| rapidspin | `'hazardClear',` |  |  | public |
| defog | `'hazardClear',` |  |  | public |
| toxicdebris | `'toxicSpikes',` |  |  | public |
| set | `BattleFactorySet, moves?: string[], item?: string,` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| happiness | `255,` |  |  | public |
| evs | `{ hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0, ...setData.set.evs },` |  |  | public |
| ivs | `{ hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31, ...setData.set.ivs },` |  |  | public |
| wantsTera | `setData.set.wantsTera,` |  |  | public |
| Uber | `5,` |  |  | public |
| OU | `4, UUBL: 4,` |  |  | public |
| UU | `3, RUBL: 3,` |  |  | public |
| RU | `2, NUBL: 2,` |  |  | public |
| NU | `1, PUBL: 1,` |  |  | public |
| PU | `0,` |  |  | public |
| typeCount | `{},` |  |  | public |
| typeComboCount | `{},` |  |  | public |
| baseFormes | `{},` |  |  | public |
| has | `{},` |  |  | public |
| wantsTeraCount | `0,` |  |  | public |
| weaknesses | `{},` |  |  | public |
| resistances | `{},` |  |  | public |
| dryskin | `['Water'], waterabsorb: ['Water'], stormdrain: ['Water'],` |  |  | public |
| flashfire | `['Fire'], heatproof: ['Fire'], waterbubble: ['Fire'], wellbakedbody: ['Fire'],` |  |  | public |
| lightningrod | `['Electric'], motordrive: ['Electric'], voltabsorb: ['Electric'],` |  |  | public |
| sapsipper | `['Grass'],` |  |  | public |
| thickfat | `['Ice', 'Fire'],` |  |  | public |
| eartheater | `['Ground'], levitate: ['Ground'],` |  |  | public |
| stealthrock | `'stealthRock',` |  |  | public |
| stoneaxe | `'stealthRock',` |  |  | public |
| spikes | `'spikes',` |  |  | public |
| ceaselessedge | `'spikes',` |  |  | public |
| toxicspikes | `'toxicSpikes',` |  |  | public |
| rapidspin | `'hazardClear',` |  |  | public |
| defog | `'hazardClear',` |  |  | public |
| toxicdebris | `'toxicSpikes',` |  |  | public |
| randomBSSFactorySets | `AnyObject` |  |  | public |
| species | `Species, teamData: RandomTeamsTypes.FactoryTeamDetails` |  |  | public |
| batonpass | `1,` |  |  | public |
| stealthrock | `1,` |  |  | public |
| toxicspikes | `1,` |  |  | public |
| trickroom | `1,` |  |  | public |
| auroraveil | `1,` |  |  | public |
| electricsurge | `"electric",` |  |  | public |
| psychicsurge | `"psychic",` |  |  | public |
| grassysurge | `"grassy",` |  |  | public |
| seedsower | `"grassy",` |  |  | public |
| mistysurge | `"misty",` |  |  | public |
| electricseed | `"electric",` |  |  | public |
| psychicseed | `"psychic",` |  |  | public |
| grassyseed | `"grassy",` |  |  | public |
| mistyseed | `"misty",` |  |  | public |
| set | `BSSFactorySet, moveVariants?: number[], itemVariants?: number, abilityVariants?: number,` |  |  | public |
| name | `setData.set.species || species.baseSpecies,` |  |  | public |
| species | `setData.set.species,` |  |  | public |
| level | `50,` |  |  | public |
| happiness | `255,` |  |  | public |
| evs | `{ hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0, ...setData.set.evs },` |  |  | public |
| ivs | `{ hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31, ...setData.set.ivs },` |  |  | public |
| nature | `setData.set.nature || "Serious",` |  |  | public |
| wantsTera | `setData.set.wantsTera,` |  |  | public |
| typeCount | `{},` |  |  | public |
| typeComboCount | `{},` |  |  | public |
| baseFormes | `{},` |  |  | public |
| has | `{},` |  |  | public |
| wantsTeraCount | `0,` |  |  | public |
| weaknesses | `{},` |  |  | public |
| resistances | `{},` |  |  | public |
| drizzle | `"raindance",` |  |  | public |
| drought | `"sunnyday",` |  |  | public |
| snowwarning | `"hail",` |  |  | public |
| sandstream | `"sandstorm",` |  |  | public |
| waterabsorb | `["Water"],` |  |  | public |
| flashfire | `["Fire"],` |  |  | public |
| lightningrod | `["Electric"],` |  |  | public |
| voltabsorb | `["Electric"],` |  |  | public |
| thickfat | `["Ice", "Fire"],` |  |  | public |
| levitate | `["Ground"],` |  |  | public |
| randomDraftFactoryMatchups | `AnyObject` |  |  | public |
| name | `species.baseSpecies,` |  |  | public |
| species | `species.name,` |  |  | public |
| gender | `set.gender,` |  |  | public |
| moves | `set.moves,` |  |  | public |
| ability | `set.ability,` |  |  | public |
| evs | `set.evs,` |  |  | public |
| ivs | `set.ivs,` |  |  | public |
| item | `set.item,` |  |  | public |
| level | `this.adjustLevel || set.level,` |  |  | public |
| shiny | `!!set.shiny,` |  |  | public |
| nature | `set.nature,` |  |  | public |
| teraType | `set.teraType,` |  |  | public |
| teraCaptain | `set.name` |  |  | public |

---

### RawBattleStream

**File:** `sim/tools/runner.ts`

**Extends:** BattleStreams.BattleStream

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| rawInputLog | `string[]` |  | ✓ | public |

---

### RawProcessWrapper

**File:** `lib/process-manager.ts`

**Implements:** ProcessWrapper, StreamWorker

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| process | `ChildProcess & { process: undefined } | Worker` |  |  | public |
| stream | `RawSubprocessStream` |  |  | public |
| pendingRelease | `Promise<void> | null` |  |  | public |
| debug | `string` | ✓ |  | public |

---

### ReadStream

**File:** `lib/streams.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| buf | `Buffer` |  |  | public |
| bufStart | `number` |  |  | public |
| bufEnd | `number` |  |  | public |
| bufCapacity | `number` |  |  | public |
| readSize | `number` |  |  | public |
| atEOF | `boolean` |  |  | public |
| errorBuf | `Error[] | null` |  |  | public |
| encoding | `BufferEncoding` |  |  | public |
| isReadable | `boolean` |  |  | public |
| isWritable | `boolean` |  |  | public |
| nodeReadableStream | `NodeJS.ReadableStream | null` |  |  | public |
| nextPush | `Promise<void>` |  |  | public |
| awaitingPush | `boolean` |  |  | public |

---

### ReadWriteStream

**File:** `lib/streams.ts`

**Extends:** ReadStream

**Implements:** WriteStream

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| nodeWritableStream | `NodeJS.WritableStream | null` |  |  | public |

---

### RecommendationsInterface

**File:** `server/chat-plugins/the-studio.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| artist | `string, title: string, url: string, description: string,` |  |  | public |
| username | `string, tags: string[], avatar?: string` |  |  | public |
| artist | `string, title: string, url: string, description: string,` |  |  | public |
| username | `string, tags: string[], avatar?: string` |  |  | public |

---

### RemoteClassifier

**File:** `server/artemis/remote.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| comment | `{ text },` |  |  | public |
| attributeScores | `{},` |  |  | public |
| query | `{` |  |  | public |
| key | `Config.perspectiveKey,` |  |  | public |
| headers | `{` |  |  | public |
| timeout | `10 * 1000, // 10s` |  |  | public |

---

### RipgrepLogSearcher

**File:** `server/chat-plugins/chatlog.ts`

**Extends:** FSLogSearcher

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| maxBuffer | `MAX_MEMORY,` |  |  | public |
| cwd | `FS.ROOT_PATH,` |  |  | public |
| search | `regexString, raw: true, date: month, room, args,` |  |  | public |

---

### RoomAuth

**File:** `server/user-groups.ts`

**Extends:** Auth

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| room | `BasicRoom` |  |  | public |

---

### RoomBattleTimer

**File:** `server/room-battle.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| battle | `RoomBattle` |  | ✓ | public |
| timer | `NodeJS.Timeout | null` |  |  | public |
| turn | `number | null` |  |  | public |
| lastDisabledByUser | `null | ID` |  |  | public |
| settings | `GameTimerSettings` |  |  | public |
| dcTimer | `!isChallenge,` |  |  | public |
| dcTimerBank | `isChallenge,` |  |  | public |
| starting | `isChallenge ? STARTING_TIME_CHALLENGE : STARTING_TIME,` |  |  | public |
| grace | `STARTING_GRACE_TIME,` |  |  | public |
| addPerTurn | `hasLongTurns ? 25 : 10,` |  |  | public |
| maxPerTurn | `isChallenge ? MAX_TURN_TIME_CHALLENGE : MAX_TURN_TIME,` |  |  | public |
| maxFirstTurn | `isChallenge ? MAX_TURN_TIME_CHALLENGE : MAX_TURN_TIME,` |  |  | public |
| timeoutAutoChoose | `false,` |  |  | public |
| accelerate | `!timerSettings && !isChallenge,` |  |  | public |

---

### Roomlog

**File:** `server/roomlogs.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| isMultichannel | `boolean` |  | ✓ | public |
| noAutoTruncate | `boolean` |  | ✓ | public |
| noLogTimes | `boolean` |  | ✓ | public |
| roomid | `RoomID` |  |  | public |
| log | `string[]` |  |  | public |
| broadcastBuffer | `string[]` |  |  | public |
| roomlogStream | `Streams.WriteStream | null` | ✓ |  | public |
| roomlogTable | `typeof roomlogTable` |  |  | public |
| roomlogFilename | `string` |  |  | public |
| numTruncatedLines | `number` |  |  | public |
| roomid | `this.roomid,` |  |  | public |
| log | `message,` |  |  | public |
| roomid | `this.roomid,` |  |  | public |
| date | `dateStr,` |  |  | public |
| query | `q, values: vals,` |  |  | public |

---

### RoundRobin

**File:** `server/tournaments/generator-round-robin.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| name | `string` |  | ✓ | public |
| isDrawingSupported | `boolean` |  | ✓ | public |
| isDoubles | `boolean` |  | ✓ | public |
| isBracketFrozen | `boolean` |  |  | public |
| players | `TournamentPlayer[]` |  |  | public |
| totalPendingMatches | `number` |  |  | public |
| perPlayerPendingMatches | `number` |  |  | public |
| matchesPerPlayer | `number` | ✓ |  | public |
| type | `'table',` |  |  | public |
| tableHeaders | `{` |  |  | public |
| state | `'unavailable',` |  |  | public |
| type | `'table',` |  |  | public |
| tableHeaders | `{` |  |  | public |
| state | `match.state,` |  |  | public |

---

### Runner

**File:** `sim/tools/runner.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| move | `0.7,` |  |  | public |
| mega | `0.6,` |  |  | public |

---

### ScavengerGameTemplate

**File:** `server/chat-plugins/scavenger-games.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| room | `Room` |  |  | public |
| playerlist | `null | string[]` |  |  | public |
| timer | `NodeJS.Timeout | null` |  |  | public |

---

### ScavengerHuntDatabase

**File:** `server/chat-plugins/scavengers.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| questions | `[],` |  |  | public |
| text | `'',` |  |  | public |
| answers | `[],` |  |  | public |
| hints | `[],` |  |  | public |
| text | `'',` |  |  | public |
| answers | `[],` |  |  | public |
| hints | `[],` |  |  | public |
| hunt | `{ hosts: FakeUser[], questions: { text: string, answers: string[], hints?: string[] }[] }` |  |  | public |

---

### ScheduleGenerator

**File:** `server/chat-plugins/seasons.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| formats | `string[][]` |  |  | public |

---

### Searcher

**File:** `server/chat-plugins/chatlog.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| results | `{ [date: string]: { [userid: string]: number } } | null,` |  |  | public |
| roomid | `RoomID, month: string, user?: ID` |  |  | public |
| context | `Chat.PageContext, search: string, roomid: RoomID, date: string, limit: number | null` |  |  | public |
| deadTime | `'Average time between lines',` |  |  | public |
| deadPercent | `'Average % of the day spent more than 5 minutes inactive',` |  |  | public |
| linesPerUser | `'Average lines per user',` |  |  | public |
| averagePresent | `'Average users present',` |  |  | public |
| totalLines | `'Average lines per day',` |  |  | public |

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

### SQLStatement

**File:** `lib/database.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| sql | `string[]` |  |  | public |
| values | `BasicSQLValue[]` |  |  | public |

---

### StatementMap

**File:** `server/private-messages/database.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| env | `SQL.TransactionEnvironment` |  |  | public |

---

### StaticServer

**File:** `lib/static-server.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| root | `string` |  |  | public |
| options | `Options` |  |  | public |
| cacheTime | `number | null` |  |  | public |
| defaultHeaders | `Headers` |  |  | public |
| pathname | `string, req: http.IncomingMessage, res: http.ServerResponse` |  |  | public |
| pathname | `string, status: number, headers: Headers, req: http.IncomingMessage, res: http.ServerResponse,` |  |  | public |
| errorCallback | `ErrorCallback` | ✓ |  | public |
| message | `http.STATUS_CODES[status],` |  |  | public |
| result | `Result, req: http.IncomingMessage, res: http.ServerResponse, errorCallback?: ErrorCallback` |  |  | public |
| pathname | `string, status: number, headers: Headers, req: http.IncomingMessage, res: http.ServerResponse` |  |  | public |
| status | `number, contentType: string, _headers: Headers, file: string, stat: fs.Stats,` |  |  | public |
| req | `http.IncomingMessage, res: http.ServerResponse` |  |  | public |
| from | `0,` |  |  | public |
| to | `0,` |  |  | public |
| valid | `false,` |  |  | public |
| status | `number, contentType: string, _headers: Headers, file: string, stat: fs.Stats,` |  |  | public |
| req | `http.IncomingMessage, res: http.ServerResponse` |  |  | public |
| status | `number, _headers: Headers, file: string, stat: fs.Stats,` |  |  | public |
| req | `http.IncomingMessage, res: http.ServerResponse` |  |  | public |
| flags | `'r',` |  |  | public |
| mode | `0o666,` |  |  | public |
| start | `startByte,` |  |  | public |

---

### StreamProcessWrapper

**File:** `lib/process-manager.ts`

**Implements:** ProcessWrapper

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| process | `ChildProcess` |  |  | public |
| pendingRelease | `Promise<void> | null` |  |  | public |
| debug | `string` | ✓ |  | public |

---

### StreamWorker

**File:** `lib/process-manager.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| stream | `Streams.ObjectReadWriteStream<string>` |  |  | public |

---

### Team

**File:** `server/chat-plugins/auction.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| id | `ID` |  |  | public |
| name | `string` |  |  | public |
| players | `Player[]` |  |  | public |
| credits | `number` |  |  | public |
| suspended | `boolean` |  |  | public |
| auction | `Auction` |  |  | private |

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

### TeamValidatorAsync

**File:** `server/team-validator-async.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| format | `Format` |  |  | public |

---

### TestTools

**File:** `test/common.js`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| debug | `true,` |  |  | public |
| forceRandomChance | `options.forceRandomChance,` |  |  | public |
| seed | `options.seed` |  |  | public |
| strictChoices | `options.strictChoices !` |  |  | public |

---

### TextFormatter

**File:** `server/chat-formatter.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| str | `string` |  | ✓ | public |
| buffers | `string[]` |  | ✓ | public |
| stack | `FormatSpan[]` |  | ✓ | public |
| isTrusted | `boolean` |  | ✓ | public |
| replaceLinebreaks | `boolean` |  | ✓ | public |
| showSyntax | `boolean` |  | ✓ | public |
| offset | `number` |  |  | public |
| default | `// do nothing` |  |  | public |

---

### TriviaSQLiteDatabase

**File:** `server/chat-plugins/trivia/database.ts`

**Implements:** TriviaDatabase

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| readyPromise | `Promise<void> | null` |  |  | public |
| legacyJSONPath | `string` | ✓ |  | private |
| leaderboardInsertion | `Statement | null` |  |  | private |
| questionInsertion | `Statement | null` |  |  | private |
| answerInsertion | `Statement | null` |  |  | private |
| gameHistoryInsertion | `Statement | null` |  |  | private |
| scoreHistoryInsertion | `Statement | null` |  |  | private |
| updateMoveEventQuestions | `Statement | null` |  |  | private |
| categoryChangeQuery | `Statement | null` |  |  | private |
| leaderboardChangeQuery | `Statement | null` |  |  | private |
| migrateCategoryQuery | `Statement | null` |  |  | private |
| historyQuery | `Statement | null` |  |  | private |
| historyScoresQuery | `Statement | null` |  |  | private |
| allQuestionsRandomOrderQuery | `Statement | null` |  |  | private |
| allQuestionsNewestFirstQuery | `Statement | null` |  |  | private |
| allQuestionsOldestFirstQuery | `Statement | null` |  |  | private |
| answersQuery | `Statement | null` |  |  | private |
| submissionsQuery | `Statement | null` |  |  | private |
| leaderboardQuery | `Statement | null` |  |  | private |
| leaderboardByUserQuery | `Statement | null` |  |  | private |
| scoreAndPointsByUser | `Statement | null` |  |  | private |
| eventQuestionQuery | `Statement | null` |  |  | private |
| categoriesQuery | `Statement | null` |  |  | private |
| questionCountQuery | `Statement | null` |  |  | private |
| categoryQuestionCountQuery | `Statement | null` |  |  | private |
| questionSearchQuery | `Statement | null` |  |  | private |
| questionExistsQuery | `Statement | null` |  |  | private |
| clearAllSubmissionsQuery | `Statement | null` |  |  | private |
| clearCategoryQuery | `Statement | null` |  |  | private |
| clearCycleLeaderboardQuery | `Statement | null` |  |  | private |
| deleteQuestionQuery | `Statement | null` |  |  | private |
| leaderboardDeletionQuery | `Statement | null` |  |  | private |
| userid | `ID,` |  |  | public |
| additions | `Record<Leaderboard, TriviaLeaderboardScore>,` |  |  | public |
| score | `additions[lb].score,` |  |  | public |
| totalPoints | `additions[lb].totalPoints,` |  |  | public |
| totalCorrectAnswers | `additions[lb].totalCorrectAnswers,` |  |  | public |
| leaderboard | `discrim,` |  |  | public |
| isSubmission | `false,` |  |  | public |
| isSubmission | `true,` |  |  | public |
| score | `0,` |  |  | public |
| totalCorrectAnswers | `0,` |  |  | public |
| totalPoints | `0,` |  |  | public |
| score | `0,` |  |  | public |
| totalCorrectAnswers | `0,` |  |  | public |
| totalPoints | `0,` |  |  | public |
| mode | `row.mode,` |  |  | public |
| category | `row.category,` |  |  | public |
| creator | `row.creator || undefined,` |  |  | public |
| givesPoints | `row.givesPoints !` |  |  | public |
| startTime | `row.time,` |  |  | public |
| categories | `string[] | 'all',` |  |  | public |
| limit | `number,` |  |  | public |
| options | `{ order: 'newestfirst' | 'oldestfirst' | 'random' }` |  |  | public |
| score | `row.score,` |  |  | public |
| totalPoints | `row.total_points,` |  |  | public |
| totalCorrectAnswers | `row.total_correct_answers,` |  |  | public |
| alltime | `{},` |  |  | public |
| nonAlltime | `{},` |  |  | public |
| cycle | `{},` |  |  | public |
| score | `row.score,` |  |  | public |
| totalPoints | `row.total_points,` |  |  | public |
| totalCorrectAnswers | `row.total_correct_answers,` |  |  | public |
| search | `string,` |  |  | public |
| options | `{ searchSubmissions: boolean, caseSensitive?: boolean }` |  |  | public |
| question | `row.question,` |  |  | public |
| category | `row.category,` |  |  | public |
| user | `row.userid,` |  |  | public |
| addedAt | `row.added_at,` |  |  | public |

---

### User

**File:** `server/users.ts`

**Extends:** Chat.MessageContext

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| inRooms | `Set<RoomID>` |  | ✓ | public |
| games | `Set<RoomID>` |  | ✓ | public |
| mmrCache | `{ [format: string]: number }` |  |  | public |
| guestNum | `number` |  |  | public |
| name | `string` |  |  | public |
| named | `boolean` |  |  | public |
| registered | `boolean` |  |  | public |
| id | `ID` |  |  | public |
| tempGroup | `GroupSymbol` |  |  | public |
| avatar | `string | number` |  |  | public |
| connected | `boolean` |  |  | public |
| connections | `Connection[]` |  |  | public |
| latestHost | `string` |  |  | public |
| latestHostType | `string` |  |  | public |
| ips | `string[]` |  |  | public |
| latestIp | `string` |  |  | public |
| locked | `ID | PunishType | null` |  |  | public |
| semilocked | `ID | PunishType | null` |  |  | public |
| namelocked | `ID | PunishType | null` |  |  | public |
| permalocked | `ID | PunishType | null` |  |  | public |
| punishmentTimer | `NodeJS.Timeout | null` |  |  | public |
| previousIDs | `ID[]` |  |  | public |
| lastChallenge | `number` |  |  | public |
| lastPM | `string` |  |  | public |
| lastMatch | `ID` |  |  | public |
| settings | `UserSettings` |  |  | public |
| battleSettings | `{` |  |  | public |
| team | `string,` |  |  | public |
| hidden | `boolean,` |  |  | public |
| inviteOnly | `boolean,` |  |  | public |
| special | `string,` | ✓ |  | public |
| isSysop | `boolean` |  |  | public |
| isStaff | `boolean` |  |  | public |
| isPublicBot | `boolean` |  |  | public |
| lastDisconnected | `number` |  |  | public |
| lastConnected | `number` |  |  | public |
| foodfight | `{ generatedTeam: string[], dish: string, ingredients: string[], timestamp: number }` | ✓ |  | public |
| friends | `Set<string>` | ✓ |  | public |
| chatQueue | `ChatQueueEntry[] | null` |  |  | public |
| chatQueueTimeout | `NodeJS.Timeout | null` |  |  | public |
| lastChatMessage | `number` |  |  | public |
| lastCommand | `string` |  |  | public |
| notified | `{` |  |  | public |
| blockChallenges | `boolean,` |  |  | public |
| blockPMs | `boolean,` |  |  | public |
| blockInvites | `boolean,` |  |  | public |
| punishment | `boolean,` |  |  | public |
| lock | `boolean,` |  |  | public |
| lastMessage | `string` |  |  | public |
| lastMessageTime | `number` |  |  | public |
| lastReportTime | `number` |  |  | public |
| s1 | `string` |  |  | public |
| s2 | `string` |  |  | public |
| s3 | `string` |  |  | public |
| autoconfirmed | `ID` |  |  | public |
| trusted | `ID` |  |  | public |
| trackRename | `string` |  |  | public |
| statusType | `StatusType` |  |  | public |
| userMessage | `string` |  |  | public |
| lastWarnedAt | `number` |  |  | public |
| blockChallenges | `false,` |  |  | public |
| blockPMs | `false,` |  |  | public |
| ignoreTickets | `false,` |  |  | public |
| hideBattlesFromTrainerCard | `false,` |  |  | public |
| blockInvites | `false,` |  |  | public |
| doNotDisturb | `false,` |  |  | public |
| blockFriendRequests | `false,` |  |  | public |
| allowFriendNotifications | `false,` |  |  | public |
| displayBattlesToFriends | `false,` |  |  | public |
| hideLogins | `false,` |  |  | public |
| team | `'',` |  |  | public |
| hidden | `false,` |  |  | public |
| inviteOnly | `false,` |  |  | public |
| blockChallenges | `false,` |  |  | public |
| blockPMs | `false,` |  |  | public |
| blockInvites | `false,` |  |  | public |
| punishment | `false,` |  |  | public |
| lock | `false,` |  |  | public |
| permission | `RoomPermission & GlobalPermission,` |  |  | public |
| target | `User | null,` |  |  | public |
| room | `BasicRoom | null,` | ✓ |  | public |
| cmd | `string,` | ✓ |  | public |
| cmdToken | `string,` | ✓ |  | public |
| permission | `string,` |  |  | public |
| target | `User | null` |  |  | public |
| room | `BasicRoom | null` |  |  | public |
| cmd | `string,` | ✓ |  | public |
| cmdToken | `string,` | ✓ |  | public |
| hiddenNextBattle | `this.battleSettings.hidden,` |  |  | public |
| inviteOnlyNextBattle | `this.battleSettings.inviteOnly,` |  |  | public |
| language | `this.language,` |  |  | public |

---

### WriteStream

**File:** `lib/streams.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| isReadable | `boolean` |  |  | public |
| isWritable | `true` |  |  | public |
| encoding | `BufferEncoding` |  |  | public |
| nodeWritableStream | `NodeJS.WritableStream | null` |  |  | public |

---

### YoutubeInterface

**File:** `server/chat-plugins/youtube.ts`

**Fields:**

| Name | Type | Optional | Readonly | Visibility |
|------|------|----------|----------|------------|
| interval | `NodeJS.Timeout | null` |  |  | public |
| intervalTime | `number` |  |  | public |
| data | `ChannelData` |  |  | public |
| query | `{ part: 'snippet,statistics', id, key: Config.youtubeKey },` |  |  | public |
| name | `data.snippet.title,` |  |  | public |
| description | `data.snippet.description,` |  |  | public |
| url | `data.snippet.customUrl,` |  |  | public |
| icon | `data.snippet.thumbnails.medium.url,` |  |  | public |
| query | `{ part: 'snippet,statistics', id, key: Config.youtubeKey },` |  |  | public |
| title | `video.snippet.title,` |  |  | public |
| description | `video.snippet.description,` |  |  | public |
| channelTitle | `video.snippet.channelTitle,` |  |  | public |
| channelUrl | `video.snippet.channelId,` |  |  | public |
| views | `video.statistics.viewCount,` |  |  | public |
| thumbnail | `video.snippet.thumbnails.default.url,` |  |  | public |
| likes | `video.statistics.likeCount,` |  |  | public |
| dislikes | `video.statistics.dislikeCount,` |  |  | public |
| query | `{` |  |  | public |
| part | `'snippet', q: name,` |  |  | public |
| key | `Config.youtubeKey, order: 'relevance',` |  |  | public |
| query | `{` |  |  | public |
| part | `'snippet', q: name, type: 'channel',` |  |  | public |
| key | `Config.youtubeKey, order: 'relevance', maxResults: limit,` |  |  | public |

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

### AddressRange

**File:** `server/ip-tools.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| minIP | `number` |  |  |
| maxIP | `number` |  |  |
| host | `string` | ✓ |  |

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

### AnnotatedChatCommands

**File:** `server/chat.ts`

---

### AnnouncementData

**File:** `server/chat-plugins/announcements.ts`

**Extends:** AnnouncementOptions

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| activityid | `'announcement'` |  | ✓ |

---

### AnnouncementOptions

**File:** `server/chat-plugins/announcements.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| activityNumber | `number` | ✓ |  |
| source | `string` |  |  |
| timeoutMins | `number` | ✓ |  |
| timerEnd | `number` | ✓ |  |

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

### AutoPunishment

**File:** `server/chat-plugins/helptickets-auto.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| modlogCount | `number` | ✓ |  |
| severity | `{ type: string[], certainty: number }` | ✓ |  |
| isSingleMessage | `boolean` | ✓ |  |
| punishment | `string` |  |  |
| ticketType | `string` |  |  |

---

### AutoSettings

**File:** `server/chat-plugins/helptickets-auto.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| punishments | `AutoPunishment[]` |  |  |
| applyPunishments | `boolean` |  |  |

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

### BattleFactorySet

**File:** `data/random-battles/gen9/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| species | `string` |  |  |
| weight | `number` |  |  |
| item | `string[]` |  |  |
| ability | `string[]` |  |  |
| nature | `string[]` |  |  |
| moves | `string[][]` |  |  |
| teraType | `string[]` |  |  |
| gender | `string` | ✓ |  |
| wantsTera | `boolean` | ✓ |  |
| evs | `Partial<StatsTable>` | ✓ |  |
| ivs | `Partial<StatsTable>` | ✓ |  |
| shiny | `boolean` | ✓ |  |

---

### BattleFactorySpecies

**File:** `data/random-battles/gen9/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| sets | `BattleFactorySet[]` |  |  |
| weight | `number` |  |  |

---

### BattleInfo

**File:** `server/chat-plugins/helptickets.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| log | `string[]` |  |  |
| url | `string` |  |  |
| title | `string` |  |  |
| players | `{ p1: ID, p2: ID, p3?: ID, p4?: ID }` |  |  |
| pokemon | `Record<string, { species: string, name?: string }[]>` |  |  |

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

### BattleOutcome

**File:** `server/chat-plugins/battlesearch.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| lost | `string` |  |  |
| won | `string` |  |  |
| turns | `string` |  |  |

---

### BattleRequestTracker

**File:** `server/room-battle.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| rqid | `number` |  |  |
| request | `string` |  |  |
| isWait | `'cantUndo' | true | false` |  |  |
| choice | `string` |  |  |

---

### BattleRoomTable

**File:** `server/rooms.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| p1 | `string` | ✓ |  |
| p2 | `string` | ✓ |  |
| minElo | `'tour' | number` | ✓ |  |

---

### BattleScriptsData

**File:** `sim/global-types.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| gen | `number` |  |  |

---

### BattleSearchResults

**File:** `server/chat-plugins/battlesearch.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| totalBattles | `number` |  |  |
| totalOutcomes | `BattleOutcome[] | null` |  |  |
| totalWins | `{ [k: string]: number }` |  |  |
| totalLosses | `{ [k: string]: number }` |  |  |
| totalTies | `number` |  |  |
| timesBattled | `{ [k: string]: number }` |  |  |

---

### BSSFactorySet

**File:** `data/random-battles/gen9/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| species | `string` |  |  |
| weight | `number` |  |  |
| item | `string[]` |  |  |
| ability | `string` |  |  |
| nature | `string` |  |  |
| moves | `string[][]` |  |  |
| teraType | `string[]` |  |  |
| gender | `string` | ✓ |  |
| wantsTera | `boolean` | ✓ |  |
| evs | `number[]` |  |  |
| ivs | `number[]` | ✓ |  |

---

### Card

**File:** `server/chat-plugins/uno.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| value | `string` |  |  |
| color | `Color` |  |  |
| changedColor | `Color` | ✓ |  |
| name | `string` |  |  |

---

### ChannelData

**File:** `server/chat-plugins/youtube.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| channels | `{ [k: string]: ChannelEntry }` |  |  |
| categories | `string[]` |  |  |
| intervalTime | `number` | ✓ |  |

---

### ChannelEntry

**File:** `server/chat-plugins/youtube.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| description | `string` |  |  |
| url | `string` |  |  |
| icon | `string` |  |  |
| videos | `number` |  |  |
| subs | `number` |  |  |
| views | `number` |  |  |
| username | `string` | ✓ |  |
| category | `string` | ✓ |  |

---

### ChatCommands

**File:** `server/chat.ts`

---

### ChatlogSearch

**File:** `server/chat-plugins/chatlog.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| raw | `boolean` | ✓ |  |
| search | `string` |  |  |
| room | `RoomID` |  |  |
| date | `string` |  |  |
| limit | `number | null` | ✓ |  |
| args | `string[]` | ✓ |  |

---

### ChatPlugin

**File:** `server/chat.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| commands | `AnnotatedChatCommands` | ✓ |  |
| pages | `PageTable` | ✓ |  |
| destroy | `()` | ✓ |  |
| roomSettings | `SettingsHandler | SettingsHandler[]` | ✓ |  |

---

### ChatRoomTable

**File:** `server/rooms.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| title | `string` |  |  |
| desc | `string` |  |  |
| userCount | `number` |  |  |
| section | `string` | ✓ |  |
| subRooms | `string[]` | ✓ |  |
| spotlight | `string` | ✓ |  |
| privacy | `RoomSettings['isPrivate']` |  |  |

---

### CheckerResult

**File:** `server/chat-plugins/helptickets-auto.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| action | `string` |  |  |
| user | `User | ID` |  |  |
| result | `Record<string, number>` |  |  |
| reason | `string` |  |  |
| roomid | `string` | ✓ |  |
| displayReason | `string` | ✓ |  |
| proof | `string` | ✓ |  |

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

### Commit

**File:** `server/chat-plugins/github.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `string` |  |  |
| message | `string` |  |  |
| author | `{ name: string, avatar_url: string }` |  |  |
| url | `string` |  |  |

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

### DashyStream

**File:** `lib/dashycode.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| codeBuf | `string` |  |  |
| buf | `number` |  |  |
| bufLength | `number` |  |  |

---

### DatabaseRequest

**File:** `server/friends.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| statement | `string` |  |  |
| type | `'all' | 'get' | 'run' | 'transaction'` |  |  |
| data | `AnyObject | any[]` |  |  |

---

### DatabaseResult

**File:** `server/friends.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| error | `string` | ✓ |  |
| result | `any` | ✓ |  |

---

### DexOrGroup

**File:** `server/chat-plugins/datasearch.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| abilities | `{ [k: string]: boolean }` |  |  |
| tiers | `{ [k: string]: boolean }` |  |  |
| doublesTiers | `{ [k: string]: boolean }` |  |  |
| colors | `{ [k: string]: boolean }` |  |  |
| formes | `{ [k: string]: boolean }` |  |  |
| gens | `{ [k: string]: boolean }` |  |  |
| moves | `{ [k: string]: boolean }` |  |  |
| types | `{ [k: string]: boolean }` |  |  |
| resists | `{ [k: string]: boolean }` |  |  |
| weak | `{ [k: string]: boolean }` |  |  |
| stats | `{ [k: string]: { [k in Direction]: { [s: string]: number | boolean } } }` |  |  |
| skip | `boolean` |  |  |

---

### DexResources

**File:** `server/chat-commands/info.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| url | `string` |  |  |
| resources | `{ resource_name: string, url: string }[]` |  |  |

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

### ElimTree

**File:** `server/tournaments/generator-elimination.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| root | `ElimNode` |  |  |
| currentLayerLeafNodes | `ElimNode[]` |  |  |
| nextLayerLeafNodes | `ElimNode[]` |  |  |

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

### FakeUser

**File:** `server/chat-plugins/scavengers.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| id | `string` |  |  |
| noUpdate | `boolean` | ✓ |  |

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

### FilterSettings

**File:** `server/chat-plugins/abuse-monitor.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| disabled | `boolean` | ✓ |  |
| thresholdIncrement | `{ turns: number, amount: number, minTurns?: number } | null` |  |  |
| threshold | `number` |  |  |
| minScore | `number` |  |  |
| specials | `{ [k: string]: { [k: number]: number | "MAXIMUM" } }` |  |  |
| replacements | `Record<string, string>` |  |  |
| punishments | `PunishmentSettings[]` |  |  |
| recommendOnly | `boolean` | ✓ |  |

---

### FilterWord

**File:** `server/chat.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| regex | `RegExp` |  |  |
| word | `string` |  |  |
| hits | `number` |  |  |
| reason | `string` | ✓ |  |
| publicReason | `string` | ✓ |  |
| replacement | `string` | ✓ |  |

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

### FormatData

**File:** `tools/set-import/sets/index.d.ts`

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

### Friend

**File:** `server/friends.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| userid | `ID` |  |  |
| friend | `ID` |  |  |
| send_login_data | `number` |  |  |
| last_login | `number` |  |  |
| public_list | `number` |  |  |
| allowing_login | `number` |  |  |

---

### GameMode

**File:** `server/chat-plugins/scavenger-games.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| id | `string` |  |  |
| mod | `Twist` |  |  |
| round | `number` | ✓ |  |
| leaderboard | `true` | ✓ |  |
| teamAnnounce | `GameModeFunction` | ✓ |  |
| getPlayerTeam | `GameModeFunction` | ✓ |  |
| advanceTeam | `GameModeFunction` | ✓ |  |

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

### Gen1RandomBattleSpecies

**File:** `data/random-battles/gen1/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| level | `number` | ✓ |  |
| moves | `ID[]` | ✓ |  |
| essentialMoves | `ID[]` | ✓ |  |
| exclusiveMoves | `ID[]` | ✓ |  |
| comboMoves | `ID[]` | ✓ |  |

---

### GenerationData

**File:** `tools/set-import/sets/index.d.ts`

---

### GitData

**File:** `server/chat-plugins/github.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| usernames | `{ [username: string]: string }` | ✓ |  |
| bans | `{ [username: string]: string }` | ✓ |  |

---

### GitHookHandler

**File:** `server/chat-plugins/github.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| server | `import('http').Server` |  |  |

---

### Global

**File:** `server/global-variables.d.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| Config | `any` |  |  |
| Chat | `any` |  |  |
| Dex | `any` |  |  |
| Teams | `any` |  |  |
| IPTools | `any` |  |  |
| Ladders | `any` |  |  |
| LoginServer | `any` |  |  |
| Monitor | `any` |  |  |
| nodeOomHeapdump | `any` |  |  |
| Punishments | `any` |  |  |
| Rooms | `any` |  |  |
| Sockets | `any` |  |  |
| TeamValidatorAsync | `any` |  |  |
| Tournaments | `any` |  |  |
| Users | `any` |  |  |
| Verifier | `any` |  |  |
| toID | `(item: any)` |  |  |
| __version | `{head: string, origin?: string, tree?: string}` |  |  |

---

### HackmonsCupEntry

**File:** `data/random-battles/gen1/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| types | `string[]` |  |  |
| baseStats | `StatsTable` |  |  |

---

### Handlers

**File:** `server/chat.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| onRoomClose | `(id: string, user: User, connection: Connection, page: boolean)` |  |  |
| onRenameRoom | `(oldId: RoomID, newID: RoomID, room: BasicRoom)` |  |  |
| onBattleStart | `(user: User, room: GameRoom)` |  |  |
| onBattleLeave | `(user: User, room: GameRoom)` |  |  |
| onRoomJoin | `(room: BasicRoom, user: User, connection: Connection)` |  |  |
| onBeforeRoomJoin | `(room: BasicRoom, user: User, connection: Connection)` |  |  |
| onDisconnect | `(user: User)` |  |  |
| onRoomDestroy | `(roomid: RoomID)` |  |  |
| onBattleEnd | `(battle: RoomBattle, winner: ID, players: ID[])` |  |  |
| onBattleCreate | `(battle: RoomBattle, players: ID[])` |  |  |
| onLadderSearch | `(user: User, connection: Connection, format: ID)` |  |  |
| onBattleRanked | `(` |  |  |
| battle | `Rooms.RoomBattle, winner: ID, ratings: (AnyObject | null | undefined)[], players: ID[]` |  |  |
| onRename | `(user: User, oldID: ID, newID: ID)` |  |  |
| onTicketCreate | `(ticket: import('./chat-plugins/helptickets').TicketState, user: User)` |  |  |
| onChallenge | `(user: User, targetUser: User, format: string | ID)` |  |  |
| onMessageOffline | `(context: Chat.CommandContext, message: string, targetUserID: ID)` |  |  |
| onBattleJoin | `(slot: string, user: User, battle: RoomBattle)` |  |  |
| onPunishUser | `(type: string, user: User, room?: Room | null)` |  |  |

---

### HangmanEntry

**File:** `server/chat-plugins/hangman.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| hints | `string[]` |  |  |
| tags | `string[]` | ✓ |  |

---

### HangmanOptions

**File:** `server/chat-plugins/hangman.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| allowCreator | `boolean` | ✓ |  |

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

### IPData

**File:** `server/chat-plugins/permalocks.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| country | `string` |  |  |
| isp | `string` |  |  |
| city | `string` |  |  |
| regionName | `string` |  |  |
| lat | `number` |  |  |
| lon | `number` |  |  |

---

### ItemDataTable

**File:** `sim/dex-items.ts`

---

### Leaderboard

**File:** `server/chat-plugins/laddertours.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| current | `LeaderboardEntry[]` | ✓ |  |
| last | `LeaderboardEntry[]` | ✓ |  |
| lookup | `Map<string, LeaderboardEntry>` |  |  |

---

### LeaderboardEntry

**File:** `server/chat-plugins/laddertours.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| rank | `number` | ✓ |  |
| elo | `number` |  |  |
| gxe | `number` |  |  |
| glicko | `number` |  |  |
| glickodev | `number` |  |  |

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

### LoggedMessage

**File:** `server/chat-plugins/responder.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| message | `string` |  |  |
| faqName | `string` |  |  |
| regex | `string` |  |  |
| date | `string` |  |  |

---

### MafiaData

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| alignments | `{ [k: string]: MafiaDataAlignment }` |  |  |
| roles | `{ [k: string]: MafiaDataRole }` |  |  |
| themes | `{ [k: string]: MafiaDataTheme }` |  |  |
| IDEAs | `{ [k: string]: MafiaDataIDEA }` |  |  |
| terms | `{ [k: string]: MafiaDataTerm }` |  |  |
| aliases | `{ [k: string]: ID }` |  |  |

---

### MafiaDataAlignment

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| plural | `string` |  |  |
| color | `string` | ✓ |  |
| buttonColor | `string` | ✓ |  |
| memo | `string[]` |  |  |
| image | `string` | ✓ |  |

---

### MafiaDataIDEA

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| memo | `undefined` | ✓ |  |
| roles | `string[]` |  |  |
| picks | `string[]` |  |  |
| choices | `number` |  |  |

---

### MafiaDataRole

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| memo | `string[]` |  |  |
| alignment | `string` | ✓ |  |
| image | `string` | ✓ |  |

---

### MafiaDataTerm

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| memo | `string[]` |  |  |

---

### MafiaDataTheme

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| memo | `undefined` | ✓ |  |
| desc | `string` |  |  |

---

### MafiaIDEAData

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| untrusted | `true` | ✓ |  |
| roles | `string[]` |  |  |
| choices | `number` |  |  |
| picks | `string[]` |  |  |

---

### MafiaIDEAModule

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| data | `MafiaIDEAData | null` |  |  |
| timer | `NodeJS.Timeout | null` |  |  |
| discardsHidden | `boolean` |  |  |
| discardsHTML | `string` |  |  |
| waitingPick | `string[]` |  |  |

---

### MafiaIDEAPlayerData

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| choices | `string[]` |  |  |
| originalChoices | `string[]` |  |  |
| picks | `{ [choice: string]: string | null }` |  |  |

---

### MafiaLogTable

**File:** `server/chat-plugins/mafia.ts`

---

### MafiaRole

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| safeName | `string` |  |  |
| id | `string` |  |  |
| memo | `string[]` |  |  |
| alignment | `string` |  |  |
| image | `string` |  |  |

---

### MafiaVote

**File:** `server/chat-plugins/mafia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| count | `number` |  |  |
| trueCount | `number` |  |  |
| lastVote | `number` |  |  |
| dir | `'up' | 'down'` |  |  |
| voters | `ID[]` |  |  |

---

### Manager

**File:** `server/chat-plugins/auction.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `ID` |  |  |
| team | `Team` |  |  |

---

### Match

**File:** `server/tournaments/generator-round-robin.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| state | `string` |  |  |
| score | `number[]` | ✓ |  |
| result | `string` | ✓ |  |

---

### MetaSettings

**File:** `server/chat-plugins/abuse-monitor.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| modlogIgnores | `Record<string, string | number[]>` | ✓ |  |

---

### MinorActivityData

**File:** `server/room-minor-activity.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| activityid | `'announcement' | 'poll'` |  | ✓ |
| activityNumber | `number` | ✓ |  |
| source | `string` | ✓ |  |
| timeoutMins | `number` | ✓ |  |
| timerEnd | `number` | ✓ |  |
| question | `string` |  |  |
| supportHTML | `boolean` |  |  |
| multiPoll | `boolean` |  |  |
| pendingVotes | `{ [userid: string]: number[] }` | ✓ |  |
| voters | `{ [k: string]: number[] }` | ✓ |  |
| voterIps | `{ [k: string]: number[] }` | ✓ |  |
| totalVotes | `number` | ✓ |  |
| isQuiz | `boolean` | ✓ |  |
| answers | `string[] | { name: string, votes: number, correct?: boolean }[]` |  |  |

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

### ModEvent

**File:** `server/chat-plugins/scavengers.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| priority | `number` |  |  |
| exec | `TwistEvent` |  |  |

---

### ModlogEntry

**File:** `server/modlog/index.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| action | `string` |  |  |
| roomID | `string` |  |  |
| visualRoomID | `string` |  |  |
| userid | `ID | null` |  |  |
| autoconfirmedID | `ID | null` |  |  |
| alts | `ID[]` |  |  |
| ip | `string | null` |  |  |
| isGlobal | `boolean` |  |  |
| loggedBy | `ID | null` |  |  |
| note | `string` |  |  |
| time | `number` |  |  |

---

### ModlogResults

**File:** `server/modlog/index.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| results | `(ModlogEntry & { entryID: number })[]` |  |  |
| duration | `number` |  |  |

---

### ModlogSearch

**File:** `server/modlog/index.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| note | `{ search: string, isExact?: boolean, isExclusion?: boolean }[]` |  |  |
| user | `{ search: string, isExact?: boolean, isExclusion?: boolean }[]` |  |  |
| ip | `{ search: string, isExclusion?: boolean }[]` |  |  |
| action | `{ search: string, isExclusion?: boolean }[]` |  |  |
| actionTaker | `{ search: string, isExclusion?: boolean }[]` |  |  |

---

### Monitor

**File:** `server/chat.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| location | `string` |  |  |
| punishment | `string` |  |  |
| label | `string` |  |  |
| condition | `string` | ✓ |  |
| monitor | `MonitorHandler` | ✓ |  |

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

### MoveOrGroup

**File:** `server/chat-plugins/datasearch.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| types | `{ [k: string]: boolean }` |  |  |
| categories | `{ [k: string]: boolean }` |  |  |
| contestTypes | `{ [k: string]: boolean }` |  |  |
| flags | `{ [k: string]: boolean }` |  |  |
| gens | `{ [k: string]: boolean }` |  |  |
| other | `{ [k: string]: boolean }` |  |  |
| mon | `{ [k: string]: boolean }` |  |  |
| property | `{ [k: string]: { [k in Direction]: number } }` |  |  |
| boost | `{ [k: string]: boolean }` |  |  |
| lower | `{ [k: string]: boolean }` |  |  |
| zboost | `{ [k: string]: boolean }` |  |  |
| status | `{ [k: string]: boolean }` |  |  |
| volatileStatus | `{ [k: string]: boolean }` |  |  |
| targets | `{ [k: string]: boolean }` |  |  |
| skip | `boolean` |  |  |
| multihit | `boolean` |  |  |

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

### MuteEntry

**File:** `server/rooms.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| userid | `ID` |  |  |
| time | `number` |  |  |
| guestNum | `number` |  |  |
| autoconfirmed | `string` |  |  |

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

### Nomination

**File:** `server/chat-plugins/permalocks.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| by | `ID` |  |  |
| ips | `string[]` |  |  |
| info | `string` |  |  |
| date | `number` |  |  |
| standing | `string` |  |  |
| alts | `string[]` |  |  |
| primaryID | `ID` |  |  |
| claimed | `ID` | ✓ |  |
| post | `string` | ✓ |  |

---

### OldRandomBattleSpecies

**File:** `data/random-battles/gen8/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| level | `number` | ✓ |  |
| moves | `ID[]` | ✓ |  |
| doublesLevel | `number` | ✓ |  |
| doublesMoves | `ID[]` | ✓ |  |
| noDynamaxMoves | `ID[]` | ✓ |  |

---

### Operators

**File:** `server/chat-plugins/calculator.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| precedence | `number` |  |  |
| associativity | `"Left" | "Right"` |  |  |

---

### OtdData

**File:** `server/chat-plugins/thing-of-the-day.ts`

---

### OtdSettings

**File:** `server/chat-plugins/thing-of-the-day.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `string` | ✓ |  |
| updateOnNom | `boolean` | ✓ |  |
| keys | `string[]` |  |  |
| title | `string` |  |  |
| keyLabels | `string[]` |  |  |
| timeLabel | `string` |  |  |
| roomid | `RoomID` |  |  |

---

### PageTable

**File:** `server/chat.ts`

---

### PendingUpdate

**File:** `lib/fs.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| isWriting | `boolean` |  |  |
| pendingDataFetcher | `(()` |  |  |
| pendingOptions | `AnyObject | null` |  |  |
| throttleTime | `number` |  |  |
| throttleTimer | `NodeJS.Timeout | null` |  |  |

---

### PerspectiveRequest

**File:** `server/artemis/remote.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| languages | `string[]` |  |  |
| requestedAttributes | `AnyObject` |  |  |
| comment | `{ text: string }` |  |  |

---

### Player

**File:** `server/chat-plugins/auction.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `ID` |  |  |
| name | `string` |  |  |
| team | `Team` | ✓ |  |
| price | `number` |  |  |
| tiersPlayed | `string[]` |  |  |
| tiersNotPlayed | `string[]` |  |  |

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

### PluginData

**File:** `server/chat-plugins/responder.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| pairs | `{ [k: string]: string[] }` |  |  |
| ignore | `string[]` | ✓ |  |

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

### PokemonSets

**File:** `tools/set-import/importer.ts`

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

### PollAnswer

**File:** `server/chat-plugins/poll.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |

---

### PollData

**File:** `server/chat-plugins/poll.ts`

**Extends:** PollOptions

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| activityid | `'poll'` |  | ✓ |

---

### PollOptions

**File:** `server/chat-plugins/poll.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| activityNumber | `number` | ✓ |  |
| question | `string` |  |  |
| supportHTML | `boolean` |  |  |
| multiPoll | `boolean` |  |  |
| pendingVotes | `{ [userid: string]: number[] }` | ✓ |  |
| voters | `{ [k: string]: number[] }` | ✓ |  |
| voterIps | `{ [k: string]: number[] }` | ✓ |  |
| maxVotes | `number` | ✓ |  |
| totalVotes | `number` | ✓ |  |
| timeoutMins | `number` | ✓ |  |
| timerEnd | `number` | ✓ |  |
| isQuiz | `boolean` | ✓ |  |
| answers | `string[] | PollAnswer[]` |  |  |

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

### PostData

**File:** `lib/net.ts`

---

### ProcessData

**File:** `server/chat-commands/admin.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| cmd | `string` |  |  |
| cpu | `string` | ✓ |  |
| time | `string` | ✓ |  |
| ram | `string` | ✓ |  |

---

### ProcessWrapper

**File:** `lib/process-manager.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| getLoad | `()` |  |  |
| process | `ChildProcess | Worker` |  |  |
| release | `()` |  |  |
| getProcess | `()` |  |  |

---

### PullRequest

**File:** `server/chat-plugins/github.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| action | `string` |  |  |
| number | `number` |  |  |
| pull_request | `{` |  |  |
| url | `string,` |  |  |
| html_url | `string,` |  |  |
| title | `string,` |  |  |
| user | `{` |  |  |
| login | `string,` |  |  |
| html_url | `string,` |  |  |
| merge_commit_sha | `string,` |  |  |
| sender | `{ login: string }` |  |  |

---

### PunishInfo

**File:** `server/punishments.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| desc | `string` |  |  |
| onActivate | `(user: User, punishment: Punishment, room: Room | null, isExactMatch: boolean)` | ✓ |  |
| activatePunishMonitor | `boolean` | ✓ |  |

---

### Punishment

**File:** `server/punishments.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| type | `string` |  |  |
| id | `ID | PunishType` |  |  |
| expireTime | `number` |  |  |
| reason | `string` |  |  |
| rest | `any[]` | ✓ |  |

---

### PunishmentEntry

**File:** `server/punishments.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| ips | `string[]` |  |  |
| userids | `ID[]` |  |  |
| punishType | `string` |  |  |
| expireTime | `number` |  |  |
| reason | `string` |  |  |
| rest | `any[]` |  |  |

---

### PunishmentSettings

**File:** `server/chat-plugins/abuse-monitor.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| count | `number` | ✓ |  |
| certainty | `number` | ✓ |  |
| type | `string` | ✓ |  |
| punishment | `typeof PUNISHMENTS[number]` |  |  |
| modlogCount | `number` | ✓ |  |
| modlogActions | `string[]` | ✓ |  |
| secondaryTypes | `Record<string, number>` | ✓ |  |
| requiresPunishment | `boolean` | ✓ |  |

---

### Push

**File:** `server/chat-plugins/github.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| commits | `Commit[]` |  |  |
| sender | `{ login: string }` |  |  |
| compare | `string` |  |  |

---

### QueuedHunt

**File:** `server/chat-plugins/scavengers.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| hosts | `{ id: string, name: string, noUpdate?: boolean }[]` |  |  |
| questions | `(string | string[])[]` |  |  |
| isHTML | `boolean` |  |  |
| staffHostId | `string` |  |  |
| staffHostName | `string` |  |  |
| gameType | `GameTypes` |  |  |

---

### Quote

**File:** `server/chat-plugins/quotes.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| userid | `string` |  |  |
| quote | `string` |  |  |
| date | `number` |  |  |

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

### ReceivedPM

**File:** `server/private-messages/index.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| time | `number` |  |  |
| sender | `string` |  |  |
| receiver | `string` |  |  |
| seen | `number | null` |  |  |
| message | `string` |  |  |

---

### Recommendation

**File:** `server/chat-plugins/the-studio.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| artist | `string` |  |  |
| title | `string` |  |  |
| url | `string` |  |  |
| videoInfo | `VideoData | null` |  |  |
| description | `string` |  |  |
| tags | `string[]` |  |  |
| userData | `{` |  |  |
| name | `string,` |  |  |
| avatar | `string,` | ✓ |  |
| likes | `number` |  |  |
| liked | `{` | ✓ |  |
| ips | `string[],` |  |  |
| userids | `string[],` |  |  |

---

### Recommendations

**File:** `server/chat-plugins/the-studio.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| suggested | `Recommendation[]` |  |  |
| saved | `Recommendation[]` |  |  |
| youtubeSearchDisabled | `boolean` | ✓ |  |

---

### RepeatedPhrase

**File:** `server/chat-plugins/repeats.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `ID` |  |  |
| phrase | `string` |  |  |
| interval | `number` |  |  |
| faq | `boolean` | ✓ |  |
| isByMessages | `boolean` | ✓ |  |
| isHTML | `boolean` | ✓ |  |

---

### ResolvedTicketInfo

**File:** `server/chat-plugins/helptickets.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| time | `number` |  |  |
| result | `string` |  |  |
| by | `string` |  |  |
| seen | `boolean` |  |  |
| staffReason | `string` |  |  |
| note | `string` | ✓ |  |

---

### ResultRow

**File:** `lib/sql.ts`

---

### ReviewRequest

**File:** `server/chat-plugins/abuse-monitor.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| staff | `string` |  |  |
| room | `string` |  |  |
| details | `string` |  |  |
| time | `number` |  |  |
| resolved | `{ by: string, time: number, details: string, result: number }` | ✓ |  |

---

### RNG

**File:** `sim/prng.ts`

---

### RoomBattleOptions

**File:** `server/room-battle.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| format | `string` |  |  |
| players | `RoomBattlePlayerOptions[]` |  |  |
| delayedStart | `boolean | 'multi'` | ✓ |  |
| challengeType | `ChallengeType` | ✓ |  |
| allowRenames | `boolean` | ✓ |  |
| rated | `number | boolean | null` | ✓ |  |
| tour | `Tournament | null` | ✓ |  |
| inputLog | `string` | ✓ |  |
| ratedMessage | `string` | ✓ |  |
| seed | `PRNGSeed` | ✓ |  |
| roomid | `RoomID` | ✓ |  |
| delayedTimer | `boolean` | ✓ |  |
| isBestOfSubBattle | `boolean` | ✓ |  |

---

### RoomBattlePlayerOptions

**File:** `server/room-battle.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| user | `User` |  |  |
| team | `string` | ✓ |  |
| rating | `number` | ✓ |  |
| inviteOnly | `boolean` | ✓ |  |
| hidden | `boolean` | ✓ |  |

---

### RoomData

**File:** `server/chat-commands/core.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| p1 | `string` | ✓ |  |

---

### RoomEvent

**File:** `server/chat-plugins/room-events.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| eventName | `string` |  |  |
| date | `string` |  |  |
| desc | `string` |  |  |
| started | `boolean` |  |  |

---

### RoomEventAlias

**File:** `server/chat-plugins/room-events.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| eventID | `ID` |  |  |

---

### RoomEventCategory

**File:** `server/chat-plugins/room-events.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| events | `ID[]` |  |  |

---

### RoomFAQ

**File:** `server/chat-plugins/room-faqs.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| source | `string` |  |  |
| alias | `boolean` | ✓ |  |
| html | `boolean` | ✓ |  |

---

### RoomlogOptions

**File:** `server/roomlogs.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| isMultichannel | `boolean` | ✓ |  |
| noAutoTruncate | `boolean` | ✓ |  |
| noLogTimes | `boolean` | ✓ |  |

---

### RoomlogRow

**File:** `server/roomlogs.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| type | `string` |  |  |
| roomid | `string` |  |  |
| userid | `string | null` |  |  |
| time | `Date` |  |  |
| log | `string` |  |  |
| content | `string | null` |  |  |

---

### RoomSettings

**File:** `server/rooms.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| title | `string` |  |  |
| auth | `{ [userid: string]: GroupSymbol }` |  |  |
| creationTime | `number` |  |  |
| section | `RoomSection` | ✓ |  |
| autojoin | `boolean` | ✓ | ✓ |
| aliases | `string[]` | ✓ |  |
| banwords | `string[]` | ✓ |  |
| isPrivate | `PrivacySetting` | ✓ |  |
| modjoin | `AuthLevel | true | null` | ✓ |  |
| modchat | `AuthLevel | null` | ✓ |  |
| staffRoom | `boolean` | ✓ |  |
| language | `ID | false` | ✓ |  |
| slowchat | `number | false` | ✓ |  |
| events | `{ [k: string]: RoomEvent | RoomEventAlias | RoomEventCategory }` | ✓ |  |
| filterStretching | `boolean` | ✓ |  |
| filterEmojis | `boolean` | ✓ |  |
| filterCaps | `boolean` | ✓ |  |
| filterLinks | `boolean` | ✓ |  |
| jeopardyDisabled | `boolean` | ✓ |  |
| mafiaDisabled | `boolean` | ✓ |  |
| unoDisabled | `boolean` | ✓ |  |
| hangmanDisabled | `boolean` | ✓ |  |
| auctionDisabled | `boolean` | ✓ |  |
| gameNumber | `number` | ✓ |  |
| highTraffic | `boolean` | ✓ |  |
| spotlight | `string` | ✓ |  |
| parentid | `string | null` | ✓ |  |
| desc | `string | null` | ✓ |  |
| introMessage | `string | null` | ✓ |  |
| staffMessage | `string | null` | ✓ |  |
| rulesLink | `string | null` | ✓ |  |
| dataCommandTierDisplay | `'tiers' | 'doubles tiers' | 'National Dex tiers' | 'numbers'` | ✓ |  |
| requestShowEnabled | `boolean | null` | ✓ |  |
| permissions | `{ [k: string]: GroupSymbol }` | ✓ |  |
| minorActivity | `PollData | AnnouncementData` | ✓ |  |
| minorActivityQueue | `MinorActivityData[]` | ✓ |  |
| repeats | `RepeatedPhrase[]` | ✓ |  |
| topics | `string[]` | ✓ |  |
| autoStartOtd | `boolean` | ✓ |  |
| autoModchat | `{` | ✓ |  |
| rank | `GroupSymbol,` |  |  |
| time | `number,` |  |  |
| active | `boolean | AuthLevel,` |  |  |
| tournaments | `TournamentRoomSettings` | ✓ |  |
| defaultFormat | `string` | ✓ |  |
| scavSettings | `AnyObject` | ✓ |  |
| scavQueue | `QueuedHunt[]` | ✓ |  |
| isPersonal | `boolean` | ✓ |  |
| isHelp | `boolean` | ✓ |  |
| noLogTimes | `boolean` | ✓ |  |
| noAutoTruncate | `boolean` | ✓ |  |
| isMultichannel | `boolean` | ✓ |  |

---

### RoomStats

**File:** `server/chat-plugins/chatlog.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| lines | `{ [k: string]: number }` |  |  |
| users | `{ [k: string]: number }` |  |  |
| days | `number` |  |  |
| deadTime | `number` |  |  |
| deadPercent | `number` |  |  |
| linesPerUser | `number` |  |  |
| totalLines | `number` |  |  |
| averagePresent | `number` |  |  |

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

### SampleTeamsData

**File:** `server/chat-plugins/sample-teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| whitelist | `{ [formatid: string]: RoomID[] }` |  |  |
| teams | `{` |  |  |
| uncategorized | `{ [k: string]: string },` |  |  |

---

### SeasonData

**File:** `server/chat-plugins/seasons.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| current | `{ period: number, year: number, formatsGeneratedAt: number, season: number }` |  |  |
| badgeholders | `{ [period: string]: { [format: string]: { [badgeType: string]: string[] } } }` |  |  |
| formatSchedule | `Record<string, string[]>` |  |  |

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

### SetCriteria

**File:** `server/chat-plugins/randombattles/index.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| moves | `{ mustHave: Move[], mustNotHave: Move[] }` |  |  |
| ability | `{ mustHave?: Ability, mustNotHave: Ability[] }` |  |  |
| item | `{ mustHave?: Item, mustNotHave: Item[] }` |  |  |
| nature | `{ mustHave?: Nature, mustNotHave: Nature[] }` |  |  |
| teraType | `{ mustHave?: TypeInfo, mustNotHave: TypeInfo[] }` |  |  |

---

### ShowRequest

**File:** `server/rooms.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| link | `string` |  |  |
| comment | `string` |  |  |
| dimensions | `[number, number, boolean]` | ✓ |  |

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

### Spotlight

**File:** `server/chat-plugins/daily-spotlight.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| image | `StoredImage | string` | ✓ |  |
| description | `string` |  |  |
| time | `number` |  |  |

---

### SQLOptions

**File:** `lib/sql.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| file | `string` |  |  |
| extension | `string` | ✓ |  |
| sqliteOptions | `sqlite.Options` | ✓ |  |
| onError | `ErrorHandler` | ✓ |  |

---

### SQLQuery

**File:** `server/modlog/index.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| query | `string` |  |  |
| args | `(string | number)[]` |  |  |

---

### SSBSet

**File:** `data/mods/gen9ssb/random-teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| species | `string` |  |  |
| ability | `string | string[]` |  |  |
| item | `string | string[]` |  |  |
| gender | `GenderName | GenderName[]` |  |  |
| moves | `(string | string[])[]` |  |  |
| signatureMove | `string` |  |  |
| evs | `{ hp?: number, atk?: number, def?: number, spa?: number, spd?: number, spe?: number }` | ✓ |  |
| ivs | `{ hp?: number, atk?: number, def?: number, spa?: number, spd?: number, spe?: number }` | ✓ |  |
| nature | `string | string[]` | ✓ |  |
| shiny | `number | boolean` | ✓ |  |
| level | `number` | ✓ |  |
| happiness | `number` | ✓ |  |
| skip | `string` | ✓ |  |
| teraType | `string | string[]` | ✓ |  |

---

### SSBSets

**File:** `data/mods/gen9ssb/random-teams.ts`

---

### StoneDeltas

**File:** `server/chat-plugins/othermetas.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| baseStats | `{ [stat in StatID]: number }` |  |  |
| bst | `number` |  |  |
| weighthg | `number` |  |  |
| heightm | `number` |  |  |
| type | `string` | ✓ |  |
| primaryTypeChange | `boolean` | ✓ |  |

---

### StoredTeam

**File:** `server/chat-plugins/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| teamid | `string` |  |  |
| team | `string` |  |  |
| ownerid | `ID` |  |  |
| format | `ID` |  |  |
| title | `string | null` |  |  |
| date | `Date` |  |  |
| private | `string | null` |  |  |
| views | `number` |  |  |

---

### SuspectsFile

**File:** `server/chat-plugins/suspect-tests.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| whitelist | `string[]` |  |  |
| suspects | `{ [format: string]: SuspectTest }` |  |  |

---

### SuspectTest

**File:** `server/chat-plugins/suspect-tests.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| tier | `string` |  |  |
| suspect | `string` |  |  |
| date | `string` |  |  |
| url | `string` |  |  |

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

### TagData

**File:** `data/tags.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| desc | `string` | ✓ |  |
| speciesFilter | `(species: Species)` | ✓ |  |
| moveFilter | `(move: Move)` | ✓ |  |
| genericFilter | `(thing: Species | Move | Item | Ability)` | ✓ |  |
| speciesNumCol | `(species: Species)` | ✓ |  |
| moveNumCol | `(move: Move)` | ✓ |  |
| genericNumCol | `(thing: Species | Move | Item | Ability)` | ✓ |  |

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

### TeamData

**File:** `data/random-battles/gen9/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| typeCount | `{ [k: string]: number }` |  |  |
| typeComboCount | `{ [k: string]: number }` |  |  |
| baseFormes | `{ [k: string]: number }` |  |  |
| megaCount | `number` | ✓ |  |
| zCount | `number` | ✓ |  |
| wantsTeraCount | `number` | ✓ |  |
| has | `{ [k: string]: number }` |  |  |
| forceResult | `boolean` |  |  |
| weaknesses | `{ [k: string]: number }` |  |  |
| resistances | `{ [k: string]: number }` |  |  |
| weather | `string` | ✓ |  |
| eeveeLimCount | `number` | ✓ |  |
| gigantamax | `boolean` | ✓ |  |

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

### TeamSearch

**File:** `server/chat-plugins/teams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| format | `string` | ✓ |  |
| owner | `string` | ✓ |  |
| pokemon | `string[]` | ✓ |  |
| moves | `string[]` | ✓ |  |
| abilities | `string[]` | ✓ |  |
| gen | `number` | ✓ |  |

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

### TextTicketInfo

**File:** `server/chat-plugins/helptickets.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| checker | `(` | ✓ |  |
| input | `string, context: string, pageId: string, user: User, reportTarget?: string` |  |  |
| title | `string` |  |  |
| disclaimer | `string` | ✓ |  |
| contextMessage | `string` | ✓ |  |
| listOnly | `boolean` | ✓ |  |
| getReviewDisplay | `(` |  |  |
| ticket | `TicketState & { text: [string, string] }, staff: User, conn: Connection, state?: AnyObject` |  |  |
| onSubmit | `(ticket: TicketState, text: [string, string], submitter: User, conn: Connection)` | ✓ |  |
| getState | `(ticket: TicketState, user: User)` | ✓ |  |

---

### TicketSettings

**File:** `server/chat-plugins/helptickets.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| responses | `{ [ticketType: string]: { [title: string]: string } }` |  |  |

---

### TicketState

**File:** `server/chat-plugins/helptickets.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| creator | `string` |  |  |
| userid | `ID` |  |  |
| open | `boolean` |  |  |
| active | `boolean` |  |  |
| type | `string` |  |  |
| created | `number` |  |  |
| claimed | `string | null` |  |  |
| claimTime | `number` | ✓ |  |
| ip | `string` |  |  |
| needsDelayWarning | `boolean` | ✓ |  |
| offline | `boolean` | ✓ |  |
| text | `[string, string]` | ✓ |  |
| resolved | `ResolvedTicketInfo` | ✓ |  |
| meta | `string` | ✓ |  |
| notes | `{ [userid: string]: string }` | ✓ |  |
| state | `AnyObject & { claimTime?: number }` | ✓ |  |
| recommended | `string[]` | ✓ |  |

---

### TopPlayer

**File:** `server/chat-plugins/trivia/trivia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `string` |  |  |
| player | `TriviaPlayer` |  |  |
| name | `string` |  |  |

---

### TourEvent

**File:** `server/chat-plugins/smogtours.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| title | `string` |  |  |
| url | `string` |  |  |
| desc | `string` |  |  |
| image | `Image` | ✓ |  |
| artistCredit | `{ url: string, name: string }` | ✓ |  |
| id | `string` |  |  |
| shortDesc | `string` |  |  |
| date | `number` |  |  |
| ends | `number` | ✓ |  |

---

### TournamentRoomSettings

**File:** `server/tournaments/index.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| allowModjoin | `boolean` | ✓ |  |
| allowScouting | `boolean` | ✓ |  |
| announcements | `boolean` | ✓ |  |
| autoconfirmedOnly | `boolean` | ✓ |  |
| autodq | `number` | ✓ |  |
| autostart | `number | boolean` | ✓ |  |
| forcePublic | `boolean` | ✓ |  |
| forceTimer | `boolean` | ✓ |  |
| playerCap | `number` | ✓ |  |
| recentToursLength | `number` | ✓ |  |
| recentTours | `{ name: string, baseFormat: string, time: number }[]` | ✓ |  |
| blockRecents | `boolean` | ✓ |  |

---

### TourTable

**File:** `server/chat-plugins/smogtours.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| title | `string` |  |  |
| tours | `TourEvent[]` |  |  |
| whitelist | `string[]` | ✓ |  |
| icon | `Image` | ✓ |  |
| desc | `string` |  |  |

---

### TrackerConfig

**File:** `server/chat-plugins/laddertours.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| format | `string` |  |  |
| prefix | `string` |  |  |
| rating | `number` |  |  |
| deadline | `string` | ✓ |  |
| cutoff | `number` | ✓ |  |
| users | `ID[]` | ✓ |  |
| showdiffs | `boolean` | ✓ |  |

---

### TransactionEnvironment

**File:** `lib/sql.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| db | `sqlite.Database` |  |  |
| statements | `Map<string, sqlite.Statement>` |  |  |

---

### Translations

**File:** `server/chat.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` | ✓ |  |
| strings | `{ [english: string]: string }` |  |  |

---

### TriviaData

**File:** `server/chat-plugins/trivia/trivia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| questions | `{ [k: string]: TriviaQuestion[] }` | ✓ |  |
| submissions | `{ [k: string]: TriviaQuestion[] }` | ✓ |  |
| leaderboard | `TriviaLeaderboardData` | ✓ |  |
| altLeaderboard | `TriviaLeaderboardData` | ✓ |  |
| history | `TriviaHistory[]` | ✓ |  |
| moveEventQuestions | `boolean` | ✓ |  |

---

### TriviaDatabase

**File:** `server/chat-plugins/trivia/database.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| userid | `ID,` |  |  |
| additions | `Record<Leaderboard, TriviaLeaderboardScore>` |  |  |
| categories | `string[] | 'all',` |  |  |
| limit | `number,` |  |  |
| options | `{ order: 'newestfirst' | 'oldestfirst' | 'random' }` |  |  |
| id | `ID,` |  |  |
| leaderboard | `Leaderboard` |  |  |
| search | `string,` |  |  |
| options | `{ searchSubmissions: boolean, caseSensitive?: boolean }` |  |  |

---

### TriviaGame

**File:** `server/chat-plugins/trivia/trivia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| mode | `string` |  |  |
| length | `keyof typeof LENGTHS | number` |  |  |
| category | `string` |  |  |
| startTime | `number` |  |  |
| creator | `string` | ✓ |  |
| givesPoints | `boolean` | ✓ |  |

---

### TriviaLeaderboardData

**File:** `server/chat-plugins/trivia/trivia.ts`

---

### TriviaLeaderboardScore

**File:** `server/chat-plugins/trivia/trivia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| score | `number` |  |  |
| totalPoints | `number` |  |  |
| totalCorrectAnswers | `number` |  |  |

---

### TriviaQuestion

**File:** `server/chat-plugins/trivia/trivia.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| question | `string` |  |  |
| category | `string` |  |  |
| answers | `string[]` |  |  |
| user | `string` | ✓ |  |
| addedAt | `number` | ✓ |  |

---

### Twist

**File:** `server/chat-plugins/scavenger-games.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| name | `string` |  |  |
| id | `string` |  |  |
| isGameMode | `true` | ✓ |  |
| desc | `string` | ✓ |  |

---

### TwitchChannel

**File:** `server/chat-plugins/youtube.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| status | `string` |  |  |
| display_name | `string` |  |  |
| name | `string` |  |  |
| language | `string` |  |  |
| created_at | `string` |  |  |
| logo | `string` |  |  |
| views | `number` |  |  |
| followers | `number` |  |  |
| video_banner | `string` |  |  |
| url | `string` |  |  |
| game | `string` |  |  |
| description | `string` |  |  |
| updated_at | `string` |  |  |

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

### UserSettings

**File:** `server/users.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| blockChallenges | `boolean | AuthLevel | 'friends'` |  |  |
| blockPMs | `boolean | AuthLevel | 'friends'` |  |  |
| ignoreTickets | `boolean` |  |  |
| hideBattlesFromTrainerCard | `boolean` |  |  |
| blockInvites | `AuthLevel | boolean` |  |  |
| doNotDisturb | `boolean` |  |  |
| blockFriendRequests | `boolean` |  |  |
| allowFriendNotifications | `boolean` |  |  |
| displayBattlesToFriends | `boolean` |  |  |
| hideLogins | `boolean` |  |  |

---

### UserTable

**File:** `server/rooms.ts`

---

### VideoData

**File:** `server/chat-plugins/youtube.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| id | `string` |  |  |
| title | `string` |  |  |
| date | `string` |  |  |
| description | `string` |  |  |
| channelTitle | `string` |  |  |
| channelUrl | `string` |  |  |
| views | `number` |  |  |
| thumbnail | `string` |  |  |
| likes | `number` |  |  |
| dislikes | `number` |  |  |

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

### WriteStreamOptions

**File:** `lib/streams.ts`

**Fields:**

| Name | Type | Optional | Readonly |
|------|------|----------|----------|
| nodeStream | `NodeJS.WritableStream` | ✓ |  |
| write | `(this: WriteStream, data: string | Buffer)` | ✓ |  |
| writeEnd | `(this: WriteStream)` | ✓ |  |

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

### AnnotatedChatCommands

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').AnnotatedChatCommands
```

---

### AnnotatedChatHandler

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').AnnotatedChatHandler
```

---

### AuthLevel

**File:** `server/user-groups.ts`

**Definition:**
```typescript
EffectiveGroupSymbol | 'unlocked' | 'trusted' | 'autoconfirmed'
```

---

### BasicRoom

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./rooms').BasicRoom
```

---

### BasicSQLValue

**File:** `lib/database.ts`

**Definition:**
```typescript
string | number | null
```

---

### battle

**File:** `test/sim/moves/explosion.js`

**Definition:**
```typescript
common.gen(1).createBattle([[
			{ species: 'golem', moves: ['explosion'] },
		], [
			{ species: 'gastly', moves: ['rage'] },
		]])
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

### BattleChallenge

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./ladders-challenges').BattleChallenge
```

---

### BattleQueue

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./battle-queue').BattleQueue
```

---

### BestOfGame

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./room-battle-bestof').BestOfGame
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

### BufferEncoding

**File:** `lib/streams.ts`

**Definition:**
```typescript
'ascii' | 'utf8' | 'utf-8' | 'utf16le' | 'ucs2' | 'ucs-2' | 'base64' | 'latin1' | 'binary' | 'hex'
```

---

### Challenge

**File:** `server/ladders-challenges.ts`

**Definition:**
```typescript
BattleChallenge | GameChallenge
```

---

### ChallengeType

**File:** `server/room-battle.ts`

**Definition:**
```typescript
'rated' | 'unrated' | 'challenge' | 'tour'
```

---

### ChannelID

**File:** `sim/battle.ts`

**Definition:**
```typescript
0 | 1 | 2 | 3 | 4
```

---

### ChannelIndex

**File:** `server/room-battle.ts`

**Definition:**
```typescript
0 | 1 | 2 | 3 | 4
```

---

### ChatCommands

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').ChatCommands
```

---

### ChatFilter

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').ChatFilter
```

---

### ChatHandler

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').ChatHandler
```

---

### ChatQueueEntry

**File:** `server/users.ts`

**Definition:**
```typescript
[string, RoomID, Connection]
```

---

### ChatRoom

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./rooms').ChatRoom
```

---

### CheckerOutput

**File:** `server/chat-plugins/helptickets-auto.ts`

**Definition:**
```typescript
void | Map<string, CheckerResult>
```

---

### ChildProcess

**File:** `lib/process-manager.ts`

**Definition:**
```typescript
child_process.ChildProcess
```

---

### ChoiceRequest

**File:** `sim/side.ts`

**Definition:**
```typescript
SwitchRequest | TeamPreviewRequest | MoveRequest | WaitRequest
```

---

### Color

**File:** `server/chat-plugins/uno.ts`

**Definition:**
```typescript
'Green' | 'Yellow' | 'Red' | 'Blue' | 'Black'
```

---

### CommandContext

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').CommandContext
```

---

### Comparable

**File:** `lib/utils.ts`

**Definition:**
```typescript
number | string | boolean | Comparable[] | { reverse: Comparable }
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

### ConfigType

**File:** `server/config-loader.ts`

**Definition:**
```typescript
InputConfig & {
	groups: { [symbol: string]: GroupInfo },
	groupsranking: EffectiveGroupSymbol[],
	greatergroupscache: { [combo: string]: GroupSymbol },
	subprocessescache: SubProcessesConfig,
	[k: string]: any,
}
```

---

### Connection

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./users').Connection
```

---

### CRQHandler

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').CRQHandler
```

---

### DatabaseManager

**File:** `lib/sql.ts`

**Definition:**
```typescript
import('./sql').SQLDatabaseManager
```

---

### DatabaseQuery

**File:** `lib/sql.ts`

**Definition:**
```typescript
{
	/** Prepare a statement - data is the statement. */
	type: 'prepare', data: string,
} | {
	/** Get all lines from a statement. Data is the params. */
	type: 'all', data: DataType, statement: string, noPrepare?: boolean,
} | {
	/** Execute raw SQL in the database. */
	type: "exec", data: string,
} | {
	/** Get one line from a prepared statement. */
	type: 'get', data: DataType, statement: string, noPrepare?: boolean,
} | {
	/** Run a prepared statement. */
	type: 'run', data: DataType, statement: string, noPrepare?: boolean,
} | {
	type: 'transaction', name: string, data: DataType,
} | {
	type: 'start', options: SQLOptions,
} | {
	type: 'load-extension', data: string,
}
```

**Fields:**

| Name | Type | Optional |
|------|------|----------|
| type | `'prepare'` |  |
| data | `string` |  |
| type | `'all'` |  |
| data | `DataType` |  |
| statement | `string` |  |
| noPrepare | `boolean` | ✓ |
| type | `"exec"` |  |
| data | `string` |  |
| type | `'get'` |  |
| data | `DataType` |  |
| statement | `string` |  |
| noPrepare | `boolean` | ✓ |
| type | `'run'` |  |
| data | `DataType` |  |
| statement | `string` |  |
| noPrepare | `boolean` | ✓ |
| type | `'transaction'` |  |
| name | `string` |  |
| data | `DataType` |  |
| type | `'start'` |  |
| options | `SQLOptions` |  |
| type | `'load-extension'` |  |
| data | `string` |  |

---

### DataType

**File:** `sim/dex.ts`

**Definition:**
```typescript
'Abilities' | 'Rulesets' | 'FormatsData' | 'Items' | 'Learnsets' | 'Moves' |
	'Natures' | 'Pokedex' | 'Scripts' | 'Conditions' | 'TypeChart' | 'PokemonGoData'
```

---

### DefaultConfig

**File:** `server/config-loader.ts`

**Definition:**
```typescript
typeof defaults
```

---

### DefaultText

**File:** `sim/global-types.ts`

**Definition:**
```typescript
AnyObject
```

---

### Direction

**File:** `server/chat-plugins/datasearch.ts`

**Definition:**
```typescript
'less' | 'greater' | 'equal'
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

### EffectiveGroupSymbol

**File:** `server/user-groups.ts`

**Definition:**
```typescript
GroupSymbol | 'whitelist'
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

### ErrorCallback

**File:** `lib/static-server.ts`

**Definition:**
```typescript
(result: Result) => boolean | void
```

---

### ErrorHandler

**File:** `lib/sql.ts`

**Definition:**
```typescript
(error: Error, data: DatabaseQuery, isParentProcess: boolean) => any
```

---

### EvalType

**File:** `lib/repl.ts`

**Definition:**
```typescript
(script: string) => unknown
```

---

### Field

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./field').Field
```

---

### FilterWord

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').FilterWord
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

### FormatSpan

**File:** `server/chat-formatter.ts`

**Definition:**
```typescript
[SpanType, number]
```

---

### GameChallenge

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./ladders-challenges').GameChallenge
```

---

### GameModeFunction

**File:** `server/chat-plugins/scavenger-games.ts`

**Definition:**
```typescript
(this: ScavengerGameTemplate, ...args: any[]) => void
```

---

### GameRoom

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./rooms').GameRoom
```

---

### GameType

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'singles' | 'doubles' | 'triples' | 'rotation' | 'multi' | 'freeforall'
```

---

### GameTypes

**File:** `server/chat-plugins/scavengers.ts`

**Definition:**
```typescript
'official' | 'regular' | 'mini' | 'unrated' | 'practice' | 'recycled'
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

### Generation

**File:** `tools/set-import/sets/index.d.ts`

**Definition:**
```typescript
{num: GenerationNum}
```

**Fields:**

| Name | Type | Optional |
|------|------|----------|
| num | `GenerationNum` |  |

---

### GenerationNum

**File:** `tools/set-import/sets/index.d.ts`

**Definition:**
```typescript
1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

---

### Generator

**File:** `server/tournaments/index.ts`

**Definition:**
```typescript
RoundRobin | Elimination
```

---

### GlobalPermission

**File:** `server/user-groups.ts`

**Definition:**
```typescript
typeof GLOBAL_PERMISSIONS[number]
```

---

### GlobalRoomState

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./rooms').GlobalRoomState
```

---

### GroupInfo

**File:** `server/user-groups.ts`

**Definition:**
```typescript
{
	symbol: GroupSymbol,
	id: ID,
	name: string,
	rank: number,
	inherit?: GroupSymbol,
	jurisdiction?: string,

	globalonly?: boolean,
	roomonly?: boolean,
	battleonly?: boolean,
	root?: boolean,
	globalGroupInPersonalRoom?: GroupSymbol,
} & {
	[P in RoomPermission | GlobalPermission]?: string | boolean
```

**Fields:**

| Name | Type | Optional |
|------|------|----------|
| symbol | `GroupSymbol` |  |
| id | `ID` |  |
| name | `string` |  |
| rank | `number` |  |
| inherit | `GroupSymbol` | ✓ |
| jurisdiction | `string` | ✓ |
| globalonly | `boolean` | ✓ |
| roomonly | `boolean` | ✓ |
| battleonly | `boolean` | ✓ |
| root | `boolean` | ✓ |
| globalGroupInPersonalRoom | `GroupSymbol` | ✓ |

---

### GroupSymbol

**File:** `server/user-groups.ts`

**Definition:**
```typescript
'~' | '#' | '★' | '*' | '@' | '%' | '☆' | '§' | '+' | '^' | ' ' | '‽' | '!'
```

---

### GroupwatchData

**File:** `server/chat-plugins/youtube.ts`

**Definition:**
```typescript
VideoData & { groupwatchType: 'youtube' } | TwitchChannel & { groupwatchType: 'twitch' }
```

---

### Handlers

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').HandlerTable
```

---

### HandlerTable

**File:** `server/chat.ts`

**Definition:**
```typescript
{ [key in keyof Handlers]?: Handlers[key] }
```

---

### Headers

**File:** `lib/static-server.ts`

**Definition:**
```typescript
Record<string, string>
```

---

### HitEffect

**File:** `sim/dex.ts`

**Definition:**
```typescript
import('./dex-moves').HitEffect
```

---

### HostFilter

**File:** `server/chat.ts`

**Definition:**
```typescript
(host: string, user: User, connection: Connection, hostType: string) => void
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

### Image

**File:** `server/chat-plugins/smogtours.ts`

**Definition:**
```typescript
[string, number, number]
```

---

### InputConfig

**File:** `server/config-loader.ts`

**Definition:**
```typescript
Omit<DefaultConfig, 'subprocesses'> & {
	subprocesses: null | 0 | 1 | SubProcessesConfig,
}
```

---

### IntrinsicElements

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat-jsx').PSElements
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

### LadderCache

**File:** `server/ladders-local.ts`

**Definition:**
```typescript
Map<string, LadderRow[] | Promise<LadderRow[]>>
```

---

### LadderRow

**File:** `server/ladders-local.ts`

**Definition:**
```typescript
[string, number, string, number, number, number, string]
```

---

### Leaderboard

**File:** `server/chat-plugins/trivia/database.ts`

**Definition:**
```typescript
'alltime' | 'nonAlltime' | 'cycle'
```

---

### LogEntry

**File:** `server/monitor.ts`

**Definition:**
```typescript
[LogLevel, string]
```

---

### LoginFilter

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').LoginFilter
```

---

### LoginServerResponse

**File:** `server/loginserver.ts`

**Definition:**
```typescript
[AnyObject, null] | [null, Error]
```

---

### LogLevel

**File:** `server/monitor.ts`

**Definition:**
```typescript
'debug' | 'notice' | 'warning' | 'error'
```

---

### MafiaEliminateType

**File:** `server/chat-plugins/mafia.ts`

**Definition:**
```typescript
typeof MafiaEliminateType[keyof typeof MafiaEliminateType]
```

---

### MafiaLog

**File:** `server/chat-plugins/mafia.ts`

**Definition:**
```typescript
{ [section in MafiaLogSection]: MafiaLogTable }
```

---

### MafiaLogSection

**File:** `server/chat-plugins/mafia.ts`

**Definition:**
```typescript
'leaderboard' | 'mvps' | 'hosts' | 'plays' | 'leavers'
```

---

### MessageHandler

**File:** `server/rooms.ts`

**Definition:**
```typescript
(room: BasicRoom, message: string) => void
```

---

### MinorActivity

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./room-minor-activity').MinorActivity
```

---

### MinorActivityData

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./room-minor-activity').MinorActivityData
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

### ModlogFormat

**File:** `tools/modlog/converter.ts`

**Definition:**
```typescript
'txt' | 'sqlite'
```

---

### ModlogID

**File:** `server/modlog/index.ts`

**Definition:**
```typescript
RoomID | 'global' | 'all'
```

---

### MonitorHandler

**File:** `server/chat.ts`

**Definition:**
```typescript
(
	this: CommandContext,
	line: FilterWord,
	room: Room | null,
	user: User,
	message: string,
	lcMessage: string,
	isStaff: boolean
) => string | false | undefined
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

### MoveEnforcementChecker

**File:** `data/random-battles/gen9/teams.ts`

**Definition:**
```typescript
(
	movePool: string[], moves: Set<string>, abilities: string[], types: string[],
	counter: MoveCounter, species: Species, teamDetails: RandomTeamsTypes.TeamDetails,
	isLead: boolean, isDoubles: boolean, teraType: string, role: RandomTeamsTypes.Role,
) => boolean
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

### NameFilter

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').NameFilter
```

---

### Nature

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./dex-data').Nature
```

---

### NicknameFilter

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').NicknameFilter
```

---

### Nonstandard

**File:** `sim/global-types.ts`

**Definition:**
```typescript
'Past' | 'Future' | 'Unobtainable' | 'CAP' | 'LGPE' | 'Custom' | 'Gigantamax'
```

---

### Operator

**File:** `server/chat-plugins/calculator.ts`

**Definition:**
```typescript
'^' | 'negative' | '%' | '/' | '*' | '+' | '-' | '('
```

---

### Options

**File:** `lib/static-server.ts`

**Definition:**
```typescript
{
	/** Root directory to serve files from. */
	root?: string,
	/** Index file when serving a directory. */
	indexFile?: string,
	/** Default extension to append to files if not found. */
	defaultExtension?: string,
	/** Cache time in seconds. null = no cache header. undefined = default (3600). 0 = no cache. */
	cacheTime?: number | null,
	/** Serve `.gz` files if available. */
	gzip?: boolean | RegExp,
	/** Custom headers for success responses (not sent on errors). */
	headers?: Headers,
	/** Server header. `null` to disable. */
	serverInfo?: string | null,
}
```

**Fields:**

| Name | Type | Optional |
|------|------|----------|
| root | `string` | ✓ |
| indexFile | `string` | ✓ |
| defaultExtension | `string` | ✓ |
| cacheTime | `number | null` | ✓ |
| gzip | `boolean | RegExp` | ✓ |
| headers | `Headers` | ✓ |
| serverInfo | `string | null` | ✓ |

---

### Other

**File:** `sim/global-types.ts`

**Definition:**
```typescript
"Unreleased" | "Illegal" | "CAP" | "CAP NFE" | "CAP LC"
```

---

### PageContext

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').PageContext
```

---

### PageHandler

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').PageHandler
```

---

### PageTable

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').PageTable
```

---

### Part

**File:** `sim/battle.ts`

**Definition:**
```typescript
string | number | boolean | Pokemon | Side | Effect | Move | null | undefined
```

---

### PartialModlogEntry

**File:** `server/modlog/index.ts`

**Definition:**
```typescript
Partial<ModlogEntry> & { action: string }
```

---

### PlayerIndex

**File:** `server/room-battle.ts`

**Definition:**
```typescript
1 | 2 | 3 | 4
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

### PrivacySetting

**File:** `server/rooms.ts`

**Definition:**
```typescript
boolean | 'hidden' | 'voice' | 'unlisted'
```

---

### PRNGSeed

**File:** `sim/prng.ts`

**Definition:**
```typescript
`${'sodium' | 'gen5' | number},${string}`
```

---

### ProcessType

**File:** `server/config-loader.ts`

**Definition:**
```typescript
(
	'localartemis' | 'remoteartemis' | 'battlesearch' | 'datasearch' | 'friends' |
	'chatdb' | 'pm' | 'modlog' | 'network' | 'simulator' | 'validator' | 'verifier'
)
```

---

### PunishmentFilter

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').PunishmentFilter
```

---

### PunishmentHandler

**File:** `server/chat-plugins/abuse-monitor.ts`

**Definition:**
```typescript
(
	user: User, room: GameRoom, response: Record<string, number>, message: string,
) => void | boolean | Promise<void | boolean>
```

---

### PunishType

**File:** `server/global-types.ts`

**Definition:**
```typescript
'#hostfilter' | '#dnsbl' | '#ipban'
```

---

### Query

**File:** `lib/sql.ts`

**Definition:**
```typescript
import('./sql').DatabaseQuery
```

---

### Referable

**File:** `sim/state.ts`

**Definition:**
```typescript
Battle | Field | Side | Pokemon | Condition | Ability | Item | Move | Species
```

---

### Replay

**File:** `server/replays.ts`

**Definition:**
```typescript
Omit<ReplayRow, 'formatid' | 'players' | 'password' | 'views'> & {
	players: string[],
	views?: number,
	password?: string | null,
}
```

---

### ReplayRow

**File:** `server/replays.ts`

**Definition:**
```typescript
{
	id: string,
	format: string,
	/** player names delimited by `,`
```

**Fields:**

| Name | Type | Optional |
|------|------|----------|
| id | `string` |  |
| format | `string` |  |

---

### RequestState

**File:** `sim/battle.ts`

**Definition:**
```typescript
'teampreview' | 'move' | 'switch' | ''
```

---

### Result

**File:** `lib/static-server.ts`

**Definition:**
```typescript
{
	status: number,
	headers: Record<string, string>,
	message: string | undefined,
	/** Have we already responded? */
	alreadySent: boolean,
}
```

**Fields:**

| Name | Type | Optional |
|------|------|----------|
| status | `number` |  |
| headers | `Record<string` |  |
| message | `string | undefined` |  |
| alreadySent | `boolean` |  |

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

### Room

**File:** `server/rooms.ts`

**Definition:**
```typescript
GameRoom | ChatRoom
```

---

### RoomBattle

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./room-battle').RoomBattle
```

---

### RoomCloseHandler

**File:** `server/chat.ts`

**Definition:**
```typescript
(id: string, user: User, connection: Connection, page: boolean) => any
```

---

### RoomGame

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./room-game').RoomGame
```

---

### RoomGamePlayer

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./room-game').RoomGamePlayer
```

---

### RoomID

**File:** `server/global-types.ts`

**Definition:**
```typescript
"" | "lobby" | "staff" | "upperstaff" | "development" | Lowercase<string> & { __isRoomID: true }
```

---

### Roomlog

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./roomlogs').Roomlog
```

---

### RoomPermission

**File:** `server/user-groups.ts`

**Definition:**
```typescript
typeof ROOM_PERMISSIONS[number]
```

---

### RoomSection

**File:** `server/chat-commands/room-settings.ts`

**Definition:**
```typescript
typeof sections[number]
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

### SettingsHandler

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').SettingsHandler
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

### SimpleRoomGame

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./room-game').SimpleRoomGame
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

### SpanType

**File:** `server/chat-formatter.ts`

**Definition:**
```typescript
'_' | '*' | '~' | '^' | '\\' | '|' | '<' | '[' | '`' | 'a' | 'u' | 'spoiler' | '>' | '('
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

### SQLInput

**File:** `lib/sql.ts`

**Definition:**
```typescript
string | number | null
```

---

### SQLRow

**File:** `lib/database.ts`

**Definition:**
```typescript
{ [k: string]: BasicSQLValue }
```

**Fields:**

| Name | Type | Optional |
|------|------|----------|
| k | `string]: BasicSQLValue` |  |

---

### SQLValue

**File:** `lib/database.ts`

**Definition:**
```typescript
BasicSQLValue | SQLStatement | SQLStatement[] | PartialOrSQL<SQLRow> | BasicSQLValue[] | undefined
```

---

### Statement

**File:** `server/private-messages/database.ts`

**Definition:**
```typescript
keyof typeof statements
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

### StatName

**File:** `tools/set-import/sets/index.d.ts`

**Definition:**
```typescript
'hp' | 'atk' | 'def' | 'spa' | 'spd' | 'spe'
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

### StatusFilter

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./chat').StatusFilter
```

---

### StatusType

**File:** `server/users.ts`

**Definition:**
```typescript
'online' | 'busy' | 'idle'
```

---

### StoredImage

**File:** `server/chat-plugins/daily-spotlight.ts`

**Definition:**
```typescript
[string, number, number]
```

---

### StreamWorker

**File:** `server/sockets.ts`

**Definition:**
```typescript
ProcessManager.StreamWorker
```

---

### SubProcessesConfig

**File:** `server/config-loader.ts`

**Definition:**
```typescript
Partial<Record<ProcessType, number>>
```

---

### TeamValidator

**File:** `sim/global-types.ts`

**Definition:**
```typescript
import('./team-validator').TeamValidator
```

---

### TicketResult

**File:** `server/chat-plugins/helptickets.ts`

**Definition:**
```typescript
'approved' | 'valid' | 'assisted' | 'denied' | 'invalid' | 'unassisted' | 'ticketban' | 'deleted'
```

---

### TierShiftTiers

**File:** `server/chat-plugins/othermetas.ts`

**Definition:**
```typescript
'UU' | 'RUBL' | 'RU' | 'NUBL' | 'NU' | 'PUBL' | 'PU' | 'ZUBL' | 'ZU' | 'NFE' | 'LC'
```

---

### TransactionEnvironment

**File:** `lib/sql.ts`

**Definition:**
```typescript
import('./sql').TransactionEnvironment
```

---

### TriviaHistory

**File:** `server/chat-plugins/trivia/trivia.ts`

**Definition:**
```typescript
TriviaGame & { scores: { [k: string]: number } }
```

---

### TriviaLadder

**File:** `server/chat-plugins/trivia/trivia.ts`

**Definition:**
```typescript
ID[][]
```

---

### TriviaLeaderboards

**File:** `server/chat-plugins/trivia/database.ts`

**Definition:**
```typescript
Record<Leaderboard, TriviaLeaderboardData>
```

---

### TwistEvent

**File:** `server/chat-plugins/scavenger-games.ts`

**Definition:**
```typescript
(this: ScavengerHunt, ...args: any[]) => void
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

### typing

**File:** `server/chat-commands/info.ts`

**Definition:**
```typescript
type1
```

---

### User

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('./users').User
```

---

### VNode

**File:** `server/global-types.ts`

**Definition:**
```typescript
import('preact').VNode
```

---

### Worker

**File:** `lib/process-manager.ts`

**Definition:**
```typescript
cluster.Worker
```

---

### ZMoveOptions

**File:** `sim/global-types.ts`

**Definition:**
```typescript
({ move: string, target: MoveTarget } | null)[]
```

---

