#!/usr/bin/env node

/**
 * Generate comprehensive item dispatch functions for mod.rs
 * Based on parsing all items and their callbacks
 */

const fs = require('fs');
const path = require('path');

// Load the items from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');
const itemsPath = path.join(workspaceRoot, 'pokemon-showdown-js', 'data', 'items.ts');
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

console.log(`Found ${items.length} items with callbacks`);

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

// Determine if a callback mutates battle state
function isMutatingCallback(callback) {
    const readOnly = [
        'onModifyAtk', 'onModifySpA', 'onModifyDef', 'onModifySpD',
        'onModifyDamage', 'onSourceModifyDamage', 'onModifySpe',
        'onModifyAccuracy', 'onSourceModifyAccuracy', 'onModifyCritRatio',
        'onBasePower', 'onSourceBasePower'
    ];
    return !readOnly.includes(callback);
}

// Generate dispatch functions
const dispatchers = [];

// Sort callbacks alphabetically
const sortedCallbacks = Array.from(callbackMap.keys()).sort();

sortedCallbacks.forEach(callback => {
    const itemIds = callbackMap.get(callback);
    const funcName = `dispatch_${camelToSnake(callback)}`;
    const isMutating = isMutatingCallback(callback);
    const battleRef = isMutating ? '&mut Battle' : '&Battle';

    // Determine parameters based on callback type
    let params = '';
    if (callback.includes('Source') && callback.includes('Damage')) {
        params = ',\n    target_pos: (usize, usize),\n    source_pos: Option<(usize, usize)>';
    } else if (callback.includes('Damage')) {
        params = ',\n    pokemon_pos: (usize, usize),\n    source_pos: Option<(usize, usize)>';
    } else if (callback === 'onAfterMoveSecondarySelf') {
        params = ',\n    source_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
    } else {
        params = ',\n    pokemon_pos: (usize, usize)';
    }

    const comment = isMutating ? '/// Dispatch ' + callback + ' callbacks (mutates battle)' : '/// Dispatch ' + callback + ' callbacks (read-only)';

    dispatcher = `${comment}
pub fn ${funcName}(
    battle: ${battleRef},
    item_id: &str${params},
) -> Option<EventResult> {
    match item_id {
        ${itemIds.slice(0, 5).map(id => `// "${id}" => Some(${id}::${camelToSnake(callback)}(battle, pokemon_pos)),`).join('\n        ')}
        ${itemIds.length > 5 ? `// ... ${itemIds.length - 5} more items` : ''}
        _ => None,
    }
}`;

    dispatchers.push(dispatcher);
});

// Print to stdout
console.log('\n// ===========================================');
console.log('// Generated Item Dispatch Functions');
console.log('// ===========================================\n');
console.log(dispatchers.join('\n\n'));
