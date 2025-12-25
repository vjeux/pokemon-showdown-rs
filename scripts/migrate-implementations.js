#!/usr/bin/env node

/**
 * Migrate implemented abilities from batch files to individual files
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');

// Get the old batch files from git
const batches = [1, 2, 3, 4];
const oldBatchContents = {};

for (const batchNum of batches) {
    try {
        const content = execSync(`git show HEAD:src/data/ability_callbacks/batch_${batchNum}.rs`, {
            cwd: workspaceRoot,
            encoding: 'utf8'
        });
        oldBatchContents[batchNum] = content;
    } catch (e) {
        console.error(`Failed to get batch_${batchNum}.rs from git:`, e.message);
    }
}

// Extract ability modules from batch files
function extractAbilityModules(batchContent) {
    const modules = {};

    // Match: pub mod ability_name { ... }
    // This regex finds the module name and its entire content
    const moduleRegex = /^pub mod (r#)?([a-z0-9_]+) \{([\s\S]*?)^}\s*$/gm;

    let match;
    while ((match = moduleRegex.exec(batchContent)) !== null) {
        const abilityName = match[2];
        const moduleContent = match[3];

        // Check if it's implemented (not just a TODO stub)
        if (!moduleContent.includes('// TODO: Implement 1-to-1 from JS')) {
            modules[abilityName] = {
                fullModule: match[0],
                content: moduleContent
            };
        }
    }

    return modules;
}

// Extract all implemented modules from all batches
const allImplementedModules = {};
for (const [batchNum, content] of Object.entries(oldBatchContents)) {
    const modules = extractAbilityModules(content);
    Object.assign(allImplementedModules, modules);
}

console.log(`Found ${Object.keys(allImplementedModules).length} implemented ability modules`);
console.log('Implemented abilities:', Object.keys(allImplementedModules).sort().join(', '));

// Now update the individual files
const abilitiesDir = path.join(workspaceRoot, 'src', 'data', 'ability_callbacks');

let updatedCount = 0;
for (const [abilityName, moduleData] of Object.entries(allImplementedModules)) {
    const filePath = path.join(abilitiesDir, `${abilityName}.rs`);

    if (!fs.existsSync(filePath)) {
        console.warn(`Warning: File ${abilityName}.rs doesn't exist, skipping`);
        continue;
    }

    const currentContent = fs.readFileSync(filePath, 'utf8');

    // Extract the header comments (//! lines) from current file
    const headerLines = [];
    for (const line of currentContent.split('\n')) {
        if (line.startsWith('//!')) {
            headerLines.push(line);
        } else if (line.trim() === '') {
            headerLines.push(line);
        } else {
            break;
        }
    }

    // Extract the use statements from the old module
    const useStatements = [
        'use crate::battle::{Battle, Arg};',
        'use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};',
        'use crate::pokemon::Pokemon;',
        'use crate::dex_data::ID;',
        'use super::{AbilityHandlerResult, Status, Effect};'
    ];

    // Clean up the module content - remove "use super::*;" and any other duplicate imports
    let cleanedContent = moduleData.content
        .replace(/^\s*use super::\*;\s*$/gm, '')
        .replace(/^\s*use crate::data::moves::MoveCategory;\s*$/gm, '')
        .replace(/^\s*use crate::data::moves::MoveTargetType;\s*$/gm, '')
        .trim();

    // Add use statements to condition module if it exists
    cleanedContent = cleanedContent.replace(
        /(pub mod condition \{\s*)/,
        `$1\n        use super::*;\n`
    );

    // Build new file content
    const newContent = [
        ...headerLines,
        '',
        ...useStatements,
        '',
        cleanedContent,
        ''
    ].join('\n');

    fs.writeFileSync(filePath, newContent);
    updatedCount++;
}

console.log(`\nMigration complete!`);
console.log(`- ${updatedCount} ability files updated with implementations`);
