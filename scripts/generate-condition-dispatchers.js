#!/usr/bin/env node

/**
 * Generate condition callback files and dispatchers
 *
 * This script:
 * 1. Reads data/conditions.ts from JavaScript
 * 2. Extracts all conditions and their callbacks
 * 3. Generates Rust files in src/data/condition_callbacks/
 * 4. Updates mod.rs with all dispatch functions
 */

const fs = require('fs');
const path = require('path');

// Load the JavaScript conditions
const conditionsPath = path.join(__dirname, '../../pokemon-showdown-ts/data/conditions.ts');
const conditionsSource = fs.readFileSync(conditionsPath, 'utf8');

// All possible callback names from JavaScript
const ALL_CALLBACKS = [
    'onStart', 'onRestart', 'onEnd',
    'onBeforeMove', 'onBeforeMovePriority',
    'onDisableMove', 'onLockMove',
    'onResidual', 'onResidualOrder', 'onResidualPriority',
    'onStallMove',
    'onModifyAtk', 'onModifyAtkPriority',
    'onTryMove', 'onTryMovePriority',
    'onTryHit', 'onTryHitPriority',
    'onTryPrimaryHit',
    'onPrepareHit',
    'onModifyMove',
    'onBasePower', 'onBasePowerPriority',
    'onModifyDef', 'onModifyDefPriority',
    'onModifySpD', 'onModifySpDPriority',
    'onModifySpe', 'onModifySpePriority',
    'onEffectiveness', 'onEffectivenessPriority',
    'onDamagingHit',
    'onAfterMove', 'onAfterMoveSecondary',
    'onSourceModifyDamage',
    'onWeather', 'onWeatherModifyDamage',
    'onFieldStart', 'onFieldEnd', 'onFieldResidual', 'onFieldResidualOrder',
    'onSwitchIn',
    'onBeforeSwitchOut', 'onBeforeSwitchOutPriority',
    'onTrapPokemon', 'onTrapPokemonPriority',
    'onDragOut', 'onDragOutPriority',
    'onImmunity',
    'onInvulnerability',
    'onTryAddVolatile',
    'onType', 'onTypePriority',
    'onMoveAborted',
    'onBeforeTurn',
    'onFlinch',
];

// Parse conditions from TypeScript source
function parseConditions(source) {
    const conditions = {};

    // Match each condition definition
    // Pattern: conditionId: { ... }
    const conditionRegex = /(\w+):\s*\{([^}]*(?:\{[^}]*\}[^}]*)*)\}/g;

    let match;
    while ((match = conditionRegex.exec(source)) !== null) {
        const conditionId = match[1];
        const body = match[2];

        // Skip non-condition entries
        if (conditionId === 'export' || conditionId === 'import' || conditionId === 'const') continue;

        // Find all callbacks in this condition
        const callbacks = [];
        ALL_CALLBACKS.forEach(callback => {
            if (body.includes(callback + '(') || body.includes(callback + ':')) {
                callbacks.push(callback);
            }
        });

        if (callbacks.length > 0 || body.includes('effectType')) {
            conditions[conditionId] = {
                callbacks,
                source: body
            };
        }
    }

    return conditions;
}

// Convert callback name to snake_case
function toSnakeCase(str) {
    return str.replace(/([A-Z])/g, '_$1').toLowerCase();
}

// Generate Rust file for a condition
function generateConditionFile(conditionId, conditionData) {
    const callbacks = conditionData.callbacks;

    let content = `//! ${conditionId.charAt(0).toUpperCase() + conditionId.slice(1)} Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

`;

    // Generate each callback function
    callbacks.forEach(callback => {
        const funcName = toSnakeCase(callback);

        content += `/// ${callback}
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// ${conditionId}: {
///     ${callback}(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn ${funcName}(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[${conditionId.toUpperCase()}_${funcName.toUpperCase()}] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

`;
    });

    return content;
}

// Generate mod.rs dispatcher
function generateModRs(conditions) {
    let content = `//! Condition Callback Handlers
//!
//! This module provides dispatch functions for condition callbacks.
//! Individual condition implementations are in separate files.

use crate::battle::Battle;
use crate::event::EventResult;
use crate::ID;

// Individual condition modules
`;

    // Module declarations
    const conditionIds = Object.keys(conditions).sort();
    conditionIds.forEach(id => {
        content += `pub mod ${id};\n`;
    });

    content += `
// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route condition events to specific condition implementations.
// They return EventResult directly, with EventResult::Continue for no match.
// =========================================================================

`;

    // Generate dispatch function for each callback type
    const allCallbacksInUse = new Set();
    Object.values(conditions).forEach(data => {
        data.callbacks.forEach(cb => allCallbacksInUse.add(cb));
    });

    const sortedCallbacks = Array.from(allCallbacksInUse).sort();

    sortedCallbacks.forEach(callback => {
        const funcName = toSnakeCase(callback);

        content += `/// Dispatch ${callback} callbacks
pub fn dispatch_${funcName}(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
`;

        // Add cases for each condition that has this callback
        conditionIds.forEach(id => {
            if (conditions[id].callbacks.includes(callback)) {
                content += `        "${id}" => ${id}::${funcName}(battle, pokemon_pos),\n`;
            }
        });

        content += `        _ => EventResult::Continue,
    }
}

`;
    });

    // Add special dispatchers that take additional parameters
    content += `/// Dispatch onTryHit callbacks (with source and target)
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_hit(battle, condition_id, source_pos, target_pos)
}

/// Dispatch onTryPrimaryHit callbacks
pub fn dispatch_on_try_primary_hit(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_primary_hit(battle, condition_id, pokemon_pos)
}
`;

    return content;
}

// Main execution
console.log('Parsing JavaScript conditions...');
const conditions = parseConditions(conditionsSource);

console.log(`Found ${Object.keys(conditions).length} conditions with callbacks:`);
Object.entries(conditions).forEach(([id, data]) => {
    console.log(`  ${id}: ${data.callbacks.join(', ')}`);
});

// Create output directory
const outDir = path.join(__dirname, '../src/data/condition_callbacks');
if (!fs.existsSync(outDir)) {
    fs.mkdirSync(outDir, { recursive: true });
}

// Generate files for each condition
console.log('\nGenerating condition files...');
Object.entries(conditions).forEach(([id, data]) => {
    // Skip if file already exists (don't overwrite implemented files)
    const filePath = path.join(outDir, `${id}.rs`);
    if (fs.existsSync(filePath)) {
        console.log(`  Skipping ${id}.rs (already exists)`);
        return;
    }

    const content = generateConditionFile(id, data);
    fs.writeFileSync(filePath, content);
    console.log(`  Generated ${id}.rs`);
});

// Generate mod.rs
console.log('\nGenerating mod.rs...');
const modContent = generateModRs(conditions);
const modPath = path.join(outDir, 'mod.rs');
fs.writeFileSync(modPath, modContent);
console.log(`  Generated mod.rs with ${Array.from(new Set(Object.values(conditions).flatMap(d => d.callbacks))).length} dispatch functions`);

console.log('\n✓ Done! Generated condition callback files and dispatchers.');
