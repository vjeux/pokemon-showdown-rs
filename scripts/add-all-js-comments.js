#!/usr/bin/env node

/**
 * Script to add JavaScript/TypeScript source code as comments above Rust methods
 * Processes ALL Rust files in the codebase
 *
 * Usage: node scripts/add-all-js-comments.js
 */

const fs = require('fs');
const path = require('path');
const ts = require('typescript');

// Comprehensive mapping of Rust files to TypeScript files
const FILE_MAPPINGS = {
    // Main sim files
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
    'random_teams.rs': 'teams.ts', // Maps to same file
    // Dex-specific files
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

function extractTSMethodsUsingCompiler(filePath) {
    const methods = new Map();
    const sourceCode = fs.readFileSync(filePath, 'utf-8');

    const sourceFile = ts.createSourceFile(
        filePath,
        sourceCode,
        ts.ScriptTarget.Latest,
        true
    );

    function visit(node) {
        // Method declarations
        if (ts.isMethodDeclaration(node) && node.name) {
            const methodName = node.name.getText(sourceFile);
            const fullText = extractNodeWithComments(node, sourceFile, sourceCode);
            addMethodVariants(methods, methodName, fullText);
        }
        // Function declarations
        else if (ts.isFunctionDeclaration(node) && node.name) {
            const functionName = node.name.getText(sourceFile);
            const fullText = extractNodeWithComments(node, sourceFile, sourceCode);
            addMethodVariants(methods, functionName, fullText);
        }
        // Arrow functions assigned to const
        else if (ts.isVariableStatement(node)) {
            node.declarationList.declarations.forEach(decl => {
                if (decl.name && ts.isIdentifier(decl.name) && decl.initializer) {
                    if (ts.isArrowFunction(decl.initializer) || ts.isFunctionExpression(decl.initializer)) {
                        const name = decl.name.getText(sourceFile);
                        const fullText = extractNodeWithComments(node, sourceFile, sourceCode);
                        addMethodVariants(methods, name, fullText);
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

function addMethodVariants(methods, name, source) {
    methods.set(name, source);
    methods.set(name.toLowerCase(), source);
    methods.set(camelToSnake(name), source);

    // Add without common prefixes
    const withoutGet = name.replace(/^get/, '');
    const withoutSet = name.replace(/^set/, '');
    const withoutIs = name.replace(/^is/, '');
    const withoutHas = name.replace(/^has/, '');

    if (withoutGet !== name) methods.set(camelToSnake(withoutGet), source);
    if (withoutSet !== name) methods.set(camelToSnake(withoutSet), source);
    if (withoutIs !== name) methods.set(camelToSnake(withoutIs), source);
    if (withoutHas !== name) methods.set(camelToSnake(withoutHas), source);
}

function extractRustMethods(rustContent) {
    const methods = [];
    const lines = rustContent.split('\n');

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Match function definitions
        const fnMatch = line.match(/^\s*((?:pub(?:\s+\([^)]+\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+))/);

        if (fnMatch) {
            const fullSignature = fnMatch[1];
            const methodName = fnMatch[2];

            methods.push({
                name: methodName,
                lineNumber: i,
                signature: fullSignature,
            });
        }
    }

    return methods;
}

function addJSCommentsToRustFile(rustPath, tsPath) {
    if (!fs.existsSync(rustPath)) {
        return;
    }

    if (!fs.existsSync(tsPath)) {
        console.log(`  ⊘ Skipping ${path.basename(rustPath)} (no TS file: ${path.basename(tsPath)})`);
        return;
    }

    const rustContent = fs.readFileSync(rustPath, 'utf-8');
    const tsMethods = extractTSMethodsUsingCompiler(tsPath);
    const rustMethods = extractRustMethods(rustContent);

    const rustLines = rustContent.split('\n');
    let addedCount = 0;
    let skippedCount = 0;

    // Process in reverse order to maintain line numbers
    for (let i = rustMethods.length - 1; i >= 0; i--) {
        const rustMethod = rustMethods[i];

        // Check if comment already exists
        let hasExistingComment = false;
        for (let j = Math.max(0, rustMethod.lineNumber - 20); j < rustMethod.lineNumber; j++) {
            if (rustLines[j].includes('// TypeScript source:') ||
                rustLines[j].includes('// JavaScript source:')) {
                hasExistingComment = true;
                break;
            }
        }

        if (hasExistingComment) {
            skippedCount++;
            continue;
        }

        // Try to find matching TS method
        let tsSource = null;
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

        if (tsSource) {
            // Format as Rust comment
            const commentLines = ['// TypeScript source:']
                .concat(tsSource.split('\n').map(line => `// ${line}`))
                .concat(['//']);

            // Insert comment before the method
            const indent = rustLines[rustMethod.lineNumber].match(/^\s*/)[0];
            const indentedCommentLines = commentLines.map(line =>
                line.trim() ? indent + line : line
            );

            rustLines.splice(rustMethod.lineNumber, 0, ...indentedCommentLines);
            addedCount++;
        }
    }

    if (addedCount > 0) {
        fs.writeFileSync(rustPath, rustLines.join('\n'), 'utf-8');
        console.log(`  ✓ ${path.basename(rustPath).padEnd(35)} - Added ${addedCount} comments (skipped ${skippedCount})`);
    } else if (skippedCount > 0) {
        console.log(`  = ${path.basename(rustPath).padEnd(35)} - All comments exist (${skippedCount})`);
    }
}

function processAllFiles() {
    const workspaceDir = '/home/builder/workspace';
    const tsDir = '/home/builder/pokemon-showdown/sim';

    console.log('Adding TypeScript source comments to ALL Rust files...\n');

    // First process the mapped files
    console.log('Processing mapped files:');
    for (const [rustFile, tsFile] of Object.entries(FILE_MAPPINGS)) {
        const rustPath = path.join(workspaceDir, 'src', rustFile);
        const tsPath = path.join(tsDir, tsFile);

        try {
            addJSCommentsToRustFile(rustPath, tsPath);
        } catch (error) {
            console.error(`  ✗ Error processing ${rustFile}:`, error.message);
        }
    }

    // Now process any remaining files that weren't in the mapping
    console.log('\nProcessing other Rust files (looking for matches):');
    const processedFiles = new Set(Object.keys(FILE_MAPPINGS).map(f => path.join(workspaceDir, 'src', f)));

    function processDirectory(dir) {
        const entries = fs.readdirSync(dir, { withFileTypes: true });

        for (const entry of entries) {
            const fullPath = path.join(dir, entry.name);

            if (entry.isDirectory()) {
                processDirectory(fullPath);
            } else if (entry.name.endsWith('.rs')) {
                if (!processedFiles.has(fullPath)) {
                    // Try to find a matching TS file
                    const baseName = entry.name.replace('.rs', '');
                    const possibleTsFiles = [
                        path.join(tsDir, baseName + '.ts'),
                        path.join(tsDir, baseName.replace(/_/g, '-') + '.ts'),
                    ];

                    let tsPath = null;
                    for (const possiblePath of possibleTsFiles) {
                        if (fs.existsSync(possiblePath)) {
                            tsPath = possiblePath;
                            break;
                        }
                    }

                    if (tsPath) {
                        try {
                            addJSCommentsToRustFile(fullPath, tsPath);
                        } catch (error) {
                            console.error(`  ✗ Error processing ${entry.name}:`, error.message);
                        }
                    }
                }
            }
        }
    }

    processDirectory(path.join(workspaceDir, 'src'));

    console.log('\n✓ Done!');
}

// Main execution
if (require.main === module) {
    processAllFiles();
}
