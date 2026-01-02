#!/usr/bin/env node
/**
 * Analyze type coverage gap between JavaScript and Rust
 */

const fs = require('fs');
const path = require('path');

const TYPES_MD = 'JAVASCRIPT_TYPES.md';
const RUST_SRC = 'src';

// Parse the markdown file to extract type information
function parseTypesMarkdown() {
    const content = fs.readFileSync(TYPES_MD, 'utf8');
    const types = [];

    const lines = content.split('\n');
    let currentType = null;
    let currentFile = null;
    let currentRustFile = null;
    let inFieldsTable = false;
    let fields = [];
    let typeCategory = null; // 'class', 'interface', or 'type'

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Detect section headers
        if (line.startsWith('## Classes')) {
            typeCategory = 'class';
            continue;
        }
        if (line.startsWith('## Interfaces')) {
            typeCategory = 'interface';
            continue;
        }
        if (line.startsWith('## Type Aliases')) {
            typeCategory = 'type';
            continue;
        }

        // Match type name headers like "### Pokemon"
        if (line.startsWith('### ') && !line.includes('---')) {
            if (currentType) {
                types.push({
                    name: currentType,
                    category: typeCategory,
                    jsFile: currentFile,
                    rustFile: currentRustFile,
                    fieldCount: fields.length,
                    fields: fields,
                });
            }
            currentType = line.slice(4).trim();
            currentFile = null;
            currentRustFile = null;
            fields = [];
            inFieldsTable = false;
        }

        // Match file location
        if (line.startsWith('**File:**')) {
            const match = line.match(/`([^`]+)`/);
            if (match) {
                currentFile = match[1];
            }
        }

        // Match Rust file location
        if (line.startsWith('**Rust:**')) {
            const match = line.match(/`([^`]+)`/);
            if (match) {
                currentRustFile = match[1];
            } else if (line.includes('*Not found*')) {
                currentRustFile = null;
            }
        }

        // Detect start of fields table
        if (line.includes('| Name | Type |')) {
            inFieldsTable = true;
            i++; // Skip separator line
            continue;
        }

        // Parse field rows
        if (inFieldsTable && line.startsWith('|')) {
            const parts = line.split('|').map(s => s.trim()).filter(s => s);
            if (parts.length >= 2 && parts[0] !== 'Name') {
                const fieldName = parts[0];
                const fieldType = parts[1].replace(/`/g, '');
                if (fieldName && fieldType && !fieldName.includes('---')) {
                    fields.push({ name: fieldName, type: fieldType });
                }
            }
        }

        // Stop parsing fields when we hit separator or new section
        if (inFieldsTable && (line === '---' || line === '' || line.startsWith('##'))) {
            inFieldsTable = false;
        }
    }

    // Add last type
    if (currentType) {
        types.push({
            name: currentType,
            category: typeCategory,
            jsFile: currentFile,
            rustFile: currentRustFile,
            fieldCount: fields.length,
            fields: fields,
        });
    }

    return types;
}

// Count fields in Rust struct/enum
function countRustFields(rustFile, typeName) {
    if (!rustFile || !fs.existsSync(rustFile)) return 0;

    const content = fs.readFileSync(rustFile, 'utf8');

    // Find the type definition
    const patterns = [
        new RegExp(`pub struct ${typeName}\\s*[<{]`, 'i'),
        new RegExp(`pub enum ${typeName}\\s*[<{]`, 'i'),
        new RegExp(`struct ${typeName}\\s*[<{]`, 'i'),
        new RegExp(`enum ${typeName}\\s*[<{]`, 'i'),
    ];

    let startIndex = -1;
    for (const pattern of patterns) {
        const match = content.match(pattern);
        if (match) {
            startIndex = content.indexOf(match[0]);
            break;
        }
    }

    if (startIndex < 0) return 0;

    // Count pub fields until closing brace
    const remaining = content.slice(startIndex);
    const fieldRegex = /^\s*pub\s+(\w+):/gm;
    const matches = remaining.match(fieldRegex);

    return matches ? matches.length : 0;
}

// Categorize missing types by importance
function categorizeMissingTypes(types) {
    const coreBattle = [];
    const dataStructures = [];
    const utilities = [];
    const testing = [];

    for (const type of types) {
        if (type.rustFile) continue; // Skip found types

        const name = type.name;
        const file = type.jsFile || '';

        // Core battle simulation types
        if (file.includes('battle') || file.includes('pokemon') || file.includes('side') ||
            file.includes('field') || file.includes('queue')) {
            coreBattle.push(type);
        }
        // Data structures (Dex, Species, Move, Ability, Item)
        else if (file.includes('dex-') || file.includes('species') || file.includes('moves') ||
                 file.includes('abilities') || file.includes('items')) {
            dataStructures.push(type);
        }
        // Testing/tools
        else if (file.includes('tools') || file.includes('runner') || file.includes('exhaustive')) {
            testing.push(type);
        }
        // Everything else (utilities, streams, validators, etc.)
        else {
            utilities.push(type);
        }
    }

    return { coreBattle, dataStructures, utilities, testing };
}

// Main analysis
function main() {
    console.log('Analyzing type coverage...\n');

    const types = parseTypesMarkdown();

    const foundTypes = types.filter(t => t.rustFile);
    const missingTypes = types.filter(t => !t.rustFile);

    console.log(`=== Summary ===`);
    console.log(`Total JavaScript types: ${types.length}`);
    console.log(`Found in Rust: ${foundTypes.length} (${Math.round(foundTypes.length/types.length*100)}%)`);
    console.log(`Missing in Rust: ${missingTypes.length} (${Math.round(missingTypes.length/types.length*100)}%)\n`);

    // Categorize missing types
    const { coreBattle, dataStructures, utilities, testing } = categorizeMissingTypes(types);

    console.log(`=== Missing Types by Category ===`);
    console.log(`Core Battle: ${coreBattle.length}`);
    console.log(`Data Structures: ${dataStructures.length}`);
    console.log(`Utilities: ${utilities.length}`);
    console.log(`Testing/Tools: ${testing.length}\n`);

    // Show most important missing types
    console.log(`=== Top 20 Missing Core Battle Types ===`);
    coreBattle
        .sort((a, b) => b.fieldCount - a.fieldCount)
        .slice(0, 20)
        .forEach((type, i) => {
            console.log(`${i + 1}. ${type.name} (${type.category}, ${type.fieldCount} fields) - ${type.jsFile}`);
        });

    console.log(`\n=== Top 20 Missing Data Structure Types ===`);
    dataStructures
        .sort((a, b) => b.fieldCount - a.fieldCount)
        .slice(0, 20)
        .forEach((type, i) => {
            console.log(`${i + 1}. ${type.name} (${type.category}, ${type.fieldCount} fields) - ${type.jsFile}`);
        });

    // Compare field counts for found types
    console.log(`\n=== Field Count Comparison (Found Types) ===`);
    console.log(`Types with significant field count differences:\n`);

    const differences = [];
    for (const type of foundTypes) {
        const rustFieldCount = countRustFields(type.rustFile, type.name);
        const diff = Math.abs(type.fieldCount - rustFieldCount);
        if (diff > 5) { // Only show types with >5 field difference
            differences.push({
                name: type.name,
                jsFields: type.fieldCount,
                rustFields: rustFieldCount,
                diff: diff,
                rustFile: type.rustFile,
            });
        }
    }

    differences
        .sort((a, b) => b.diff - a.diff)
        .slice(0, 15)
        .forEach((item, i) => {
            console.log(`${i + 1}. ${item.name}: JS=${item.jsFields}, Rust=${item.rustFields}, diff=${item.diff}`);
            console.log(`   ${item.rustFile}\n`);
        });

    // Breakdown by category
    console.log(`\n=== Type Coverage by Category ===`);
    const classes = types.filter(t => t.category === 'class');
    const interfaces = types.filter(t => t.category === 'interface');
    const typeAliases = types.filter(t => t.category === 'type');

    console.log(`Classes: ${classes.filter(t => t.rustFile).length}/${classes.length} (${Math.round(classes.filter(t => t.rustFile).length/classes.length*100)}%)`);
    console.log(`Interfaces: ${interfaces.filter(t => t.rustFile).length}/${interfaces.length} (${Math.round(interfaces.filter(t => t.rustFile).length/interfaces.length*100)}%)`);
    console.log(`Type Aliases: ${typeAliases.filter(t => t.rustFile).length}/${typeAliases.length} (${Math.round(typeAliases.filter(t => t.rustFile).length/typeAliases.length*100)}%)`);
}

main();
