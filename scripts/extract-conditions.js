#!/usr/bin/env node

/**
 * Extract conditions data from Pokemon Showdown TypeScript codebase
 * Generates a conditions.json file with duration and other simple fields
 */

const fs = require('fs');
const path = require('path');

// Load conditions.js from dist
const conditionsPath = path.join(__dirname, '../../pokemon-showdown-ts/dist/data/conditions.js');
const content = fs.readFileSync(conditionsPath, 'utf8');

// Extract the Conditions object
// The file exports: const Conditions = { ... }
const conditionsMatch = content.match(/const Conditions = \{([\s\S]*)\};/);
if (!conditionsMatch) {
    console.error('Could not find Conditions object in file');
    process.exit(1);
}

// Parse the Conditions object structure
const conditionsText = conditionsMatch[1];
const conditions = {};

// Split by top-level entries (condition names followed by {)
const lines = conditionsText.split('\n');
let currentCondition = null;
let braceDepth = 0;
let conditionLines = [];

for (const line of lines) {
    // Match condition ID at the start (e.g., "  stall: {")
    const idMatch = line.match(/^\s+(\w+):\s*\{/);
    if (idMatch && braceDepth === 0) {
        // Save previous condition if exists
        if (currentCondition) {
            parseCondition(currentCondition, conditionLines.join('\n'), conditions);
        }
        // Start new condition
        currentCondition = idMatch[1];
        conditionLines = [line];
        braceDepth = 1;
        continue;
    }

    if (currentCondition) {
        conditionLines.push(line);
        // Track brace depth
        const openBraces = (line.match(/\{/g) || []).length;
        const closeBraces = (line.match(/\}/g) || []).length;
        braceDepth += openBraces - closeBraces;

        // Condition complete
        if (braceDepth === 0) {
            parseCondition(currentCondition, conditionLines.join('\n'), conditions);
            currentCondition = null;
            conditionLines = [];
        }
    }
}

function parseCondition(id, text, output) {
    const data = {};

    // Extract duration (if present)
    const durationMatch = text.match(/duration:\s*(\d+)/);
    if (durationMatch) {
        data.duration = parseInt(durationMatch[1], 10);
    }

    // Only add to output if there's any extractable data
    if (Object.keys(data).length > 0) {
        output[id] = data;
    }
}

// Write to JSON file
const outputPath = path.join(__dirname, '../data/conditions.json');
fs.writeFileSync(outputPath, JSON.stringify(conditions, null, 2));

console.log(`Extracted ${Object.keys(conditions).length} conditions with duration`);
console.log(`Written to: ${outputPath}`);
console.log('Conditions:', Object.keys(conditions).join(', '));
