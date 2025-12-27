#!/usr/bin/env node

/**
 * Generate comprehensive item dispatch functions for mod.rs
 * Based on parsing all items and their callbacks
 */

const fs = require('fs');
const path = require('path');

// Load the items from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '../..');
const itemsPath = path.join(workspaceRoot, 'pokemon-showdown-ts', 'data', 'items.ts');
const itemsContent = fs.readFileSync(itemsPath, 'utf8');

// Parse items - extract each item definition
const itemRegex = /^\t([a-z0-9]+): \{([\s\S]*?)^\t\},?$/gm;
const items = [];
let match;

while ((match = itemRegex.exec(itemsContent)) !== null) {
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
        items.push({ id, callbacks });
    }
}

console.error(`Found ${items.length} items with callbacks`);

// Build a map of callback -> items
const callbackMap = new Map();
items.forEach(item => {
    item.callbacks.forEach(callback => {
        if (!callbackMap.has(callback)) {
            callbackMap.set(callback, []);
        }
        callbackMap.get(callback).push(item.id);
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
    if (callback.includes('Source') && callback.includes('Damage')) {
        params = ',\n    _item_id: &str,\n    _pokemon_pos: (usize, usize)';
    } else if (callback.includes('Damage')) {
        params = ',\n    _item_id: &str,\n    _pokemon_pos: (usize, usize)';
    } else if (callback === 'onAfterMoveSecondarySelf') {
        params = ',\n    _item_id: &str,\n    _source_pos: (usize, usize),\n    _target_pos: Option<(usize, usize)>';
    } else {
        params = ',\n    _item_id: &str,\n    _pokemon_pos: (usize, usize)';
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
console.log('//! Item Callback Handlers');
console.log('//!');
console.log('//! This module provides dispatch functions for item callbacks.');
console.log('//! Individual item implementations will be added as needed.');
console.log('');
console.log('use crate::battle::Battle;');
console.log('use crate::event::EventResult;');
console.log('');
console.log('// =========================================================================');
console.log('// DISPATCH FUNCTIONS');
console.log('//');
console.log('// These functions route item events to specific item implementations.');
console.log('// They return EventResult directly, with EventResult::Continue for no match.');
console.log('// =========================================================================');
console.log('');
console.log(dispatchers.join('\n\n'));
