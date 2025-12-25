# Infrastructure Needs for Remaining Abilities

This document tracks the handler types and systems needed to implement the remaining ~111 abilities (out of 314 total).

## Current Status: 203/314 (64.6%) Complete

## Missing Handler Types

### High Priority (affects 10+ abilities each)

1. **onResidual** - End-of-turn effects
   - Affected abilities: harvest, healer, pickup, hydration, icebody (partial), solarpower (partial), ~15 total
   - Signature: `fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult`
   - Needs: Called at end of turn for each active Pokemon

2. **onModifyMove with mutable MoveDef** - Modify move properties
   - Affected abilities: skilllink, stench, myceliummight (partial), ~20 total
   - Current: `fn on_modify_move(battle: &mut Battle, move_: &MoveDef, pokemon: &Pokemon)`
   - Needs: `fn on_modify_move(battle: &mut Battle, move_: &mut MoveDef, pokemon: &Pokemon)`
   - Requires: Mutable reference to allow setting move.secondaries, move.ignoreAbility, etc.

3. **onAllySetStatus** - Prevent/modify ally status
   - Affected abilities: flowerveil, sweetveil, pastelveil, ~5 total
   - Signature: `fn on_ally_set_status(battle: &mut Battle, status: &Status, target: &Pokemon, source: Option<&Pokemon>, ability_holder: &Pokemon) -> AbilityHandlerResult`

4. **onAllyTryBoost** - Prevent/modify ally stat changes
   - Affected abilities: flowerveil, ~3 total
   - Signature: `fn on_ally_try_boost(battle: &mut Battle, boosts: &mut HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, ability_holder: &Pokemon) -> AbilityHandlerResult`

5. **onAllyTryAddVolatile** - Prevent ally volatile status
   - Affected abilities: sweetveil, ~2 total
   - Signature: `fn on_ally_try_add_volatile(battle: &mut Battle, status_id: &str, target: &Pokemon, ability_holder: &Pokemon) -> AbilityHandlerResult`

6. **onAllyFaint** - Triggered when ally faints
   - Affected abilities: receiver, powerofalchemy, ~3 total
   - Signature: `fn on_ally_faint(battle: &mut Battle, fainted: &Pokemon, ability_holder: &Pokemon) -> AbilityHandlerResult`

### Medium Priority (affects 5-10 abilities each)

7. **onDisableMove** - Choice item/ability locking
   - Affected abilities: gorillatactics, ~5 total
   - Signature: `fn on_disable_move(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult`
   - Needs: Access to pokemon.last_move or choice-locked move

8. **onAnyRedirectTarget** - Redirect moves to this Pokemon
   - Affected abilities: lightningrod, stormdrain, ~2 total
   - Signature: `fn on_any_redirect_target(battle: &mut Battle, target: &Pokemon, source: &Pokemon, move_: &MoveDef, ability_holder: &Pokemon) -> AbilityHandlerResult`

9. **onFoeTryEatItem** - Prevent foe from eating items
   - Affected abilities: unnerve, ~1 total
   - Signature: `fn on_foe_try_eat_item(battle: &mut Battle, item: &str, eater: &Pokemon, ability_holder: &Pokemon) -> AbilityHandlerResult`

10. **onPrepareHit** - Before move execution
    - Affected abilities: libero, protean, parentalbond, ~3 total
    - Signature: `fn on_prepare_hit(battle: &mut Battle, source: &Pokemon, target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult`

### Low Priority (affects <5 abilities each)

11. **onAfterMoveSecondary** - After move secondary effects
    - Affected abilities: pickpocket, magician, ~2 total

12. **onSourceModifySecondaries** - Modify move secondaries
    - Affected abilities: parentalbond, ~1 total

13. **onTakeItem** - When item is taken/lost
    - Affected abilities: stickyhold, ~1 total

14. **onCheckShow** - Visual ability display
    - Affected abilities: naturalcure, ~1 total

15. **onSwitchOut** - When switching out
    - Affected abilities: naturalcure, ~1 total

16. **onTryHitSide** / **onAllyTryHitSide** - Side-wide move blocking
    - Affected abilities: magicbounce, rebound, sapsipper, soundproof, ~4 total

17. **onFoeTrapPokemon** / **onFoeMaybeTrapPokemon** - Trapping mechanics
    - Affected abilities: magnetpull, shadowtag, ~2 total

18. **onDragOut** - Already exists but not wired for some abilities
    - Affected abilities: guarddog (partial), ~1 total

## Complex Systems Needed

### Forme Change System
Required for ~15 abilities including:
- stancechange, flowergift, forecast, iceface, disguise, schooling, shieldsdown
- hungerswitch, mimicry, powerconstruct, battlebond (partial)
- zerotohero, terashift

Needs:
- `pokemon.forme_change(new_forme: &str)` method
- Species forme tracking and stats
- Ability state preservation during forme change

### Transform System
Required for ~2 abilities:
- imposter, illusion (partial)

Needs:
- Full Pokemon state copying
- Move, stats, appearance copying
- Transform volatile status

### Weather/Terrain Change Handlers
Partially exist but need completion:
- onWeatherChange (flowergift, forecast, ~5 abilities)
- onTerrainChange (mimicry, quarkdrive, protosynthesis)

### Terastallization System
Required for ~3 abilities:
- teraformzero, terashell, terashift

Needs:
- onAfterTerastallization handler
- Tera type tracking
- Tera state management

### Event System Extensions
The event system exists (`single_event`, `run_event`) but needs:
- More event types documented
- Handler registration for new event types
- Event priority ordering

## Implementation Strategy

### Phase 1: Core Handlers (Immediate - 40+ abilities)
1. Implement onResidual handler (~15 abilities)
2. Make MoveDef mutable in onModifyMove (~20 abilities)
3. Implement ally protection handlers (onAllySetStatus, onAllyTryBoost, etc.) (~10 abilities)

### Phase 2: Move Mechanics (Medium term - 20+ abilities)
1. Implement onDisableMove for choice locking (~5 abilities)
2. Implement onAnyRedirectTarget for move redirection (~2 abilities)
3. Implement onPrepareHit for pre-move effects (~3 abilities)
4. Implement onAfterMoveSecondary and related (~5 abilities)

### Phase 3: Complex Systems (Long term - 30+ abilities)
1. Implement forme change system (~15 abilities)
2. Implement transform system (~2 abilities)
3. Complete weather/terrain systems (~10 abilities)
4. Implement terastallization (~3 abilities)

### Phase 4: Edge Cases (Final - 20+ abilities)
1. Implement item interaction handlers
2. Implement trapping mechanics
3. Complete all partial implementations
4. Add missing handler types

## Testing Needs

Each new handler type should have:
1. Unit tests for the handler mechanism
2. Integration tests with at least 2 abilities using it
3. Documentation of handler parameters and return values
4. Examples in existing ability implementations

## Notes

- Many abilities are already 80-90% complete, just missing one handler
- Some abilities have multiple handlers, only some of which are implemented
- Priority should be given to handlers that unlock the most abilities
- The battle engine's event system is robust and can support new handlers with minimal changes
- Most handlers follow consistent patterns that can be copied from existing implementations
