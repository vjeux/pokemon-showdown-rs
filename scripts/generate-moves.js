#!/usr/bin/env node

/**
 * Generate Rust move stubs from TypeScript moves
 * Each move with callbacks gets its own file
 * Run: docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && node scripts/generate-moves.js"
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

// Load the moves from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');
const movesPath = path.join(workspaceRoot, 'pokemon-showdown-js', 'data', 'moves.ts');
const movesContent = fs.readFileSync(movesPath, 'utf8');

// Parse moves - extract each move definition
// Moves are structured as: moveid: { ... },
const moveRegex = /^\t([a-z0-9]+): \{([\s\S]*?)^\t\},?$/gm;
const moves = [];
let match;

while ((match = moveRegex.exec(movesContent)) !== null) {
    const id = match[1];
    const content = match[2];

    // Extract basic properties
    const nameMatch = content.match(/name:\s*"([^"]+)"/);
    const numMatch = content.match(/num:\s*(-?\d+)/);
    const categoryMatch = content.match(/category:\s*"([^"]+)"/);
    const typeMatch = content.match(/type:\s*"([^"]+)"/);

    // Extract all callbacks (functions) with their JS source
    const callbacks = [];
    const seenCallbacks = new Set();

    // Match callback functions with their implementations
    // Matches: onCallbackName(args) { ... }
    const callbackImplRegex = /(on\w+)(?:Priority|Order|SubOrder)?\s*\(([\s\S]*?)\)\s*\{([\s\S]*?)^\t\t\}/gm;
    let callbackMatch;

    while ((callbackMatch = callbackImplRegex.exec(content)) !== null) {
        const callbackName = callbackMatch[1];
        const args = callbackMatch[2];
        const body = callbackMatch[3];

        if (!seenCallbacks.has(callbackName)) {
            seenCallbacks.add(callbackName);
            callbacks.push({
                name: callbackName,
                jsSource: `${callbackName}(${args}) {${body}\n\t\t}`
            });
        }
    }

    // Check for condition block
    const hasCondition = content.includes('condition: {');

    // Check for secondary effect
    const hasSecondary = content.includes('secondary: {');

    moves.push({
        id,
        name: nameMatch ? nameMatch[1] : id,
        num: numMatch ? parseInt(numMatch[1]) : 0,
        category: categoryMatch ? categoryMatch[1] : 'Unknown',
        type: typeMatch ? typeMatch[1] : 'Unknown',
        callbacks,
        hasCondition,
        hasSecondary
    });
}

console.log(`Found ${moves.length} moves`);

// Filter moves that have callbacks or conditions
const movesWithCallbacks = moves.filter(m => m.callbacks.length > 0 || m.hasCondition);
console.log(`Found ${movesWithCallbacks.length} moves with callbacks or conditions`);

// Sort moves alphabetically
movesWithCallbacks.sort((a, b) => a.id.localeCompare(b.id));

// Create move_callbacks directory if it doesn't exist
const movesDir = path.join(workspaceRoot, 'src', 'data', 'move_callbacks');
if (!fs.existsSync(movesDir)) {
    fs.mkdirSync(movesDir, { recursive: true });
}

// Generate individual file for each move with callbacks
let generatedCount = 0;
for (const move of movesWithCallbacks) {
    const rustCode = `//! ${move.name} Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

${move.callbacks.map(callback => {
    const rustFuncName = camelToSnake(callback.name);
    return `/// ${callback.name}(...)
///
/// \`\`\`text
/// JS Source (data/moves.ts):
${callback.jsSource.split('\n').map(line => '/// ' + line).join('\n')}
/// \`\`\`
pub fn ${rustFuncName}(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}
`;
}).join('\n')}
${move.hasCondition ? `
// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
` : ''}`;

    const filePath = path.join(movesDir, `${move.id}.rs`);
    fs.writeFileSync(filePath, rustCode);
    generatedCount++;
}

console.log(`Generated ${generatedCount} move files`);

// Generate mod.rs to export all moves
const modContent = `//! Move Callback Handlers
//!
//! This module exports all move callback implementations.
//! Each move with callbacks is in its own file.

// Common types
mod common;
pub use common::*;

// Individual move modules
${movesWithCallbacks.map(m =>
    `pub mod ${rustModName(m.id)};`
).join('\n')}
`;

const modPath = path.join(movesDir, 'mod.rs');
fs.writeFileSync(modPath, modContent);
console.log(`Generated ${modPath}`);

// Generate common.rs with shared types
const commonContent = `//! Common types for move callbacks

use crate::dex_data::ID;

/// Result from a move handler
#[derive(Debug, Clone)]
pub enum MoveHandlerResult {
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

/// Status object
#[derive(Debug, Clone, Default)]
pub struct Status {
    pub id: String,
}

/// Effect object
#[derive(Debug, Clone, Default)]
pub struct Effect {
    pub id: String,
    pub effect_type: String,
    pub status: Option<String>,
}
`;

const commonPath = path.join(movesDir, 'common.rs');
fs.writeFileSync(commonPath, commonContent);
console.log(`Generated ${commonPath}`);

// Generate MOVES_TODO.md
let todoContent = `# Moves Implementation Tracking

Total: ${moves.length} moves
Moves with callbacks: ${movesWithCallbacks.length}

## Moves with Callbacks (alphabetically)
${movesWithCallbacks.map(m => `- [ ] ${m.id} - ${m.name} (${m.category}, ${m.type}) - ${m.callbacks.length} callback${m.callbacks.length !== 1 ? 's' : ''}: ${m.callbacks.map(cb => cb.name).join(', ')}`).join('\n')}

## Statistics

By callback type:
${(() => {
    const callbackCount = {};
    movesWithCallbacks.forEach(m => {
        m.callbacks.forEach(cb => {
            callbackCount[cb.name] = (callbackCount[cb.name] || 0) + 1;
        });
    });
    return Object.entries(callbackCount)
        .sort((a, b) => b[1] - a[1])
        .map(([cb, count]) => `- ${cb}: ${count} moves`)
        .join('\n');
})()}
`;

const todoPath = path.join(workspaceRoot, 'MOVES_TODO.md');
fs.writeFileSync(todoPath, todoContent);
console.log(`Generated ${todoPath}`);

console.log('\nGeneration complete!');
console.log(`- ${moves.length} moves processed`);
console.log(`- ${movesWithCallbacks.length} moves with callbacks`);
console.log(`- ${generatedCount} individual move files created`);
console.log(`- mod.rs and common.rs created`);
console.log(`- MOVES_TODO.md created`);
