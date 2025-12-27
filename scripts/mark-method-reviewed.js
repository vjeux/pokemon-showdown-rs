#!/usr/bin/env node

/**
 * Script to mark methods as reviewed in BATTLE_TODO.md
 * Usage: node scripts/mark-method-reviewed.js "method_name" line_number
 */

const fs = require('fs');

const methodName = process.argv[2];
const lineNumber = process.argv[3];

if (!methodName || !lineNumber) {
    console.error('Usage: node scripts/mark-method-reviewed.js "method_name" line_number');
    process.exit(1);
}

const todoPath = '/home/builder/workspace/BATTLE_TODO.md';
const content = fs.readFileSync(todoPath, 'utf-8');

// Find and replace the checkbox for this method
const pattern = `- \\[ \\] \`${methodName}\` \\(line ${lineNumber}\\)`;
const replacement = `- [x] \`${methodName}\` (line ${lineNumber})`;

const newContent = content.replace(new RegExp(pattern), replacement);

if (newContent === content) {
    console.error(`Method \`${methodName}\` at line ${lineNumber} not found in BATTLE_TODO.md`);
    process.exit(1);
}

fs.writeFileSync(todoPath, newContent, 'utf-8');
console.log(`✓ Marked \`${methodName}\` (line ${lineNumber}) as reviewed`);
