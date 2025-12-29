#!/usr/bin/env node

/**
 * Fix item dispatcher signatures to include move_id parameters
 * This updates the dispatcher functions in item_callbacks/mod.rs to match
 * the updated signatures from generate-items.js
 */

const fs = require('fs');
const path = require('path');

const modPath = path.join(__dirname, 'src', 'data', 'item_callbacks', 'mod.rs');
let modContent = fs.readFileSync(modPath, 'utf8');

// Fixes for dispatcher signatures that need move_id added
const fixes = [
  // onBasePower: should be (item_id, base_power, pokemon_pos, target_pos, move_id)
  {
    find: /\/\/\/ Dispatch onBasePower callbacks\npub fn dispatch_on_base_power\(\n    _battle: &mut Battle,\n    _item_id: &str,\n    _pokemon_pos: \(usize, usize\),\n\) -> EventResult \{\n    EventResult::Continue\n\}/,
    replace: `/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    _battle: &mut Battle,
    _item_id: &str,
    _base_power: i32,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}`
  },

  // onModifyDamage: should be (item_id, damage, pokemon_pos, target_pos, move_id)
  {
    find: /\/\/\/ Dispatch onModifyDamage callbacks\npub fn dispatch_on_modify_damage\(\n    _battle: &mut Battle,\n    _item_id: &str,\n    _pokemon_pos: \(usize, usize\),\n\) -> EventResult \{\n    EventResult::Continue\n\}/,
    replace: `/// Dispatch onModifyDamage callbacks
pub fn dispatch_on_modify_damage(
    _battle: &mut Battle,
    _item_id: &str,
    _damage: i32,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}`
  },

  // onSourceModifyDamage: should be (item_id, damage, source_pos, target_pos, move_id)
  {
    find: /\/\/\/ Dispatch onSourceModifyDamage callbacks\npub fn dispatch_on_source_modify_damage\(\n    _battle: &mut Battle,\n    _item_id: &str,\n    _pokemon_pos: \(usize, usize\),\n\) -> EventResult \{\n    EventResult::Continue\n\}/,
    replace: `/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    _battle: &mut Battle,
    _item_id: &str,
    _damage: i32,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}`
  },

  // onTryHit: should be (item_id, target_pos, source_pos, move_id)
  {
    find: /\/\/\/ Dispatch onTryHit callbacks\npub fn dispatch_on_try_hit\(\n    battle: &mut Battle,\n    item_id: &str,\n    pokemon_pos: \(usize, usize\),\n    source_pos: \(usize, usize\),\n\) -> EventResult \{/,
    replace: `/// Dispatch onTryHit callbacks
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
    move_id: &str,
) -> EventResult {`
  },

  // Fix the onTryHit match arms to use new parameter names
  {
    find: /match ID::from\(item_id\)\.as_str\(\) \{\n        "safetygoggles" => safetygoggles::on_try_hit\(battle, pokemon_pos, source_pos\),/,
    replace: `match ID::from(item_id).as_str() {
        "safetygoggles" => safetygoggles::on_try_hit(battle, target_pos, source_pos, move_id),`
  },

  // onDamagingHit: should be (item_id, damage, target_pos, source_pos, move_id)
  {
    find: /\/\/\/ Dispatch onDamagingHit callbacks\npub fn dispatch_on_damaging_hit\(\n    _battle: &mut Battle,\n    _item_id: &str,\n    _pokemon_pos: \(usize, usize\),\n\) -> EventResult \{\n    EventResult::Continue\n\}/,
    replace: `/// Dispatch onDamagingHit callbacks
pub fn dispatch_on_damaging_hit(
    _battle: &mut Battle,
    _item_id: &str,
    _damage: i32,
    _target_pos: (usize, usize),
    _source_pos: (usize, usize),
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}`
  },

  // onAfterMoveSecondarySelf: should be (item_id, source_pos, target_pos, move_id)
  {
    find: /\/\/\/ Dispatch onAfterMoveSecondarySelf callbacks\npub fn dispatch_on_after_move_secondary_self\(\n    _battle: &mut Battle,\n    _item_id: &str,\n    _source_pos: \(usize, usize\),\n    _target_pos: Option<\(usize, usize\)>,\n\) -> EventResult \{\n    EventResult::Continue\n\}/,
    replace: `/// Dispatch onAfterMoveSecondarySelf callbacks
pub fn dispatch_on_after_move_secondary_self(
    _battle: &mut Battle,
    _item_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}`
  },

  // onModifyMove: should be (item_id, move_id, pokemon_pos)
  {
    find: /\/\/\/ Dispatch onModifyMove callbacks\npub fn dispatch_on_modify_move\(\n    _battle: &mut Battle,\n    _item_id: &str,\n    _pokemon_pos: \(usize, usize\),\n\) -> EventResult \{\n    EventResult::Continue\n\}/,
    replace: `/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    _battle: &mut Battle,
    _item_id: &str,
    _move_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}`
  },
];

// Apply all fixes
let fixCount = 0;
fixes.forEach((fix, idx) => {
  const before = modContent;
  modContent = modContent.replace(fix.find, fix.replace);
  if (before !== modContent) {
    fixCount++;
    console.log(`✓ Applied fix ${idx + 1}`);
  } else {
    console.log(`✗ Fix ${idx + 1} not applied (pattern not found)`);
  }
});

fs.writeFileSync(modPath, modContent);
console.log(`\nApplied ${fixCount}/${fixes.length} fixes to item_callbacks/mod.rs`);
