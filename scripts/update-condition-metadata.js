#!/usr/bin/env node

/**
 * Update conditions.json with metadata from JavaScript
 *
 * This script:
 * 1. Loads data/conditions.ts from JavaScript
 * 2. Extracts all static metadata fields (not just callbacks)
 * 3. Updates conditions.json with complete metadata
 */

const fs = require('fs');
const path = require('path');

// Load the actual Conditions object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Conditions;
try {
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Conditions = Dex.data.Conditions;
    console.log('Loaded conditions from dist/sim');
} catch (e) {
    console.error('Error loading conditions:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

// Extract all metadata from a condition object
function extractMetadata(conditionId, conditionData) {
    const metadata = {
        name: conditionData.name || conditionId,
    };

    // Extract static data fields (not functions)
    const dataFields = [
        'effectType', 'duration', 'noCopy', 'affectsFainted',
        'onModifySpePriority', 'onModifySpDPriority', 'onModifyAtkPriority',
        'onResidualOrder', 'onResidualSubOrder', 'onFieldResidualOrder',
        'onBeforeMovePriority', 'onDamagingHitOrder', 'onTrapPokemon',
        'counterMin', 'counterMax'
    ];

    for (const field of dataFields) {
        if (conditionData[field] !== undefined && typeof conditionData[field] !== 'function') {
            metadata[field] = conditionData[field];
        }
    }

    // Extract callback flags
    const callbacks = [];
    for (const key in conditionData) {
        if (!conditionData.hasOwnProperty(key)) continue;
        if (typeof conditionData[key] === 'function') {
            callbacks.push(key);
            metadata[key] = true;
        }
    }

    return { metadata, callbacks };
}

// Parse all conditions
console.log('Extracting metadata from conditions...');
const conditions = {};

for (const conditionId in Conditions) {
    if (!Conditions.hasOwnProperty(conditionId)) continue;

    const conditionData = Conditions[conditionId];
    const { metadata, callbacks } = extractMetadata(conditionId, conditionData);

    conditions[conditionId] = metadata;

    if (callbacks.length > 0) {
        console.log(`  ${conditionId}: ${callbacks.length} callbacks`);
    }
}

console.log(`\nFound ${Object.keys(conditions).length} conditions`);

// Write updated conditions.json
const conditionsJsonPath = path.join(__dirname, '../data/conditions.json');
fs.writeFileSync(conditionsJsonPath, JSON.stringify(conditions, null, 2));
console.log(`  Updated ${conditionsJsonPath} with metadata`);

console.log('\n✓ Done! Updated conditions.json with complete metadata.');
