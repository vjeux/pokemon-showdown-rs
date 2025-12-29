#!/usr/bin/env node

/**
 * Fix TODOs in battle.rs to wire up actual values from context
 * Based on available context in handle_item_event and handle_move_event functions
 */

const fs = require('fs');
const path = require('path');

const battlePath = path.join(__dirname, 'src', 'battle.rs');
let battleContent = fs.readFileSync(battlePath, 'utf8');

const fixes = [
  // BasePower event for items: extract move info from active_move
  {
    find: /"BasePower" => \{\s*item_callbacks::dispatch_on_base_power\(self, item_id\.as_str\(\), 0, pokemon_pos, None, ""\) \/\/ TODO: Wire through actual base_power, target_pos, move_id\s*\}/,
    replace: `"BasePower" => {
                // Extract move info from active_move (clone to avoid borrow issues)
                let (base_power, move_id_str) = if let Some(ref active_move) = self.active_move {
                    (active_move.base_power, active_move.id.to_string())
                } else {
                    (0, String::new())
                };
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_base_power(self, item_id.as_str(), base_power, pokemon_pos, target_pos, &move_id_str)
            }`
  },

  // DamagingHit event for items: damage is in relay_var, source/target from current_event
  {
    find: /"DamagingHit" => \{\s*item_callbacks::dispatch_on_damaging_hit\(self, item_id\.as_str\(\), 0, pokemon_pos, pokemon_pos, ""\) \/\/ TODO: Wire through actual damage, target_pos, source_pos, move_id\s*\}/,
    replace: `"DamagingHit" => {
                let damage = relay_var.unwrap_or(0);
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                // target is the pokemon being hit, source is the attacker
                let target_pos = pokemon_pos;
                let source_pos = source.unwrap_or(pokemon_pos);
                item_callbacks::dispatch_on_damaging_hit(self, item_id.as_str(), damage, target_pos, source_pos, &move_id_str)
            }`
  },

  // ModifyDamage event for items: damage is in relay_var
  {
    find: /"ModifyDamage" => \{\s*item_callbacks::dispatch_on_modify_damage\(self, item_id\.as_str\(\), 0, pokemon_pos, None, ""\) \/\/ TODO: Wire through actual damage, target_pos, move_id\s*\}/,
    replace: `"ModifyDamage" => {
                let damage = relay_var.unwrap_or(0);
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_modify_damage(self, item_id.as_str(), damage, pokemon_pos, target_pos, &move_id_str)
            }`
  },

  // ModifyMove event for items: move_id from active_move
  {
    find: /"ModifyMove" => \{\s*item_callbacks::dispatch_on_modify_move\(self, item_id\.as_str\(\), "", pokemon_pos\) \/\/ TODO: Wire through actual move_id\s*\}/,
    replace: `"ModifyMove" => {
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                item_callbacks::dispatch_on_modify_move(self, item_id.as_str(), &move_id_str, pokemon_pos)
            }`
  },

  // SourceModifyDamage event for items
  {
    find: /"SourceModifyDamage" => item_callbacks::dispatch_on_source_modify_damage\(\s*self,\s*item_id\.as_str\(\),\s*0, pokemon_pos, pokemon_pos, "" \/\/ TODO: Wire through actual damage, source_pos, target_pos, move_id\s*\)/,
    replace: `"SourceModifyDamage" => {
                let damage = relay_var.unwrap_or(0);
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                let source_pos = pokemon_pos;
                let target_pos = target.unwrap_or(pokemon_pos);
                item_callbacks::dispatch_on_source_modify_damage(self, item_id.as_str(), damage, source_pos, target_pos, &move_id_str)
            }`
  },

  // TryHit event for items
  {
    find: /"TryHit" => \{\s*if let Some\(source_pos\) = source \{\s*item_callbacks::dispatch_on_try_hit\(self, item_id\.as_str\(\), pokemon_pos, source_pos, ""\) \/\/ TODO: Wire through actual move_id\s*\} else \{\s*EventResult::Continue\s*\}\s*\}/,
    replace: `"TryHit" => {
                if let Some(source_pos) = source {
                    let move_id_str = if let Some(ref active_move) = self.active_move {
                        active_move.id.to_string()
                    } else {
                        String::new()
                    };
                    item_callbacks::dispatch_on_try_hit(self, item_id.as_str(), pokemon_pos, source_pos, &move_id_str)
                } else {
                    EventResult::Continue
                }
            }`
  },

  // AfterMoveSecondarySelf event for items
  {
    find: /"AfterMoveSecondarySelf" => \{\s*if let Some\(source_pos\) = source \{\s*item_callbacks::dispatch_on_after_move_secondary_self\(\s*self,\s*item_id\.as_str\(\),\s*source_pos,\s*target,\s*"", \/\/ TODO: Wire through actual move_id\s*\)\s*\} else \{\s*EventResult::Continue\s*\}\s*\}/,
    replace: `"AfterMoveSecondarySelf" => {
                if let Some(source_pos) = source {
                    let move_id_str = if let Some(ref active_move) = self.active_move {
                        active_move.id.to_string()
                    } else {
                        String::new()
                    };
                    item_callbacks::dispatch_on_after_move_secondary_self(
                        self,
                        item_id.as_str(),
                        source_pos,
                        target,
                        &move_id_str,
                    )
                } else {
                    EventResult::Continue
                }
            }`
  },

  // ModifyType event for moves
  {
    find: /"ModifyType" => move_callbacks::dispatch_on_modify_type\(self, move_id, source_pos, None\), \/\/ TODO: Wire through actual target_pos/,
    replace: '"ModifyType" => move_callbacks::dispatch_on_modify_type(self, move_id, source_pos, target),'
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
