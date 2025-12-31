#!/usr/bin/env node

/**
 * Smart fix for item dispatchers - reads actual parameter names from function signatures
 */

const fs = require('fs');
const path = require('path');

const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');

// Parse all item callback files to find what callbacks they implement
function getImplementedCallbacks() {
    const callbacksDir = path.join(workspaceRoot, 'src', 'data', 'item_callbacks');
    const files = fs.readdirSync(callbacksDir).filter(f => f.endsWith('.rs') && f !== 'mod.rs');

    const implemented = {}; // { callback_name: [item_ids] }

    for (const file of files) {
        const itemId = file.replace('.rs', '');
        const content = fs.readFileSync(path.join(callbacksDir, file), 'utf8');

        // Find all pub fn on_* declarations
        const funcRegex = /pub fn (on_\w+)\(/g;
        let match;
        while ((match = funcRegex.exec(content)) !== null) {
            const callbackName = match[1];
            if (!implemented[callbackName]) {
                implemented[callbackName] = [];
            }
            implemented[callbackName].push(itemId);
        }
    }

    return implemented;
}

// Extract parameter names from a dispatcher function
function extractDispatcherParams(content, funcName) {
    const dispatchFunc = `dispatch_${funcName}`;

    // Find the function signature
    const funcRegex = new RegExp(
        `pub fn ${dispatchFunc}\\(([\\s\\S]*?)\\) -> EventResult`,
        ''
    );

    const match = content.match(funcRegex);
    if (!match) {
        return null;
    }

    const paramsBlock = match[1];

    // Extract parameter names (handle both _battle and battle)
    const params = [];
    const paramLines = paramsBlock.split(',');

    for (const line of paramLines) {
        const paramMatch = line.match(/\b(_?\w+):\s*(?:&mut\s+)?[\w:]+/);
        if (paramMatch) {
            params.push(paramMatch[1].trim());
        }
    }

    return params;
}

// Get call parameters by checking an existing working example in the dispatcher
function getCallParamsFromExisting(content, funcName) {
    const dispatchFunc = `dispatch_${funcName}`;

    // Find existing match arms to see the pattern
    const regex = new RegExp(
        `pub fn ${dispatchFunc}\\([\\s\\S]*?\\) -> EventResult \\{[\\s\\S]*?match[\\s\\S]*?\\{([\\s\\S]*?)_ => EventResult::Continue`,
        ''
    );

    const match = content.match(regex);
    if (!match) {
        return null;
    }

    const matchBlock = match[1];

    // Find first non-empty match arm
    const armRegex = /"([^"]+)"\s*=>\s*\w+::on_\w+\(([^)]+)\)/;
    const armMatch = matchBlock.match(armRegex);

    if (armMatch) {
        // Return params without trailing comma
        let params = armMatch[2].trim();
        // Remove trailing comma if present
        if (params.endsWith(',')) {
            params = params.slice(0, -1).trim();
        }
        return params;
    }

    return null;
}

// Update mod.rs dispatchers
function updateDispatchers() {
    const modPath = path.join(workspaceRoot, 'src', 'data', 'item_callbacks', 'mod.rs');
    let content = fs.readFileSync(modPath, 'utf8');

    const implemented = getImplementedCallbacks();

    // For each callback, update the dispatcher
    for (const [funcName, items] of Object.entries(implemented)) {
        const dispatchFunc = `dispatch_${funcName}`;

        // Sort items alphabetically
        items.sort();

        // Get call parameters from existing example or build from signature
        let callParams = getCallParamsFromExisting(content, funcName);

        if (!callParams) {
            console.log(`⚠️  No existing example for ${dispatchFunc}, skipping`);
            continue;
        }

        // Find the dispatcher function
        const regex = new RegExp(
            `(pub fn ${dispatchFunc}\\([\\s\\S]*?\\) -> EventResult \\{[\\s\\S]*?match[\\s\\S]*?\\{)([\\s\\S]*?)(\\s+_ => EventResult::Continue,\\s+\\})`,
            ''
        );

        const match = content.match(regex);
        if (match) {
            // Generate new match arms with correct parameters
            const matchArms = items.map(item => {
                return `        "${item}" => ${item}::${funcName}(${callParams}),`;
            }).join('\n');

            const replacement = `${match[1]}\n${matchArms}\n${match[3]}`;
            content = content.replace(regex, replacement);

            console.log(`Updated ${dispatchFunc}: ${items.length} items`);
        } else {
            console.log(`⚠️  Could not find ${dispatchFunc} in mod.rs`);
        }
    }

    fs.writeFileSync(modPath, content);
    console.log('\n✓ Updated mod.rs');
}

console.log('Smart fixing item callback dispatchers...\n');

try {
    updateDispatchers();
    console.log('\n✓ All fixes applied successfully!');
    console.log('   Please run: cargo build');
} catch (error) {
    console.error('❌ Error:', error.message);
    console.trace(error);
    process.exit(1);
}
