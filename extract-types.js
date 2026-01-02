#!/usr/bin/env node
/**
 * Extract all type definitions from Pokemon Showdown sim/ folder (battle simulation)
 * Finds classes, interfaces, types, and their fields from the sim/ directory only
 */

const fs = require('fs');
const path = require('path');

// Configuration
const SHOWDOWN_PATH = process.argv[2] || '../pokemon-showdown';
const OUTPUT_FILE = 'JAVASCRIPT_TYPES.md';

const typeDefinitions = new Map();
const processedFiles = new Set();

/**
 * Recursively find all .ts and .js files in sim/ folder only
 */
function findSourceFiles(dir, files = []) {
    const simDir = path.join(dir, 'sim');

    if (!fs.existsSync(simDir)) {
        console.error(`sim/ directory not found in: ${dir}`);
        return files;
    }

    function scanDirectory(currentDir) {
        const entries = fs.readdirSync(currentDir, { withFileTypes: true });

        for (const entry of entries) {
            const fullPath = path.join(currentDir, entry.name);

            if (entry.isDirectory()) {
                // Skip node_modules and other common directories
                if (!['node_modules', '.git', 'dist', 'build'].includes(entry.name)) {
                    scanDirectory(fullPath);
                }
            } else if (entry.isFile() && (entry.name.endsWith('.ts') || entry.name.endsWith('.js'))) {
                files.push(fullPath);
            }
        }
    }

    scanDirectory(simDir);
    return files;
}

/**
 * Extract class definition with fields
 */
function extractClass(content, startIndex, fileName) {
    const classMatch = content.slice(startIndex).match(/class\s+(\w+)(?:\s+extends\s+([\w.]+))?(?:\s+implements\s+([\w,\s]+))?\s*\{/);

    if (!classMatch) return null;

    const className = classMatch[1];
    const extendsClass = classMatch[2] || null;
    const implementsInterfaces = classMatch[3] ? classMatch[3].split(',').map(s => s.trim()) : [];

    // Find the class body
    const classStart = startIndex + classMatch.index + classMatch[0].length;
    let braceCount = 1;
    let classEnd = classStart;

    for (let i = classStart; i < content.length && braceCount > 0; i++) {
        if (content[i] === '{') braceCount++;
        if (content[i] === '}') braceCount--;
        classEnd = i;
    }

    const classBody = content.slice(classStart, classEnd);
    const fields = [];

    // Extract field declarations
    // Match: field: type, field?: type, readonly field: type, public/private field: type
    const fieldRegex = /^\s*(readonly\s+)?(public\s+|private\s+|protected\s+)?(\w+)(\?)?:\s*([^;=\n]+)/gm;
    let fieldMatch;

    while ((fieldMatch = fieldRegex.exec(classBody)) !== null) {
        const isReadonly = !!fieldMatch[1];
        const visibility = fieldMatch[2] ? fieldMatch[2].trim() : 'public';
        const fieldName = fieldMatch[3];
        const isOptional = !!fieldMatch[4];
        const fieldType = fieldMatch[5].trim();

        // Skip if this looks like it's inside a method or nested class
        const lineStart = classBody.lastIndexOf('\n', fieldMatch.index);
        const line = classBody.slice(lineStart, fieldMatch.index + fieldMatch[0].length);
        if (line.includes('(') || line.includes('function') || line.includes('=>')) {
            continue;
        }

        fields.push({
            name: fieldName,
            type: fieldType,
            optional: isOptional,
            readonly: isReadonly,
            visibility
        });
    }

    return {
        name: className,
        type: 'class',
        extends: extendsClass,
        implements: implementsInterfaces,
        fields,
        file: fileName
    };
}

/**
 * Extract interface definition
 */
function extractInterface(content, startIndex, fileName) {
    const interfaceMatch = content.slice(startIndex).match(/interface\s+(\w+)(?:\s+extends\s+([\w,\s]+))?\s*\{/);

    if (!interfaceMatch) return null;

    const interfaceName = interfaceMatch[1];
    const extendsInterfaces = interfaceMatch[2] ? interfaceMatch[2].split(',').map(s => s.trim()) : [];

    // Find the interface body
    const interfaceStart = startIndex + interfaceMatch.index + interfaceMatch[0].length;
    let braceCount = 1;
    let interfaceEnd = interfaceStart;

    for (let i = interfaceStart; i < content.length && braceCount > 0; i++) {
        if (content[i] === '{') braceCount++;
        if (content[i] === '}') braceCount--;
        interfaceEnd = i;
    }

    const interfaceBody = content.slice(interfaceStart, interfaceEnd);
    const fields = [];

    // Extract field declarations
    const fieldRegex = /^\s*(readonly\s+)?(\w+)(\?)?:\s*([^;=\n]+)/gm;
    let fieldMatch;

    while ((fieldMatch = fieldRegex.exec(interfaceBody)) !== null) {
        const isReadonly = !!fieldMatch[1];
        const fieldName = fieldMatch[2];
        const isOptional = !!fieldMatch[3];
        const fieldType = fieldMatch[4].trim();

        fields.push({
            name: fieldName,
            type: fieldType,
            optional: isOptional,
            readonly: isReadonly
        });
    }

    return {
        name: interfaceName,
        type: 'interface',
        extends: extendsInterfaces,
        fields,
        file: fileName
    };
}

/**
 * Extract type alias definition
 */
function extractTypeAlias(content, startIndex, fileName) {
    const typeMatch = content.slice(startIndex).match(/type\s+(\w+)\s*=\s*([^;]+);/);

    if (!typeMatch) return null;

    const typeName = typeMatch[1];
    const typeDefinition = typeMatch[2].trim();

    // Check if it's an object type
    if (typeDefinition.startsWith('{')) {
        const fields = [];
        const fieldRegex = /(\w+)(\?)?:\s*([^,;}\n]+)/g;
        let fieldMatch;

        while ((fieldMatch = fieldRegex.exec(typeDefinition)) !== null) {
            fields.push({
                name: fieldMatch[1],
                type: fieldMatch[3].trim(),
                optional: !!fieldMatch[2]
            });
        }

        return {
            name: typeName,
            type: 'type',
            definition: typeDefinition,
            fields,
            file: fileName
        };
    }

    return {
        name: typeName,
        type: 'type',
        definition: typeDefinition,
        fields: [],
        file: fileName
    };
}

/**
 * Process a single file
 */
function processFile(filePath) {
    if (processedFiles.has(filePath)) return;
    processedFiles.add(filePath);

    const content = fs.readFileSync(filePath, 'utf8');
    const relativePath = path.relative(SHOWDOWN_PATH, filePath);

    // Find all type definitions

    // Classes
    let classRegex = /\bclass\s+\w+/g;
    let match;
    while ((match = classRegex.exec(content)) !== null) {
        const classDef = extractClass(content, match.index, relativePath);
        if (classDef && classDef.fields.length > 0) {
            typeDefinitions.set(`class:${classDef.name}`, classDef);
        }
    }

    // Interfaces
    let interfaceRegex = /\binterface\s+\w+/g;
    while ((match = interfaceRegex.exec(content)) !== null) {
        const interfaceDef = extractInterface(content, match.index, relativePath);
        if (interfaceDef) {
            typeDefinitions.set(`interface:${interfaceDef.name}`, interfaceDef);
        }
    }

    // Type aliases
    let typeRegex = /\btype\s+\w+\s*=/g;
    while ((match = typeRegex.exec(content)) !== null) {
        const typeDef = extractTypeAlias(content, match.index, relativePath);
        if (typeDef) {
            typeDefinitions.set(`type:${typeDef.name}`, typeDef);
        }
    }
}

/**
 * Generate markdown output
 */
function generateMarkdown() {
    let output = '# JavaScript Type Definitions from Pokemon Showdown (sim/ folder)\n\n';
    output += `Generated: ${new Date().toISOString()}\n`;
    output += `Source: ${SHOWDOWN_PATH}/sim/\n`;
    output += `Total types: ${typeDefinitions.size}\n\n`;

    output += '## Table of Contents\n\n';

    // Group by type
    const classes = [];
    const interfaces = [];
    const types = [];

    for (const [key, def] of typeDefinitions.entries()) {
        if (def.type === 'class') classes.push(def);
        else if (def.type === 'interface') interfaces.push(def);
        else if (def.type === 'type') types.push(def);
    }

    output += `- [Classes](#classes) (${classes.length})\n`;
    output += `- [Interfaces](#interfaces) (${interfaces.length})\n`;
    output += `- [Type Aliases](#type-aliases) (${types.length})\n\n`;

    output += '---\n\n';

    // Classes
    output += '## Classes\n\n';
    classes.sort((a, b) => a.name.localeCompare(b.name));

    for (const classDef of classes) {
        output += `### ${classDef.name}\n\n`;
        output += `**File:** \`${classDef.file}\`\n\n`;

        if (classDef.extends) {
            output += `**Extends:** ${classDef.extends}\n\n`;
        }

        if (classDef.implements.length > 0) {
            output += `**Implements:** ${classDef.implements.join(', ')}\n\n`;
        }

        if (classDef.fields.length > 0) {
            output += '**Fields:**\n\n';
            output += '| Name | Type | Optional | Readonly | Visibility |\n';
            output += '|------|------|----------|----------|------------|\n';

            for (const field of classDef.fields) {
                const opt = field.optional ? '✓' : '';
                const ro = field.readonly ? '✓' : '';
                output += `| ${field.name} | \`${field.type}\` | ${opt} | ${ro} | ${field.visibility} |\n`;
            }
            output += '\n';
        }

        output += '---\n\n';
    }

    // Interfaces
    output += '## Interfaces\n\n';
    interfaces.sort((a, b) => a.name.localeCompare(b.name));

    for (const interfaceDef of interfaces) {
        output += `### ${interfaceDef.name}\n\n`;
        output += `**File:** \`${interfaceDef.file}\`\n\n`;

        if (interfaceDef.extends.length > 0) {
            output += `**Extends:** ${interfaceDef.extends.join(', ')}\n\n`;
        }

        if (interfaceDef.fields.length > 0) {
            output += '**Fields:**\n\n';
            output += '| Name | Type | Optional | Readonly |\n';
            output += '|------|------|----------|----------|\n';

            for (const field of interfaceDef.fields) {
                const opt = field.optional ? '✓' : '';
                const ro = field.readonly ? '✓' : '';
                output += `| ${field.name} | \`${field.type}\` | ${opt} | ${ro} |\n`;
            }
            output += '\n';
        }

        output += '---\n\n';
    }

    // Type Aliases
    output += '## Type Aliases\n\n';
    types.sort((a, b) => a.name.localeCompare(b.name));

    for (const typeDef of types) {
        output += `### ${typeDef.name}\n\n`;
        output += `**File:** \`${typeDef.file}\`\n\n`;
        output += `**Definition:**\n\`\`\`typescript\n${typeDef.definition}\n\`\`\`\n\n`;

        if (typeDef.fields.length > 0) {
            output += '**Fields:**\n\n';
            output += '| Name | Type | Optional |\n';
            output += '|------|------|----------|\n';

            for (const field of typeDef.fields) {
                const opt = field.optional ? '✓' : '';
                output += `| ${field.name} | \`${field.type}\` | ${opt} |\n`;
            }
            output += '\n';
        }

        output += '---\n\n';
    }

    return output;
}

/**
 * Main execution
 */
function main() {
    console.log(`Scanning ${SHOWDOWN_PATH}/sim/ for type definitions...`);

    const files = findSourceFiles(SHOWDOWN_PATH);
    console.log(`Found ${files.length} source files in sim/`);

    for (const file of files) {
        try {
            processFile(file);
        } catch (err) {
            console.error(`Error processing ${file}:`, err.message);
        }
    }

    console.log(`Extracted ${typeDefinitions.size} type definitions`);

    const markdown = generateMarkdown();
    fs.writeFileSync(OUTPUT_FILE, markdown);

    console.log(`\nOutput written to ${OUTPUT_FILE}`);
    console.log(`\nSummary:`);

    const classes = Array.from(typeDefinitions.values()).filter(d => d.type === 'class');
    const interfaces = Array.from(typeDefinitions.values()).filter(d => d.type === 'interface');
    const types = Array.from(typeDefinitions.values()).filter(d => d.type === 'type');

    console.log(`  Classes: ${classes.length}`);
    console.log(`  Interfaces: ${interfaces.length}`);
    console.log(`  Type Aliases: ${types.length}`);
    console.log(`  Total: ${typeDefinitions.size}`);
}

main();
