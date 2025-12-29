#!/usr/bin/env node

const fs = require('fs');

let battleContent = fs.readFileSync('src/battle.rs', 'utf8');

// Final set of fixes
const fixes = [
  // SourceModifySpAPriority: (move_id: &str) - currently has pokemon_pos, ""
  {
    find: /"SourceModifySpAPriority" => {\s*ability_callbacks::dispatch_on_source_modify_sp_a_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifySpAPriority" => {\n                ability_callbacks::dispatch_on_source_modify_sp_a_priority(\n                    self,\n                    ability_id.as_str(),\n                "", // TODO: Wire through actual move_id'
  },

  // SourceTryHeal: (damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"SourceTryHeal" => ability_callbacks::dispatch_on_source_try_heal\(\s*self,\s*ability_id\.as_str\(\)\s*\)/g,
    replace: '"SourceTryHeal" => ability_callbacks::dispatch_on_source_try_heal(\n                self,\n                ability_id.as_str(),\n                0, Some(pokemon_pos), None, None\n            )'
  },

  // SourceTryPrimaryHit: (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"SourceTryPrimaryHit" => ability_callbacks::dispatch_on_source_try_primary_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"SourceTryPrimaryHit" => ability_callbacks::dispatch_on_source_try_primary_hit(\n                self,\n                ability_id.as_str(),\n                Some(pokemon_pos), None, None\n            )'
  },

  // TakeItem: (pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>)
  {
    find: /"TakeItem" => {\s*ability_callbacks::dispatch_on_take_item\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: '"TakeItem" => {\n                ability_callbacks::dispatch_on_take_item(self, ability_id.as_str(), pokemon_pos, None)'
  },

  // TryAddVolatile: (status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"TryAddVolatile" => ability_callbacks::dispatch_on_try_add_volatile\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"TryAddVolatile" => ability_callbacks::dispatch_on_try_add_volatile(\n                self,\n                ability_id.as_str(),\n                \"\", pokemon_pos, None, None\n            )'
  },

  // TryBoost: (boost: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"TryBoost" => {\s*ability_callbacks::dispatch_on_try_boost\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: '"TryBoost" => {\n                ability_callbacks::dispatch_on_try_boost(self, ability_id.as_str(), \"\", pokemon_pos, None, None)'
  },

  // TryBoostPriority: (boost: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"TryBoostPriority" => ability_callbacks::dispatch_on_try_boost_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"TryBoostPriority" => ability_callbacks::dispatch_on_try_boost_priority(\n                self,\n                ability_id.as_str(),\n                \"\", pokemon_pos, None, None\n            )'
  },

  // TryEatItem: no params
  {
    find: /"TryEatItem" => {\s*ability_callbacks::dispatch_on_try_eat_item\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: '"TryEatItem" => {\n                ability_callbacks::dispatch_on_try_eat_item(self, ability_id.as_str())'
  },

  // TryEatItemPriority: no params
  {
    find: /"TryEatItemPriority" => ability_callbacks::dispatch_on_try_eat_item_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"TryEatItemPriority" => ability_callbacks::dispatch_on_try_eat_item_priority(\n                self,\n                ability_id.as_str()\n            )'
  },

  // TryHeal: (damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"TryHeal" => {\s*ability_callbacks::dispatch_on_try_heal\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: '"TryHeal" => {\n                ability_callbacks::dispatch_on_try_heal(self, ability_id.as_str(), 0, Some(pokemon_pos), None, None)'
  },

  // TryHitPriority: (target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str)
  {
    find: /"TryHitPriority" => ability_callbacks::dispatch_on_try_hit_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"TryHitPriority" => ability_callbacks::dispatch_on_try_hit_priority(\n                self,\n                ability_id.as_str(),\n                pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // Additional checks for no-param dispatchers that might have pokemon_pos
  {
    find: /ability_callbacks::dispatch_on_start\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_start(self, ability_id.as_str())'
  },
  {
    find: /ability_callbacks::dispatch_on_switch_in\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_switch_in(self, ability_id.as_str())'
  },
  {
    find: /ability_callbacks::dispatch_on_switch_in_priority\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_switch_in_priority(self, ability_id.as_str())'
  },
  {
    find: /ability_callbacks::dispatch_on_switch_out\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_switch_out(self, ability_id.as_str())'
  },
  {
    find: /ability_callbacks::dispatch_on_terrain_change\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_terrain_change(self, ability_id.as_str())'
  },
  {
    find: /ability_callbacks::dispatch_on_update\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_update(self, ability_id.as_str())'
  },
  {
    find: /ability_callbacks::dispatch_on_weather\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_weather(self, ability_id.as_str())'
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
  }
});

fs.writeFileSync('src/battle.rs', battleContent);
console.log(`\nApplied ${fixCount} fixes to battle.rs`);
