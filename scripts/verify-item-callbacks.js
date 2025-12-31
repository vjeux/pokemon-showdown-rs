#!/usr/bin/env node

/**
 * Verify that all item callbacks are properly registered in:
 * 1. Dispatcher (mod.rs dispatch_on_* functions)
 * 2. has_callback (has_callback.rs item_has_callback function)
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

// Parse mod.rs to find what's in dispatchers
function getDispatchedCallbacks() {
    const modPath = path.join(workspaceRoot, 'src', 'data', 'item_callbacks', 'mod.rs');
    const content = fs.readFileSync(modPath, 'utf8');

    const dispatched = {}; // { callback_name: [item_ids] }

    // Find all dispatch_on_* functions - handle multi-line format
    const dispatchRegex = /pub fn (dispatch_on_\w+)\([\s\S]*?\) -> EventResult \{[\s\S]*?match[\s\S]*?\{([\s\S]*?)\s+_ => EventResult::Continue,\s+\}/g;

    let match;
    while ((match = dispatchRegex.exec(content)) !== null) {
        const dispatchFunc = match[1];
        const callbackName = dispatchFunc.replace('dispatch_', '');
        const matchBlock = match[2];

        // Extract item IDs from match arms like: "leftovers" => leftovers::on_residual
        const itemRegex = /"([^"]+)"\s*=>/g;
        const items = [];
        let itemMatch;
        while ((itemMatch = itemRegex.exec(matchBlock)) !== null) {
            items.push(itemMatch[1]);
        }

        dispatched[callbackName] = items;
    }

    return dispatched;
}

// Parse has_callback.rs to find what's in item_has_callback
function getHasCallbackItems() {
    const hasCallbackPath = path.join(workspaceRoot, 'src', 'battle', 'has_callback.rs');
    const content = fs.readFileSync(hasCallbackPath, 'utf8');

    const hasCallback = {}; // { callback_name: [item_ids] }

    // Find item_has_callback function
    const funcStart = content.indexOf('fn item_has_callback');
    const funcEnd = content.indexOf('\n    }', funcStart + 1000); // Find end of function
    const funcContent = content.substring(funcStart, funcEnd);

    // Find all callback matches like: "onResidual" => matches!(item_id, "ejectpack" | "leftovers" ...)
    const matchRegex = /"(on\w+)"\s*=>\s*matches!\([^,]+,\s*([^)]+)\)/g;

    let match;
    while ((match = matchRegex.exec(funcContent)) !== null) {
        const callbackName = match[1];
        const itemsStr = match[2];

        // Extract item IDs from the matches! pattern like: "ejectpack" | "leftovers" | "mirrorherb"
        const itemRegex = /"([^"]+)"/g;
        const items = [];
        let itemMatch;
        while ((itemMatch = itemRegex.exec(itemsStr)) !== null) {
            items.push(itemMatch[1]);
        }

        hasCallback[callbackName] = items;
    }

    return hasCallback;
}

// Convert snake_case to camelCase for comparison
function snakeToCamel(str) {
    return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

// Convert on_residual to onResidual
function funcNameToEventName(funcName) {
    // on_residual -> onResidual
    const parts = funcName.split('_');
    return 'on' + parts.slice(1).map(p => p.charAt(0).toUpperCase() + p.slice(1)).join('');
}

console.log('Verifying item callback registration...\n');

const implemented = getImplementedCallbacks();
const dispatched = getDispatchedCallbacks();
const hasCallback = getHasCallbackItems();

let hasErrors = false;

// Check each implemented callback
for (const [funcName, items] of Object.entries(implemented)) {
    const eventName = funcNameToEventName(funcName);
    const dispatchKey = funcName;

    console.log(`\n=== ${eventName} (${items.length} items) ===`);

    // Check dispatcher
    const dispatchedItems = dispatched[dispatchKey] || [];
    const missingFromDispatcher = items.filter(item => !dispatchedItems.includes(item));

    if (missingFromDispatcher.length > 0) {
        console.log(`❌ Missing from dispatcher (mod.rs):`);
        console.log(`   ${missingFromDispatcher.join(', ')}`);
        hasErrors = true;
    } else {
        console.log(`✓ All items in dispatcher`);
    }

    // Check has_callback
    const hasCallbackItems = hasCallback[eventName] || [];
    const missingFromHasCallback = items.filter(item => !hasCallbackItems.includes(item));

    if (missingFromHasCallback.length > 0) {
        console.log(`❌ Missing from has_callback (has_callback.rs):`);
        console.log(`   ${missingFromHasCallback.join(', ')}`);
        hasErrors = true;
    } else {
        console.log(`✓ All items in has_callback`);
    }

    // Check for extras in dispatcher (items in dispatcher but not implemented)
    const extraInDispatcher = dispatchedItems.filter(item => !items.includes(item));
    if (extraInDispatcher.length > 0) {
        console.log(`⚠️  Extra in dispatcher (not implemented):`);
        console.log(`   ${extraInDispatcher.join(', ')}`);
    }

    // Check for extras in has_callback
    const extraInHasCallback = hasCallbackItems.filter(item => !items.includes(item));
    if (extraInHasCallback.length > 0) {
        console.log(`⚠️  Extra in has_callback (not implemented):`);
        console.log(`   ${extraInHasCallback.join(', ')}`);
    }
}

console.log('\n');

if (hasErrors) {
    console.log('❌ ERRORS FOUND - Some items are not properly registered!');
    console.log('   Please update:');
    console.log('   1. src/data/item_callbacks/mod.rs (dispatchers)');
    console.log('   2. src/battle/has_callback.rs (item_has_callback function)');
    process.exit(1);
} else {
    console.log('✓ All implemented item callbacks are properly registered!');
    process.exit(0);
}
