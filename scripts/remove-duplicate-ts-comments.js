#!/usr/bin/env node

/**
 * Script to remove duplicate TypeScript source comment blocks
 * Keeps only the first occurrence of each duplicate
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

function findRustFiles(dir) {
    const files = [];
    const entries = fs.readdirSync(dir, { withFileTypes: true });

    for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isDirectory()) {
            files.push(...findRustFiles(fullPath));
        } else if (entry.name.endsWith('.rs')) {
            files.push(fullPath);
        }
    }

    return files;
}

function extractTSCommentBlocks(content) {
    const blocks = [];
    const lines = content.split('\n');

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Look for "// TypeScript source:" marker
        if (line.trim() === '// TypeScript source:') {
            const blockStart = i;
            let blockEnd = i;

            // Find the end of the comment block
            // It ends with a line that is just "//" or a non-comment line
            for (let j = i + 1; j < lines.length; j++) {
                const currentLine = lines[j];
                const trimmed = currentLine.trim();

                // If we hit a non-comment line, we're done
                if (!trimmed.startsWith('//')) {
                    blockEnd = j - 1;
                    break;
                }

                // If we hit the closing "//" marker, we're done
                if (trimmed === '//') {
                    blockEnd = j;
                    break;
                }
            }

            // Extract the full block including the marker and closing
            const blockLines = lines.slice(blockStart, blockEnd + 1);
            const blockText = blockLines.join('\n');

            // Create a hash of the actual TypeScript code (excluding the marker and closing)
            const tsCodeLines = blockLines.slice(1, -1); // Skip "// TypeScript source:" and "//"
            const tsCode = tsCodeLines.map(l => l.replace(/^\/\/\s?/, '')).join('\n');
            const hash = crypto.createHash('md5').update(tsCode).digest('hex');

            blocks.push({
                start: blockStart,
                end: blockEnd,
                lines: blockLines,
                text: blockText,
                hash: hash,
                tsCode: tsCode,
            });

            // Skip past this block
            i = blockEnd;
        }
    }

    return blocks;
}

function removeDuplicates(filePath) {
    const content = fs.readFileSync(filePath, 'utf-8');
    const blocks = extractTSCommentBlocks(content);

    if (blocks.length === 0) {
        return { removed: 0, kept: 0 };
    }

    // Track which hashes we've seen
    const seenHashes = new Map(); // hash -> first occurrence block
    const blocksToRemove = [];

    for (const block of blocks) {
        if (seenHashes.has(block.hash)) {
            // This is a duplicate
            blocksToRemove.push(block);
        } else {
            // First occurrence
            seenHashes.set(block.hash, block);
        }
    }

    if (blocksToRemove.length === 0) {
        return { removed: 0, kept: blocks.length };
    }

    // Remove duplicates by rebuilding the file
    const lines = content.split('\n');
    const newLines = [];

    let i = 0;
    while (i < lines.length) {
        // Check if this line starts a block to remove
        const blockToRemove = blocksToRemove.find(b => b.start === i);

        if (blockToRemove) {
            // Skip this entire block
            i = blockToRemove.end + 1;
        } else {
            // Keep this line
            newLines.push(lines[i]);
            i++;
        }
    }

    // Write back
    fs.writeFileSync(filePath, newLines.join('\n'), 'utf-8');

    return {
        removed: blocksToRemove.length,
        kept: seenHashes.size,
    };
}

function main() {
    const srcDir = '/home/builder/workspace/src';
    console.log('Removing duplicate TypeScript comment blocks...\n');

    const rustFiles = findRustFiles(srcDir);
    let totalRemoved = 0;
    let totalKept = 0;

    for (const file of rustFiles) {
        const result = removeDuplicates(file);

        if (result.removed > 0) {
            const relativePath = path.relative(srcDir, file);
            console.log(`  ✓ ${relativePath.padEnd(40)} - Removed ${result.removed} duplicates (kept ${result.kept})`);
            totalRemoved += result.removed;
        }

        totalKept += result.kept;
    }

    console.log(`\n✓ Done!`);
    console.log(`  Total duplicates removed: ${totalRemoved}`);
    console.log(`  Total unique blocks kept: ${totalKept}`);
}

if (require.main === module) {
    main();
}
