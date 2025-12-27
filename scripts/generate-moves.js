#!/usr/bin/env node

/**
 * Generate Rust move stubs from TypeScript moves
 * Each move with callbacks gets its own file
 * Run: docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && node scripts/generate-moves.js"
 */

const fs = require('fs');
const path = require('path');

// Helper function to generate parameters based on callback name (standardized)
function generateParameters(callbackName, jsArgs) {
    // Always include battle
    let params = ['battle: &mut Battle'];

    // Standard signatures for each callback type
    // This ensures all callbacks of the same type have the same signature
    const standardSignatures = {
        'basePowerCallback': ['pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'damageCallback': ['pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'beforeMoveCallback': ['pokemon_pos: (usize, usize)'],
        'beforeTurnCallback': ['pokemon_pos: (usize, usize)'],
        'priorityChargeCallback': ['pokemon_pos: (usize, usize)'],
        'durationCallback': ['target_pos: Option<(usize, usize)>', 'source_pos: Option<(usize, usize)>', 'effect_id: Option<&str>'],
        // Standard onXxx callbacks
        'onHit': ['pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'onTryHit': ['source_pos: (usize, usize)', 'target_pos: (usize, usize)'],
        'onAfterHit': ['source_pos: (usize, usize)', 'target_pos: (usize, usize)'],
        'onBasePower': ['base_power: i32', 'pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'onModifyMove': ['pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'onTry': ['source_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'onTryMove': ['source_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'onPrepareHit': ['pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'onAfterMove': ['source_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        // onDamage callback - TypeScript: onDamage(damage, target, source, effect)
        'onDamage': ['damage: i32', 'target_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>', 'effect_id: Option<&str>'],
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
            'move': 'move_id: &str',
            'effect': 'effect_id: Option<&str>',
            'damage': 'damage: i32',
            'basePower': 'base_power: i32',
            'accuracy': 'accuracy: i32',
            'status': 'status: Option<&str>',
            'field': 'field_pos: Option<(usize, usize)>',
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

// Load the moves from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');
const showdownDir = process.env.SHOWDOWN_DIR || 'pokemon-showdown';
const movesPath = path.join(path.dirname(workspaceRoot), showdownDir, 'data', 'moves.ts');
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
    // We need to match curly braces carefully to get individual callbacks
    const lines = content.split('\n');
    let i = 0;
    while (i < lines.length) {
        const line = lines[i];
        // Match callback name and opening (both onXxx and xxxCallback patterns)
        const match = line.match(/^\t\t((?:on\w+|\w+Callback))(?:Priority|Order|SubOrder)?\s*\((.*)\)\s*\{/);
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

                // Count braces (simple counting, doesn't handle strings perfectly but works for our case)
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

                // Normalize indentation: remove common leading whitespace
                const bodyLines = body.split('\n').filter(line => line.trim() !== '');
                if (bodyLines.length > 0) {
                    // Find minimum indentation (count leading tabs/spaces)
                    const minIndent = Math.min(...bodyLines.map(line => {
                        const match = line.match(/^(\s*)/);
                        return match ? match[1].length : 0;
                    }));

                    // Remove the common indentation and add one level (4 spaces)
                    const normalizedBody = body.split('\n')
                        .map(line => {
                            if (line.trim() === '') return '';
                            const unindented = line.slice(minIndent);
                            return unindented ? '    ' + unindented : '';
                        })
                        .join('\n')
                        .replace(/\s+$/, ''); // Only trim trailing whitespace, not leading

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

    // Parse condition block callbacks
    const conditionCallbacks = [];
    const conditionMatch = content.match(/condition: \{([\s\S]*?)^\t\t\},?/m);
    if (conditionMatch) {
        const conditionContent = conditionMatch[1];
        const conditionLines = conditionContent.split('\n');
        let j = 0;
        const seenConditionCallbacks = new Set();

        while (j < conditionLines.length) {
            const line = conditionLines[j];
            const match = line.match(/^\t\t\t((?:on\w+|\w+Callback))(?:Priority|Order|SubOrder)?\s*\((.*)\)\s*\{/);
            if (match) {
                const callbackName = match[1];
                const args = match[2];
                let body = '';
                let braceCount = 1;
                j++;

                while (j < conditionLines.length && braceCount > 0) {
                    const currentLine = conditionLines[j];
                    body += currentLine + '\n';

                    for (const char of currentLine) {
                        if (char === '{') braceCount++;
                        if (char === '}') braceCount--;
                    }

                    if (braceCount === 0) {
                        body = body.replace(/^\t\t\t\},?\s*$/m, '');
                        break;
                    }
                    j++;
                }

                if (!seenConditionCallbacks.has(callbackName)) {
                    seenConditionCallbacks.add(callbackName);

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

                        conditionCallbacks.push({
                            name: callbackName,
                            args: args,
                            jsSource: `${callbackName}(${args}) {\n${normalizedBody}\n}`
                        });
                    } else {
                        conditionCallbacks.push({
                            name: callbackName,
                            args: args,
                            jsSource: `${callbackName}(${args}) {}`
                        });
                    }
                }
            }
            j++;
        }
    }

    // Check for secondary effect
    const hasSecondary = content.includes('secondary: {');

    moves.push({
        id,
        name: nameMatch ? nameMatch[1] : id,
        num: numMatch ? parseInt(numMatch[1]) : 0,
        category: categoryMatch ? categoryMatch[1] : 'Unknown',
        type: typeMatch ? typeMatch[1] : 'Unknown',
        callbacks,
        conditionCallbacks,
        hasSecondary
    });
}

console.log(`Found ${moves.length} moves`);

// Filter moves that have callbacks or conditions
const movesWithCallbacks = moves.filter(m => m.callbacks.length > 0 || m.conditionCallbacks.length > 0);
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

use crate::battle::Battle;
use crate::event::EventResult;

${move.callbacks.map(callback => {
    const rustFuncName = camelToSnake(callback.name);
    const params = generateParameters(callback.name, callback.args);
    // Format JS source: replace all tabs with 4 spaces
    const formattedSource = callback.jsSource
        .split('\n')
        .map(line => line.replace(/\t/g, '    ')) // Replace all tabs with 4 spaces
        .map(line => '/// ' + line)
        .join('\n');

    return `${formattedSource}
pub fn ${rustFuncName}(${params}) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
`;
}).join('\n')}
${move.conditionCallbacks.length > 0 ? `pub mod condition {
    use super::*;

${move.conditionCallbacks.map(callback => {
    const rustFuncName = camelToSnake(callback.name);
    const params = generateParameters(callback.name, callback.args);
    const formattedSource = callback.jsSource
        .split('\n')
        .map(line => line.replace(/\t/g, '    '))
        .map(line => '    /// ' + line)
        .join('\n');

    return `${formattedSource}
    pub fn ${rustFuncName}(${params}) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
`;
}).join('\n').trimEnd()}
}
` : ''}`;

    const filePath = path.join(movesDir, `${move.id}.rs`);
    fs.writeFileSync(filePath, rustCode);
    generatedCount++;
}

console.log(`Generated ${generatedCount} move files`);

// Build a map of callback -> moves for dispatcher generation
const callbackMap = new Map();
movesWithCallbacks.forEach(move => {
    move.callbacks.forEach(callback => {
        if (!callbackMap.has(callback.name)) {
            callbackMap.set(callback.name, []);
        }
        callbackMap.get(callback.name).push(move.id);
    });
});

// Build a map of condition callback -> moves for dispatcher generation
const conditionCallbackMap = new Map();
movesWithCallbacks.forEach(move => {
    move.conditionCallbacks.forEach(callback => {
        if (!conditionCallbackMap.has(callback.name)) {
            conditionCallbackMap.set(callback.name, []);
        }
        conditionCallbackMap.get(callback.name).push(move.id);
    });
});

// Generate dispatch functions
const sortedCallbacks = Array.from(callbackMap.keys()).sort();
const dispatchers = sortedCallbacks.map(callback => {
    const funcName = `dispatch_${camelToSnake(callback)}`;
    const rustCallbackName = camelToSnake(callback);
    const moveIds = callbackMap.get(callback);

    // Determine parameters based on callback type - match standardSignatures
    // move_id is ALWAYS the first param after battle for routing, then the callback-specific params
    let params = '';
    let callParams = '';

    if (callback === 'basePowerCallback') {
        params = ',\n    move_id: &str,\n    pokemon_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, pokemon_pos, target_pos';
    } else if (callback === 'damageCallback') {
        params = ',\n    move_id: &str,\n    pokemon_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, pokemon_pos, target_pos';
    } else if (callback === 'beforeMoveCallback' || callback === 'beforeTurnCallback' || callback === 'priorityChargeCallback') {
        params = ',\n    move_id: &str,\n    pokemon_pos: (usize, usize)';
        callParams = 'battle, pokemon_pos';
    } else if (callback === 'onHit') {
        params = ',\n    move_id: &str,\n    pokemon_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, pokemon_pos, target_pos';
    } else if (callback === 'onTryHit' || callback === 'onAfterHit') {
        params = ',\n    move_id: &str,\n    source_pos: (usize, usize),\n    target_pos: (usize, usize)';
        callParams = 'battle, source_pos, target_pos';
    } else if (callback === 'onBasePower') {
        params = ',\n    move_id: &str,\n    base_power: i32,\n    pokemon_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, base_power, pokemon_pos, target_pos';
    } else if (callback === 'onModifyMove') {
        params = ',\n    move_id: &str,\n    pokemon_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, pokemon_pos, target_pos';
    } else if (callback === 'onTry' || callback === 'onTryMove') {
        params = ',\n    move_id: &str,\n    source_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, source_pos, target_pos';
    } else if (callback === 'onPrepareHit' || callback === 'onAfterMove') {
        params = ',\n    move_id: &str,\n    pokemon_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, pokemon_pos, target_pos';
    } else {
        // Default for callbacks not explicitly handled
        params = ',\n    move_id: &str,\n    pokemon_pos: (usize, usize)';
        callParams = 'battle, pokemon_pos';
    }

    const matchArms = moveIds.map(moveId => {
        return `        "${moveId}" => ${rustModName(moveId)}::${rustCallbackName}(${callParams}),`;
    }).join('\n');

    return `/// Dispatch ${callback} callbacks
pub fn ${funcName}(
    battle: &mut Battle${params},
) -> EventResult {
    match move_id {
${matchArms}
        _ => EventResult::Continue,
    }
}`;
}).join('\n\n');

// Generate condition dispatch functions
const sortedConditionCallbacks = Array.from(conditionCallbackMap.keys()).sort();
const conditionDispatchers = sortedConditionCallbacks.map(callback => {
    const funcName = `dispatch_condition_${camelToSnake(callback)}`;
    const rustCallbackName = camelToSnake(callback);
    const moveIds = conditionCallbackMap.get(callback);

    // Determine parameters based on callback type
    let params = '';
    let callParams = '';
    if (callback.includes('Damage') || callback === 'onTryHit' || callback === 'onHit' || callback === 'onAfterHit') {
        params = ',\n    move_id: &str,\n    source_pos: (usize, usize),\n    target_pos: (usize, usize)';
        callParams = 'battle, source_pos, target_pos';
    } else if (callback === 'onTryMove' || callback === 'onModifyType') {
        params = ',\n    move_id: &str,\n    source_pos: (usize, usize)';
        callParams = 'battle, source_pos';
    } else if (callback === 'onBasePower' || callback === 'onAfterMove') {
        params = ',\n    move_id: &str,\n    source_pos: (usize, usize),\n    target_pos: Option<(usize, usize)>';
        callParams = 'battle, source_pos, target_pos';
    } else {
        params = ',\n    move_id: &str,\n    source_pos: (usize, usize)';
        callParams = 'battle, source_pos';
    }

    const matchArms = moveIds.map(moveId => {
        return `        "${moveId}" => ${rustModName(moveId)}::condition::${rustCallbackName}(${callParams}),`;
    }).join('\n');

    return `/// Dispatch condition ${callback} callbacks
pub fn ${funcName}(
    battle: &mut Battle${params},
) -> EventResult {
    match move_id {
${matchArms}
        _ => EventResult::Continue,
    }
}`;
}).join('\n\n');

// Generate mod.rs to export all moves
const modContent = `//! Move Callback Handlers
//!
//! This module exports all move callback implementations.
//! Each move with callbacks is in its own file.

use crate::battle::Battle;
use crate::event::EventResult;

// Individual move modules
${movesWithCallbacks.map(m =>
    `pub mod ${rustModName(m.id)};`
).join('\n')}

// Dispatch functions
${dispatchers}

// Condition dispatch functions
${conditionDispatchers}
`;

const modPath = path.join(movesDir, 'mod.rs');
fs.writeFileSync(modPath, modContent);
console.log(`Generated ${modPath}`);

// Generate MOVES_TODO.md
let todoContent = `# Moves Implementation Tracking

Total: ${moves.length} moves
Moves with callbacks: ${movesWithCallbacks.length}

## Moves with Callbacks (alphabetically)
${movesWithCallbacks.map(m => {
    const totalCallbacks = m.callbacks.length + m.conditionCallbacks.length;
    const allCallbacks = [
        ...m.callbacks.map(cb => cb.name),
        ...m.conditionCallbacks.map(cb => `condition::${cb.name}`)
    ];
    return `- [ ] ${m.id} - ${m.name} (${m.category}, ${m.type}) - ${totalCallbacks} callback${totalCallbacks !== 1 ? 's' : ''}: ${allCallbacks.join(', ')}`;
}).join('\n')}

## Statistics

By callback type:
${(() => {
    const callbackCount = {};
    movesWithCallbacks.forEach(m => {
        m.callbacks.forEach(cb => {
            callbackCount[cb.name] = (callbackCount[cb.name] || 0) + 1;
        });
        m.conditionCallbacks.forEach(cb => {
            const key = `condition::${cb.name}`;
            callbackCount[key] = (callbackCount[key] || 0) + 1;
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
console.log(`- mod.rs created`);
console.log(`- MOVES_TODO.md created`);
