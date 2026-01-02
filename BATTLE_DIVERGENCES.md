# Battle Module Divergences - JavaScript vs Rust

This document tracks divergences between the JavaScript implementation in `pokemon-showdown/sim/battle.ts` and the Rust implementation in `pokemon-showdown-rs/src/battle/`.

## Executive Summary (2026-01-02)

**Initial Assessment:**

- **Total files in battle/**: 151
- **Files with TODOs/NOTEs**: 90 (60%)
- **Files potentially complete**: 61 (40%)
- **Total TODO/NOTE comments**: 190
- **Status**: Initial comprehensive scan and categorization in progress

## Quick Statistics

| Category | Count | Percentage |
|----------|-------|------------|
| Total Files | 151 | 100% |
| Files with Issues | 90 | 60% |
| Clean Files | 61 | 40% |
| Total TODOs/NOTEs | 190 | - |

## Work Strategy

Given the large scope (151 files, 190 TODOs), the approach will be:

1. **Phase 1: Categorization** - Group files by type of issue
2. **Phase 2: Critical Path** - Fix blocking/infrastructure issues first
3. **Phase 3: Systematic Completion** - Work through remaining files alphabetically
4. **Phase 4: Verification** - Ensure all files have 1:1 equivalence

## File Categories

### Category A: Complete Stubs (Need Full Implementation)

Files that are placeholder stubs:
- check_e_v_balance.rs - EV balance validation
- get_team.rs - Team data retrieval
- get_callback.rs - Callback retrieval (architectural difference)
- find_battle_event_handlers.rs - Event handler discovery
- find_field_event_handlers.rs - Field event handlers
- find_pokemon_event_handlers.rs - Pokemon event handlers
- find_side_event_handlers.rs - Side event handlers

### Category B: Partial Implementations

Files with significant missing functionality:
- get_requests.rs - Missing Pokemon.get_move_request_data()
- handle_ability_event.rs - Missing parameter wire-through
- faint_messages.rs - Missing formeRegression
- end_turn.rs - Missing swapPosition, canDynamaxNow
- boost.rs - Needs migration to boost_new()

### Category C: Minor TODOs

Files with documentation or optimization notes:
- each_event.rs - Research notes about update timing
- do_switch.rs - Architecture note about hazards

### Category D: Clean Files (61 files)

Files with no TODOs - need verification for 1:1 equivalence.

## Methodology

1. List all files in src/battle/
2. Grep for TODOs and NOTEs in each file
3. Categorize files by completeness
4. Systematically verify 1:1 equivalence with JavaScript
5. Implement missing features
6. Document all changes

---

## Session Log

### 2026-01-02 - Session Start

Starting comprehensive 1:1 verification of battle/ folder.

**Initial Scan Results:**
- 151 total files
- 90 files with TODOs/NOTEs (190 total comments)
- 61 files appear clean (need verification)

**Next Steps:**
1. Create detailed inventory of all TODOs
2. Prioritize critical infrastructure items
3. Begin systematic implementation

