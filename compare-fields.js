#!/usr/bin/env node
/**
 * Compare fields between JavaScript and Rust for specific types
 */

const fs = require('fs');

const TYPES_MD = 'JAVASCRIPT_TYPES.md';

// Parse fields for a specific type from markdown
function getJavaScriptFields(typeName) {
    const content = fs.readFileSync(TYPES_MD, 'utf8');
    const lines = content.split('\n');

    let inType = false;
    let inFieldsTable = false;
    const fields = [];

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Found the type
        if (line === `### ${typeName}`) {
            inType = true;
            continue;
        }

        // Left the type section
        if (inType && line.startsWith('### ') && line !== `### ${typeName}`) {
            break;
        }

        // Found fields table
        if (inType && line.includes('| Name | Type |')) {
            inFieldsTable = true;
            i++; // Skip separator
            continue;
        }

        // Parse field row
        if (inFieldsTable && line.startsWith('|')) {
            const parts = line.split('|').map(s => s.trim()).filter(s => s);
            if (parts.length >= 2 && parts[0] !== 'Name' && !parts[0].includes('---')) {
                const name = parts[0];
                const type = parts[1].replace(/`/g, '');
                const optional = parts.length > 2 ? parts[2] === '✓' : false;
                const readonly = parts.length > 3 ? parts[3] === '✓' : false;
                if (name && type) {
                    fields.push({ name, type, optional, readonly });
                }
            }
        }

        // End of fields table
        if (inFieldsTable && (line === '---' || line === '')) {
            inFieldsTable = false;
        }
    }

    return fields;
}

// Extract Rust fields from a file
function getRustFields(rustFile, typeName) {
    if (!fs.existsSync(rustFile)) {
        return { error: 'File not found' };
    }

    const content = fs.readFileSync(rustFile, 'utf8');

    // Find the struct/enum definition
    const patterns = [
        new RegExp(`pub struct ${typeName}\\s*[<{]`),
        new RegExp(`struct ${typeName}\\s*[<{]`),
    ];

    let startIndex = -1;
    for (const pattern of patterns) {
        const match = content.match(pattern);
        if (match) {
            startIndex = content.indexOf(match[0]);
            break;
        }
    }

    if (startIndex < 0) {
        return { error: 'Type definition not found' };
    }

    // Extract fields from the struct definition
    const remaining = content.slice(startIndex);
    const lines = remaining.split('\n');
    const fields = [];

    let braceCount = 0;
    let inStruct = false;

    for (const line of lines) {
        // Track brace depth
        for (const char of line) {
            if (char === '{') {
                braceCount++;
                inStruct = true;
            }
            if (char === '}') {
                braceCount--;
                if (braceCount === 0) {
                    inStruct = false;
                    break;
                }
            }
        }

        if (!inStruct && braceCount === 0) break;

        // Parse pub field: name: Type,
        const fieldMatch = line.match(/^\s*pub\s+(\w+):\s*(.+?),?\s*$/);
        if (fieldMatch) {
            const name = fieldMatch[1];
            let type = fieldMatch[2].replace(/,$/, '').trim();
            fields.push({ name, type });
        }
    }

    return { fields };
}

// Compare fields for a type
function compareType(typeName, rustFile) {
    console.log(`\n${'='.repeat(80)}`);
    console.log(`Comparing ${typeName}`);
    console.log(`${'='.repeat(80)}\n`);

    const jsFields = getJavaScriptFields(typeName);
    console.log(`JavaScript has ${jsFields.length} fields`);

    if (!rustFile) {
        console.log(`Rust: TYPE NOT FOUND\n`);
        console.log(`All ${jsFields.length} fields are missing:\n`);
        jsFields.forEach((field, i) => {
            console.log(`${i + 1}. ${field.name}: ${field.type}${field.optional ? ' (optional)' : ''}${field.readonly ? ' (readonly)' : ''}`);
        });
        return;
    }

    const rustResult = getRustFields(rustFile, typeName);

    if (rustResult.error) {
        console.log(`Rust: ${rustResult.error}\n`);
        return;
    }

    const rustFields = rustResult.fields;
    console.log(`Rust has ${rustFields.length} fields\n`);

    // Create maps for easy lookup
    const jsFieldMap = new Map(jsFields.map(f => [f.name, f]));
    const rustFieldMap = new Map(rustFields.map(f => [f.name, f]));

    // Find missing fields in Rust
    const missingInRust = jsFields.filter(f => !rustFieldMap.has(f.name));

    // Find extra fields in Rust (not in JS)
    const extraInRust = rustFields.filter(f => !jsFieldMap.has(f.name));

    // Find matching fields
    const matchingFields = jsFields.filter(f => rustFieldMap.has(f.name));

    console.log(`\n--- Missing in Rust (${missingInRust.length} fields) ---\n`);
    missingInRust.forEach((field, i) => {
        console.log(`${i + 1}. ${field.name}: ${field.type}${field.optional ? ' (optional)' : ''}${field.readonly ? ' (readonly)' : ''}`);
    });

    if (extraInRust.length > 0) {
        console.log(`\n--- Extra in Rust (not in JS) (${extraInRust.length} fields) ---\n`);
        extraInRust.forEach((field, i) => {
            console.log(`${i + 1}. ${field.name}: ${field.type}`);
        });
    }

    console.log(`\n--- Matching fields (${matchingFields.length} fields) ---\n`);
    matchingFields.slice(0, 10).forEach((field, i) => {
        const rustField = rustFieldMap.get(field.name);
        console.log(`${i + 1}. ${field.name}: JS=${field.type}, Rust=${rustField.type}`);
    });
    if (matchingFields.length > 10) {
        console.log(`   ... and ${matchingFields.length - 10} more matching fields`);
    }
}

// Main
console.log('Field-by-Field Comparison for Critical Types\n');

// Compare Battle
compareType('Battle', 'src/battle.rs');

// Compare Pokemon
compareType('Pokemon', 'src/pokemon.rs');

// Compare BattleActions
compareType('BattleActions', 'src/battle_actions.rs');

// Compare Side
compareType('Side', 'src/side.rs');

// Compare Field
compareType('Field', 'src/field.rs');

// Missing request types
console.log('\n\n' + '='.repeat(80));
console.log('MISSING REQUEST/RESPONSE TYPES (Critical for player interaction)');
console.log('='.repeat(80));

compareType('PokemonSwitchRequestData', null);
compareType('PokemonMoveRequestData', null);
compareType('SideRequestData', null);
