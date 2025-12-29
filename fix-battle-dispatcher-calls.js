#!/usr/bin/env node

/**
 * Fix battle.rs calls to item and move dispatchers
 * Updates calls to pass the new required parameters
 */

const fs = require('fs');
const path = require('path');

const battlePath = path.join(__dirname, 'src', 'battle.rs');
let battleContent = fs.readFileSync(battlePath, 'utf8');

const fixes = [
  // item_callbacks::dispatch_on_base_power: needs base_power, target_pos, move_id
  {
    find: /item_callbacks::dispatch_on_base_power\(self, item_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'item_callbacks::dispatch_on_base_power(self, item_id.as_str(), 0, pokemon_pos, None, "") // TODO: Wire through actual base_power, target_pos, move_id'
  },

  // item_callbacks::dispatch_on_damaging_hit: needs damage, target_pos, source_pos, move_id
  {
    find: /item_callbacks::dispatch_on_damaging_hit\(self, item_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'item_callbacks::dispatch_on_damaging_hit(self, item_id.as_str(), 0, pokemon_pos, pokemon_pos, "") // TODO: Wire through actual damage, target_pos, source_pos, move_id'
  },

  // item_callbacks::dispatch_on_modify_damage: needs damage, target_pos, move_id
  {
    find: /item_callbacks::dispatch_on_modify_damage\(self, item_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'item_callbacks::dispatch_on_modify_damage(self, item_id.as_str(), 0, pokemon_pos, None, "") // TODO: Wire through actual damage, target_pos, move_id'
  },

  // item_callbacks::dispatch_on_modify_move: needs move_id before pokemon_pos
  {
    find: /item_callbacks::dispatch_on_modify_move\(self, item_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'item_callbacks::dispatch_on_modify_move(self, item_id.as_str(), "", pokemon_pos) // TODO: Wire through actual move_id'
  },

  // item_callbacks::dispatch_on_source_modify_damage: needs damage, source_pos (change param name), target_pos, move_id
  {
    find: /"SourceModifyDamage" => item_callbacks::dispatch_on_source_modify_damage\(\s*self,\s*item_id\.as_str\(\),\s*pokemon_pos,?\s*\)/g,
    replace: '"SourceModifyDamage" => item_callbacks::dispatch_on_source_modify_damage(\n                self,\n                item_id.as_str(),\n                0, pokemon_pos, pokemon_pos, "" // TODO: Wire through actual damage, source_pos, target_pos, move_id\n            )'
  },

  // item_callbacks::dispatch_on_try_hit: needs move_id (4 params -> 5 params)
  {
    find: /item_callbacks::dispatch_on_try_hit\(self, item_id\.as_str\(\), pokemon_pos, source_pos\)/g,
    replace: 'item_callbacks::dispatch_on_try_hit(self, item_id.as_str(), pokemon_pos, source_pos, "") // TODO: Wire through actual move_id'
  },

  // item_callbacks::dispatch_on_after_move_secondary_self: needs move_id
  {
    find: /item_callbacks::dispatch_on_after_move_secondary_self\(\s*self,\s*item_id\.as_str\(\),\s*pokemon_pos,\s*target_pos,?\s*\)/g,
    replace: 'item_callbacks::dispatch_on_after_move_secondary_self(\n                        self,\n                        item_id.as_str(),\n                        pokemon_pos,\n                        target_pos,\n                        "" // TODO: Wire through actual move_id\n                    )'
  },

  // move_callbacks::dispatch_on_modify_type: needs target_pos parameter
  {
    find: /move_callbacks::dispatch_on_modify_type\(self, move_id, source_pos\)/g,
    replace: 'move_callbacks::dispatch_on_modify_type(self, move_id, source_pos, None) // TODO: Wire through actual target_pos'
  },
];

// Apply all fixes
let fixCount = 0;
fixes.forEach((fix, idx) => {
  const before = battleContent;
  battleContent = battleContent.replace(fix.find, fix.replace);
  if (before !== battleContent) {
    fixCount++;
    console.log(`✓ Applied fix ${idx + 1}`);
  } else {
    console.log(`✗ Fix ${idx + 1} not applied (pattern not found)`);
  }
});

fs.writeFileSync(battlePath, battleContent);
console.log(`\nApplied ${fixCount}/${fixes.length} fixes to battle.rs`);
