#!/usr/bin/env node
/**
 * Map JavaScript types to Rust definitions and cross-reference them
 */

const fs = require('fs');
const path = require('path');

const TYPES_MD = 'JAVASCRIPT_TYPES.md';
const RUST_SRC = 'src';

// Parse the markdown file to extract all type definitions
function parseTypesMarkdown() {
    const content = fs.readFileSync(TYPES_MD, 'utf8');
    const types = [];

    const lines = content.split('\n');
    let currentType = null;
    let currentFile = null;
    let inFieldsTable = false;
    let fields = [];

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Match type name headers like "### Pokemon"
        if (line.startsWith('### ') && !line.includes('---')) {
            if (currentType) {
                types.push({
                    name: currentType,
                    jsFile: currentFile,
                    fields: fields,
                    rustFile: null
                });
            }
            currentType = line.slice(4).trim();
            currentFile = null;
            fields = [];
            inFieldsTable = false;
        }

        // Match file location like "**File:** `sim/pokemon.ts`"
        if (line.startsWith('**File:**')) {
            const match = line.match(/`([^`]+)`/);
            if (match) {
                currentFile = match[1];
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
            jsFile: currentFile,
            fields: fields,
            rustFile: null
        });
    }

    return types;
}

// Find Rust files
function findRustFiles(dir, files = []) {
    const entries = fs.readdirSync(dir, { withFileTypes: true });

    for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);

        if (entry.isDirectory() && !['target', 'node_modules', '.git'].includes(entry.name)) {
            findRustFiles(fullPath, files);
        } else if (entry.isFile() && entry.name.endsWith('.rs')) {
            files.push(fullPath);
        }
    }

    return files;
}

// Search for type definition in Rust files
function findRustDefinition(typeName) {
    const rustFiles = findRustFiles(RUST_SRC);

    // Common patterns for Rust type definitions
    const patterns = [
        new RegExp(`pub struct ${typeName}\\s*[<{]`, 'i'),
        new RegExp(`pub enum ${typeName}\\s*[<{]`, 'i'),
        new RegExp(`pub type ${typeName}\\s*=`, 'i'),
        new RegExp(`struct ${typeName}\\s*[<{]`, 'i'),
        new RegExp(`enum ${typeName}\\s*[<{]`, 'i'),
    ];

    for (const file of rustFiles) {
        const content = fs.readFileSync(file, 'utf8');

        for (const pattern of patterns) {
            if (pattern.test(content)) {
                return path.relative('.', file);
            }
        }
    }

    return null;
}

// Add JavaScript type reference comment to Rust file
function addJsTypeComment(rustFile, jsType) {
    if (!rustFile || !fs.existsSync(rustFile)) return false;

    const content = fs.readFileSync(rustFile, 'utf8');
    const lines = content.split('\n');

    // Find the type definition
    const typePatterns = [
        new RegExp(`pub struct ${jsType.name}\\s*[<{]`),
        new RegExp(`pub enum ${jsType.name}\\s*[<{]`),
        new RegExp(`pub type ${jsType.name}\\s*=`),
        new RegExp(`struct ${jsType.name}\\s*[<{]`),
        new RegExp(`enum ${jsType.name}\\s*[<{]`),
    ];

    let typeLineIndex = -1;
    for (let i = 0; i < lines.length; i++) {
        for (const pattern of typePatterns) {
            if (pattern.test(lines[i])) {
                typeLineIndex = i;
                break;
            }
        }
        if (typeLineIndex >= 0) break;
    }

    if (typeLineIndex < 0) return false;

    // Check if comment already exists
    if (typeLineIndex > 0 && lines[typeLineIndex - 1].includes('JavaScript equivalent:')) {
        return false; // Already has comment
    }

    // Build the comment
    const comment = [];
    comment.push(`/// JavaScript equivalent: ${jsType.name} (${jsType.jsFile})`);

    if (jsType.fields.length > 0 && jsType.fields.length <= 5) {
        comment.push(`/// Fields: ${jsType.fields.map(f => f.name).join(', ')}`);
    } else if (jsType.fields.length > 5) {
        comment.push(`/// ${jsType.fields.length} fields in JavaScript`);
    }

    // Insert comment before type definition
    lines.splice(typeLineIndex, 0, comment.join('\n'));

    fs.writeFileSync(rustFile, lines.join('\n'));
    return true;
}

// Main execution
function main() {
    console.log('Parsing JavaScript types from markdown...');
    const types = parseTypesMarkdown();
    console.log(`Found ${types.length} JavaScript types`);

    console.log('\nMapping to Rust definitions...');
    let foundCount = 0;
    let notFoundCount = 0;
    let commentedCount = 0;

    for (const type of types) {
        const rustFile = findRustDefinition(type.name);
        type.rustFile = rustFile;

        if (rustFile) {
            foundCount++;
            console.log(`  ✓ ${type.name} -> ${rustFile}`);

            // Add comment to Rust file
            if (addJsTypeComment(rustFile, type)) {
                commentedCount++;
            }
        } else {
            notFoundCount++;
            console.log(`  ✗ ${type.name} (not found in Rust)`);
        }
    }

    console.log(`\n\nUpdating markdown with Rust file references...`);
    updateMarkdownWithRustFiles(types);

    console.log(`\n\nSummary:`);
    console.log(`  Total JavaScript types: ${types.length}`);
    console.log(`  Found in Rust: ${foundCount} (${Math.round(foundCount/types.length*100)}%)`);
    console.log(`  Not found in Rust: ${notFoundCount}`);
    console.log(`  Comments added: ${commentedCount}`);
}

// Update markdown file with Rust file references
function updateMarkdownWithRustFiles(types) {
    const content = fs.readFileSync(TYPES_MD, 'utf8');
    const lines = content.split('\n');
    const output = [];

    let currentTypeName = null;
    let typeMap = new Map(types.map(t => [t.name, t]));
    let rustLineAdded = false;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Match type name headers
        if (line.startsWith('### ') && !line.includes('---')) {
            currentTypeName = line.slice(4).trim();
            rustLineAdded = false;
        }

        // Skip existing Rust lines
        if (line.startsWith('**Rust:**')) {
            continue;
        }

        // Add Rust file info after **File:** line
        if (line.startsWith('**File:**') && currentTypeName && !rustLineAdded) {
            output.push(line);

            const typeInfo = typeMap.get(currentTypeName);
            if (typeInfo) {
                if (typeInfo.rustFile) {
                    output.push('');
                    output.push(`**Rust:** \`${typeInfo.rustFile}\``);
                } else {
                    output.push('');
                    output.push(`**Rust:** *Not found*`);
                }
                rustLineAdded = true;
            }
            continue;
        }

        output.push(line);
    }

    fs.writeFileSync(TYPES_MD, output.join('\n'));
}

main();
