#!/usr/bin/env node

const fs = require('fs');

// Load the fix mapping
const fixes = JSON.parse(fs.readFileSync('fix-mapping.json', 'utf8'));

// Read battle.rs
let battleContent = fs.readFileSync('src/battle.rs', 'utf8');

// Manual fixes for specific patterns that need special handling
const manualFixes = {
  // These take additional params beyond the standard ones
  'AnyModifyAccuracy': {
    find: /ability_callbacks::dispatch_on_any_modify_accuracy\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_accuracy(self, ability_id.as_str(), 0, Some(pokemon_pos), None, "")'
  },
  'AnyModifyAccuracyPriority': {
    find: /ability_callbacks::dispatch_on_any_modify_accuracy_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_accuracy_priority(self, ability_id.as_str(), 0, Some(pokemon_pos), None, "")'
  },
  'AnyModifyAtk': {
    find: /ability_callbacks::dispatch_on_any_modify_atk\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_atk(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'AnyModifyDamage': {
    find: /ability_callbacks::dispatch_on_any_modify_damage\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_damage(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'AnyModifyDef': {
    find: /ability_callbacks::dispatch_on_any_modify_def\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_def(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'AnyModifySpA': {
    find: /ability_callbacks::dispatch_on_any_modify_sp_a\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_sp_a(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'AnyModifySpD': {
    find: /ability_callbacks::dispatch_on_any_modify_sp_d\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_sp_d(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'AnyInvulnerability': {
    find: /ability_callbacks::dispatch_on_any_invulnerability\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_invulnerability(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },
  'AnyInvulnerabilityPriority': {
    find: /ability_callbacks::dispatch_on_any_invulnerability_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_invulnerability_priority(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },
  'AnyTryPrimaryHit': {
    find: /ability_callbacks::dispatch_on_any_try_primary_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_try_primary_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'AnyRedirectTarget': {
    find: /ability_callbacks::dispatch_on_any_redirect_target\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_any_redirect_target(self, ability_id.as_str(), pokemon_pos, None, "")'
  },
  'AnySetWeather': {
    find: /ability_callbacks::dispatch_on_any_set_weather\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_set_weather(self, ability_id.as_str(), "")'
  },
  'BasePower': {
    find: /ability_callbacks::dispatch_on_base_power\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_base_power(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'BasePowerPriority': {
    find: /ability_callbacks::dispatch_on_base_power_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_base_power_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'BeforeMove': {
    find: /ability_callbacks::dispatch_on_before_move\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_before_move(self, ability_id.as_str(), pokemon_pos, None, "")'
  },
  'BeforeMovePriority': {
    find: /ability_callbacks::dispatch_on_before_move_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_before_move_priority(self, ability_id.as_str(), pokemon_pos, None, "")'
  },
  'CriticalHit': {
    find: /ability_callbacks::dispatch_on_critical_hit\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_critical_hit(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },
  'DamagingHit': {
    find: /ability_callbacks::dispatch_on_damaging_hit\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_damaging_hit(self, ability_id.as_str(), 0, Some(pokemon_pos), None, "")'
  },
  'DamagingHitOrder': {
    find: /ability_callbacks::dispatch_on_damaging_hit_order\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_damaging_hit_order(self, ability_id.as_str(), 0, Some(pokemon_pos), None, "")'
  },
  'DeductPP': {
    find: /ability_callbacks::dispatch_on_deduct_p_p\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_deduct_p_p(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'Effectiveness': {
    find: /ability_callbacks::dispatch_on_effectiveness\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_effectiveness(self, ability_id.as_str(), 0, pokemon_pos, "", "")'
  },
  'Hit': {
    find: /ability_callbacks::dispatch_on_hit\(self, ability_id\.as_str\(\), pokemon_pos, ""\)/g,
    replace: 'ability_callbacks::dispatch_on_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'ModifyAccuracy': {
    find: /ability_callbacks::dispatch_on_modify_accuracy\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_accuracy(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifyAccuracyPriority': {
    find: /ability_callbacks::dispatch_on_modify_accuracy_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_accuracy_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifyAtk': {
    find: /ability_callbacks::dispatch_on_modify_atk\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_atk(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifyAtkPriority': {
    find: /ability_callbacks::dispatch_on_modify_atk_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_atk_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifyCritRatio': {
    find: /ability_callbacks::dispatch_on_modify_crit_ratio\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_crit_ratio(self, ability_id.as_str(), 0, pokemon_pos, None)'
  },
  'ModifyDamage': {
    find: /ability_callbacks::dispatch_on_modify_damage\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_damage(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifyDef': {
    find: /ability_callbacks::dispatch_on_modify_def\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_def(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifyDefPriority': {
    find: /ability_callbacks::dispatch_on_modify_def_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_def_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifyMove': {
    find: /ability_callbacks::dispatch_on_modify_move\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_move(self, ability_id.as_str(), "")'
  },
  'ModifyMovePriority': {
    find: /ability_callbacks::dispatch_on_modify_move_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_move_priority(self, ability_id.as_str(), "")'
  },
  'ModifyPriority': {
    find: /ability_callbacks::dispatch_on_modify_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_priority(self, ability_id.as_str(), 0, pokemon_pos, None, "")'
  },
  'ModifySTAB': {
    find: /ability_callbacks::dispatch_on_modify_s_t_a_b\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_s_t_a_b(self, ability_id.as_str(), 0.0, pokemon_pos, (0, 0), "")'
  },
  'ModifySecondaries': {
    find: /ability_callbacks::dispatch_on_modify_secondaries\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_secondaries(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'ModifySpA': {
    find: /ability_callbacks::dispatch_on_modify_sp_a\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_sp_a(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifySpAPriority': {
    find: /ability_callbacks::dispatch_on_modify_sp_a_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_sp_a_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'ModifySpe': {
    find: /ability_callbacks::dispatch_on_modify_spe\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_spe(self, ability_id.as_str(), 0, pokemon_pos)'
  },
  'ModifyType': {
    find: /ability_callbacks::dispatch_on_modify_type\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_type(self, ability_id.as_str(), "", pokemon_pos, None)'
  },
  'ModifyTypePriority': {
    find: /ability_callbacks::dispatch_on_modify_type_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_type_priority(self, ability_id.as_str(), "", pokemon_pos, None)'
  },
  'ModifyWeight': {
    find: /ability_callbacks::dispatch_on_modify_weight\(self, ability_id\.as_str\(\), pokemon_pos\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_weight(self, ability_id.as_str(), 0, pokemon_pos)'
  },
  'ModifyWeightPriority': {
    find: /ability_callbacks::dispatch_on_modify_weight_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_modify_weight_priority(self, ability_id.as_str(), 0, pokemon_pos)'
  },
  'PrepareHit': {
    find: /ability_callbacks::dispatch_on_prepare_hit\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_prepare_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'SourceAfterFaint': {
    find: /ability_callbacks::dispatch_on_source_after_faint\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_source_after_faint(self, ability_id.as_str(), pokemon_pos, (0, 0), None)'
  },
  'SourceBasePower': {
    find: /ability_callbacks::dispatch_on_source_base_power\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_base_power(self, ability_id.as_str(), 0, "")'
  },
  'SourceBasePowerPriority': {
    find: /ability_callbacks::dispatch_on_source_base_power_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_base_power_priority(self, ability_id.as_str(), 0, "")'
  },
  'SourceDamagingHit': {
    find: /ability_callbacks::dispatch_on_source_damaging_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_damaging_hit(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'SourceModifyAccuracy': {
    find: /ability_callbacks::dispatch_on_source_modify_accuracy\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_accuracy(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'SourceModifyAccuracyPriority': {
    find: /ability_callbacks::dispatch_on_source_modify_accuracy_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_accuracy_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'SourceModifyAtk': {
    find: /ability_callbacks::dispatch_on_source_modify_atk\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_atk(self, ability_id.as_str(), "")'
  },
  'SourceModifyAtkPriority': {
    find: /ability_callbacks::dispatch_on_source_modify_atk_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_atk_priority(self, ability_id.as_str(), "")'
  },
  'SourceModifyDamage': {
    find: /ability_callbacks::dispatch_on_source_modify_damage\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_damage(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'SourceModifyDamagePriority': {
    find: /ability_callbacks::dispatch_on_source_modify_damage_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_damage_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")'
  },
  'SourceModifySecondaries': {
    find: /ability_callbacks::dispatch_on_source_modify_secondaries\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_secondaries(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'SourceModifySpA': {
    find: /ability_callbacks::dispatch_on_source_modify_sp_a\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_sp_a(self, ability_id.as_str(), "")'
  },
  'SourceModifySpAPriority': {
    find: /ability_callbacks::dispatch_on_source_modify_sp_a_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_modify_sp_a_priority(self, ability_id.as_str(), "")'
  },
  'SourceTryPrimaryHit': {
    find: /ability_callbacks::dispatch_on_source_try_primary_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_try_primary_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'SourceTryHeal': {
    find: /ability_callbacks::dispatch_on_source_try_heal\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_source_try_heal(self, ability_id.as_str(), 0, Some(pokemon_pos), None, None)'
  },
  'TakeItem': {
    find: /ability_callbacks::dispatch_on_take_item\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_take_item(self, ability_id.as_str(), None, pokemon_pos, None)'
  },
  'TryHeal': {
    find: /ability_callbacks::dispatch_on_try_heal\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_try_heal(self, ability_id.as_str(), Some(pokemon_pos), None, None)'
  },
  'TryHit': {
    find: /ability_callbacks::dispatch_on_try_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_try_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'TryHitPriority': {
    find: /ability_callbacks::dispatch_on_try_hit_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_try_hit_priority(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
  },
  'FoeTryMove': {
    find: /ability_callbacks::dispatch_on_foe_try_move\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_foe_try_move(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },
  'FractionalPriority': {
    find: /ability_callbacks::dispatch_on_fractional_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_fractional_priority(self, ability_id.as_str(), 0.0, pokemon_pos, None, "")'
  },
  'FractionalPriorityPriority': {
    find: /ability_callbacks::dispatch_on_fractional_priority_priority\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",?\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_fractional_priority_priority(self, ability_id.as_str(), 0.0, pokemon_pos, None, "")'
  },
  'AllyBasePowerInline': {
    find: /ability_callbacks::dispatch_on_ally_base_power\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_ally_base_power(self, ability_id.as_str(), 0, "")'
  },
  'AllyBasePowerPriorityInline': {
    find: /ability_callbacks::dispatch_on_ally_base_power_priority\(self, ability_id\.as_str\(\), pokemon_pos, ""\s*(?:\/\/ TODO[^\n]*)?\)/g,
    replace: 'ability_callbacks::dispatch_on_ally_base_power_priority(self, ability_id.as_str(), 0, "")'
  },
};

// Apply manual fixes
Object.values(manualFixes).forEach(fix => {
  battleContent = battleContent.replace(fix.find, fix.replace);
});

// Write the fixed content back
fs.writeFileSync('src/battle.rs', battleContent);

console.log('Applied all fixes to battle.rs');
