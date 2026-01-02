# Type Coverage Gaps - JavaScript to Rust

This document tracks the gaps between JavaScript and Rust type definitions to achieve 1:1 equivalence.

**Generated**: 2026-01-02
**Analysis Scripts**: `analyze-type-coverage.js`, `compare-fields.js`

## Summary Statistics

- **Total JavaScript types**: 217 (from sim/ folder)
- **Found in Rust**: 76 (35%)
- **Missing in Rust**: 141 (65%)

## Critical Struct Field Gaps

### Battle Struct
**File**: `src/battle.rs`
**JavaScript**: 153 fields
**Rust**: 48 fields
**Missing**: 99 fields (64%)

**Missing Fields** (Top 20):
1. `id: ID` (readonly)
2. `debugMode: boolean` (readonly)
3. `forceRandomChance: boolean | null` (readonly)
4. `strictChoices: boolean` (readonly)
5. `format: Format` (readonly)
6. `formatData: EffectState` (readonly)
7. `reportExactHP: boolean`
8. `reportPercentages: boolean`
9. `supportCancel: boolean`
10. `actions: BattleActions`
11. `faintQueue: { target: Pokemon, source: Pokemon | null, effect: Effect | null }[]` (readonly)
12. `inputLog: string[]` (readonly)
13. `messageLog: string[]` (readonly)
14. `sentLogPos: number`
15. `sentEnd: boolean`
16. `requestState: RequestState`
17. `activeMove: ActiveMove | null`
18. `activePokemon: Pokemon | null`
19. `activeTarget: Pokemon | null`
20. `lastMove: ActiveMove | null`

**Complete Missing List**: See `compare-fields.js` output for all 99 fields

### Pokemon Struct
**File**: `src/pokemon.rs`
**JavaScript**: 167 fields
**Rust**: 99 fields
**Missing**: 68 fields (40%)

**Missing Fields** (Top 20):
1. `side: Side` (readonly)
2. `battle: Battle` (readonly)
3. `set: PokemonSet` (readonly)
4. `fullname: string` (readonly)
5. `bondTriggered: boolean`
6. `heroMessageDisplayed: boolean`
7. `swordBoost: boolean`
8. `shieldBoost: boolean`
9. `syrupTriggered: boolean`
10. `stellarBoostedTypes: string[]`
11. `isStarted: boolean`
12. `duringMove: boolean`
13. `canMegaEvo: string | false | null | undefined`
14. `canMegaEvoX: string | false | null | undefined`
15. `canMegaEvoY: string | false | null | undefined`
16. `canUltraBurst: string | null | undefined`
17. `canGigantamax: string | null` (readonly)
18. `canTerastallize: string | false | null`
19. `staleness: 'internal' | 'external'` (optional)
20. `pendingStaleness: 'internal' | 'external'` (optional)

**Complete Missing List**: See `compare-fields.js` output for all 68 fields

### Side Struct
**File**: `src/side.rs`
**JavaScript**: Fields need detailed comparison
**Rust**: Fields need detailed comparison

### Field Struct
**File**: `src/field.rs`
**JavaScript**: Fields need detailed comparison
**Rust**: Fields need detailed comparison

## Critical Missing Request/Response Types

These types are **completely missing** in Rust but critical for player interaction:

### PokemonSwitchRequestData
**File**: `sim/side.ts`
**Status**: ❌ **NOT FOUND** in Rust
**Fields**: 14

1. `ident: string`
2. `details: string`
3. `condition: string`
4. `active: boolean`
5. `stats: StatsExceptHPTable`
6. `moves: ID[]`
7. `baseAbility: ID`
8. `item: ID`
9. `pokeball: ID`
10. `ability: ID` (optional)
11. `commanding: boolean` (optional)
12. `reviving: boolean` (optional)
13. `teraType: string` (optional)
14. `terastallized: string` (optional)

### PokemonMoveRequestData
**File**: `sim/side.ts`
**Status**: ❌ **NOT FOUND** in Rust
**Fields**: 13

1. `moves: { move: string, id: ID, target?: string, disabled?: string | boolean, disabledSource?: string }[]`
2. `maybeDisabled: boolean` (optional)
3. `maybeLocked: boolean` (optional)
4. `trapped: boolean` (optional)
5. `maybeTrapped: boolean` (optional)
6. `canMegaEvo: boolean` (optional)
7. `canMegaEvoX: boolean` (optional)
8. `canMegaEvoY: boolean` (optional)
9. `canUltraBurst: boolean` (optional)
10. `canZMove: ZMoveOptions | null` (optional)
11. `canDynamax: boolean` (optional)
12. `maxMoves: DynamaxOptions` (optional)
13. `canTerastallize: string` (optional)

### SideRequestData
**File**: `sim/side.ts`
**Status**: ❌ **NOT FOUND** in Rust
**Fields**: 4

1. `name: string`
2. `id: SideID`
3. `pokemon: PokemonSwitchRequestData[]`
4. `noCancel: boolean` (optional)

**Note**: Rust has `SideRequest` in `choice.rs` but it appears incomplete (only 3 fields: name, id, pokemon)

## Missing Types by Category

### Core Battle Types (14 missing)
Types directly related to battle simulation and player interaction.

**Priority: HIGH**

1. PokemonSwitchRequestData (14 fields) - **CRITICAL**
2. PokemonMoveRequestData (13 fields) - **CRITICAL**
3. SideRequestData (4 fields) - **CRITICAL**
4. MoveRequest (move/switch choice requests)
5. SwitchRequest
6. TeamPreviewRequest
7. WaitRequest
8. ChoiceRequest (union of above)
9. DynamaxOptions
10. ZMoveOptions
11. EventListener
12. EventListenerWithoutPriority
13. EventInfo (partially implemented)
14. Attacker

### Data Structure Types (52 missing)
Types for Pokemon data, moves, abilities, items, etc.

**Priority: MEDIUM**

Missing:
- Species-related: SpeciesAbility, SpeciesFormatsData, SpeciesDataTable, etc.
- Move-related: MoveDataTable, MoveEventMethods, MoveFlags, MoveHitData
- Ability-related: AbilityDataTable, AbilityEventMethods, AbilityFlags
- Item-related: ItemDataTable, FlingData
- Dex-related: DexTableData, TextTableData, AliasesTable
- Many more (see JAVASCRIPT_TYPES.md)

### Utility Types (62 missing)
Helper types, streams, validators, etc.

**Priority: LOW**

Missing:
- BattleStreams: BattleTextStream, BattlePlayer, RawBattleStream
- Runners: Runner, ExhaustiveRunner, RandomPlayerAI, CoordinatedPlayerAI
- Validators: TeamValidator, PokemonSources
- Testing: DualStream, Pool, Pools
- Many more (see JAVASCRIPT_TYPES.md)

### Testing/Tools Types (13 missing)
Types used for testing and development tools.

**Priority: VERY LOW**

Missing:
- ExhaustiveRunnerOptions
- AIOptions
- RunnerOptions
- FactoryTeamDetails
- TeamDetails
- RandomFactorySet
- RandomDraftFactorySet
- RandomSet
- RandomSetData
- RandomSpeciesData
- Pools
- Pool
- TeamGenerator

## Implementation Priority

### Phase 1: Critical Request/Response Types (IMMEDIATE)
**Goal**: Enable proper player interaction and request handling

1. ✅ Define PokemonSwitchRequestData struct
2. ✅ Define PokemonMoveRequestData struct
3. ✅ Define SideRequestData struct
4. ✅ Define DynamaxOptions struct
5. ✅ Define ZMoveOptions type
6. ✅ Update ChoiceRequest to use proper request types
7. ✅ Add missing fields to existing request types

**Estimated Impact**: Enables proper player choice requests and battle requests

### Phase 2: Battle Struct Fields (HIGH PRIORITY)
**Goal**: Add missing 99 fields to achieve JavaScript equivalence

Focus on:
- Request state fields (`requestState`, `sentRequests`)
- Active move tracking (`activeMove`, `activePokemon`, `activeTarget`)
- Logging fields (`inputLog`, `messageLog`, `sentLogPos`, `sentEnd`)
- Format data fields (`format`, `formatData`, `id`, `debugMode`)
- Battle actions (`actions`)
- Faint queue (already exists, verify completeness)

**Estimated Impact**: Proper battle state tracking and debugging

### Phase 3: Pokemon Struct Fields (HIGH PRIORITY)
**Goal**: Add missing 68 fields to achieve JavaScript equivalence

Focus on:
- Core references (`side`, `battle`, `set`, `fullname`)
- Mega/Z/Dynamax/Tera flags (`canMegaEvo`, `canMegaEvoX`, `canMegaEvoY`, `canUltraBurst`, `canGigantamax`, `canTerastallize`)
- State tracking (`bondTriggered`, `heroMessageDisplayed`, `swordBoost`, `shieldBoost`, `syrupTriggered`)
- Advanced features (`stellarBoostedTypes`, `staleness`, `pendingStaleness`, `volatileStaleness`)

**Estimated Impact**: Proper Pokemon state management and mechanics

### Phase 4: Side and Field Structs (MEDIUM PRIORITY)
**Goal**: Add missing fields to achieve JavaScript equivalence

Need to run detailed field comparison first to identify exact gaps.

### Phase 5: Data Structure Types (MEDIUM PRIORITY)
**Goal**: Implement missing Dex-related types

Focus on:
- Species data structures
- Move data structures
- Ability data structures
- Item data structures

### Phase 6: Utility and Testing Types (LOW PRIORITY)
**Goal**: Implement remaining types for completeness

These are less critical for core 1:1 equivalence but needed for full feature parity.

## Implementation Notes

### Two-Phase Borrow Pattern Required
Many new fields will require mutable access patterns. Use the established two-phase borrow pattern:

```rust
// Phase 1: Extract data immutably
let (data1, data2) = {
    let pokemon = battle.pokemon_at(side, slot)?;
    (pokemon.field1, pokemon.field2)
};

// Phase 2: Mutable operations
let pokemon_mut = battle.pokemon_at_mut(side, slot)?;
pokemon_mut.modify();
```

### Lifetime Considerations
Some JavaScript fields like `battle`, `side`, `set` hold references to parent objects. In Rust, these need to be represented as indices or IDs rather than direct references to avoid lifetime issues:

```rust
// JavaScript: battle: Battle (reference)
// Rust: battle_id: usize (index into battle array)
```

### Serialization
All new structs must derive `Serialize` and `Deserialize` for battle state persistence.

### Documentation
All new types must include:
- `/// JavaScript equivalent:` comment with file location
- Field count comment (e.g., `/// 14 fields in JavaScript`)
- Individual field documentation

## Current Status

- ✅ Type extraction script created (`extract-types.js`)
- ✅ Type cross-referencing script created (`map-types.js`)
- ✅ Coverage analysis script created (`analyze-type-coverage.js`)
- ✅ Field comparison script created (`compare-fields.js`)
- ✅ JAVASCRIPT_TYPES.md reference document created
- ✅ Rust files annotated with JavaScript equivalence comments
- ⏳ **NEXT**: Implement Phase 1 - Critical Request/Response Types
