#!/usr/bin/env node

/**
 * Update abilities.json with callback information
 *
 * This script:
 * 1. Loads data/abilities.ts from JavaScript by requiring it
 * 2. Introspects each ability to find which properties are functions
 * 3. Updates abilities.json with callback boolean flags
 */

const fs = require('fs');
const path = require('path');

// Load the actual Abilities object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Abilities;
try {
    // Try to require the compiled abilities from dist
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Abilities = Dex.data.Abilities;
    console.log('Loaded abilities from dist/sim');
} catch (e) {
    console.error('Error loading abilities:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

// Extract callbacks from an ability object
function extractCallbacks(abilityId, abilityData) {
    const callbacks = [];

    // Iterate through all properties of the ability
    for (const key in abilityData) {
        // Skip if not own property
        if (!abilityData.hasOwnProperty(key)) continue;

        // Check if it's a function - that's the only requirement for a callback
        if (typeof abilityData[key] === 'function') {
            callbacks.push(key);
        }
    }

    return callbacks;
}

// Parse all abilities
console.log('Extracting callbacks from abilities...');
const abilities = {};

for (const abilityId in Abilities) {
    if (!Abilities.hasOwnProperty(abilityId)) continue;

    const abilityData = Abilities[abilityId];
    const callbacks = extractCallbacks(abilityId, abilityData);

    if (callbacks.length > 0) {
        abilities[abilityId] = {
            callbacks,
            data: abilityData
        };
    }
}

console.log(`\nFound ${Object.keys(abilities).length} abilities with callbacks:`);
Object.entries(abilities).forEach(([id, data]) => {
    console.log(`  ${id}: ${data.callbacks.join(', ')}`);
});

// Update abilities.json with callback information
console.log('\nUpdating abilities.json with callback information...');
const abilitiesJsonPath = path.join(__dirname, '../data/abilities.json');
let abilitiesJson = {};

// Load existing abilities.json
if (fs.existsSync(abilitiesJsonPath)) {
    abilitiesJson = JSON.parse(fs.readFileSync(abilitiesJsonPath, 'utf8'));
}

// Add callback flags for each ability
for (const abilityId in Abilities) {
    if (!Abilities.hasOwnProperty(abilityId)) continue;

    const abilityData = Abilities[abilityId];
    const callbacks = extractCallbacks(abilityId, abilityData);

    if (!abilitiesJson[abilityId]) {
        abilitiesJson[abilityId] = {};
    }

    // Add callback boolean flags
    callbacks.forEach(callback => {
        abilitiesJson[abilityId][callback] = true;
    });

    // Preserve existing metadata (name, num, rating, etc.)
    if (abilityData.name) {
        abilitiesJson[abilityId].name = abilityData.name;
    }
    if (abilityData.num !== undefined) {
        abilitiesJson[abilityId].num = abilityData.num;
    }
    if (abilityData.rating !== undefined) {
        abilitiesJson[abilityId].rating = abilityData.rating;
    }
    if (abilityData.isNonstandard !== undefined) {
        abilitiesJson[abilityId].isNonstandard = abilityData.isNonstandard;
    }
    if (abilityData.suppressWeather !== undefined) {
        abilitiesJson[abilityId].suppressWeather = abilityData.suppressWeather;
    }
}

// Write updated abilities.json
fs.writeFileSync(abilitiesJsonPath, JSON.stringify(abilitiesJson, null, 2));
console.log(`  Updated ${abilitiesJsonPath} with callback flags`);

console.log('\n✓ Done! Updated abilities.json with callback information.');
