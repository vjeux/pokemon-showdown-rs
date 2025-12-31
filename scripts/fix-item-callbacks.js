#!/usr/bin/env node

/**
 * Automatically fix item callback registration by updating:
 * 1. mod.rs dispatchers
 * 2. has_callback.rs item_has_callback function
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

// Convert on_residual to onResidual
function funcNameToEventName(funcName) {
    const parts = funcName.split('_');
    return 'on' + parts.slice(1).map(p => p.charAt(0).toUpperCase() + p.slice(1)).join('');
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

        // Find the dispatcher function - handle multi-line format with ID::from conversion
        const regex = new RegExp(
            `(pub fn ${dispatchFunc}\\([\\s\\S]*?\\) -> EventResult \\{[\\s\\S]*?match[\\s\\S]*?\\{)([\\s\\S]*?)(\\s+_ => EventResult::Continue,\\s+\\})`,
            ''
        );

        const match = content.match(regex);
        if (match) {
            // Generate new match arms
            const matchArms = items.map(item => {
                return `        "${item}" => ${item}::${funcName}(battle, pokemon_pos),`;
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

// Update has_callback.rs
function updateHasCallback() {
    const hasCallbackPath = path.join(workspaceRoot, 'src', 'battle', 'has_callback.rs');
    let content = fs.readFileSync(hasCallbackPath, 'utf8');

    const implemented = getImplementedCallbacks();

    // Find the item_has_callback function
    const funcStart = content.indexOf('fn item_has_callback');
    const funcEnd = content.lastIndexOf('\n    }\n', funcStart + 10000) + 6; // Find end of function
    const beforeFunc = content.substring(0, funcStart);
    const afterFunc = content.substring(funcEnd);

    // Build the new function content
    let funcContent = `fn item_has_callback(&self, item_id: &str, event_id: &str) -> bool {\n`;
    funcContent += `        // Gen 5+ special case: items use onStart during SwitchIn events\n`;
    funcContent += `        // This matches JavaScript getCallback() behavior (same logic as abilities)\n`;
    funcContent += `        // Items don't have onAnySwitchIn callbacks, so always check onStart if gen >= 5\n`;
    funcContent += `        if self.gen >= 5 && event_id == "onSwitchIn" {\n`;
    funcContent += `            // This recursive call is safe because event_id != "onSwitchIn" so won't trigger special case\n`;
    funcContent += `            if self.item_has_callback(item_id, "onStart") {\n`;
    funcContent += `                return true;\n`;
    funcContent += `            }\n`;
    funcContent += `        }\n\n`;
    funcContent += `        match event_id {\n`;

    // Sort callbacks alphabetically by event name
    const sortedCallbacks = Object.entries(implemented)
        .map(([funcName, items]) => [funcNameToEventName(funcName), items])
        .sort((a, b) => a[0].localeCompare(b[0]));

    for (const [eventName, items] of sortedCallbacks) {
        // Sort items alphabetically
        items.sort();

        funcContent += `            "${eventName}" => matches!(\n`;
        funcContent += `                item_id,\n`;

        // Split into lines of ~80 chars
        const itemsPerLine = [];
        let currentLine = [];
        let currentLength = 0;

        for (const item of items) {
            const itemStr = `"${item}"`;
            if (currentLength + itemStr.length + 3 > 80 && currentLine.length > 0) {
                itemsPerLine.push(currentLine.join(' | '));
                currentLine = [itemStr];
                currentLength = itemStr.length;
            } else {
                currentLine.push(itemStr);
                currentLength += itemStr.length + 3;
            }
        }
        if (currentLine.length > 0) {
            itemsPerLine.push(currentLine.join(' | '));
        }

        for (let i = 0; i < itemsPerLine.length; i++) {
            funcContent += `                ${itemsPerLine[i]}`;
            if (i < itemsPerLine.length - 1) {
                funcContent += ' |';
            }
            funcContent += '\n';
        }

        funcContent += `            ),\n`;
    }

    funcContent += `            _ => false,\n`;
    funcContent += `        }\n`;
    funcContent += `    }`;

    const newContent = beforeFunc + funcContent + afterFunc;
    fs.writeFileSync(hasCallbackPath, newContent);
    console.log('✓ Updated has_callback.rs');
}

console.log('Fixing item callback registration...\n');

try {
    updateDispatchers();
    updateHasCallback();
    console.log('\n✓ All fixes applied successfully!');
    console.log('   Please run: cargo build');
} catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
}
