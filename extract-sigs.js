#!/usr/bin/env node

const fs = require('fs');

// Read the mod.rs to extract all dispatcher signatures
const modContent = fs.readFileSync('src/data/ability_callbacks/mod.rs', 'utf8');

// Extract dispatcher signatures
const lines = modContent.split('\n');
const dispatcherSigs = new Map();

for (let i = 0; i < lines.length; i++) {
  const line = lines[i];
  if (line.includes('pub fn dispatch_')) {
    const match = line.match(/pub fn (dispatch_\w+)\(/);
    if (match) {
      const funcName = match[1];
      let params = [];

      // Collect parameters until we find ) -> EventResult
      let j = i;
      let paramText = '';
      while (j < lines.length && !lines[j].includes(') -> EventResult')) {
        paramText += lines[j] + ' ';
        j++;
      }
      if (j < lines.length) {
        paramText += lines[j];
      }

      // Extract params between ability_id: &str, and ) -> EventResult
      const paramsMatch = paramText.match(/ability_id: &str,\s*([^)]*)\) -> EventResult/);
      if (paramsMatch && paramsMatch[1].trim()) {
        const rawParams = paramsMatch[1].trim();
        params = rawParams.split(',').map(p => p.trim()).filter(p => p);
      }

      dispatcherSigs.set(funcName, params);
    }
  }
}

console.log(`Found ${dispatcherSigs.size} dispatchers\n`);

// Create fix mappings
const fixes = new Map();

dispatcherSigs.forEach((params, funcName) => {
  const callArgs = params.map(param => {
    const paramType = param.split(':')[1].trim();

    // Generate appropriate default value
    if (paramType.includes('Option<(usize, usize)>')) {
      // Check if it's target_pos or source_pos
      if (param.includes('target_pos')) return 'Some(pokemon_pos)';
      return 'None';
    }
    if (paramType.includes('Option<&str>')) return 'None';
    if (paramType.includes('(usize, usize)')) return '(0, 0)';
    if (paramType.includes('&str')) return '""';
    if (paramType.includes('i32')) return '0';
    return 'None';
  });

  fixes.set(funcName, callArgs);
});

// Print top errors we need to fix
const priorityFixes = [
  'dispatch_on_any_modify_accuracy',
  'dispatch_on_any_modify_accuracy_priority',
  'dispatch_on_any_modify_atk',
  'dispatch_on_any_modify_damage',
  'dispatch_on_any_modify_def',
  'dispatch_on_any_modify_sp_a',
  'dispatch_on_any_modify_sp_d',
  'dispatch_on_any_invulnerability',
  'dispatch_on_any_invulnerability_priority',
  'dispatch_on_any_try_primary_hit',
  'dispatch_on_modify_accuracy',
  'dispatch_on_modify_accuracy_priority',
  'dispatch_on_modify_atk',
  'dispatch_on_modify_atk_priority',
  'dispatch_on_modify_def',
  'dispatch_on_modify_damage',
  'dispatch_on_modify_move',
  'dispatch_on_modify_move_priority',
  'dispatch_on_modify_priority',
  'dispatch_on_modify_sp_a',
  'dispatch_on_modify_sp_a_priority',
  'dispatch_on_modify_type',
  'dispatch_on_modify_type_priority',
  'dispatch_on_fractional_priority',
  'dispatch_on_fractional_priority_priority',
  'dispatch_on_source_modify_accuracy',
  'dispatch_on_source_modify_accuracy_priority',
  'dispatch_on_source_modify_atk',
  'dispatch_on_source_modify_atk_priority',
  'dispatch_on_source_modify_damage',
  'dispatch_on_source_modify_damage_priority',
  'dispatch_on_source_modify_sp_a',
  'dispatch_on_source_modify_sp_a_priority',
  'dispatch_on_base_power',
  'dispatch_on_base_power_priority',
  'dispatch_on_hit',
  'dispatch_on_try_hit',
  'dispatch_on_try_hit_priority',
  'dispatch_on_prepare_hit',
  'dispatch_on_critical_hit',
  'dispatch_on_damaging_hit',
  'dispatch_on_damaging_hit_order',
  'dispatch_on_effectiveness',
  'dispatch_on_before_move',
  'dispatch_on_before_move_priority',
  'dispatch_on_modify_s_t_a_b',
  'dispatch_on_foe_try_move',
  'dispatch_on_any_redirect_target',
  'dispatch_on_source_base_power',
  'dispatch_on_source_base_power_priority',
  'dispatch_on_source_damaging_hit',
  'dispatch_on_source_modify_secondaries',
  'dispatch_on_modify_secondaries',
  'dispatch_on_source_try_primary_hit',
  'dispatch_on_source_after_faint',
  'dispatch_on_source_try_heal',
  'dispatch_on_try_heal',
  'dispatch_on_take_item',
  'dispatch_on_deduct_p_p',
  'dispatch_on_modify_crit_ratio',
  'dispatch_on_modify_weight',
  'dispatch_on_modify_weight_priority',
  'dispatch_on_ally_base_power',
  'dispatch_on_ally_base_power_priority',
];

console.log('Priority fixes:\n');
priorityFixes.forEach(name => {
  const args = fixes.get(name);
  if (args && args.length > 0) {
    console.log(`${name}(`);
    console.log(`    self,`);
    console.log(`    ability_id.as_str(),`);
    args.forEach((arg, idx) => {
      const comma = idx < args.length - 1 ? ',' : '';
      console.log(`    ${arg}${comma}`);
    });
    console.log(`)`);
    console.log('');
  }
});

// Save to JSON for programmatic use
fs.writeFileSync('fix-mapping.json', JSON.stringify(Object.fromEntries(fixes), null, 2));
console.log(`\nSaved fix mapping to fix-mapping.json`);
