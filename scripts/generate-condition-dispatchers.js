#!/usr/bin/env node

/**
 * Generate condition callback files and dispatchers
 *
 * This script:
 * 1. Loads data/conditions.ts from JavaScript by requiring it
 * 2. Introspects each condition to find which properties are functions
 * 3. Generates Rust files in src/data/condition_callbacks/
 * 4. Updates mod.rs with all dispatch functions
 */

const fs = require('fs');
const path = require('path');

// Load the actual Conditions object from the transpiled JavaScript
// First we need to build the TypeScript if not already done
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Conditions;
try {
    // Try to require the compiled conditions from dist
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Conditions = Dex.data.Conditions;
    console.log('Loaded conditions from dist/sim');
} catch (e) {
    console.error('Error loading conditions:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

// Convert callback name to snake_case
function toSnakeCase(str) {
    return str.replace(/([A-Z])/g, '_$1').toLowerCase();
}

// Extract callbacks from a condition object
function extractCallbacks(conditionId, conditionData) {
    const callbacks = [];

    // Iterate through all properties of the condition
    for (const key in conditionData) {
        // Skip if not own property
        if (!conditionData.hasOwnProperty(key)) continue;

        // Check if it's a function - that's the only requirement for a callback
        if (typeof conditionData[key] === 'function') {
            callbacks.push(key);
        }
    }

    return callbacks;
}

// Parse all conditions
console.log('Extracting callbacks from conditions...');
const conditions = {};

for (const conditionId in Conditions) {
    if (!Conditions.hasOwnProperty(conditionId)) continue;

    const conditionData = Conditions[conditionId];
    const callbacks = extractCallbacks(conditionId, conditionData);

    if (callbacks.length > 0) {
        conditions[conditionId] = {
            callbacks,
            data: conditionData
        };
    }
}

console.log(`\nFound ${Object.keys(conditions).length} conditions with callbacks:`);
Object.entries(conditions).forEach(([id, data]) => {
    console.log(`  ${id}: ${data.callbacks.join(', ')}`);
});

// Update conditions.json with callback information
console.log('\nUpdating conditions.json with callback information...');
const conditionsJsonPath = path.join(__dirname, '../data/conditions.json');
let conditionsJson = {};

// Load existing conditions.json
if (fs.existsSync(conditionsJsonPath)) {
    conditionsJson = JSON.parse(fs.readFileSync(conditionsJsonPath, 'utf8'));
}

// Add callback flags for each condition
for (const conditionId in Conditions) {
    if (!Conditions.hasOwnProperty(conditionId)) continue;

    const conditionData = Conditions[conditionId];
    const callbacks = extractCallbacks(conditionId, conditionData);

    if (!conditionsJson[conditionId]) {
        conditionsJson[conditionId] = {};
    }

    // Add callback boolean flags
    callbacks.forEach(callback => {
        conditionsJson[conditionId][callback] = true;
    });

    // Also preserve existing metadata (name, effectType, etc.)
    if (conditionData.name) {
        conditionsJson[conditionId].name = conditionData.name;
    } else {
        conditionsJson[conditionId].name = conditionId;
    }

    if (conditionData.effectType) {
        conditionsJson[conditionId].effectType = conditionData.effectType;
    }

    if (conditionData.duration) {
        conditionsJson[conditionId].duration = conditionData.duration;
    }

    // Preserve priority/order/suborder metadata
    for (const key in conditionData) {
        if (key.endsWith('Priority') || key.endsWith('Order') || key.endsWith('SubOrder')) {
            conditionsJson[conditionId][key] = conditionData[key];
        }
    }

    // Preserve other metadata flags
    if (conditionData.noCopy !== undefined) {
        conditionsJson[conditionId].noCopy = conditionData.noCopy;
    }
    if (conditionData.counterMax !== undefined) {
        conditionsJson[conditionId].counterMax = conditionData.counterMax;
    }
}

// Write updated conditions.json
fs.writeFileSync(conditionsJsonPath, JSON.stringify(conditionsJson, null, 2));
console.log(`  Updated ${conditionsJsonPath} with callback flags`);

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
