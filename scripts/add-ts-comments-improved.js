#!/usr/bin/env node

/**
 * Script to add JavaScript/TypeScript source code as comments above Rust methods
 * IMPROVED VERSION - prevents duplicates by tracking which TS sources are already used
 *
 * Usage: node scripts/add-ts-comments-improved.js
 */

const fs = require('fs');
const path = require('path');
const ts = require('typescript');
const crypto = require('crypto');

// Comprehensive mapping of Rust files to TypeScript files
const FILE_MAPPINGS = {
    'battle_actions.rs': 'battle-actions.ts',
    'battle.rs': 'battle.ts',
    'battle_queue.rs': 'battle-queue.ts',
    'battle_stream.rs': 'battle-stream.ts',
    'pokemon.rs': 'pokemon.ts',
    'side.rs': 'side.ts',
    'field.rs': 'field.ts',
    'dex.rs': 'dex.ts',
    'dex_data.rs': 'dex-data.ts',
    'prng.rs': 'prng.ts',
    'state.rs': 'state.ts',
    'team_validator.rs': 'team-validator.ts',
    'teams.rs': 'teams.ts',
    'random_teams.rs': 'teams.ts',
    'abilities.rs': 'dex-abilities.ts',
    'items.rs': 'dex-items.ts',
    'data/abilities.rs': 'dex-abilities.ts',
    'data/items.rs': 'dex-items.ts',
    'data/species.rs': 'dex-species.ts',
    'data/conditions.rs': 'dex-conditions.ts',
    'data/formats.rs': 'dex-formats.ts',
    'data/natures.rs': 'dex-data.ts',
    'data/typechart.rs': 'dex-data.ts',
};

function camelToSnake(str) {
    return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

function snakeToCamel(str) {
    return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

function hashString(str) {
    return crypto.createHash('md5').update(str).digest('hex');
}

function extractTSMethodsUsingCompiler(filePath) {
    const methods = new Map(); // name -> { source, hash }
    const sourceCode = fs.readFileSync(filePath, 'utf-8');

    const sourceFile = ts.createSourceFile(
        filePath,
        sourceCode,
        ts.ScriptTarget.Latest,
        true
    );

    function visit(node) {
        if (ts.isMethodDeclaration(node) && node.name) {
            const methodName = node.name.getText(sourceFile);
            const fullText = extractNodeWithComments(node, sourceFile, sourceCode);
            const hash = hashString(fullText);
            addMethodVariants(methods, methodName, fullText, hash);
        }
        else if (ts.isFunctionDeclaration(node) && node.name) {
            const functionName = node.name.getText(sourceFile);
            const fullText = extractNodeWithComments(node, sourceFile, sourceCode);
            const hash = hashString(fullText);
            addMethodVariants(methods, functionName, fullText, hash);
        }
        else if (ts.isVariableStatement(node)) {
            node.declarationList.declarations.forEach(decl => {
                if (decl.name && ts.isIdentifier(decl.name) && decl.initializer) {
                    if (ts.isArrowFunction(decl.initializer) || ts.isFunctionExpression(decl.initializer)) {
                        const name = decl.name.getText(sourceFile);
                        const fullText = extractNodeWithComments(node, sourceFile, sourceCode);
                        const hash = hashString(fullText);
                        addMethodVariants(methods, name, fullText, hash);
                    }
                }
            });
        }

        ts.forEachChild(node, visit);
    }

    visit(sourceFile);
    return methods;
}

function extractNodeWithComments(node, sourceFile, sourceCode) {
    const fullStart = node.getFullStart();
    const end = node.getEnd();

    const jsDocComments = ts.getJSDocCommentsAndTags(node);
    let commentStart = fullStart;

    if (jsDocComments && jsDocComments.length > 0) {
        commentStart = jsDocComments[0].pos;
    }

    const leadingComments = ts.getLeadingCommentRanges(sourceCode, fullStart);
    if (leadingComments && leadingComments.length > 0) {
        commentStart = Math.min(commentStart, leadingComments[0].pos);
    }

    return sourceCode.substring(commentStart, end);
}

function addMethodVariants(methods, name, source, hash) {
    const variants = [
        name,
        name.toLowerCase(),
        camelToSnake(name),
        name.replace(/^get/, ''),
        name.replace(/^set/, ''),
        name.replace(/^is/, ''),
        name.replace(/^has/, ''),
    ];

    for (const variant of variants) {
        const key = variant.toLowerCase();
        if (!methods.has(key)) {
            methods.set(key, { source, hash });
        }
    }
}

function extractRustMethods(rustContent) {
    const methods = [];
    const lines = rustContent.split('\n');

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const fnMatch = line.match(/^\s*((?:pub(?:\s+\([^)]+\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+))/);

        if (fnMatch) {
            methods.push({
                name: fnMatch[2],
                lineNumber: i,
                signature: fnMatch[1],
            });
        }
    }

    return methods;
}

function extractExistingTSComments(rustContent) {
    const hashes = new Set();
    const lines = rustContent.split('\n');

    for (let i = 0; i < lines.length; i++) {
        if (lines[i].trim() === '// TypeScript source:') {
            // Extract the block
            const blockLines = [];
            for (let j = i + 1; j < lines.length; j++) {
                const line = lines[j];
                if (!line.trim().startsWith('//')) break;
                if (line.trim() === '//') break;
                blockLines.push(line.replace(/^\/\/\s?/, ''));
            }
            const tsCode = blockLines.join('\n');
            const hash = hashString(tsCode);
            hashes.add(hash);
        }
    }

    return hashes;
}

function addJSCommentsToRustFile(rustPath, tsPath) {
    if (!fs.existsSync(rustPath)) {
        return;
    }

    if (!fs.existsSync(tsPath)) {
        return;
    }

    const rustContent = fs.readFileSync(rustPath, 'utf-8');
    const tsMethods = extractTSMethodsUsingCompiler(tsPath);
    const rustMethods = extractRustMethods(rustContent);
    const existingHashes = extractExistingTSComments(rustContent);

    const rustLines = rustContent.split('\n');
    let addedCount = 0;
    let skippedExisting = 0;
    let skippedNoMatch = 0;

    // Track which TS sources we've already added in this run
    const usedHashesThisRun = new Set();

    // Process in reverse order to maintain line numbers
    for (let i = rustMethods.length - 1; i >= 0; i--) {
        const rustMethod = rustMethods[i];

        // Try to find matching TS method
        let tsData = null;
        const nameVariants = [
            rustMethod.name.toLowerCase(),
            snakeToCamel(rustMethod.name).toLowerCase(),
        ];

        for (const variant of nameVariants) {
            if (tsMethods.has(variant)) {
                tsData = tsMethods.get(variant);
                break;
            }
        }

        if (!tsData) {
            skippedNoMatch++;
            continue;
        }

        // Check if this TS source is already in the file
        if (existingHashes.has(tsData.hash)) {
            skippedExisting++;
            continue;
        }

        // Check if we've already used this TS source in this run
        if (usedHashesThisRun.has(tsData.hash)) {
            skippedExisting++;
            continue;
        }

        // Mark this hash as used
        usedHashesThisRun.add(tsData.hash);

        // Format as Rust comment
        const commentLines = ['// TypeScript source:']
            .concat(tsData.source.split('\n').map(line => `// ${line}`))
            .concat(['//']);

        // Insert comment before the method
        const indent = rustLines[rustMethod.lineNumber].match(/^\s*/)[0];
        const indentedCommentLines = commentLines.map(line =>
            line.trim() ? indent + line : line
        );

        rustLines.splice(rustMethod.lineNumber, 0, ...indentedCommentLines);
        addedCount++;
    }

    if (addedCount > 0) {
        fs.writeFileSync(rustPath, rustLines.join('\n'), 'utf-8');
        console.log(`  ✓ ${path.basename(rustPath).padEnd(35)} - Added ${addedCount} (skipped ${skippedExisting} existing, ${skippedNoMatch} no match)`);
    } else if (skippedExisting > 0) {
        console.log(`  = ${path.basename(rustPath).padEnd(35)} - All comments exist`);
    }
}

function processAllFiles() {
    const workspaceDir = '/home/builder/workspace';
    const tsDir = '/home/builder/pokemon-showdown/sim';

    console.log('Adding TypeScript source comments (duplicate-safe)...\n');

    for (const [rustFile, tsFile] of Object.entries(FILE_MAPPINGS)) {
        const rustPath = path.join(workspaceDir, 'src', rustFile);
        const tsPath = path.join(tsDir, tsFile);

        try {
            addJSCommentsToRustFile(rustPath, tsPath);
        } catch (error) {
            console.error(`  ✗ Error processing ${rustFile}:`, error.message);
        }
    }

    console.log('\n✓ Done!');
}

if (require.main === module) {
    processAllFiles();
}
