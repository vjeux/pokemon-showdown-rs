#!/usr/bin/env node

/**
 * Script to add JavaScript/TypeScript source code as comments above Rust methods
 *
 * Usage: node scripts/add-js-comments.js <rust-file> <ts-file>
 */

const fs = require('fs');
const path = require('path');

// Mapping of Rust files to TypeScript files
const FILE_MAPPINGS = {
    'battle_actions.rs': 'battle-actions.ts',
    'battle.rs': 'battle.ts',
    'pokemon.rs': 'pokemon.ts',
    'side.rs': 'side.ts',
    'field.rs': 'field.ts',
    'dex.rs': 'dex.ts',
    'prng.rs': 'prng.ts',
};

function extractTSMethods(tsContent) {
    const methods = new Map();

    // Pattern to match TypeScript methods/functions
    // Captures: method name and full method body
    const patterns = [
        // Class methods: methodName(...): type { ... }
        /(?:(?:public|private|protected|static|async|readonly)\s+)*(\w+)\s*\([^)]*\)\s*:\s*[^{]+\{/g,
        // Functions: function name(...): type { ... } or const name = (...) => { ... }
        /(?:export\s+)?(?:function|const)\s+(\w+)\s*[=\(]/g,
    ];

    // Find all method starts
    const methodStarts = [];
    for (const pattern of patterns) {
        let match;
        while ((match = pattern.exec(tsContent)) !== null) {
            const methodName = match[1];
            const startPos = match.index;
            methodStarts.push({ name: methodName, start: startPos });
        }
    }

    // For each method, extract the full source
    for (let i = 0; i < methodStarts.length; i++) {
        const { name, start } = methodStarts[i];

        // Find the matching closing brace
        let braceCount = 0;
        let inMethod = false;
        let methodStart = start;
        let methodEnd = start;

        for (let j = start; j < tsContent.length; j++) {
            const char = tsContent[j];

            if (char === '{') {
                if (!inMethod) {
                    inMethod = true;
                    methodStart = j;
                }
                braceCount++;
            } else if (char === '}') {
                braceCount--;
                if (inMethod && braceCount === 0) {
                    methodEnd = j + 1;
                    break;
                }
            }
        }

        if (inMethod && methodEnd > methodStart) {
            // Extract full method including leading comments
            let extractStart = methodStart;

            // Look backwards for JSDoc comments
            let searchStart = Math.max(0, start - 500);
            let beforeMethod = tsContent.substring(searchStart, start);
            let jsdocMatch = beforeMethod.match(/\/\*\*[\s\S]*?\*\/\s*$/);
            if (jsdocMatch) {
                extractStart = searchStart + jsdocMatch.index;
            } else {
                // Look for line comments
                let lines = beforeMethod.split('\n');
                let commentLines = [];
                for (let k = lines.length - 1; k >= 0; k--) {
                    if (lines[k].trim().startsWith('//')) {
                        commentLines.unshift(lines[k]);
                    } else if (lines[k].trim() === '') {
                        continue;
                    } else {
                        break;
                    }
                }
                if (commentLines.length > 0) {
                    extractStart = start - commentLines.join('\n').length - commentLines.length;
                }
            }

            const methodSource = tsContent.substring(extractStart, methodEnd);

            // Store with various name formats
            methods.set(name, methodSource);
            methods.set(name.toLowerCase(), methodSource);
            methods.set(camelToSnake(name), methodSource);
        }
    }

    return methods;
}

function camelToSnake(str) {
    return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

function snakeToCamel(str) {
    return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

function extractRustMethods(rustContent) {
    const methods = [];

    // Pattern to match Rust function/method definitions
    // Matches: pub fn name(...) or fn name(...)
    const fnPattern = /(?:\/\/\/[^\n]*\n)*(?:\/\/[^\n]*\n)*\s*((?:pub(?:\s+\([^)]+\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\([^)]*\)(?:\s*->\s*[^{]+)?)\s*\{/g;

    let match;
    while ((match = fnPattern.exec(rustContent)) !== null) {
        const fullSignature = match[1];
        const methodName = match[2];
        const startPos = match.index;

        methods.push({
            name: methodName,
            start: startPos,
            signature: fullSignature,
        });
    }

    return methods;
}

function addJSCommentsToRustFile(rustPath, tsPath) {
    console.log(`Processing ${rustPath} with ${tsPath}...`);

    if (!fs.existsSync(rustPath)) {
        console.error(`Rust file not found: ${rustPath}`);
        return;
    }

    if (!fs.existsSync(tsPath)) {
        console.error(`TypeScript file not found: ${tsPath}`);
        return;
    }

    const rustContent = fs.readFileSync(rustPath, 'utf-8');
    const tsContent = fs.readFileSync(tsPath, 'utf-8');

    const tsMethods = extractTSMethods(tsContent);
    const rustMethods = extractRustMethods(rustContent);

    console.log(`Found ${tsMethods.size} TypeScript methods`);
    console.log(`Found ${rustMethods.length} Rust methods`);

    // Build new Rust content with JS comments
    let newRustContent = rustContent;
    let offset = 0;

    for (const rustMethod of rustMethods) {
        // Try to find matching TS method
        let tsSource = null;

        // Try different name variations
        const nameVariants = [
            rustMethod.name,
            snakeToCamel(rustMethod.name),
            rustMethod.name.toLowerCase(),
        ];

        for (const variant of nameVariants) {
            if (tsMethods.has(variant)) {
                tsSource = tsMethods.get(variant);
                break;
            }
        }

        if (!tsSource) {
            // Try partial matches
            for (const [tsName, source] of tsMethods) {
                if (tsName.includes(rustMethod.name) || rustMethod.name.includes(tsName)) {
                    tsSource = source;
                    break;
                }
            }
        }

        if (tsSource) {
            // Check if comment already exists
            const beforeMethod = newRustContent.substring(
                Math.max(0, rustMethod.start + offset - 500),
                rustMethod.start + offset
            );

            if (beforeMethod.includes('// JavaScript source:') ||
                beforeMethod.includes('// TypeScript source:')) {
                console.log(`  Skipping ${rustMethod.name} (comment already exists)`);
                continue;
            }

            // Format as Rust comment
            const commentLines = tsSource.split('\n').map(line => `// ${line}`);
            const comment = `// TypeScript source:\n${commentLines.join('\n')}\n//\n`;

            // Insert comment before the method
            const insertPos = rustMethod.start + offset;
            newRustContent = newRustContent.substring(0, insertPos) +
                           comment +
                           newRustContent.substring(insertPos);

            offset += comment.length;
            console.log(`  ✓ Added comment for ${rustMethod.name}`);
        } else {
            console.log(`  ✗ No TypeScript source found for ${rustMethod.name}`);
        }
    }

    // Write back to file
    fs.writeFileSync(rustPath, newRustContent, 'utf-8');
    console.log(`✓ Updated ${rustPath}\n`);
}

function processAllFiles() {
    const workspaceDir = '/home/builder/workspace';
    const tsDir = '/home/builder/pokemon-showdown/sim';

    for (const [rustFile, tsFile] of Object.entries(FILE_MAPPINGS)) {
        const rustPath = path.join(workspaceDir, 'src', rustFile);
        const tsPath = path.join(tsDir, tsFile);

        addJSCommentsToRustFile(rustPath, tsPath);
    }
}

// Main execution
if (require.main === module) {
    const args = process.argv.slice(2);

    if (args.length === 2) {
        // Process specific files
        addJSCommentsToRustFile(args[0], args[1]);
    } else {
        // Process all files
        processAllFiles();
    }
}
