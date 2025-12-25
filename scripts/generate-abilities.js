#!/usr/bin/env node

/**
 * Generate Rust ability stubs from TypeScript abilities
 * Run: docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && node scripts/generate-abilities.js"
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

// Load the abilities from TypeScript file
// In docker, the workspace is at /home/builder/workspace
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');
const abilitiesPath = path.join(workspaceRoot, 'pokemon-showdown-js', 'data', 'abilities.ts');
const abilitiesContent = fs.readFileSync(abilitiesPath, 'utf8');

// Parse abilities - extract each ability definition
const abilityRegex = /^\t([a-z0-9]+): \{([\s\S]*?)^\t\},?$/gm;
const abilities = [];
let match;

while ((match = abilityRegex.exec(abilitiesContent)) !== null) {
    const id = match[1];
    const content = match[2];

    // Extract basic properties
    const nameMatch = content.match(/name:\s*"([^"]+)"/);
    const numMatch = content.match(/num:\s*(\d+)/);
    const ratingMatch = content.match(/rating:\s*([\d.]+)/);
    const flagsMatch = content.match(/flags:\s*\{([^}]*)\}/);

    // Extract all callbacks (functions)
    const callbacks = [];
    const callbackRegex = /(\w+)(?:Priority)?[:\(]/g;
    let callbackMatch;
    const seenCallbacks = new Set();

    while ((callbackMatch = callbackRegex.exec(content)) !== null) {
        const callbackName = callbackMatch[1];
        if (callbackName.startsWith('on') && !seenCallbacks.has(callbackName)) {
            seenCallbacks.add(callbackName);
            callbacks.push(callbackName);
        }
    }

    // Check for condition block
    const hasCondition = content.includes('condition: {');

    abilities.push({
        id,
        name: nameMatch ? nameMatch[1] : id,
        num: numMatch ? parseInt(numMatch[1]) : 0,
        rating: ratingMatch ? parseFloat(ratingMatch[1]) : 0,
        flags: flagsMatch ? flagsMatch[1].trim() : '',
        callbacks,
        hasCondition,
        fullContent: match[0] // Full JS source
    });
}

console.log(`Found ${abilities.length} abilities`);

// Sort abilities alphabetically
abilities.sort((a, b) => a.id.localeCompare(b.id));

// Divide into 4 batches
const batchSize = Math.ceil(abilities.length / 4);
const batches = [
    abilities.slice(0, batchSize),
    abilities.slice(batchSize, batchSize * 2),
    abilities.slice(batchSize * 2, batchSize * 3),
    abilities.slice(batchSize * 3)
];

// Generate ability_callbacks batch files
for (let i = 0; i < 4; i++) {
    const batch = batches[i];
    const batchNum = i + 1;

    let rustCode = `//! Ability Callback Handlers - Batch ${batchNum}
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file contains ability callback implementations for batch ${batchNum}.
//! Generated from data/abilities.ts
//!
//! Abilities in this batch (${batch.length}):
${batch.map(a => `//! - ${a.id}`).join('\n')}

use crate::battle::{Battle, Arg};
use crate::data::moves::MoveDef;
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

`;

    // Generate module for each ability with callbacks
    for (const ability of batch) {
        if (ability.callbacks.length === 0 && !ability.hasCondition) {
            continue; // Skip abilities with no callbacks
        }

        rustCode += `
// -----------------------------------------------------------------------------
// ${ability.id.toUpperCase()}
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
${ability.fullContent.split('\n').map(line => '// ' + line).join('\n')}

pub mod ${rustModName(ability.id)} {
    use super::*;
`;

        // Generate stub for each callback
        for (const callback of ability.callbacks) {
            const rustFuncName = camelToSnake(callback);
            rustCode += `
    /// ${callback}(...)
    pub fn ${rustFuncName}(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
`;
        }

        if (ability.hasCondition) {
            rustCode += `
    // Condition handlers
    pub mod condition {
        use super::*;

        // TODO: Implement condition handlers
    }
`;
        }

        rustCode += `}
`;
    }

    // Write batch file
    const batchPath = path.join(workspaceRoot, 'src', 'data', 'ability_callbacks', `batch_${batchNum}.rs`);
    fs.writeFileSync(batchPath, rustCode);
    console.log(`Generated ${batchPath} with ${batch.length} abilities`);
}

// Generate mod.rs to export all batches
const modContent = `//! Ability Callback Handlers
//!
//! This module re-exports all ability callback implementations.

// Common types
mod common;
pub use common::*;

// Batch modules
mod batch_1;
mod batch_2;
mod batch_3;
mod batch_4;

// Re-export all ability modules
pub use batch_1::*;
pub use batch_2::*;
pub use batch_3::*;
pub use batch_4::*;
`;

const modPath = path.join(workspaceRoot, 'src', 'data', 'ability_callbacks', 'mod.rs');
fs.writeFileSync(modPath, modContent);
console.log(`Generated ${modPath}`);

// Generate common.rs with shared types
const commonContent = `//! Common types for ability callbacks

use crate::dex_data::ID;

/// Result from an ability handler
#[derive(Debug, Clone)]
pub enum AbilityHandlerResult {
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

const commonPath = path.join(workspaceRoot, 'src', 'data', 'ability_callbacks', 'common.rs');
fs.writeFileSync(commonPath, commonContent);
console.log(`Generated ${commonPath}`);

// Generate ABILITIES_TODO.md
let todoContent = `# Abilities Implementation Tracking

Total: ${abilities.length} abilities

## Batch 1 (${batches[0].length} abilities)
${batches[0].map(a => `- [ ] ${a.id} ${a.callbacks.length > 0 ? `(${a.callbacks.length} callbacks)` : '(data only)'}`).join('\n')}

## Batch 2 (${batches[1].length} abilities)
${batches[1].map(a => `- [ ] ${a.id} ${a.callbacks.length > 0 ? `(${a.callbacks.length} callbacks)` : '(data only)'}`).join('\n')}

## Batch 3 (${batches[2].length} abilities)
${batches[2].map(a => `- [ ] ${a.id} ${a.callbacks.length > 0 ? `(${a.callbacks.length} callbacks)` : '(data only)'}`).join('\n')}

## Batch 4 (${batches[3].length} abilities)
${batches[3].map(a => `- [ ] ${a.id} ${a.callbacks.length > 0 ? `(${a.callbacks.length} callbacks)` : '(data only)'}`).join('\n')}
`;

const todoPath = path.join(workspaceRoot, 'ABILITIES_TODO.md');
fs.writeFileSync(todoPath, todoContent);
console.log(`Generated ${todoPath}`);

console.log('\nGeneration complete!');
console.log(`- ${abilities.length} abilities processed`);
console.log(`- 4 batch files created`);
console.log(`- mod.rs and common.rs created`);
console.log(`- ABILITIES_TODO.md created`);