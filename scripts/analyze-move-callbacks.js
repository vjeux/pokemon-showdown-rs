#!/usr/bin/env node

/**
 * Analyze move callbacks to identify priority/order properties vs actual callbacks
 * Similar to the item callback analysis we did
 */

const fs = require('fs');
const path = require('path');

// Load the actual Moves object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Moves;
try {
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Moves = Dex.data.Moves;
    console.log('Loaded moves from dist/sim');
} catch (e) {
    console.error('Error loading moves:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

console.log('\nAnalyzing move callbacks...\n');

// Collect all unique callbacks
const callbackToMoves = {};
const priorityProperties = new Set();
const nonFunctionProperties = {};

for (const moveId in Moves) {
    if (!Moves.hasOwnProperty(moveId)) continue;

    const moveData = Moves[moveId];

    for (const key in moveData) {
        if (!moveData.hasOwnProperty(key)) continue;

        // Check if it starts with 'on' - typical callback naming
        if (key.startsWith('on')) {
            const value = moveData[key];
            const valueType = typeof value;

            if (valueType === 'function') {
                // This is a real callback function
                if (!callbackToMoves[key]) {
                    callbackToMoves[key] = [];
                }
                callbackToMoves[key].push(moveId);
            } else {
                // This is NOT a function - it's metadata
                if (!nonFunctionProperties[key]) {
                    nonFunctionProperties[key] = [];
                }
                nonFunctionProperties[key].push({
                    move: moveId,
                    value: value,
                    type: valueType
                });

                // Track priority/order properties
                if (key.endsWith('Priority') || key.endsWith('Order') || key.endsWith('SubOrder')) {
                    priorityProperties.add(key);
                }
            }
        }
    }
}

console.log('=== REAL CALLBACK FUNCTIONS ===');
console.log(`Found ${Object.keys(callbackToMoves).length} unique callback functions:\n`);
Object.keys(callbackToMoves).sort().forEach(callback => {
    console.log(`  ${callback}: ${callbackToMoves[callback].length} moves`);
});

console.log('\n=== NON-FUNCTION PROPERTIES (should NOT have dispatchers) ===');
if (Object.keys(nonFunctionProperties).length > 0) {
    console.log('Found properties starting with "on" that are NOT functions:\n');
    Object.keys(nonFunctionProperties).sort().forEach(prop => {
        const examples = nonFunctionProperties[prop].slice(0, 3);
        console.log(`  ${prop}: ${nonFunctionProperties[prop].length} occurrences`);
        examples.forEach(ex => {
            console.log(`    - ${ex.move}: ${JSON.stringify(ex.value)} (${ex.type})`);
        });
    });
} else {
    console.log('✓ No non-function properties starting with "on" found!');
}

console.log('\n=== PRIORITY/ORDER PROPERTIES ===');
if (priorityProperties.size > 0) {
    console.log('Properties ending with Priority/Order/SubOrder:\n');
    Array.from(priorityProperties).sort().forEach(prop => {
        console.log(`  ${prop}`);
    });
} else {
    console.log('✓ No priority/order properties found!');
}

// Now check move_callbacks/mod.rs for stub dispatchers
const modRsPath = path.join(__dirname, '../src/data/move_callbacks/mod.rs');
if (fs.existsSync(modRsPath)) {
    const modRsContent = fs.readFileSync(modRsPath, 'utf8');

    console.log('\n=== CHECKING move_callbacks/mod.rs ===');

    // Check for any dispatchers for non-function properties
    const issues = [];
    for (const prop in nonFunctionProperties) {
        const funcName = toSnakeCase(prop);
        const pattern = new RegExp(`dispatch_${funcName}`, 'g');
        if (pattern.test(modRsContent)) {
            issues.push(prop);
        }
    }

    if (issues.length > 0) {
        console.log('⚠ Found dispatchers for non-function properties:');
        issues.forEach(prop => console.log(`  - ${prop}`));
    } else {
        console.log('✓ No dispatchers found for non-function properties!');
    }
}

function toSnakeCase(str) {
    return str
        .replace(/^on/, '')
        .replace(/([A-Z])/g, '_$1')
        .toLowerCase()
        .replace(/^_/, '');
}

console.log('\n✓ Analysis complete!');
