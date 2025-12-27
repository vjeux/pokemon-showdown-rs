#!/usr/bin/env node

/**
 * Generate comprehensive condition dispatch functions for mod.rs
 * Based on parsing all conditions and their callbacks
 */

const fs = require('fs');
const path = require('path');

// Load the conditions from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '../..');
const conditionsPath = path.join(workspaceRoot, 'pokemon-showdown-ts', 'data', 'conditions.ts');
const conditionsContent = fs.readFileSync(conditionsPath, 'utf8');

// Parse conditions - extract each condition definition
const conditionRegex = /^\t([a-z0-9]+): \{([\s\S]*?)^\t\},?$/gm;
const conditions = [];
let match;

while ((match = conditionRegex.exec(conditionsContent)) !== null) {
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
        conditions.push({ id, callbacks });
    }
}

console.error(`Found ${conditions.length} conditions with callbacks`);

// Build a map of callback -> conditions
const callbackMap = new Map();
conditions.forEach(condition => {
    condition.callbacks.forEach(callback => {
        if (!callbackMap.has(callback)) {
            callbackMap.set(callback, []);
        }
        callbackMap.get(callback).push(condition.id);
    });
});

// Convert camelCase to snake_case
function camelToSnake(str) {
    return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

// Generate dispatch functions
const dispatchers = [];

// Sort callbacks alphabetically
const sortedCallbacks = Array.from(callbackMap.keys()).sort();

sortedCallbacks.forEach(callback => {
    const funcName = `dispatch_${camelToSnake(callback)}`;

    // Determine parameters based on callback type
    let params = '';
    if (callback.includes('Damage') || callback === 'onTryHit' || callback === 'onHit') {
        params = ',\n    _condition_id: &str,\n    _pokemon_pos: (usize, usize)';
    } else if (callback === 'onSideStart' || callback === 'onSideEnd' || callback === 'onSideResidual') {
        params = ',\n    _condition_id: &str,\n    _side_idx: usize';
    } else {
        params = ',\n    _condition_id: &str,\n    _pokemon_pos: (usize, usize)';
    }

    dispatcher = `/// Dispatch ${callback} callbacks
pub fn ${funcName}(
    _battle: &mut Battle${params},
) -> EventResult {
    EventResult::Continue
}`;

    dispatchers.push(dispatcher);
});

// Print to stdout
console.log('//! Condition Callback Handlers');
console.log('//!');
console.log('//! This module provides dispatch functions for condition callbacks.');
console.log('//! Individual condition implementations will be added as needed.');
console.log('');
console.log('use crate::battle::Battle;');
console.log('use crate::event::EventResult;');
console.log('');
console.log('// =========================================================================');
console.log('// DISPATCH FUNCTIONS');
console.log('//');
console.log('// These functions route condition events to specific condition implementations.');
console.log('// They return EventResult directly, with EventResult::Continue for no match.');
console.log('// =========================================================================');
console.log('');
console.log(dispatchers.join('\n\n'));
