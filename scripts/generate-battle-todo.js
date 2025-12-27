#!/usr/bin/env node

/**
 * Script to extract all Rust method signatures from battle*.rs files
 * and create BATTLE_TODO.md
 */

const fs = require('fs');
const path = require('path');

const BATTLE_FILES = [
    'src/battle.rs',
    'src/battle_actions.rs',
    'src/battle_queue.rs',
    'src/battle_stream.rs',
];

function extractMethods(filePath) {
    const content = fs.readFileSync(filePath, 'utf-8');
    const lines = content.split('\n');
    const methods = [];

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Match function definitions
        // Matches: pub fn, pub(crate) fn, fn, pub async fn, pub unsafe fn, etc.
        const fnMatch = line.match(/^\s*((?:pub(?:\s*\([^)]+\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+))/);

        if (fnMatch) {
            const fullSignature = fnMatch[1];
            const methodName = fnMatch[2];
            const isPublic = fullSignature.includes('pub');

            // Extract full signature including generics and parameters
            let signature = line.trim();
            let currentLine = i;

            // If the signature spans multiple lines, collect them
            while (currentLine < lines.length && !signature.includes('{') && !signature.includes(';')) {
                currentLine++;
                if (currentLine < lines.length) {
                    signature += ' ' + lines[currentLine].trim();
                }
            }

            // Remove the opening brace or semicolon
            signature = signature.replace(/\s*\{.*$/, '').replace(/\s*;.*$/, '');

            methods.push({
                name: methodName,
                signature: signature,
                isPublic: isPublic,
                lineNumber: i + 1,
            });
        }
    }

    return methods;
}

function generateMarkdown() {
    let markdown = '# Battle Module TODO - Method Reference\n\n';
    markdown += 'This file contains all Rust methods from battle*.rs files.\n';
    markdown += 'Use this as a reference to track which methods need TypeScript source comments.\n\n';
    markdown += `Generated: ${new Date().toISOString().split('T')[0]}\n\n`;
    markdown += '---\n\n';

    for (const file of BATTLE_FILES) {
        const filePath = `/home/builder/workspace/${file}`;

        if (!fs.existsSync(filePath)) {
            console.log(`Skipping ${file} - not found`);
            continue;
        }

        const methods = extractMethods(filePath);
        const publicMethods = methods.filter(m => m.isPublic);
        const privateMethods = methods.filter(m => !m.isPublic);

        markdown += `## ${path.basename(file)}\n\n`;
        markdown += `**Total methods:** ${methods.length} (${publicMethods.length} public, ${privateMethods.length} private)\n\n`;

        if (publicMethods.length > 0) {
            markdown += '### Public Methods\n\n';
            for (const method of publicMethods) {
                markdown += `- [ ] \`${method.name}\` (line ${method.lineNumber})\n`;
            }
            markdown += '\n';
        }

        if (privateMethods.length > 0) {
            markdown += '### Private Methods\n\n';
            for (const method of privateMethods) {
                markdown += `- [ ] \`${method.name}\` (line ${method.lineNumber})\n`;
            }
            markdown += '\n';
        }

        markdown += '---\n\n';
    }

    // Add summary statistics
    markdown += '## Summary Statistics\n\n';
    let totalMethods = 0;
    let totalPublic = 0;
    let totalPrivate = 0;

    for (const file of BATTLE_FILES) {
        const filePath = `/home/builder/workspace/${file}`;
        if (fs.existsSync(filePath)) {
            const methods = extractMethods(filePath);
            const publicCount = methods.filter(m => m.isPublic).length;
            const privateCount = methods.filter(m => !m.isPublic).length;

            markdown += `- **${path.basename(file)}**: ${methods.length} methods (${publicCount} public, ${privateCount} private)\n`;

            totalMethods += methods.length;
            totalPublic += publicCount;
            totalPrivate += privateCount;
        }
    }

    markdown += `\n**Grand Total:** ${totalMethods} methods (${totalPublic} public, ${totalPrivate} private)\n`;

    return markdown;
}

const markdown = generateMarkdown();
fs.writeFileSync('/home/builder/workspace/BATTLE_TODO.md', markdown, 'utf-8');

console.log('✓ Generated BATTLE_TODO.md');
console.log('');

// Print summary
const lines = markdown.split('\n');
const summaryStart = lines.findIndex(l => l.includes('## Summary Statistics'));
if (summaryStart !== -1) {
    console.log(lines.slice(summaryStart).join('\n'));
}
