#!/usr/bin/env node

const fs = require('fs');

let battleContent = fs.readFileSync('src/battle.rs', 'utf8');

// Final batch of fixes based on actual dispatcher signatures
const fixes = [
  // ChangeBoost: (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /ability_callbacks::dispatch_on_change_boost\(self, ability_id\.as_str\(\), None, ""\)/g,
    replace: 'ability_callbacks::dispatch_on_change_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)'
  },

  // EmergencyExit: (target_pos: Option<(usize, usize)>)
  {
    find: /"EmergencyExit" => ability_callbacks::dispatch_on_emergency_exit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"EmergencyExit" => ability_callbacks::dispatch_on_emergency_exit(\n                self,\n                ability_id.as_str(),\n                Some(pokemon_pos)\n            )'
  },

  // ModifyTypePriority: (move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>)
  {
    find: /"ModifyTypePriority" => ability_callbacks::dispatch_on_modify_type_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"ModifyTypePriority" => ability_callbacks::dispatch_on_modify_type_priority(\n                self,\n                ability_id.as_str(),\n                "",\n            pokemon_pos, None, // TODO: Wire through actual move_id\n            )'
  },

  // PrepareHit: (source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /ability_callbacks::dispatch_on_prepare_hit\(self, ability_id\.as_str\(\), pokemon_pos, \(0, 0\), ""\)/g,
    replace: 'ability_callbacks::dispatch_on_prepare_hit(self, ability_id.as_str(), None, Some(pokemon_pos), "")'
  },

  // SideConditionStart: (source_pos: Option<(usize, usize)>)
  {
    find: /"SideConditionStart" => ability_callbacks::dispatch_on_side_condition_start\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"SideConditionStart" => ability_callbacks::dispatch_on_side_condition_start(\n                self,\n                ability_id.as_str(),\n                None\n            )'
  },

  // SourceAfterFaint: (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>)
  {
    find: /ability_callbacks::dispatch_on_source_after_faint\(self, ability_id\.as_str\(\), pokemon_pos, \(0, 0\), None\)/g,
    replace: 'ability_callbacks::dispatch_on_source_after_faint(self, ability_id.as_str(), Some(pokemon_pos), None, None)'
  },

  // SourceBasePower: (base_power: i32, move_id: &str)
  {
    find: /"SourceBasePower" => ability_callbacks::dispatch_on_source_base_power\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceBasePower" => ability_callbacks::dispatch_on_source_base_power(\n                self,\n                ability_id.as_str(),\n                0,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceBasePowerPriority: (base_power: i32, move_id: &str)
  {
    find: /"SourceBasePowerPriority" => ability_callbacks::dispatch_on_source_base_power_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceBasePowerPriority" => ability_callbacks::dispatch_on_source_base_power_priority(\n                self,\n                ability_id.as_str(),\n                0,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceDamagingHit: (damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"SourceDamagingHit" => ability_callbacks::dispatch_on_source_damaging_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceDamagingHit" => ability_callbacks::dispatch_on_source_damaging_hit(\n                self,\n                ability_id.as_str(),\n                0, Some(pokemon_pos), None,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceModifyAccuracy: (accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str)
  {
    find: /"SourceModifyAccuracy" => ability_callbacks::dispatch_on_source_modify_accuracy\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifyAccuracy" => ability_callbacks::dispatch_on_source_modify_accuracy(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceModifyAccuracyPriority: (accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str)
  {
    find: /"SourceModifyAccuracyPriority" => {\s*ability_callbacks::dispatch_on_source_modify_accuracy_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifyAccuracyPriority" => {\n                ability_callbacks::dispatch_on_source_modify_accuracy_priority(\n                    self,\n                    ability_id.as_str(),\n                    0, pokemon_pos, pokemon_pos,\n                "", // TODO: Wire through actual move_id'
  },

  // SourceModifyAtk: (move_id: &str)
  {
    find: /"SourceModifyAtk" => ability_callbacks::dispatch_on_source_modify_atk\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifyAtk" => ability_callbacks::dispatch_on_source_modify_atk(\n                self,\n                ability_id.as_str(),\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceModifyAtkPriority: (move_id: &str)
  {
    find: /"SourceModifyAtkPriority" => ability_callbacks::dispatch_on_source_modify_atk_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifyAtkPriority" => ability_callbacks::dispatch_on_source_modify_atk_priority(\n                self,\n                ability_id.as_str(),\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceModifyDamage: (damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str)
  {
    find: /"SourceModifyDamage" => ability_callbacks::dispatch_on_source_modify_damage\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifyDamage" => ability_callbacks::dispatch_on_source_modify_damage(\n                self,\n                ability_id.as_str(),\n                0, pokemon_pos, pokemon_pos,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceModifyDamagePriority: (damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str)
  {
    find: /"SourceModifyDamagePriority" => {\s*ability_callbacks::dispatch_on_source_modify_damage_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifyDamagePriority" => {\n                ability_callbacks::dispatch_on_source_modify_damage_priority(\n                    self,\n                    ability_id.as_str(),\n                    0, pokemon_pos, pokemon_pos,\n                "", // TODO: Wire through actual move_id'
  },

  // SourceModifySecondaries: (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"SourceModifySecondaries" => ability_callbacks::dispatch_on_source_modify_secondaries\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifySecondaries" => ability_callbacks::dispatch_on_source_modify_secondaries(\n                self,\n                ability_id.as_str(),\n                Some(pokemon_pos), None,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceModifySpA: (move_id: &str)
  {
    find: /"SourceModifySpA" => ability_callbacks::dispatch_on_source_modify_sp_a\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifySpA" => ability_callbacks::dispatch_on_source_modify_sp_a(\n                self,\n                ability_id.as_str(),\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceModifySpAPriority: (move_id: &str)
  {
    find: /"SourceModifySpAPriority" => ability_callbacks::dispatch_on_source_modify_sp_a_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceModifySpAPriority" => ability_callbacks::dispatch_on_source_modify_sp_a_priority(\n                self,\n                ability_id.as_str(),\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // SourceTryHeal: no params (just battle, ability_id)
  {
    find: /"SourceTryHeal" => ability_callbacks::dispatch_on_source_try_heal\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"SourceTryHeal" => ability_callbacks::dispatch_on_source_try_heal(\n                self,\n                ability_id.as_str()\n            )'
  },

  // SourceTryPrimaryHit: (target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str)
  {
    find: /"SourceTryPrimaryHit" => ability_callbacks::dispatch_on_source_try_primary_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*\/\/ TODO[^)]*\)/g,
    replace: '"SourceTryPrimaryHit" => ability_callbacks::dispatch_on_source_try_primary_hit(\n                self,\n                ability_id.as_str(),\n                Some(pokemon_pos), None,\n            "", // TODO: Wire through actual move_id\n            )'
  },

  // TakeItem: no params (just battle, ability_id)
  {
    find: /"TakeItem" => ability_callbacks::dispatch_on_take_item\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"TakeItem" => ability_callbacks::dispatch_on_take_item(\n                self,\n                ability_id.as_str()\n            )'
  },

  // TryBoost: no params (just battle, ability_id)
  {
    find: /"TryBoost" => ability_callbacks::dispatch_on_try_boost\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"TryBoost" => ability_callbacks::dispatch_on_try_boost(\n                self,\n                ability_id.as_str()\n            )'
  },

  // TryHeal: no params (just battle, ability_id)
  {
    find: /"TryHeal" => ability_callbacks::dispatch_on_try_heal\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: '"TryHeal" => ability_callbacks::dispatch_on_try_heal(\n                self,\n                ability_id.as_str()\n            )'
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
