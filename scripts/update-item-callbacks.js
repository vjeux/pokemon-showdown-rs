#!/usr/bin/env node

/**
 * Update items.json with callback information
 *
 * This script:
 * 1. Loads data/items.ts from JavaScript by requiring it
 * 2. Introspects each item to find which properties are functions
 * 3. Updates items.json with callback boolean flags
 */

const fs = require('fs');
const path = require('path');

// Load the actual Items object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Items;
try {
    // Try to require the compiled items from dist
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Items = Dex.data.Items;
    console.log('Loaded items from dist/sim');
} catch (e) {
    console.error('Error loading items:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

// Extract callbacks from an item object
function extractCallbacks(itemId, itemData) {
    const callbacks = [];

    // Iterate through all properties of the item
    for (const key in itemData) {
        // Skip if not own property
        if (!itemData.hasOwnProperty(key)) continue;

        // Skip non-callback properties
        if (!key.startsWith('on')) continue;

        // Skip priority/order/suborder metadata (they end with Priority, Order, or SubOrder)
        if (key.endsWith('Priority') || key.endsWith('Order') || key.endsWith('SubOrder')) {
            continue;
        }

        // Check if it's a function
        if (typeof itemData[key] === 'function') {
            callbacks.push(key);
        }
    }

    return callbacks;
}

// Parse all items
console.log('Extracting callbacks from items...');
const items = {};

for (const itemId in Items) {
    if (!Items.hasOwnProperty(itemId)) continue;

    const itemData = Items[itemId];
    const callbacks = extractCallbacks(itemId, itemData);

    if (callbacks.length > 0) {
        items[itemId] = {
            callbacks,
            data: itemData
        };
    }
}

console.log(`\nFound ${Object.keys(items).length} items with callbacks:`);
Object.entries(items).forEach(([id, data]) => {
    console.log(`  ${id}: ${data.callbacks.join(', ')}`);
});

// Update items.json with callback information
console.log('\nUpdating items.json with callback information...');
const itemsJsonPath = path.join(__dirname, '../data/items.json');
let itemsJson = {};

// Load existing items.json
if (fs.existsSync(itemsJsonPath)) {
    itemsJson = JSON.parse(fs.readFileSync(itemsJsonPath, 'utf8'));
}

// Add callback flags for each item
for (const itemId in Items) {
    if (!Items.hasOwnProperty(itemId)) continue;

    const itemData = Items[itemId];
    const callbacks = extractCallbacks(itemId, itemData);

    if (!itemsJson[itemId]) {
        itemsJson[itemId] = {};
    }

    // Add callback boolean flags
    callbacks.forEach(callback => {
        itemsJson[itemId][callback] = true;
    });

    // Also preserve existing metadata (name, num, gen, etc.)
    if (itemData.name) {
        itemsJson[itemId].name = itemData.name;
    } else {
        itemsJson[itemId].name = itemId;
    }

    if (itemData.num !== undefined) {
        itemsJson[itemId].num = itemData.num;
    }

    if (itemData.gen !== undefined) {
        itemsJson[itemId].gen = itemData.gen;
    }

    // Preserve priority/order/suborder metadata
    for (const key in itemData) {
        if (key.endsWith('Priority') || key.endsWith('Order') || key.endsWith('SubOrder')) {
            itemsJson[itemId][key] = itemData[key];
        }
    }

    // Preserve other important metadata
    if (itemData.spritenum !== undefined) {
        itemsJson[itemId].spritenum = itemData.spritenum;
    }
    if (itemData.megaStone !== undefined) {
        itemsJson[itemId].megaStone = itemData.megaStone;
    }
    if (itemData.megaEvolves !== undefined) {
        itemsJson[itemId].megaEvolves = itemData.megaEvolves;
    }
    if (itemData.itemUser !== undefined) {
        itemsJson[itemId].itemUser = itemData.itemUser;
    }
    if (itemData.isNonstandard !== undefined) {
        itemsJson[itemId].isNonstandard = itemData.isNonstandard;
    }
    if (itemData.fling !== undefined) {
        itemsJson[itemId].fling = itemData.fling;
    }
    if (itemData.ignoreKlutz !== undefined) {
        itemsJson[itemId].ignoreKlutz = itemData.ignoreKlutz;
    }
    if (itemData.naturalGift !== undefined) {
        itemsJson[itemId].naturalGift = itemData.naturalGift;
    }
    if (itemData.zMove !== undefined) {
        itemsJson[itemId].zMove = itemData.zMove;
    }
    if (itemData.zMoveType !== undefined) {
        itemsJson[itemId].zMoveType = itemData.zMoveType;
    }
    if (itemData.zMoveFrom !== undefined) {
        itemsJson[itemId].zMoveFrom = itemData.zMoveFrom;
    }
    if (itemData.forcedForme !== undefined) {
        itemsJson[itemId].forcedForme = itemData.forcedForme;
    }
}

// Write updated items.json
fs.writeFileSync(itemsJsonPath, JSON.stringify(itemsJson, null, 2));
console.log(`  Updated ${itemsJsonPath} with callback flags`);

console.log('\n✓ Done! Updated items.json with callback information.');
