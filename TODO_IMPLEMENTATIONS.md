# TODO Implementations - JavaScript 1:1 Feature Parity

## Overview
Found 79 TODOs across battle*.rs files indicating incomplete features.
These need to be implemented to achieve true 1:1 JavaScript compatibility.

## Critical Missing Features (Core Battle Mechanics)

### Z-Moves & Max Moves
- [ ] battle.rs:5727 - Z-Move transformation in get_action_speed
- [ ] battle.rs:5731 - Max Move transformation in get_action_speed  
- [ ] battle_actions.rs:2762 - Z-move transformation in use_move
- [ ] battle_actions.rs:2779 - Max move transformation in use_move
- [ ] battle_actions.rs:2880 - runZPower for status Z-moves

### Move Execution
- [ ] battle.rs:5735 - Get move priority from Dex
- [ ] battle.rs:5747 - runEvent('ModifyPriority')
- [ ] battle.rs:5755 - Set move.priority field
- [ ] battle_actions.rs:2799 - ModifyTarget event
- [ ] battle_actions.rs:2805 - getRandomTarget implementation
- [ ] battle_actions.rs:2821 - Set move source effect
- [ ] battle_actions.rs:2899 - getMoveTargets for multi-target handling
- [ ] battle_actions.rs:2918 - PP deduction with Pressure

### Pokemon & Side Management  
- [ ] battle.rs:3492 - adjacentAllies() Pokemon helper
- [ ] battle.rs:3516 - adjacentFoes() Pokemon helper
- [ ] battle.rs:5989 - side.clearChoice() implementation
- [ ] battle.rs:5996 - side.activeRequest field
- [ ] battle.rs:6000 - ruleTable.pickedTeamSize
- [ ] battle.rs:6023 - isChoiceDone() check

### Events & Callbacks
- [ ] battle.rs:803 - Format callbacks (onBegin)
- [ ] battle.rs:806 - ruleTable iteration and subformat callbacks
- [ ] battle.rs:2695 - runEvent('BeforeFaint')
- [ ] battle.rs:2717 - runEvent('Faint')
- [ ] battle.rs:2793 - runEvent('AfterFaint')
- [ ] battle.rs:4195 - ChangeBoost event
- [ ] battle.rs:4202 - TryBoost event
- [ ] battle.rs:5096 - runEvent('DisableMove')
- [ ] battle.rs:5115 - TrapPokemon and MaybeTrapPokemon events
- [ ] battle.rs:5119 - Foe ability trapping check (Gen 3+)
- [ ] battle.rs:5905 - Swap events

### Gen-Specific Features
- [ ] battle.rs:756-757 - Multi battle side conditions sharing
- [ ] battle.rs:2784 - Gen 2-3 queue cancellation
- [ ] battle.rs:6160 - Gen 1 no-progress checks
- [ ] battle.rs:6161 - Staleness checks
- [ ] battle.rs:6162 - Berry cycling checks

### Dynamax & Special Mechanics
- [ ] battle.rs:5047 - Dynamax 3-turn removal
- [ ] battle.rs:5050 - Gen 1 partial trapping cleanup
- [ ] battle.rs:5549 - Zacian/Zamazenta forme changes
- [ ] battle.rs:5557 - Format callbacks in runAction
- [ ] battle.rs:5561 - Switch in all active Pokemon

### Missing Infrastructure
- [ ] battle.rs:819 - customRules display
- [ ] battle.rs:1176 - Gen 5+ faintData tracking
- [ ] battle.rs:2135 - Migrate switch-in ability logic to ability_callbacks/
- [ ] battle.rs:2762 - is_started field (tracks if pokemon sent out)
- [ ] battle.rs:2779-2780 - queue.clear() and Bide damage clearing
- [ ] battle.rs:3808 - Substitute HP tracking
- [ ] battle.rs:4071 - effect.effectType === 'Move' check
- [ ] battle.rs:4087 - Migrate to boost_new() method
- [ ] battle.rs:5619 - support_cancel field
- [ ] battle.rs:6007 - Full getRequests() logic
- [ ] battle.rs:6020 - sentRequests field
- [ ] battle_actions.rs:2730 - Current effect tracking
- [ ] battle_actions.rs:2788 - Active move priority inheritance
- [ ] battle_queue.rs:903 - Unclear behavior documentation

## Implementation Priority

### P0 - Critical (Breaks core mechanics)
1. Move execution events (ModifyTarget, ModifyPriority)
2. Pokemon adjacency helpers (adjacentAllies, adjacentFoes)
3. Faint events (BeforeFaint, Faint, AfterFaint)
4. Boost events (ChangeBoost, TryBoost)

### P1 - Important (Missing major features)
1. Z-Move transformation
2. Max Move transformation  
3. Format callbacks
4. Side management (clearChoice, activeRequest)

### P2 - Nice-to-have (Edge cases, gen-specific)
1. Gen-specific mechanics
2. Dynamax features
3. Custom rules display

## Next Steps

1. Start with P0 critical implementations
2. Work through each TODO systematically
3. Test each implementation against JavaScript behavior
4. Commit after each major feature completion

