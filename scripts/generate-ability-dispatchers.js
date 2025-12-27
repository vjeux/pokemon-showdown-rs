#!/usr/bin/env node

/**
 * Generate comprehensive ability dispatch functions for mod.rs
 * Based on parsing all abilities and their callbacks
 */

const fs = require('fs');
const path = require('path');

// Load the abilities from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '../..');
const abilitiesPath = path.join(workspaceRoot, 'pokemon-showdown-ts', 'data', 'abilities.ts');
const abilitiesContent = fs.readFileSync(abilitiesPath, 'utf8');

// Parse abilities - extract each ability definition
const abilityRegex = /^\t([a-z0-9]+): \{([\s\S]*?)^\t\},?$/gm;
const abilities = [];
let match;

while ((match = abilityRegex.exec(abilitiesContent)) !== null) {
    const id = match[1];
    const content = match[2];

    // Extract all callbacks (functions)
    const callbacks = [];
    const callbackRegex = /^\t\t(on[A-Z]\w+)(?:Priority|Order|SubOrder)?[\(:\s]/gm;
    let callbackMatch;
    const seenCallbacks = new Set();

    while ((callbackMatch = callbackRegex.exec(content)) !== null) {
        const callbackName = callbackMatch[1];
        if (!seenCallbacks.has(callbackName)) {
            seenCallbacks.add(callbackName);
            callbacks.push(callbackName);
        }
    }

    if (callbacks.length > 0) {
        abilities.push({ id, callbacks });
    }
}

console.error(`Found ${abilities.length} abilities with callbacks`);

// Build a map of callback -> abilities
const callbackMap = new Map();
abilities.forEach(ability => {
    ability.callbacks.forEach(callback => {
        if (!callbackMap.has(callback)) {
            callbackMap.set(callback, []);
        }
        callbackMap.get(callback).push(ability.id);
    });
});

// Convert camelCase to snake_case
function camelToSnake(str) {
    return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

// Determine if a callback mutates battle state
function isMutatingCallback(callback) {
    const readOnly = [
        'onModifyAtk', 'onModifySpA', 'onModifyDef', 'onModifySpD',
        'onModifyDamage', 'onSourceModifyDamage', 'onModifySpe',
        'onModifyAccuracy', 'onSourceModifyAccuracy', 'onModifyCritRatio',
        'onBasePower', 'onSourceBasePower', 'onModifyType', 'onTryHit',
        'onTryMove', 'onModifyMove', 'onModifySTAB', 'onModifyWeight',
        'onModifyPriority'
    ];
    return !readOnly.includes(callback);
}

// Generate dispatch functions
const dispatchers = [];

// Sort callbacks alphabetically
const sortedCallbacks = Array.from(callbackMap.keys()).sort();

sortedCallbacks.forEach(callback => {
    const abilityIds = callbackMap.get(callback);
    const funcName = `dispatch_${camelToSnake(callback)}`;
    const isMutating = isMutatingCallback(callback);
    const battleRef = isMutating ? '&mut Battle' : '&Battle';

    // Determine parameters based on callback type
    let params = '';
    if (callback.includes('Source') && callback.includes('Damage')) {
        params = ',\n    _ability_id: &str,\n    _pokemon_pos: (usize, usize)';
    } else if (callback.includes('Damage')) {
        params = ',\n    _ability_id: &str,\n    _pokemon_pos: (usize, usize)';
    } else if (callback === 'onSwitchIn' || callback === 'onStart') {
        params = ',\n    _ability_id: &str,\n    _pokemon_pos: (usize, usize)';
    } else {
        params = ',\n    _ability_id: &str,\n    _pokemon_pos: (usize, usize)';
    }

    const comment = isMutating ? '/// Dispatch ' + callback + ' callbacks (mutates battle)' : '/// Dispatch ' + callback + ' callbacks (read-only)';

    // Count abilities per callback
    const abilityCount = abilityIds.length;
    const abilityList = abilityIds.slice(0, 10).map(id => `"${id}"`).join(', ');
    const more = abilityCount > 10 ? `, ... (${abilityCount - 10} more)` : '';

    dispatcher = `${comment}
/// Abilities: ${abilityList}${more}
pub fn ${funcName}(
    _battle: ${battleRef}${params},
) -> EventResult {
    // TODO: Implement dispatch for ${abilityCount} abilities
    EventResult::Continue
}`;

    dispatchers.push(dispatcher);
});

// Print to stdout
console.log('//! Ability Callback Handlers');
console.log('//!');
console.log('//! This module provides dispatch functions for ability callbacks.');
console.log('//! Individual ability implementations will be added as needed.');
console.log('');
console.log('use crate::battle::Battle;');
console.log('use crate::event::EventResult;');
console.log('');
console.log('// =========================================================================');
console.log('// DISPATCH FUNCTIONS');
console.log('//');
console.log('// These functions route ability events to specific ability implementations.');
console.log('// They return EventResult directly, with EventResult::Continue for no match.');
console.log('// =========================================================================');
console.log('');
console.log(dispatchers.join('\n\n'));
