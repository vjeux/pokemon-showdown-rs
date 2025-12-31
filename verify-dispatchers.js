#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Parse dispatcher functions from move_callbacks/mod.rs
function parseMoveDispatchers(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    const dispatchers = {};

    // Find all dispatch_on_* functions (not dispatch_condition_*)
    const dispatcherRegex = /pub fn (dispatch_on_\w+)\([^)]+\) -> EventResult \{[^}]*match move_id \{([^}]+?)\n\s+_\s*=>/gs;

    let match;
    while ((match = dispatcherRegex.exec(content)) !== null) {
        const functionName = match[1];
        const matchBlock = match[2];

        // Skip condition dispatchers
        if (functionName.includes('condition')) continue;

        // Extract event name (e.g., "dispatch_on_hit" -> "Hit")
        let eventName = functionName.replace('dispatch_on_', '');
        // Convert snake_case to PascalCase
        eventName = eventName.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join('');

        // Extract move IDs from the match block
        const moveIds = [];
        const moveRegex = /"([^"]+)"\s*=>/g;
        let moveMatch;
        while ((moveMatch = moveRegex.exec(matchBlock)) !== null) {
            moveIds.push(moveMatch[1]);
        }

        dispatchers[eventName] = moveIds;
    }

    return dispatchers;
}

// Parse condition dispatchers from condition_callbacks.rs
function parseConditionDispatchers(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    const dispatchers = {};

    // Find dispatch_on_* functions for conditions
    // Look for pattern: if condition_id != "something" { return ...; }
    const dispatcherRegex = /pub fn dispatch_on_(\w+)\([^)]*condition_id: &str[^)]*\) -> EventResult \{[\s\S]*?if condition_id != "(\w+)"/g;

    let match;
    while ((match = dispatcherRegex.exec(content)) !== null) {
        const eventName = match[1];
        const conditionId = match[2];

        // Convert snake_case to PascalCase
        const eventNamePascal = eventName.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join('');

        if (!dispatchers[eventNamePascal]) {
            dispatchers[eventNamePascal] = [];
        }
        dispatchers[eventNamePascal].push(conditionId);
    }

    return dispatchers;
}

// Parse condition dispatchers from move_callbacks/mod.rs
function parseConditionDispatchersFromMoveCallbacks(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    const dispatchers = {};

    // Find dispatch_condition_on_* functions
    const dispatcherRegex = /pub fn dispatch_condition_on_(\w+)\([^)]+\) -> EventResult \{[^}]*match condition_id \{([^}]+?)\n\s+_\s*=>/gs;

    let match;
    while ((match = dispatcherRegex.exec(content)) !== null) {
        const eventName = match[1];
        const matchBlock = match[2];

        // Convert snake_case to PascalCase
        const eventNamePascal = eventName.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join('');

        // Extract condition IDs from the match block
        const conditionIds = [];
        const conditionRegex = /"([^"]+)"\s*=>/g;
        let conditionMatch;
        while ((conditionMatch = conditionRegex.exec(matchBlock)) !== null) {
            conditionIds.push(conditionMatch[1]);
        }

        if (!dispatchers[eventNamePascal]) {
            dispatchers[eventNamePascal] = [];
        }
        dispatchers[eventNamePascal].push(...conditionIds);
    }

    return dispatchers;
}

// Parse has_callback function from has_callback.rs
function parseHasCallback(filePath, effectType) {
    const content = fs.readFileSync(filePath, 'utf8');
    const callbacks = {};

    // Find the appropriate has_callback function
    const functionPattern = effectType === 'move'
        ? /pub fn move_has_callback\(&self, move_id: &str, event_id: &str\) -> bool \{[\s\S]*?match normalized \{([\s\S]*?)\n\s+_\s*=>/
        : effectType === 'condition'
        ? /pub fn condition_has_callback\(&self, condition_id: &str, event_id: &str\) -> bool \{[\s\S]*?match normalized \{([\s\S]*?)\n\s+_\s*=>/
        : null;

    if (!functionPattern) return callbacks;

    const functionMatch = content.match(functionPattern);
    if (!functionMatch) return callbacks;

    const matchBlock = functionMatch[1];

    // Extract event -> effect_ids mappings
    const eventRegex = /"(\w+)"\s*=>\s*matches!\(\s*(?:move_id|condition_id|ability_id|item_id),\s*([^)]+)\)/gs;

    let match;
    while ((match = eventRegex.exec(matchBlock)) !== null) {
        const eventName = match[1];
        const effectsBlock = match[2];

        // Extract effect IDs
        const effectIds = [];
        const effectRegex = /"([^"]+)"/g;
        let effectMatch;
        while ((effectMatch = effectRegex.exec(effectsBlock)) !== null) {
            effectIds.push(effectMatch[1]);
        }

        callbacks[eventName] = effectIds;
    }

    return callbacks;
}

// Compare and report discrepancies
function compareDispatchersAndCallbacks(dispatchers, callbacks, effectType) {
    console.log(`\n=== Verifying ${effectType} dispatchers and has_callback ===\n`);

    let hasErrors = false;

    // Check all events in dispatchers
    const allEvents = new Set([...Object.keys(dispatchers), ...Object.keys(callbacks)]);

    for (const event of Array.from(allEvents).sort()) {
        const dispatcherEffects = new Set(dispatchers[event] || []);
        const callbackEffects = new Set(callbacks[event] || []);

        // Find effects only in dispatcher
        const onlyInDispatcher = [...dispatcherEffects].filter(e => !callbackEffects.has(e));
        // Find effects only in has_callback
        const onlyInCallback = [...callbackEffects].filter(e => !dispatcherEffects.has(e));

        if (onlyInDispatcher.length > 0 || onlyInCallback.length > 0) {
            hasErrors = true;
            console.log(`❌ Event: ${event}`);

            if (onlyInDispatcher.length > 0) {
                console.log(`   In dispatcher but NOT in has_callback:`);
                onlyInDispatcher.sort().forEach(e => console.log(`     - ${e}`));
            }

            if (onlyInCallback.length > 0) {
                console.log(`   In has_callback but NOT in dispatcher:`);
                onlyInCallback.sort().forEach(e => console.log(`     - ${e}`));
            }

            console.log();
        } else if (dispatcherEffects.size > 0) {
            console.log(`✅ Event: ${event} (${dispatcherEffects.size} ${effectType}s)`);
        }
    }

    return !hasErrors;
}

// Main
function main() {
    const modRsPath = path.join(__dirname, 'src/data/move_callbacks/mod.rs');
    const conditionCallbacksPath = path.join(__dirname, 'src/data/condition_callbacks.rs');
    const hasCallbackPath = path.join(__dirname, 'src/battle/has_callback.rs');

    console.log('Parsing move dispatchers...');
    const moveDispatchers = parseMoveDispatchers(modRsPath);

    console.log('Parsing condition dispatchers from condition_callbacks.rs...');
    const conditionDispatchers1 = parseConditionDispatchers(conditionCallbacksPath);

    console.log('Parsing condition dispatchers from move_callbacks/mod.rs...');
    const conditionDispatchers2 = parseConditionDispatchersFromMoveCallbacks(modRsPath);

    // Merge condition dispatchers
    const conditionDispatchers = {};
    for (const [event, conditions] of Object.entries(conditionDispatchers1)) {
        conditionDispatchers[event] = conditions;
    }
    for (const [event, conditions] of Object.entries(conditionDispatchers2)) {
        if (!conditionDispatchers[event]) {
            conditionDispatchers[event] = [];
        }
        conditionDispatchers[event].push(...conditions);
    }

    console.log('Parsing move has_callback...');
    const moveCallbacks = parseHasCallback(hasCallbackPath, 'move');

    console.log('Parsing condition has_callback...');
    const conditionCallbacks = parseHasCallback(hasCallbackPath, 'condition');

    const moveSuccess = compareDispatchersAndCallbacks(moveDispatchers, moveCallbacks, 'move');
    const conditionSuccess = compareDispatchersAndCallbacks(conditionDispatchers, conditionCallbacks, 'condition');

    if (moveSuccess && conditionSuccess) {
        console.log('\n✅ All dispatchers and has_callback are in sync!\n');
        process.exit(0);
    } else {
        console.log('\n❌ Found discrepancies between dispatchers and has_callback\n');
        process.exit(1);
    }
}

main();
