#!/usr/bin/env node

/**
 * Generate comprehensive move dispatch functions for mod.rs
 * Based on parsing all moves and their callbacks
 */

const fs = require('fs');
const path = require('path');

// Load the moves from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '../..');
const movesPath = path.join(workspaceRoot, 'pokemon-showdown-ts', 'data', 'moves.ts');
const movesContent = fs.readFileSync(movesPath, 'utf8');

// Parse moves - extract each move definition
const moveRegex = /^\t([a-z0-9]+): \{([\s\S]*?)^\t\},?$/gm;
const moves = [];
let match;

while ((match = moveRegex.exec(movesContent)) !== null) {
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
        moves.push({ id, callbacks });
    }
}

console.error(`Found ${moves.length} moves with callbacks`);

// Build a map of callback -> moves
const callbackMap = new Map();
moves.forEach(move => {
    move.callbacks.forEach(callback => {
        if (!callbackMap.has(callback)) {
            callbackMap.set(callback, []);
        }
        callbackMap.get(callback).push(move.id);
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
    if (callback.includes('Damage') || callback === 'onTryHit' || callback === 'onHit' || callback === 'onAfterHit') {
        params = ',\n    _move_id: &str,\n    _source_pos: (usize, usize),\n    _target_pos: (usize, usize)';
    } else if (callback === 'onTryMove' || callback === 'onModifyType') {
        params = ',\n    _move_id: &str,\n    _source_pos: (usize, usize)';
    } else if (callback === 'onBasePower' || callback === 'onAfterMove') {
        params = ',\n    _move_id: &str,\n    _source_pos: (usize, usize),\n    _target_pos: Option<(usize, usize)>';
    } else {
        params = ',\n    _move_id: &str,\n    _source_pos: (usize, usize)';
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
console.log('//! Move Callback Handlers');
console.log('//!');
console.log('//! This module provides dispatch functions for move callbacks.');
console.log('//! Individual move implementations will be added as needed.');
console.log('');
console.log('use crate::battle::Battle;');
console.log('use crate::event::EventResult;');
console.log('');
console.log('// =========================================================================');
console.log('// DISPATCH FUNCTIONS');
console.log('//');
console.log('// These functions route move events to specific move implementations.');
console.log('// They return EventResult directly, with EventResult::Continue for no match.');
console.log('// =========================================================================');
console.log('');
console.log(dispatchers.join('\n\n'));
