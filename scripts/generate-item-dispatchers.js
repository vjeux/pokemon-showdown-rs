#!/usr/bin/env node

/**
 * Generate item dispatcher functions in mod.rs
 *
 * This script:
 * 1. Loads data/items.ts from JavaScript
 * 2. Introspects each item to find which properties are functions
 * 3. Updates src/data/item_callbacks/mod.rs with dispatch functions
 */

const fs = require('fs');
const path = require('path');

// Load the actual Items object from the transpiled JavaScript
const projectRoot = path.join(__dirname, '../../pokemon-showdown-ts');

let Items;
try {
    // Try to require the compiled items from dist
    const Dex = require(path.join(projectRoot, 'dist/sim/dex.js')).Dex;
    Items = Dex.data.Items;
    console.log('Loaded items from dist/sim');
} catch (e) {
    console.error('Error loading items:', e.message);
    console.error('Make sure pokemon-showdown-ts is built');
    process.exit(1);
}

// Convert callback name to snake_case
function toSnakeCase(str) {
    return str.replace(/([A-Z])/g, '_$1').toLowerCase();
}

// Extract callbacks from an item object
function extractCallbacks(itemId, itemData) {
    const callbacks = [];

    // Iterate through all properties of the item
    for (const key in itemData) {
        // Skip if not own property
        if (!itemData.hasOwnProperty(key)) continue;

        // Check if it's a function - that's a callback
        if (typeof itemData[key] === 'function') {
            callbacks.push(key);
        }
    }

    return callbacks;
}

// Parse all items
console.log('Extracting callbacks from items...');
const items = {};

for (const itemId in Items) {
    if (!Items.hasOwnProperty(itemId)) continue;

    const itemData = Items[itemId];
    const callbacks = extractCallbacks(itemId, itemData);

    if (callbacks.length > 0) {
        items[itemId] = {
            callbacks,
            data: itemData
        };
    }
}

console.log(`\nFound ${Object.keys(items).length} items with callbacks`);

// Group items by callback type
const callbackToItems = {};
Object.entries(items).forEach(([id, data]) => {
    data.callbacks.forEach(cb => {
        if (!callbackToItems[cb]) {
            callbackToItems[cb] = [];
        }
        callbackToItems[cb].push(id);
    });
});

console.log(`\nFound ${Object.keys(callbackToItems).length} unique callback types:`);
Object.entries(callbackToItems).sort().forEach(([cb, itemList]) => {
    console.log(`  ${cb}: ${itemList.length} items`);
});

// Function signature patterns for each event type
// These MUST match TypeScript definitions (source of truth)
// Extracted from TypeScript item callbacks
const signaturePatterns = {
    onAfterBoost: `    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    boost: &crate::dex_data::BoostsTable,`,
    onAfterMoveSecondary: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    move_id: &str,`,
    onAfterMoveSecondarySelf: `    battle: &mut Battle,
    item_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: &str,`,
    onAfterSetStatus: `    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),`,
    onAfterSubDamage: `    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,`,
    onAnyAfterMega: `    battle: &mut Battle,
    item_id: &str,`,
    onAnyAfterMove: `    battle: &mut Battle,
    item_id: &str,`,
    onAnyAfterTerastallization: `    battle: &mut Battle,
    item_id: &str,`,
    onAnyPseudoWeatherChange: `    battle: &mut Battle,
    item_id: &str,`,
    onAnySwitchIn: `    battle: &mut Battle,
    item_id: &str,`,
    onAttract: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,`,
    onBasePower: `    battle: &mut Battle,
    item_id: &str,
    base_power: i32,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,`,
    onChargeMove: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: &str,`,
    onDamage: `    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,`,
    onDamagingHit: `    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    target_pos: (usize, usize),
    source_pos: (usize, usize),`,
    onDisableMove: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onEat: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onEffectiveness: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,`,
    onEnd: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onFoeAfterBoost: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
    boost: &crate::dex_data::BoostsTable,`,
    onFractionalPriority: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    priority: f64,`,
    onHit: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    move_id: &str,`,
    onImmunity: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    immunity_type: &str,`,
    onMaybeTrapPokemon: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onModifyAccuracy: `    battle: &mut Battle,
    item_id: &str,`,
    onModifyAtk: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onModifyCritRatio: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    crit_ratio: i32,`,
    onModifyDamage: `    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,`,
    onModifyDef: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onModifyMove: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,`,
    onModifySecondaries: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    secondaries: &mut Vec<crate::battle_actions::SecondaryEffect>,`,
    onModifySpA: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onModifySpD: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onModifySpe: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onModifyWeight: `    battle: &mut Battle,
    item_id: &str,
    weighthg: i32,`,
    onResidual: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onSetAbility: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,`,
    onSourceModifyAccuracy: `    battle: &mut Battle,
    item_id: &str,
    accuracy: i32,
    target_pos: Option<(usize, usize)>,`,
    onSourceModifyDamage: `    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    source_pos: (usize, usize),
    target_pos: (usize, usize),`,
    onSourceTryPrimaryHit: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    move_id: &str,`,
    onStart: `    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,`,
    onSwitchIn: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onTakeItem: `    battle: &mut Battle,
    item_id: &str,
    item_pos: Option<(usize, usize)>,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,`,
    onTerrainChange: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onTrapPokemon: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onTryBoost: `    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    boost: &mut crate::dex_data::BoostsTable,`,
    onTryEatItem: `    battle: &mut Battle,
    item_id: &str,
    item: &str,
    pokemon_pos: (usize, usize),`,
    onTryHeal: `    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,`,
    onTryHit: `    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    source_pos: (usize, usize),`,
    onUpdate: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onUse: `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`,
    onUseItem: `    battle: &mut Battle,
    item_id: &str,
    item: &str,
    pokemon_pos: (usize, usize),`,
};

// Call patterns for each event type (how to call the item's callback)
// These specify how to call the Rust implementations with the dispatcher's parameters
const callPatterns = {
    onAfterBoost: (item) => `${item}::on_after_boost(battle, target_pos, boost)`,
    onAfterMoveSecondary: (item) => `${item}::on_after_move_secondary(battle, target_pos, source_pos, move_id)`,
    onAfterMoveSecondarySelf: (item) => `${item}::on_after_move_secondary_self(battle, source_pos, target_pos, move_id)`,
    onAfterSetStatus: (item) => `${item}::on_after_set_status(battle, target_pos)`,
    onAfterSubDamage: (item) => `${item}::on_after_sub_damage(battle, damage, target_pos, source_pos, effect_id)`,
    onAnyAfterMega: (item) => `${item}::on_any_after_mega(battle)`,
    onAnyAfterMove: (item) => `${item}::on_any_after_move(battle)`,
    onAnyAfterTerastallization: (item) => `${item}::on_any_after_terastallization(battle)`,
    onAnyPseudoWeatherChange: (item) => `${item}::on_any_pseudo_weather_change(battle)`,
    onAnySwitchIn: (item) => `${item}::on_any_switch_in(battle)`,
    onAttract: (item) => `${item}::on_attract(battle, target_pos, source_pos)`,
    onBasePower: (item) => `${item}::on_base_power(battle, base_power, pokemon_pos, target_pos)`,
    onChargeMove: (item) => `${item}::on_charge_move(battle, pokemon_pos, target_pos, move_id)`,
    onDamage: (item) => `${item}::on_damage(battle, damage, target_pos, source_pos, effect_id)`,
    onDamagingHit: (item) => `${item}::on_damaging_hit(battle, damage, target_pos, source_pos)`,
    onDisableMove: (item) => `${item}::on_disable_move(battle, pokemon_pos)`,
    onEat: (item) => `${item}::on_eat(battle, pokemon_pos)`,
    onEffectiveness: (item) => `${item}::on_effectiveness(battle, target_pos)`,
    onEnd: (item) => `${item}::on_end(battle, pokemon_pos)`,
    onFoeAfterBoost: (item) => `${item}::on_foe_after_boost(battle, target_pos, source_pos, effect_id, boost)`,
    onFractionalPriority: (item) => `${item}::on_fractional_priority(battle, pokemon_pos, priority)`,
    onHit: (item) => `${item}::on_hit(battle, target_pos, source_pos, move_id)`,
    onImmunity: (item) => `${item}::on_immunity(battle, pokemon_pos, immunity_type)`,
    onMaybeTrapPokemon: (item) => `${item}::on_maybe_trap_pokemon(battle, pokemon_pos)`,
    onModifyAccuracy: (item) => `${item}::on_modify_accuracy(battle)`,
    onModifyAtk: (item) => `${item}::on_modify_atk(battle, pokemon_pos)`,
    onModifyCritRatio: (item) => `${item}::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio)`,
    onModifyDamage: (item) => `${item}::on_modify_damage(battle, damage, pokemon_pos, target_pos)`,
    onModifyDef: (item) => `${item}::on_modify_def(battle, pokemon_pos)`,
    onModifyMove: (item) => `${item}::on_modify_move(battle, pokemon_pos, target_pos)`,
    onModifySecondaries: (item) => `${item}::on_modify_secondaries(battle, pokemon_pos, secondaries)`,
    onModifySpA: (item) => `${item}::on_modify_sp_a(battle, pokemon_pos)`,
    onModifySpD: (item) => `${item}::on_modify_sp_d(battle, pokemon_pos)`,
    onModifySpe: (item) => `${item}::on_modify_spe(battle, pokemon_pos)`,
    onModifyWeight: (item) => `${item}::on_modify_weight(battle, weighthg)`,
    onResidual: (item) => `${item}::on_residual(battle, pokemon_pos)`,
    onSetAbility: (item) => `${item}::on_set_ability(battle, target_pos, source_pos, effect_id)`,
    onSourceModifyAccuracy: (item) => `${item}::on_source_modify_accuracy(battle, accuracy, target_pos)`,
    onSourceModifyDamage: (item) => `${item}::on_source_modify_damage(battle, damage, source_pos, target_pos)`,
    onSourceTryPrimaryHit: (item) => `${item}::on_source_try_primary_hit(battle, target_pos, source_pos, move_id)`,
    onStart: (item) => `${item}::on_start(battle, target_pos)`,
    onSwitchIn: (item) => `${item}::on_switch_in(battle, pokemon_pos)`,
    onTakeItem: (item) => `${item}::on_take_item(battle, item_pos, pokemon_pos, source_pos)`,
    onTerrainChange: (item) => `${item}::on_terrain_change(battle, pokemon_pos)`,
    onTrapPokemon: (item) => `${item}::on_trap_pokemon(battle, pokemon_pos)`,
    onTryBoost: (item) => `${item}::on_try_boost(battle, target_pos, boost)`,
    onTryEatItem: (item) => `${item}::on_try_eat_item(battle, item, pokemon_pos)`,
    onTryHeal: (item) => `${item}::on_try_heal(battle, damage, target_pos, source_pos, effect_id)`,
    onTryHit: (item) => `${item}::on_try_hit(battle, target_pos, source_pos)`,
    onUpdate: (item) => `${item}::on_update(battle, pokemon_pos)`,
    onUse: (item) => `${item}::on_use(battle, pokemon_pos)`,
    onUseItem: (item) => `${item}::on_use_item(battle, item, pokemon_pos)`,
};

// Default signature and call pattern for simple events
const defaultSignature = `    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),`;
const defaultCallPattern = (item, eventName) => `${item}::${toSnakeCase(eventName)}(battle, pokemon_pos)`;

// Generate dispatcher function code
function generateDispatcher(callback, itemList) {
    const funcName = toSnakeCase(callback);
    const signature = signaturePatterns[callback] || defaultSignature;
    const callPattern = callPatterns[callback] || defaultCallPattern;

    let code = `/// Dispatch ${callback} callbacks\n`;
    code += `pub fn dispatch_${funcName}(\n`;
    code += signature + '\n';
    code += `) -> EventResult {\n`;
    code += `    match item_id {\n`;

    // Sort items for consistent output
    itemList.sort().forEach(item => {
        const call = callPattern(item, callback);
        code += `        "${item}" => ${call},\n`;
    });

    code += `        _ => EventResult::Continue,\n`;
    code += `    }\n`;
    code += `}\n`;

    return code;
}

// Read existing mod.rs to preserve module declarations and other code
const modRsPath = path.join(__dirname, '../src/data/item_callbacks/mod.rs');
let modRsContent = fs.readFileSync(modRsPath, 'utf8');

// Find the section with dispatcher functions
// They start after the module declarations and before the end of file
const dispatcherStartMarker = /\/\/ DISPATCH FUNCTIONS|\/\/\/ Dispatch on/;
const match = modRsContent.search(dispatcherStartMarker);

if (match === -1) {
    console.error('Could not find dispatcher section in mod.rs');
    process.exit(1);
}

// Extract the part before dispatchers (module declarations, imports, etc.)
const beforeDispatchers = modRsContent.substring(0, match);

// Generate new dispatcher section
let dispatcherSection = `// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route item events to specific item implementations.
// Auto-generated by scripts/generate-item-dispatchers.js
// =========================================================================

`;

// Generate dispatchers for all callback types
const sortedCallbacks = Object.keys(callbackToItems).sort();
sortedCallbacks.forEach(callback => {
    dispatcherSection += generateDispatcher(callback, callbackToItems[callback]);
    dispatcherSection += '\n';
});

// Generate stub dispatchers for priority/order callbacks that have no implementations yet
// NOTE: All priority/order properties that are numbers (not functions) in items.ts
// should NOT have dispatchers. Examples:
// - onTrapPokemonPriority: -10  (number, not function)
// - onBasePowerPriority: 15  (number, not function)
// - onPlate: 'Bug'  (string, not function)
// - onMemory: 'Fire'  (string, not function)
// - onNegateImmunity: false  (boolean, not function)
//
// These are metadata values, not callbacks, so they don't need dispatchers.
// If a callback truly has no implementations yet but IS a function in TypeScript,
// it can be added here as a stub.
const stubCallbacks = [
    // Currently no stubs needed - all TypeScript callbacks have been mapped
];

stubCallbacks.forEach(callback => {
    const funcName = toSnakeCase(callback);
    dispatcherSection += `/// Dispatch ${callback} callbacks (stub - no implementations)\n`;
    dispatcherSection += `pub fn dispatch_${funcName}(\n`;
    dispatcherSection += `    _battle: &mut Battle,\n`;
    dispatcherSection += `    _item_id: &str,\n`;
    dispatcherSection += `    _pokemon_pos: (usize, usize),\n`;
    dispatcherSection += `) -> EventResult {\n`;
    dispatcherSection += `    EventResult::Continue\n`;
    dispatcherSection += `}\n\n`;
});


// Write the new mod.rs
const newModRs = beforeDispatchers + dispatcherSection;
fs.writeFileSync(modRsPath, newModRs);

console.log(`\n✓ Generated ${sortedCallbacks.length} dispatch functions in mod.rs`);
console.log(`  Updated: ${modRsPath}`);
