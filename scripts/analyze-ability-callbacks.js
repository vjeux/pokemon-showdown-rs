#!/usr/bin/env node

/**
 * Analyze ability callbacks to identify priority/order properties vs actual callbacks
 */

const fs = require('fs');
const path = require('path');

// Load the actual Abilities object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Abilities;
try {
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Abilities = Dex.data.Abilities;
    console.log('Loaded abilities from dist/sim');
} catch (e) {
    console.error('Error loading abilities:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

console.log('\nAnalyzing ability callbacks...\n');

// Collect all unique callbacks
const callbackToAbilities = {};
const priorityProperties = new Set();
const nonFunctionProperties = {};

for (const abilityId in Abilities) {
    if (!Abilities.hasOwnProperty(abilityId)) continue;

    const abilityData = Abilities[abilityId];

    for (const key in abilityData) {
        if (!abilityData.hasOwnProperty(key)) continue;

        // Check if it starts with 'on' - typical callback naming
        if (key.startsWith('on')) {
            const value = abilityData[key];
            const valueType = typeof value;

            if (valueType === 'function') {
                // This is a real callback function
                if (!callbackToAbilities[key]) {
                    callbackToAbilities[key] = [];
                }
                callbackToAbilities[key].push(abilityId);
            } else {
                // This is NOT a function - it's metadata
                if (!nonFunctionProperties[key]) {
                    nonFunctionProperties[key] = [];
                }
                nonFunctionProperties[key].push({
                    ability: abilityId,
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
console.log(`Found ${Object.keys(callbackToAbilities).length} unique callback functions:\n`);
Object.keys(callbackToAbilities).sort().forEach(callback => {
    console.log(`  ${callback}: ${callbackToAbilities[callback].length} abilities`);
});

console.log('\n=== NON-FUNCTION PROPERTIES (should NOT have dispatchers) ===');
if (Object.keys(nonFunctionProperties).length > 0) {
    console.log('Found properties starting with "on" that are NOT functions:\n');
    Object.keys(nonFunctionProperties).sort().forEach(prop => {
        const examples = nonFunctionProperties[prop].slice(0, 3);
        console.log(`  ${prop}: ${nonFunctionProperties[prop].length} occurrences`);
        examples.forEach(ex => {
            console.log(`    - ${ex.ability}: ${JSON.stringify(ex.value)} (${ex.type})`);
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

// Now check ability_callbacks/mod.rs for stub dispatchers
const modRsPath = path.join(__dirname, '../src/data/ability_callbacks/mod.rs');
if (fs.existsSync(modRsPath)) {
    const modRsContent = fs.readFileSync(modRsPath, 'utf8');

    console.log('\n=== CHECKING ability_callbacks/mod.rs ===');

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
