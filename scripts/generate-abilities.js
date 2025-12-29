#!/usr/bin/env node

/**
 * Generate Rust ability stubs from TypeScript abilities
 * Each ability with callbacks gets its own file
 * Run: docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && node scripts/generate-abilities.js"
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
        // Damage modification
        'onSourceModifyDamage': ['damage: i32', 'source_pos: (usize, usize)', 'target_pos: (usize, usize)', 'move_id: &str'],
        'onDamage': ['damage: i32', 'target_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>', 'effect_id: Option<&str>'],
        'onModifyDamage': ['damage: i32', 'source_pos: (usize, usize)', 'target_pos: (usize, usize)', 'move_id: &str'],

        // Type/effectiveness modifications
        'onEffectiveness': ['damage: i32', 'target_pos: (usize, usize)', 'type_str: &str', 'move_id: &str'],
        'onModifyTypePriority': ['move_id: &str', 'pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],
        'onModifyType': ['move_id: &str', 'pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],

        // Accuracy
        'onSourceModifyAccuracyPriority': ['accuracy: i32', 'target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],
        'onSourceModifyAccuracy': ['accuracy: i32', 'target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],
        'onModifyAccuracy': ['accuracy: i32', 'target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],

        // Stats
        'onModifyAtk': ['atk: i32', 'attacker_pos: (usize, usize)', 'defender_pos: (usize, usize)', 'move_id: &str'],
        'onModifyDef': ['def: i32', 'defender_pos: (usize, usize)', 'attacker_pos: (usize, usize)', 'move_id: &str'],
        'onModifySpA': ['spa: i32', 'attacker_pos: (usize, usize)', 'defender_pos: (usize, usize)', 'move_id: &str'],
        'onModifySpD': ['spd: i32', 'defender_pos: (usize, usize)', 'attacker_pos: (usize, usize)', 'move_id: &str'],
        'onModifySpe': ['spe: i32', 'pokemon_pos: (usize, usize)'],
        'onModifyWeight': ['weight: i32', 'pokemon_pos: (usize, usize)'],
        'onModifyCritRatio': ['crit_ratio: i32', 'source_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>'],

        // Move priority
        'onModifyPriority': ['priority: i32', 'pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>', 'move_id: &str'],

        // Hit events
        'onTryHit': ['target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],
        'onTryHitPriority': ['target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],
        'onHit': ['target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],
        'onAfterMoveSecondarySelf': ['source_pos: (usize, usize)', 'target_pos: (usize, usize)', 'move_id: &str'],
        'onAfterMoveSecondary': ['target_pos: (usize, usize)', 'source_pos: (usize, usize)', 'move_id: &str'],

        // Turn events
        'onStart': ['pokemon_pos: (usize, usize)'],
        'onEnd': ['pokemon_pos: (usize, usize)'],
        'onBeforeTurn': ['pokemon_pos: (usize, usize)'],
        'onAfterTurn': ['pokemon_pos: (usize, usize)'],
        'onBeforeMove': ['pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>', 'move_id: &str'],
        'onAfterMove': ['source_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>', 'move_id: &str'],

        // Switch events
        'onSwitchIn': ['pokemon_pos: (usize, usize)'],
        'onSwitchOut': ['pokemon_pos: (usize, usize)'],

        // Status
        'onSetStatus': ['status_id: &str', 'target_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>', 'effect_id: Option<&str>'],
        'onTryAddVolatile': ['status_id: &str', 'target_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>', 'effect_id: Option<&str>'],

        // Immunity
        'onTryBoost': ['boost: &str', 'target_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>', 'effect_id: Option<&str>'],
        'onImmunity': ['type_or_status: &str', 'pokemon_pos: (usize, usize)'],

        // Weather/Terrain
        'onWeather': ['weather_id: &str', 'pokemon_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>'],
        'onWeatherModifyDamage': ['damage: i32', 'attacker_pos: (usize, usize)', 'defender_pos: (usize, usize)', 'move_id: &str'],
        'onTerrainStart': ['terrain_id: &str', 'pokemon_pos: (usize, usize)'],

        // Residual
        'onResidual': ['pokemon_pos: (usize, usize)'],
        'onResidualOrder': ['pokemon_pos: (usize, usize)'],
        'onResidualSubOrder': ['pokemon_pos: (usize, usize)'],

        // Misc
        'onFaint': ['pokemon_pos: (usize, usize)'],
        'onSourceFaint': ['source_pos: (usize, usize)', 'target_pos: (usize, usize)', 'effect_id: Option<&str>'],
        'onBasePower': ['base_power: i32', 'attacker_pos: (usize, usize)', 'defender_pos: (usize, usize)', 'move_id: &str'],
        'onFoeTrapPokemon': ['pokemon_pos: (usize, usize)'],
        'onFoeBeforeMove': ['pokemon_pos: (usize, usize)', 'target_pos: Option<(usize, usize)>', 'move_id: &str'],
        'onAnyModifyBoost': ['boosts: &str', 'pokemon_pos: (usize, usize)'],
        'onAllySetStatus': ['status_id: &str', 'target_pos: (usize, usize)', 'source_pos: Option<(usize, usize)>', 'effect_id: Option<&str>'],
        'onUpdate': ['pokemon_pos: (usize, usize)'],
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

// Load the abilities from TypeScript file
const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');
const showdownDir = process.env.SHOWDOWN_DIR || 'pokemon-showdown';
const abilitiesPath = path.join(path.dirname(workspaceRoot), showdownDir, 'data', 'abilities.ts');
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

    // Extract all callbacks (functions) with their JS source
    const callbacks = [];
    const seenCallbacks = new Set();

    // Match callback functions with their implementations
    const lines = content.split('\n');
    let i = 0;
    while (i < lines.length) {
        const line = lines[i];
        // Match callback name and opening (both onXxx and xxxCallback patterns)
        // Keep Priority, Order, and SubOrder suffixes as part of the callback name
        const match = line.match(/^\t\t((?:on\w+|\w+Callback)(?:Priority|Order|SubOrder)?)\s*\((.*)\)\s*\{/);
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
            const match = line.match(/^\t\t\t((?:on\w+|\w+Callback)(?:Priority|Order|SubOrder)?)\s*\((.*)\)\s*\{/);
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

    abilities.push({
        id,
        name: nameMatch ? nameMatch[1] : id,
        num: numMatch ? parseInt(numMatch[1]) : 0,
        rating: ratingMatch ? parseFloat(ratingMatch[1]) : 0,
        callbacks,
        conditionCallbacks
    });
}

console.log(`Found ${abilities.length} abilities`);

// Filter abilities that have callbacks or conditions
const abilitiesWithCallbacks = abilities.filter(a => a.callbacks.length > 0 || a.conditionCallbacks.length > 0);
console.log(`Found ${abilitiesWithCallbacks.length} abilities with callbacks or conditions`);

// Sort abilities alphabetically
abilitiesWithCallbacks.sort((a, b) => a.id.localeCompare(b.id));

// Create ability_callbacks directory if it doesn't exist
const abilitiesDir = path.join(workspaceRoot, 'src', 'data', 'ability_callbacks');
if (!fs.existsSync(abilitiesDir)) {
    fs.mkdirSync(abilitiesDir, { recursive: true });
}

// Generate individual file for each ability with callbacks
let generatedCount = 0;
for (const ability of abilitiesWithCallbacks) {
    const rustCode = `//! ${ability.name} Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

${ability.callbacks.map(callback => {
    const rustFuncName = camelToSnake(callback.name);
    // All ability callbacks use standard signature
    const params = 'battle: &mut Battle, pokemon_pos: (usize, usize)';
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
}).join('\n')}
${ability.conditionCallbacks.length > 0 ? `pub mod condition {
    use super::*;

${ability.conditionCallbacks.map(callback => {
    const rustFuncName = camelToSnake(callback.name);
    // All ability callbacks use standard signature
    const params = 'battle: &mut Battle, pokemon_pos: (usize, usize)';
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

    const filePath = path.join(abilitiesDir, `${ability.id}.rs`);
    fs.writeFileSync(filePath, rustCode);
    generatedCount++;
}

console.log(`Generated ${generatedCount} ability files`);

// Build a map of callback -> abilities for dispatcher generation
const callbackMap = new Map();
abilitiesWithCallbacks.forEach(ability => {
    ability.callbacks.forEach(callback => {
        if (!callbackMap.has(callback.name)) {
            callbackMap.set(callback.name, []);
        }
        callbackMap.get(callback.name).push(ability.id);
    });
});

// Build a map of condition callback -> abilities for dispatcher generation
const conditionCallbackMap = new Map();
abilitiesWithCallbacks.forEach(ability => {
    ability.conditionCallbacks.forEach(callback => {
        if (!conditionCallbackMap.has(callback.name)) {
            conditionCallbackMap.set(callback.name, []);
        }
        conditionCallbackMap.get(callback.name).push(ability.id);
    });
});

// Generate dispatch functions
const sortedCallbacks = Array.from(callbackMap.keys()).sort();
const dispatchers = sortedCallbacks.map(callback => {
    const funcName = `dispatch_${camelToSnake(callback)}`;
    const rustCallbackName = camelToSnake(callback);
    const abilityIds = callbackMap.get(callback);

    // All ability dispatchers use a standard signature
    const matchArms = abilityIds.map(abilityId => {
        return `        "${abilityId}" => ${rustModName(abilityId)}::${rustCallbackName}(battle, pokemon_pos),`;
    }).join('\n');

    return `/// Dispatch ${callback} callbacks
pub fn ${funcName}(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
${matchArms}
        _ => EventResult::Continue,
    }
}`;
}).join('\n\n');

// Generate Priority/Order/SubOrder variant dispatchers
// These are aliases that call the base dispatcher with the same signature
const priorityVariantDispatchers = sortedCallbacks.flatMap(callback => {
    const variants = [];
    const baseFuncName = `dispatch_${camelToSnake(callback)}`;

    // Always generate Priority, Order, and SubOrder variants
    // (even for callbacks that already end with these suffixes)
    const priorityCallback = callback + 'Priority';
    const priorityFuncName = `dispatch_${camelToSnake(priorityCallback)}`;

    variants.push(`/// Dispatch ${priorityCallback} callbacks (alias for ${callback})
pub fn ${priorityFuncName}(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    ${baseFuncName}(battle, ability_id, pokemon_pos)
}`);

    // Also generate Order and SubOrder variants
    const orderCallback = callback + 'Order';
    const orderFuncName = `dispatch_${camelToSnake(orderCallback)}`;

    variants.push(`/// Dispatch ${orderCallback} callbacks (alias for ${callback})
pub fn ${orderFuncName}(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    ${baseFuncName}(battle, ability_id, pokemon_pos)
}`);

    const subOrderCallback = callback + 'SubOrder';
    const subOrderFuncName = `dispatch_${camelToSnake(subOrderCallback)}`;

    variants.push(`/// Dispatch ${subOrderCallback} callbacks (alias for ${callback})
pub fn ${subOrderFuncName}(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    ${baseFuncName}(battle, ability_id, pokemon_pos)
}`);

    return variants;
}).join('\n\n');

// Generate condition dispatch functions
const sortedConditionCallbacks = Array.from(conditionCallbackMap.keys()).sort();
const conditionDispatchers = sortedConditionCallbacks.map(callback => {
    const funcName = `dispatch_condition_${camelToSnake(callback)}`;
    const rustCallbackName = camelToSnake(callback);
    const abilityIds = conditionCallbackMap.get(callback);

    // Determine parameters based on callback type
    let params = ',\n    ability_id: &str,\n    pokemon_pos: (usize, usize)';
    let callParams = 'battle, pokemon_pos';

    const matchArms = abilityIds.map(abilityId => {
        return `        "${abilityId}" => ${rustModName(abilityId)}::condition::${rustCallbackName}(${callParams}),`;
    }).join('\n');

    return `/// Dispatch condition ${callback} callbacks
pub fn ${funcName}(
    battle: &mut Battle${params},
) -> EventResult {
    match ability_id {
${matchArms}
        _ => EventResult::Continue,
    }
}`;
}).join('\n\n');

// Generate mod.rs to export all abilities
const modContent = `//! Ability Callback Handlers
//!
//! This module exports all ability callback implementations.
//! Each ability with callbacks is in its own file.

use crate::battle::Battle;
use crate::event::EventResult;

// Individual ability modules
${abilitiesWithCallbacks.map(a =>
    `pub mod ${rustModName(a.id)};`
).join('\n')}

// Dispatch functions
${dispatchers}

// Priority/Order/SubOrder variant dispatchers (aliases)
${priorityVariantDispatchers}

// Condition dispatch functions
${conditionDispatchers}
`;

const modPath = path.join(abilitiesDir, 'mod.rs');
fs.writeFileSync(modPath, modContent);
console.log(`Generated ${modPath}`);

// Generate ABILITIES_TODO.md
let todoContent = `# Abilities Implementation Tracking

Total: ${abilities.length} abilities
Abilities with callbacks: ${abilitiesWithCallbacks.length}

## Abilities with Callbacks (alphabetically)
${abilitiesWithCallbacks.map(a => {
    const totalCallbacks = a.callbacks.length + a.conditionCallbacks.length;
    const allCallbacks = [
        ...a.callbacks.map(cb => cb.name),
        ...a.conditionCallbacks.map(cb => `condition::${cb.name}`)
    ];
    return `- [ ] ${a.id} - ${a.name} - ${totalCallbacks} callback${totalCallbacks !== 1 ? 's' : ''}: ${allCallbacks.join(', ')}`;
}).join('\n')}

## Statistics

By callback type:
${(() => {
    const callbackCount = {};
    abilitiesWithCallbacks.forEach(a => {
        a.callbacks.forEach(cb => {
            callbackCount[cb.name] = (callbackCount[cb.name] || 0) + 1;
        });
        a.conditionCallbacks.forEach(cb => {
            const key = `condition::${cb.name}`;
            callbackCount[key] = (callbackCount[key] || 0) + 1;
        });
    });
    return Object.entries(callbackCount)
        .sort((a, b) => b[1] - a[1])
        .map(([cb, count]) => `- ${cb}: ${count} abilities`)
        .join('\n');
})()}
`;

const todoPath = path.join(workspaceRoot, 'ABILITIES_TODO.md');
fs.writeFileSync(todoPath, todoContent);
console.log(`Generated ${todoPath}`);

console.log('\nGeneration complete!');
console.log(`- ${abilities.length} abilities processed`);
console.log(`- ${abilitiesWithCallbacks.length} abilities with callbacks`);
console.log(`- ${generatedCount} individual ability files created`);
console.log(`- mod.rs created`);
console.log(`- ABILITIES_TODO.md created`);
