#!/usr/bin/env node

/**
 * Generate Rust item stubs from TypeScript items
 * Each item with callbacks gets its own file
 * Run: docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && node scripts/generate-items.js"
 */

const fs = require('fs');
const path = require('path');

// Helper function to generate parameters based on callback name (standardized)
function generateParameters(callbackName, jsArgs) {
    // Always include battle
    let params = ['battle: &mut Battle'];

    // Standard signatures for each callback type
    const standardSignatures = {
        // Item-specific callbacks
        'onEat': ['pokemon_pos: (usize, usize)'],
        'onTakeItem': ['item_pos: Option<(usize, usize)>', 'pokemon_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>'],
        'onStart': ['target_pos: Option<(usize, usize)>'],
        'onEnd': ['pokemon_pos: (usize, usize)'],
        'onUpdate': ['pokemon_pos: (usize, usize)'],
        'onResidual': ['pokemon_pos: (usize, usize)'],
        'onBasePower': ['base_power: i32', 'pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>', 'move_id: &str'],
        'onModifyDamage': ['damage: i32', 'pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>', 'move_id: &str'],
        'onSourceModifyDamage': ['damage: i32', 'source_pos: (usize, usize)', 'target_pos: (usize, usize)', 'move_id: &str'],
        'onTryHit': ['target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],
        'onDamagingHit': ['damage: i32', 'target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],
        'onAfterMoveSecondarySelf': ['source_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>', 'move_id: &str'],
        'onModifyMove': ['move_id: &str', 'pokemon_pos: (usize, usize)'],
        'onTryEatItem': ['item_id: &str', 'pokemon_pos: (usize, usize)'],
    };

    // Use standard signature if available, otherwise parse JS args
    if (standardSignatures[callbackName]) {
        params.push(...standardSignatures[callbackName]);
    } else {
        // Parse JS arguments to determine what Rust parameters we need
        const args = jsArgs.split(',').map(a => a.trim()).filter(a => a);

        // Map common JS parameter names to Rust types
        const paramMap = {
            'pokemon': 'pokemon_pos: (usize, usize)',
            'target': 'target_pos: Option<(usize, usize)>',
            'source': 'source_pos: Option<(usize, usize)>',
            'item': 'item_id: &str',
            'effect': 'effect_id: Option<&str>',
            'damage': 'damage: i32',
            'basePower': 'base_power: i32',
            'move': 'move_id: &str',
        };

        // Add parameters based on JS args
        for (const arg of args) {
            if (paramMap[arg]) {
                params.push(paramMap[arg]);
            }
        }
    }

    return params.join(', ');
}

// Helper function to convert camelCase to snake_case
function camelToSnake(str) {
    return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

// Helper function to escape Rust keywords in module names
const RUST_KEYWORDS = new Set(['static', 'type', 'move', 'self', 'super', 'crate', 'as', 'async', 'await', 'break', 'const', 'continue', 'dyn', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop', 'match', 'mod', 'mut', 'pub', 'ref', 'return', 'struct', 'trait', 'true', 'unsafe', 'use', 'where', 'while']);
function rustModName(name) {
    if (RUST_KEYWORDS.has(name)) {
        return `r#${name}`;
    }
    return name;
}

// Load the items from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');
const showdownDir = process.env.SHOWDOWN_DIR || 'pokemon-showdown';
const itemsPath = path.join(path.dirname(workspaceRoot), showdownDir, 'data', 'items.ts');
const itemsContent = fs.readFileSync(itemsPath, 'utf8');

// Parse items - extract each item definition
// Items are structured as: itemid: { ... },
const itemRegex = /^\t([a-z0-9]+): \{([\s\S]*?)^\t\},?$/gm;
const items = [];
let match;

while ((match = itemRegex.exec(itemsContent)) !== null) {
    const id = match[1];
    const content = match[2];

    // Extract basic properties
    const nameMatch = content.match(/name:\s*"([^"]+)"/);
    const numMatch = content.match(/num:\s*(-?\d+)/);
    const genMatch = content.match(/gen:\s*(\d+)/);

    // Extract all callbacks (functions) with their JS source
    const callbacks = [];
    const seenCallbacks = new Set();

    // Match callback functions with their implementations
    const lines = content.split('\n');
    let i = 0;
    while (i < lines.length) {
        const line = lines[i];
        // Match callback name and opening (onXxx pattern for items)
        const match = line.match(/^\t\t(on\w+)(?:Priority|Order|SubOrder)?\s*\((.*)\)\s*\{/);
        if (match) {
            const callbackName = match[1];
            const args = match[2];
            let body = '';
            let braceCount = 1;
            i++;

            // Extract the function body by counting braces
            while (i < lines.length && braceCount > 0) {
                const currentLine = lines[i];
                body += currentLine + '\n';

                // Count braces
                for (const char of currentLine) {
                    if (char === '{') braceCount++;
                    if (char === '}') braceCount--;
                }

                if (braceCount === 0) {
                    // Remove the closing brace line and trailing comma
                    body = body.replace(/^\t\t\},?\s*$/m, '');
                    break;
                }
                i++;
            }

            if (!seenCallbacks.has(callbackName)) {
                seenCallbacks.add(callbackName);

                // Normalize indentation
                const bodyLines = body.split('\n').filter(line => line.trim() !== '');
                if (bodyLines.length > 0) {
                    const minIndent = Math.min(...bodyLines.map(line => {
                        const match = line.match(/^(\s*)/);
                        return match ? match[1].length : 0;
                    }));

                    const normalizedBody = body.split('\n')
                        .map(line => {
                            if (line.trim() === '') return '';
                            const unindented = line.slice(minIndent);
                            return unindented ? '    ' + unindented : '';
                        })
                        .join('\n')
                        .replace(/\s+$/, '');

                    callbacks.push({
                        name: callbackName,
                        args: args,
                        jsSource: `${callbackName}(${args}) {\n${normalizedBody}\n}`
                    });
                } else {
                    callbacks.push({
                        name: callbackName,
                        args: args,
                        jsSource: `${callbackName}(${args}) {}`
                    });
                }
            }
        }
        i++;
    }

    items.push({
        id,
        name: nameMatch ? nameMatch[1] : id,
        num: numMatch ? parseInt(numMatch[1]) : 0,
        gen: genMatch ? parseInt(genMatch[1]) : 1,
        callbacks
    });
}

console.log(`Found ${items.length} items`);

// Filter items that have callbacks
const itemsWithCallbacks = items.filter(i => i.callbacks.length > 0);
console.log(`Found ${itemsWithCallbacks.length} items with callbacks`);

// Sort items alphabetically
itemsWithCallbacks.sort((a, b) => a.id.localeCompare(b.id));

// Create item_callbacks directory if it doesn't exist
const itemsDir = path.join(workspaceRoot, 'src', 'data', 'item_callbacks');
if (!fs.existsSync(itemsDir)) {
    fs.mkdirSync(itemsDir, { recursive: true });
}

// Generate individual file for each item with callbacks
let generatedCount = 0;
for (const item of itemsWithCallbacks) {
    const rustCode = `//! ${item.name} Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

${item.callbacks.map(callback => {
    const rustFuncName = camelToSnake(callback.name);
    const params = generateParameters(callback.name, callback.args);
    // Format JS source: replace all tabs with 4 spaces
    const formattedSource = callback.jsSource
        .split('\n')
        .map(line => line.replace(/\t/g, '    '))
        .map(line => '/// ' + line)
        .join('\n');

    return `${formattedSource}
pub fn ${rustFuncName}(${params}) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
`;
}).join('\n')}`;

    const filePath = path.join(itemsDir, `${item.id}.rs`);
    fs.writeFileSync(filePath, rustCode);
    generatedCount++;
}

console.log(`Generated ${generatedCount} item files`);

// Build a map of callback -> items for dispatcher generation
const callbackMap = new Map();
itemsWithCallbacks.forEach(item => {
    item.callbacks.forEach(callback => {
        if (!callbackMap.has(callback.name)) {
            callbackMap.set(callback.name, []);
        }
        callbackMap.get(callback.name).push(item.id);
    });
});

// Generate dispatch functions
const sortedCallbacks = Array.from(callbackMap.keys()).sort();
const dispatchers = sortedCallbacks.map(callback => {
    const funcName = `dispatch_${camelToSnake(callback)}`;
    const rustCallbackName = camelToSnake(callback);
    const itemIds = callbackMap.get(callback);

    // Determine parameters based on callback type
    let params = ',\n    item_id: &str,\n    pokemon_pos: (usize, usize)';
    let callParams = 'battle, pokemon_pos';

    if (callback === 'onEat') {
        params = ',\n    item_id: &str,\n    pokemon_pos: (usize, usize)';
        callParams = 'battle, pokemon_pos';
    } else if (callback === 'onBasePower') {
        params = ',\n    item_id: &str,\n    base_power: i32,\n    pokemon_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, base_power, pokemon_pos, target_pos';
    } else if (callback === 'onTryHit') {
        params = ',\n    item_id: &str,\n    target_pos: (usize, usize),\n    source_pos: (usize, usize)';
        callParams = 'battle, target_pos, source_pos';
    }

    const matchArms = itemIds.map(itemId => {
        return `        "${itemId}" => ${rustModName(itemId)}::${rustCallbackName}(${callParams}),`;
    }).join('\n');

    return `/// Dispatch ${callback} callbacks
pub fn ${funcName}(
    battle: &mut Battle${params},
) -> EventResult {
    match item_id {
${matchArms}
        _ => EventResult::Continue,
    }
}`;
}).join('\n\n');

// Generate mod.rs to export all items
const modContent = `//! Item Callback Handlers
//!
//! This module exports all item callback implementations.
//! Each item with callbacks is in its own file.

use crate::battle::Battle;
use crate::event::EventResult;

// Individual item modules
${itemsWithCallbacks.map(i =>
    `pub mod ${rustModName(i.id)};`
).join('\n')}

// Dispatch functions
${dispatchers}
`;

const modPath = path.join(itemsDir, 'mod.rs');
fs.writeFileSync(modPath, modContent);
console.log(`Generated ${modPath}`);

// Generate ITEMS_TODO.md
let todoContent = `# Items Implementation Tracking

Total: ${items.length} items
Items with callbacks: ${itemsWithCallbacks.length}

## Items with Callbacks (alphabetically)
${itemsWithCallbacks.map(i => {
    const totalCallbacks = i.callbacks.length;
    const allCallbacks = i.callbacks.map(cb => cb.name);
    return `- [ ] ${i.id} - ${i.name} (Gen ${i.gen}) - ${totalCallbacks} callback${totalCallbacks !== 1 ? 's' : ''}: ${allCallbacks.join(', ')}`;
}).join('\n')}

## Statistics

By callback type:
${(() => {
    const callbackCount = {};
    itemsWithCallbacks.forEach(i => {
        i.callbacks.forEach(cb => {
            callbackCount[cb.name] = (callbackCount[cb.name] || 0) + 1;
        });
    });
    return Object.entries(callbackCount)
        .sort((a, b) => b[1] - a[1])
        .map(([cb, count]) => `- ${cb}: ${count} items`)
        .join('\n');
})()}

## Items by Generation
${(() => {
    const genCount = {};
    itemsWithCallbacks.forEach(i => {
        genCount[i.gen] = (genCount[i.gen] || 0) + 1;
    });
    return Object.entries(genCount)
        .sort((a, b) => parseInt(a[0]) - parseInt(b[0]))
        .map(([gen, count]) => `- Gen ${gen}: ${count} items`)
        .join('\n');
})()}
`;

const todoPath = path.join(workspaceRoot, 'ITEMS_TODO.md');
fs.writeFileSync(todoPath, todoContent);
console.log(`Generated ${todoPath}`);

console.log('\nGeneration complete!');
console.log(`- ${items.length} items processed`);
console.log(`- ${itemsWithCallbacks.length} items with callbacks`);
console.log(`- ${generatedCount} individual item files created`);
console.log(`- mod.rs created`);
console.log(`- ITEMS_TODO.md created`);
