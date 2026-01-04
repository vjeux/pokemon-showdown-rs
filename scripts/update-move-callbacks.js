#!/usr/bin/env node

/**
 * Update moves.json with callback information
 *
 * This script:
 * 1. Loads data/moves.ts from JavaScript by requiring it
 * 2. Introspects each move to find which properties are functions
 * 3. Updates moves.json with callback boolean flags
 */

const fs = require('fs');
const path = require('path');

// Load the actual Moves object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Moves;
try {
    // Try to require the compiled moves from dist
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Moves = Dex.data.Moves;
    console.log('Loaded moves from dist/sim');
} catch (e) {
    console.error('Error loading moves:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

// Extract callbacks from a move object
function extractCallbacks(moveId, moveData) {
    const callbacks = [];

    // Iterate through all properties of the move
    for (const key in moveData) {
        // Skip if not own property
        if (!moveData.hasOwnProperty(key)) continue;

        // Check if it's a function - that's the only requirement for a callback
        if (typeof moveData[key] === 'function') {
            callbacks.push(key);
        }
    }

    return callbacks;
}

// Parse all moves
console.log('Extracting callbacks from moves...');
const moves = {};

for (const moveId in Moves) {
    if (!Moves.hasOwnProperty(moveId)) continue;

    const moveData = Moves[moveId];
    const callbacks = extractCallbacks(moveId, moveData);

    if (callbacks.length > 0) {
        moves[moveId] = {
            callbacks,
            data: moveData
        };
    }
}

console.log(`\nFound ${Object.keys(moves).length} moves with callbacks:`);
Object.entries(moves).forEach(([id, data]) => {
    console.log(`  ${id}: ${data.callbacks.join(', ')}`);
});

// Update moves.json with callback information
console.log('\nUpdating moves.json with callback information...');
const movesJsonPath = path.join(__dirname, '../data/moves.json');
let movesJson = {};

// Load existing moves.json
if (fs.existsSync(movesJsonPath)) {
    movesJson = JSON.parse(fs.readFileSync(movesJsonPath, 'utf8'));
}

// Add callback flags for each move
for (const moveId in Moves) {
    if (!Moves.hasOwnProperty(moveId)) continue;

    const moveData = Moves[moveId];
    const callbacks = extractCallbacks(moveId, moveData);

    if (!movesJson[moveId]) {
        movesJson[moveId] = {};
    }

    // Add callback boolean flags
    callbacks.forEach(callback => {
        movesJson[moveId][callback] = true;
    });

    // Preserve existing metadata (name, num, type, category, etc.)
    if (moveData.name) {
        movesJson[moveId].name = moveData.name;
    }
    if (moveData.num !== undefined) {
        movesJson[moveId].num = moveData.num;
    }
    if (moveData.type) {
        movesJson[moveId].type = moveData.type;
    }
    if (moveData.category) {
        movesJson[moveId].category = moveData.category;
    }
    if (moveData.basePower !== undefined) {
        movesJson[moveId].basePower = moveData.basePower;
    }
    if (moveData.accuracy !== undefined) {
        movesJson[moveId].accuracy = moveData.accuracy;
    }
    if (moveData.pp !== undefined) {
        movesJson[moveId].pp = moveData.pp;
    }
    if (moveData.priority !== undefined) {
        movesJson[moveId].priority = moveData.priority;
    }
    if (moveData.flags !== undefined) {
        movesJson[moveId].flags = moveData.flags;
    }
    if (moveData.secondary !== undefined) {
        movesJson[moveId].secondary = moveData.secondary;
    }
    if (moveData.secondaries !== undefined) {
        movesJson[moveId].secondaries = moveData.secondaries;
    }
    if (moveData.target !== undefined) {
        movesJson[moveId].target = moveData.target;
    }
    if (moveData.isNonstandard !== undefined) {
        movesJson[moveId].isNonstandard = moveData.isNonstandard;
    }
    if (moveData.isZ !== undefined) {
        movesJson[moveId].isZ = moveData.isZ;
    }
    if (moveData.isMax !== undefined) {
        movesJson[moveId].isMax = moveData.isMax;
    }
    if (moveData.zMove !== undefined) {
        movesJson[moveId].zMove = moveData.zMove;
    }
    if (moveData.maxMove !== undefined) {
        movesJson[moveId].maxMove = moveData.maxMove;
    }

    // Preserve priority/order/suborder metadata
    for (const key in moveData) {
        if (key.endsWith('Priority') || key.endsWith('Order') || key.endsWith('SubOrder')) {
            movesJson[moveId][key] = moveData[key];
        }
    }
}

// Write updated moves.json
fs.writeFileSync(movesJsonPath, JSON.stringify(movesJson, null, 2));
console.log(`  Updated ${movesJsonPath} with callback flags`);

console.log('\n✓ Done! Updated moves.json with callback information.');
