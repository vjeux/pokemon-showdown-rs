#!/usr/bin/env node

/**
 * Analyze item callbacks to identify priority/order properties vs actual callbacks
 * Similar analysis for items that we already ran for moves and abilities
 */

const fs = require('fs');
const path = require('path');

// Load the actual Items object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Items;
try {
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Items = Dex.data.Items;
    console.log('Loaded items from dist/sim');
} catch (e) {
    console.error('Error loading items:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

console.log('\nAnalyzing item callbacks...\n');

// Collect all unique callbacks
const callbackToItems = {};
const priorityProperties = new Set();
const nonFunctionProperties = {};

for (const itemId in Items) {
    if (!Items.hasOwnProperty(itemId)) continue;

    const itemData = Items[itemId];

    for (const key in itemData) {
        if (!itemData.hasOwnProperty(key)) continue;

        // Check if it starts with 'on' - typical callback naming
        if (key.startsWith('on')) {
            const value = itemData[key];
            const valueType = typeof value;

            if (valueType === 'function') {
                // This is a real callback function
                if (!callbackToItems[key]) {
                    callbackToItems[key] = [];
                }
                callbackToItems[key].push(itemId);
            } else {
                // This is NOT a function - it's metadata
                if (!nonFunctionProperties[key]) {
                    nonFunctionProperties[key] = [];
                }
                nonFunctionProperties[key].push({
                    item: itemId,
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
console.log(`Found ${Object.keys(callbackToItems).length} unique callback functions:\n`);
Object.keys(callbackToItems).sort().forEach(callback => {
    console.log(`  ${callback}: ${callbackToItems[callback].length} items`);
});

console.log('\n=== NON-FUNCTION PROPERTIES (should NOT have dispatchers) ===');
if (Object.keys(nonFunctionProperties).length > 0) {
    console.log('Found properties starting with "on" that are NOT functions:\n');
    Object.keys(nonFunctionProperties).sort().forEach(prop => {
        const examples = nonFunctionProperties[prop].slice(0, 3);
        console.log(`  ${prop}: ${nonFunctionProperties[prop].length} occurrences`);
        examples.forEach(ex => {
            console.log(`    - ${ex.item}: ${JSON.stringify(ex.value)} (${ex.type})`);
        });
    });

    console.log('\n⚠ These properties should NOT have dispatchers!');
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

// Now check item_callbacks/mod.rs for stub dispatchers
const modRsPath = path.join(__dirname, '../src/data/item_callbacks/mod.rs');
if (fs.existsSync(modRsPath)) {
    const modRsContent = fs.readFileSync(modRsPath, 'utf8');

    console.log('\n=== CHECKING item_callbacks/mod.rs ===');

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
