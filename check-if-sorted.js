#!/usr/bin/env node

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');

// Get items WITHOUT sorting
const allItems = Object.values(dex.items.all()).map(i => i.id);

// Check if they're already sorted
const sortedItems = [...allItems].sort();
const isSorted = JSON.stringify(allItems) === JSON.stringify(sortedItems);

console.log('Items are already sorted:', isSorted);

if (!isSorted) {
    console.log('\nFirst difference:');
    for (let i = 0; i < allItems.length; i++) {
        if (allItems[i] !== sortedItems[i]) {
            console.log(`Index ${i}: unsorted="${allItems[i]}" sorted="${sortedItems[i]}"`);
            break;
        }
    }
}
