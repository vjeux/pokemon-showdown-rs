#!/usr/bin/env node

/**
 * Extract Pokemon marked as "Past" or "Future" in Gen 8
 * Generates a simple JSON array of Pokemon IDs that are unavailable in Gen 8
 */

const fs = require('fs');
const path = require('path');

// Load gen8 formats data
const gen8Path = path.join(__dirname, '../../pokemon-showdown/data/mods/gen8/formats-data.ts');
const content = fs.readFileSync(gen8Path, 'utf8');

// Extract Pokemon marked as Past or Future
const unavailablePokemon = [];
const lines = content.split('\n');
let currentPokemon = null;

for (const line of lines) {
    // Match Pokemon ID (e.g., "bulbasaur: {")
    const idMatch = line.match(/^\s+(\w+):\s*\{/);
    if (idMatch) {
        currentPokemon = idMatch[1];
    }

    // Check for isNonstandard: "Past" or "Future"
    if (currentPokemon && line.includes('isNonstandard') && (line.includes('Past') || line.includes('Future'))) {
        unavailablePokemon.push(currentPokemon);
        currentPokemon = null;
    }

    // Reset on closing brace
    if (line.match(/^\s+\},?\s*$/)) {
        currentPokemon = null;
    }
}

// Write to JSON file
const outputPath = path.join(__dirname, '../data/gen8-past.json');
fs.writeFileSync(outputPath, JSON.stringify(unavailablePokemon, null, 2));

console.log(`Extracted ${unavailablePokemon.length} Pokemon unavailable in Gen 8`);
console.log(`Written to: ${outputPath}`);
