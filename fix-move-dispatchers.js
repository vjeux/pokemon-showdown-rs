#!/usr/bin/env node

/**
 * Fix move dispatcher signatures to include proper parameters
 * This updates the dispatcher function signatures in move_callbacks/mod.rs
 * to match the updated signatures from generate-moves.js
 * Note: Individual callback calls are not updated - those will need to be
 * updated incrementally as callbacks are implemented
 */

const fs = require('fs');
const path = require('path');

const modPath = path.join(__dirname, 'src', 'data', 'move_callbacks', 'mod.rs');
let modContent = fs.readFileSync(modPath, 'utf8');

// Just update dispatcher signatures to accept the right parameters
// Don't update individual callback calls - those can be updated incrementally
const fixes = [
  // onModifyMove: should accept target_pos parameter
  {
    find: /\/\/\/ Dispatch onModifyMove callbacks\npub fn dispatch_on_modify_move\(\n    battle: &mut Battle,\n    move_id: &str,\n    pokemon_pos: \(usize, usize\),\n\) -> EventResult \{/,
    replace: `/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    battle: &mut Battle,
    move_id: &str,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {`
  },

  // onModifyType: should accept target_pos parameter
  {
    find: /\/\/\/ Dispatch onModifyType callbacks\npub fn dispatch_on_modify_type\(\n    battle: &mut Battle,\n    move_id: &str,\n    pokemon_pos: \(usize, usize\),\n\) -> EventResult \{/,
    replace: `/// Dispatch onModifyType callbacks
pub fn dispatch_on_modify_type(
    battle: &mut Battle,
    move_id: &str,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {`
  },

  // onPrepareHit: should include move_id parameter (if not already present)
  {
    find: /\/\/\/ Dispatch onPrepareHit callbacks\npub fn dispatch_on_prepare_hit\(\n    battle: &mut Battle,\n    move_id: &str,\n    pokemon_pos: \(usize, usize\),\n    target_pos: Option<\(usize, usize\)>,\n\) -> EventResult \{/,
    replace: `/// Dispatch onPrepareHit callbacks
pub fn dispatch_on_prepare_hit(
    battle: &mut Battle,
    move_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {`
  },

  // onAfterMove: should include move_id parameter
  {
    find: /\/\/\/ Dispatch onAfterMove callbacks\npub fn dispatch_on_after_move\(\n    battle: &mut Battle,\n    move_id: &str,\n    pokemon_pos: \(usize, usize\),\n    target_pos: Option<\(usize, usize\)>,\n\) -> EventResult \{/,
    replace: `/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    battle: &mut Battle,
    move_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {`
  },

  // Update onAfterMove match arms to use source_pos
  {
    find: /"relicsong" => relicsong::on_after_move\(battle, pokemon_pos, target_pos\),/g,
    replace: '"relicsong" => relicsong::on_after_move(battle, source_pos, target_pos, move_id),'
  },

  // onMoveFail: should use proper parameter names (target_pos first, then source_pos)
  {
    find: /\/\/\/ Dispatch onMoveFail callbacks\npub fn dispatch_on_move_fail\(\n    battle: &mut Battle,\n    move_id: &str,\n    pokemon_pos: \(usize, usize\),\n    target_pos: \(usize, usize\),\n\) -> EventResult \{/,
    replace: `/// Dispatch onMoveFail callbacks
pub fn dispatch_on_move_fail(
    battle: &mut Battle,
    move_id: &str,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {`
  },

  // Update onMoveFail match arms to use new parameter order
  {
    find: /"highjumpkick" => highjumpkick::on_move_fail\(battle, pokemon_pos, target_pos\),/g,
    replace: '"highjumpkick" => highjumpkick::on_move_fail(battle, target_pos, source_pos, move_id),'
  },
  {
    find: /"jumpkick" => jumpkick::on_move_fail\(battle, pokemon_pos, target_pos\),/g,
    replace: '"jumpkick" => jumpkick::on_move_fail(battle, target_pos, source_pos, move_id),'
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
console.log(`\nApplied ${fixCount}/${fixes.length} fixes to move_callbacks/mod.rs`);
