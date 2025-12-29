#!/usr/bin/env node

const fs = require('fs');

let battleContent = fs.readFileSync('src/battle.rs', 'utf8');

// Fixes based on actual dispatcher signatures from real-dispatcher-mapping.json
const fixes = [
  // DamagePriority: (damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"DamagePriority" => ability_callbacks::dispatch_on_damage_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"DamagePriority" => ability_callbacks::dispatch_on_damage_priority(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, None, None\n            )'
  },

  // DamagingHitOrder: (damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"DamagingHitOrder" => ability_callbacks::dispatch_on_damaging_hit_order\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"DamagingHitOrder" => ability_callbacks::dispatch_on_damaging_hit_order(\n                self,\n                ability_id.as_str(),\n                0,\n            Some(pokemon_pos), None,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // DeductPP: (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>)
  {
    find: /ability_callbacks::dispatch_on_deduct_p_p\(self, ability_id\.as_str\(\), pokemon_pos, \(0, 0\), ""\)/g,
    replace: 'ability_callbacks::dispatch_on_deduct_p_p(self, ability_id.as_str(), Some(pokemon_pos), None)'
  },

  // FoeAfterBoost: (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /"FoeAfterBoost" => ability_callbacks::dispatch_on_foe_after_boost\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"FoeAfterBoost" => ability_callbacks::dispatch_on_foe_after_boost(\n                self,\n                ability_id.as_str(),\n                Some(pokemon_pos), None, None\n            )'
  },

  // FoeMaybeTrapPokemon: (pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>)
  {
    find: /"FoeMaybeTrapPokemon" => ability_callbacks::dispatch_on_foe_maybe_trap_pokemon\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"FoeMaybeTrapPokemon" => ability_callbacks::dispatch_on_foe_maybe_trap_pokemon(\n                self,\n                ability_id.as_str(),\n                pokemon_pos, None\n            )'
  },

  // FoeTryEatItem: no params (just battle, ability_id)
  {
    find: /"FoeTryEatItem" => ability_callbacks::dispatch_on_foe_try_eat_item\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"FoeTryEatItem" => ability_callbacks::dispatch_on_foe_try_eat_item(\n                self,\n                ability_id.as_str()\n            )'
  },

  // FractionalPriority: (pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"FractionalPriority" => ability_callbacks::dispatch_on_fractional_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"FractionalPriority" => ability_callbacks::dispatch_on_fractional_priority(\n                self,\n                ability_id.as_str(),\n                pokemon_pos,\n            None,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // FractionalPriorityPriority: (pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"FractionalPriorityPriority" => {\s*ability_callbacks::dispatch_on_fractional_priority_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"FractionalPriorityPriority" => {\n                ability_callbacks::dispatch_on_fractional_priority_priority(\n                    self,\n                    ability_id.as_str(),\n                    pokemon_pos,\n                None,\n                "", // TODO: Wire through actual move_id'
  },

  // ModifyAccuracy: (accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str)
  {
    find: /"ModifyAccuracy" => ability_callbacks::dispatch_on_modify_accuracy\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifyAccuracy" => ability_callbacks::dispatch_on_modify_accuracy(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // ModifyAccuracyPriority: (accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str)
  {
    find: /"ModifyAccuracyPriority" => ability_callbacks::dispatch_on_modify_accuracy_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifyAccuracyPriority" => ability_callbacks::dispatch_on_modify_accuracy_priority(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // ModifyAtkPriority: (atk: i32, pokemon_pos: (usize, usize), target_pos: (usize, usize), move_id: &str)
  {
    find: /"ModifyAtkPriority" => ability_callbacks::dispatch_on_modify_atk_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifyAtkPriority" => ability_callbacks::dispatch_on_modify_atk_priority(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // ModifyMovePriority: (move_id: &str)
  {
    find: /"ModifyMovePriority" => ability_callbacks::dispatch_on_modify_move_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifyMovePriority" => ability_callbacks::dispatch_on_modify_move_priority(\n                self,\n                ability_id.as_str(),\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // ModifyPriority: (priority: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"ModifyPriority" => ability_callbacks::dispatch_on_modify_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifyPriority" => ability_callbacks::dispatch_on_modify_priority(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, None,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // ModifySTAB: (source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"ModifySTAB" => ability_callbacks::dispatch_on_modify_s_t_a_b\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifySTAB" => ability_callbacks::dispatch_on_modify_s_t_a_b(\n                self,\n                ability_id.as_str(),\n                None, Some(pokemon_pos),\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // ModifySecondaries: no params (just battle, ability_id)
  {
    find: /ability_callbacks::dispatch_on_modify_secondaries\(self, ability_id\.as_str\(\), pokemon_pos, \(0, 0\), ""\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_secondaries(self, ability_id.as_str())'
  },

  // ModifySpAPriority: (sp_a: i32, pokemon_pos: (usize, usize), target_pos: (usize, usize), move_id: &str)
  {
    find: /"ModifySpAPriority" => ability_callbacks::dispatch_on_modify_sp_a_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifySpAPriority" => ability_callbacks::dispatch_on_modify_sp_a_priority(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },
];

// Apply all fixes
let fixCount = 0;
fixes.forEach((fix, idx) => {
  const before = battleContent;
  battleContent = battleContent.replace(fix.find, fix.replace);
  if (before !== battleContent) {
    fixCount++;
    console.log(`✓ Applied fix ${idx + 1}: ${fix.find.toString().substring(0, 50)}...`);
  }
});

fs.writeFileSync('src/battle.rs', battleContent);
console.log(`\nApplied ${fixCount} fixes to battle.rs`);
