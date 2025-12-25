#!/usr/bin/env node

/**
 * Generate Rust item stubs from TypeScript items
 * Each item with callbacks gets its own file
 * Run: docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && node scripts/generate-items.js"
 */

const fs = require('fs');
const path = require('path');

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
const itemsPath = path.join(workspaceRoot, 'pokemon-showdown-js', 'data', 'items.ts');
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

    // Extract all callbacks (functions)
    const callbacks = [];
    const callbackRegex = /(\w+)(?:Priority|Order|SubOrder)?[:\(]/g;
    let callbackMatch;
    const seenCallbacks = new Set();

    while ((callbackMatch = callbackRegex.exec(content)) !== null) {
        const callbackName = callbackMatch[1];
        if (callbackName.startsWith('on') && !seenCallbacks.has(callbackName)) {
            seenCallbacks.add(callbackName);
            callbacks.push(callbackName);
        }
    }

    // Check for special properties
    const hasFling = content.includes('fling: {');
    const hasMegaStone = content.includes('megaStone:');
    const hasBoosts = content.includes('boosts: {');

    items.push({
        id,
        name: nameMatch ? nameMatch[1] : id,
        num: numMatch ? parseInt(numMatch[1]) : 0,
        gen: genMatch ? parseInt(genMatch[1]) : 0,
        callbacks,
        hasFling,
        hasMegaStone,
        hasBoosts,
        fullContent: match[0] // Full JS source
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
//!
//! \`\`\`text
//! JS Source (data/items.ts):
${item.fullContent.split('\n').map(line => '//! ' + line).join('\n')}
//! \`\`\`

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{ItemHandlerResult, ItemDef};

${item.callbacks.map(callback => {
    const rustFuncName = camelToSnake(callback);
    return `/// ${callback}(...)
pub fn ${rustFuncName}(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
`;
}).join('\n')}`;

    const filePath = path.join(itemsDir, `${item.id}.rs`);
    fs.writeFileSync(filePath, rustCode);
    generatedCount++;
}

console.log(`Generated ${generatedCount} item files`);

// Generate mod.rs to export all items
const modContent = `//! Item Callback Handlers
//!
//! This module exports all item callback implementations.
//! Each item with callbacks is in its own file.

// Common types
mod common;
pub use common::*;

// Individual item modules
${itemsWithCallbacks.map(i =>
    `pub mod ${rustModName(i.id)};`
).join('\n')}
`;

const modPath = path.join(itemsDir, 'mod.rs');
fs.writeFileSync(modPath, modContent);
console.log(`Generated ${modPath}`);

// Generate common.rs with shared types
const commonContent = `//! Common types for item callbacks

use crate::dex_data::ID;

/// Result from an item handler
#[derive(Debug, Clone)]
pub enum ItemHandlerResult {
    /// No return value (undefined in JS)
    Undefined,
    /// Return false
    False,
    /// Return true
    True,
    /// Return null (blocks action)
    Null,
    /// Return 0
    Zero,
    /// Return a number
    Number(i32),
    /// Chain modifier (numerator, denominator)
    ChainModify(u32, u32),
}

/// Item definition placeholder
/// TODO: Import actual ItemDef from items module when available
#[derive(Debug, Clone, Default)]
pub struct ItemDef {
    pub id: ID,
    pub name: String,
}
`;

const commonPath = path.join(itemsDir, 'common.rs');
fs.writeFileSync(commonPath, commonContent);
console.log(`Generated ${commonPath}`);

// Generate ITEMS_TODO.md
let todoContent = `# Items Implementation Tracking

Total: ${items.length} items
Items with callbacks: ${itemsWithCallbacks.length}

## Items with Callbacks (alphabetically)
${itemsWithCallbacks.map(i => `- [ ] ${i.id} - ${i.name} (Gen ${i.gen}) - ${i.callbacks.length} callback${i.callbacks.length !== 1 ? 's' : ''}: ${i.callbacks.join(', ')}`).join('\n')}

## Statistics

By callback type:
${(() => {
    const callbackCount = {};
    itemsWithCallbacks.forEach(i => {
        i.callbacks.forEach(cb => {
            callbackCount[cb] = (callbackCount[cb] || 0) + 1;
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
console.log(`- mod.rs and common.rs created`);
console.log(`- ITEMS_TODO.md created`);
