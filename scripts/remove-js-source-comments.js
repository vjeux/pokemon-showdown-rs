#!/usr/bin/env node

/**
 * Script to remove "/// JS Source" comment blocks from battle.rs
 */

const fs = require('fs');
const path = require('path');

function removeJSSourceComments(filePath) {
    const content = fs.readFileSync(filePath, 'utf-8');
    const lines = content.split('\n');
    const newLines = [];

    let inJSSourceBlock = false;
    let removedCount = 0;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();

        // Check if this line starts a JS Source block
        if (trimmed.startsWith('/// JS Source')) {
            inJSSourceBlock = true;
            removedCount++;
            continue; // Skip this line
        }

        // If we're in a JS Source block, skip lines until we find a non-doc-comment
        if (inJSSourceBlock) {
            if (trimmed.startsWith('///')) {
                continue; // Skip this line
            } else {
                // We've reached the end of the block
                inJSSourceBlock = false;
                // Don't skip this line, it's the actual code
                newLines.push(line);
            }
        } else {
            newLines.push(line);
        }
    }

    fs.writeFileSync(filePath, newLines.join('\n'), 'utf-8');

    console.log(`Removed ${removedCount} JS Source comment blocks from ${path.basename(filePath)}`);
    return removedCount;
}

const battlePath = '/home/builder/workspace/src/battle.rs';
removeJSSourceComments(battlePath);
