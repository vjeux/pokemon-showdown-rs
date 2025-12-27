#!/usr/bin/env node

/**
 * Script to add JavaScript/TypeScript source code as comments above Rust methods
 * Uses TypeScript compiler API for robust parsing
 *
 * Usage: node scripts/add-js-comments-ts.js
 */

const fs = require('fs');
const path = require('path');
const ts = require('typescript');

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

function camelToSnake(str) {
    return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

function snakeToCamel(str) {
    return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

function extractTSMethodsUsingCompiler(filePath) {
    const methods = new Map();
    const sourceCode = fs.readFileSync(filePath, 'utf-8');

    // Create source file
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

    // Get JSDoc comments
    const jsDocComments = ts.getJSDocCommentsAndTags(node);
    let commentStart = fullStart;

    if (jsDocComments && jsDocComments.length > 0) {
        commentStart = jsDocComments[0].pos;
    }

    // Get leading trivia (comments)
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

    // Add without prefixes
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
    console.log(`\nProcessing ${path.basename(rustPath)} with ${path.basename(tsPath)}...`);

    if (!fs.existsSync(rustPath)) {
        console.error(`  ✗ Rust file not found: ${rustPath}`);
        return;
    }

    if (!fs.existsSync(tsPath)) {
        console.error(`  ✗ TypeScript file not found: ${tsPath}`);
        return;
    }

    const rustContent = fs.readFileSync(rustPath, 'utf-8');
    const tsMethods = extractTSMethodsUsingCompiler(tsPath);
    const rustMethods = extractRustMethods(rustContent);

    console.log(`  Found ${tsMethods.size} unique TypeScript methods`);
    console.log(`  Found ${rustMethods.length} Rust methods`);

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
        console.log(`  ✓ Added ${addedCount} comments, skipped ${skippedCount}`);
    } else {
        console.log(`  = No changes (skipped ${skippedCount})`);
    }
}

function processAllFiles() {
    const workspaceDir = '/home/builder/workspace';
    const tsDir = '/home/builder/pokemon-showdown/sim';

    console.log('Adding TypeScript source comments to Rust files...\n');

    for (const [rustFile, tsFile] of Object.entries(FILE_MAPPINGS)) {
        const rustPath = path.join(workspaceDir, 'src', rustFile);
        const tsPath = path.join(tsDir, tsFile);

        try {
            addJSCommentsToRustFile(rustPath, tsPath);
        } catch (error) {
            console.error(`Error processing ${rustFile}:`, error.message);
        }
    }

    console.log('\n✓ Done!');
}

// Main execution
if (require.main === module) {
    processAllFiles();
}
