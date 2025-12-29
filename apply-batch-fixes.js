#!/usr/bin/env node

const fs = require('fs');

let battleContent = fs.readFileSync('src/battle.rs', 'utf8');

// Fixes based on actual dispatcher signatures from real-dispatcher-mapping.json
const fixes = [
  // Line 8289, 8295: AllyBasePower - expects (base_power: i32, move_id: &str)
  {
    find: /"AllyBasePower" => ability_callbacks::dispatch_on_ally_base_power\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"AllyBasePower" => ability_callbacks::dispatch_on_ally_base_power(\n                self,\n                ability_id.as_str(),\n                0,\n            "", // TODO: Wire through actual move_id\n            )'
  },
  {
    find: /"AllyBasePowerPriority" => ability_callbacks::dispatch_on_ally_base_power_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"AllyBasePowerPriority" => ability_callbacks::dispatch_on_ally_base_power_priority(\n                self,\n                ability_id.as_str(),\n                0,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // Line 8381, 8389: AnyBasePower - expects (base_power: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"AnyBasePower" => ability_callbacks::dispatch_on_any_base_power\(\s*self,\s*ability_id\.as_str\(\),\s*0,\s*\/\/ TODO[^)]+pokemon_pos,\s*\(0,\s*0\),\s*\/\/ TODO[^)]+(""),?\s*\/\/ TODO[^)]*\)/g,
    replace: '"AnyBasePower" => ability_callbacks::dispatch_on_any_base_power(\n                self,\n                ability_id.as_str(),\n                0, // TODO: Wire through actual base_power\n                Some(pokemon_pos),\n                None, // TODO: Wire through actual target_pos\n                "", // TODO: Wire through actual move_id\n            )'
  },
  {
    find: /"AnyBasePowerPriority" => ability_callbacks::dispatch_on_any_base_power_priority\(\s*self,\s*ability_id\.as_str\(\),\s*0,\s*\/\/ TODO[^)]+pokemon_pos,\s*\(0,\s*0\),\s*\/\/ TODO[^)]+(""),?\s*\/\/ TODO[^)]*\)/g,
    replace: '"AnyBasePowerPriority" => ability_callbacks::dispatch_on_any_base_power_priority(\n                self,\n                ability_id.as_str(),\n                0, // TODO: Wire through actual base_power\n                Some(pokemon_pos),\n                None, // TODO: Wire through actual target_pos\n                "", // TODO: Wire through actual move_id\n            )'
  },

  // Line 8489: AnyTryMove - expects (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /ability_callbacks::dispatch_on_any_try_move\(self, ability_id\.as_str\(\)\)/g,
    replace: 'ability_callbacks::dispatch_on_any_try_move(self, ability_id.as_str(), Some(pokemon_pos), None, None)'
  },

  // Line 8501: BasePowerPriority - expects (base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str)
  {
    find: /"BasePowerPriority" => ability_callbacks::dispatch_on_base_power_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"BasePowerPriority" => ability_callbacks::dispatch_on_base_power_priority(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // Line 8510: BeforeMovePriority - expects (pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"BeforeMovePriority" => ability_callbacks::dispatch_on_before_move_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"BeforeMovePriority" => ability_callbacks::dispatch_on_before_move_priority(\n                self,\n                ability_id.as_str(),\n                pokemon_pos,\n            None,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // Line 8522: ChangeBoost - expects (boosts: &HashMap<String, i32>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  // Currently has too many args
  {
    find: /ability_callbacks::dispatch_on_change_boost\(self, ability_id\.as_str\(\), "", Some\(pokemon_pos\), None, None\)/g,
    replace: 'ability_callbacks::dispatch_on_change_boost(self, ability_id.as_str(), None, "")'
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
