#!/usr/bin/env node

/**
 * Generate can_dispatch methods for has_callback.rs
 * This script parses the dispatcher match statements in mod.rs files
 * and generates the corresponding can_dispatch boolean check methods.
 *
 * Run: node scripts/generate-can-dispatch.js
 */

const fs = require('fs');
const path = require('path');

const workspaceRoot = process.env.WORKSPACE_ROOT || path.join(__dirname, '..');

// Convert snake_case to camelCase for event names
function snakeToCamel(str) {
    return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

// Parse a dispatcher function to extract event name and IDs
function parseDispatcher(dispatcherCode) {
    // Extract function name: pub fn dispatch_on_xxx
    const funcMatch = dispatcherCode.match(/pub fn dispatch_(on_\w+|base_power)\(/);
    if (!funcMatch) return null;

    const fullFunc = funcMatch[1];

    // Skip Priority/Order/SubOrder variants - these are not separate events
    if (fullFunc.endsWith('_priority') || fullFunc.endsWith('_order') || fullFunc.endsWith('_sub_order')) {
        return null;
    }

    const eventSnake = fullFunc;
    // Convert to event name: on_start -> onStart, on_base_power -> BasePower
    let eventName;
    if (eventSnake === 'on_base_power' || eventSnake === 'on_source_base_power' ||
        eventSnake === 'on_ally_base_power' || eventSnake === 'on_any_base_power') {
        // Special case: on_base_power dispatcher handles "BasePower" event (no "on" prefix)
        // on_ally_base_power -> AllyBasePower, on_source_base_power -> SourceBasePower, etc.
        const parts = eventSnake.split('_');
        eventName = parts.slice(1).map((word, i) =>
            word.charAt(0).toUpperCase() + word.slice(1)
        ).join('');
    } else {
        // Normal case: on_start -> onStart
        eventName = 'on' + eventSnake.slice(3).split('_').map((word, i) =>
            i === 0 ? word.charAt(0).toUpperCase() + word.slice(1)
            : word.charAt(0).toUpperCase() + word.slice(1)
          ).join('');
    }

    // Extract all match arms: "id" => module::function(...)
    const matchRegex = /"([^"]+)"\s*=>/g;
    const ids = [];
    let match;
    while ((match = matchRegex.exec(dispatcherCode)) !== null) {
        ids.push(match[1]);
    }

    return { eventName, ids };
}

// Parse all dispatchers from a mod.rs file
function parseModFile(filePath, type) {
    const content = fs.readFileSync(filePath, 'utf8');

    // Find all dispatcher functions
    const dispatcherRegex = /\/\/\/ Dispatch[\s\S]*?pub fn dispatch_\w+\([\s\S]*?\n\}/g;
    const dispatchers = content.match(dispatcherRegex) || [];

    const eventMap = new Map(); // eventName -> [ids]

    for (const dispatcher of dispatchers) {
        const parsed = parseDispatcher(dispatcher);
        if (parsed) {
            const { eventName, ids } = parsed;
            if (!eventMap.has(eventName)) {
                eventMap.set(eventName, []);
            }
            eventMap.get(eventName).push(...ids);
        }
    }

    // Remove duplicates
    for (const [eventName, ids] of eventMap.entries()) {
        eventMap.set(eventName, [...new Set(ids)].sort());
    }

    return eventMap;
}

// Generate can_dispatch method from event map
function generateCanDispatchMethod(eventMap, effectType, paramName) {
    const events = Array.from(eventMap.keys()).sort();

    let code = `    /// Check if ${effectType === 'an' ? 'an' : 'a'} ${effectType} has a callback for an event\n`;
    code += `    fn ${paramName}_has_callback(&self, ${paramName}_id: &str, event_id: &str) -> bool {\n`;

    if (events.length === 0) {
        code += `        // No ${paramName} dispatchers implemented yet\n`;
        code += `        false\n`;
        code += `    }\n`;
        return code;
    }

    code += `        match event_id {\n`;

    for (const eventName of events) {
        const ids = eventMap.get(eventName);

        // Skip events with no IDs
        if (ids.length === 0) {
            continue;
        }

        code += `            "${eventName}" => matches!(\n`;
        code += `                ${paramName}_id,\n`;

        // Split into groups of 5 for readability
        for (let i = 0; i < ids.length; i += 5) {
            const group = ids.slice(i, i + 5);
            const line = group.map(id => `"${id}"`).join(' | ');
            code += `                ${line}`;
            if (i + 5 < ids.length) {
                code += ' |\n';
            }
        }

        code += `\n            ),\n`;
    }

    code += `            _ => false,\n`;
    code += `        }\n`;
    code += `    }\n`;

    return code;
}

// Main execution
console.log('Parsing dispatcher files...\n');

// Parse ability dispatchers
const abilityModPath = path.join(workspaceRoot, 'src', 'data', 'ability_callbacks', 'mod.rs');
const abilityEvents = parseModFile(abilityModPath, 'ability');
console.log(`Found ${abilityEvents.size} ability event types`);

// Parse item dispatchers
const itemModPath = path.join(workspaceRoot, 'src', 'data', 'item_callbacks', 'mod.rs');
const itemEvents = parseModFile(itemModPath, 'item');
console.log(`Found ${itemEvents.size} item event types`);

// Parse move dispatchers
const moveModPath = path.join(workspaceRoot, 'src', 'data', 'move_callbacks', 'mod.rs');
const moveEvents = parseModFile(moveModPath, 'move');
console.log(`Found ${moveEvents.size} move event types\n`);

// Generate has_callback.rs
const hasCallbackCode = `use crate::*;

impl Battle {

    /// Check if an effect has a callback for a specific event
    /// This is a Rust helper to replicate JavaScript's getCallback() check
    /// without actually executing the callback
    ///
    /// Returns true if the effect has a handler for the event, false otherwise
    pub fn has_callback(&self, effect_id: &ID, event_id: &str) -> bool {
        let effect_str = effect_id.as_str();

        // Check abilities
        if self.dex.abilities().get(effect_str).is_some() {
            return self.ability_has_callback(effect_str, event_id);
        }

        // Check items
        if self.dex.items().get(effect_str).is_some() {
            return self.item_has_callback(effect_str, event_id);
        }

        // Check moves
        if self.dex.moves().get(effect_str).is_some() {
            return self.move_has_callback(effect_str, event_id);
        }

        // Check conditions (status, volatile, weather, terrain)
        if crate::data::conditions::get_condition(effect_id).is_some() {
            return self.condition_has_callback(effect_str, event_id);
        }

        // Check species - species can have callbacks like onSwitchIn for form changes
        if self.dex.species().get(effect_str).is_some() {
            return self.species_has_callback(effect_str, event_id);
        }

        false
    }

${generateCanDispatchMethod(abilityEvents, 'an ability', 'ability')}
${generateCanDispatchMethod(itemEvents, 'an item', 'item')}
${generateCanDispatchMethod(moveEvents, 'a move', 'move')}
    /// Check if a condition has a callback for an event
    fn condition_has_callback(&self, _condition_id: &str, event_id: &str) -> bool {
        // Conditions don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }

    /// Check if a species has a callback for an event
    fn species_has_callback(&self, _species_id: &str, event_id: &str) -> bool {
        // Species don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }
}
`;

const hasCallbackPath = path.join(workspaceRoot, 'src', 'battle', 'has_callback.rs');
fs.writeFileSync(hasCallbackPath, hasCallbackCode);

console.log(`Generated ${hasCallbackPath}`);
console.log('\nDone!');
