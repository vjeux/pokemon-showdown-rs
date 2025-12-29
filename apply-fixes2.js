#!/usr/bin/env node

const fs = require('fs');

let battleContent = fs.readFileSync('src/battle.rs', 'utf8');

// Second batch of fixes with correct signatures
const fixes2 = [
  // Fix AnyModifyAccuracy - takes 3 params: accuracy, target_pos, source_pos (NO move_id)
  {
    find: /ability_callbacks::dispatch_on_any_modify_accuracy\(self, ability_id\.as_str\(\), 0, Some\(pokemon_pos\), None, ""\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_accuracy(self, ability_id.as_str(), 0, Some(pokemon_pos), None)'
  },
  {
    find: /ability_callbacks::dispatch_on_any_modify_accuracy_priority\(self, ability_id\.as_str\(\), 0, Some\(pokemon_pos\), None, ""\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_accuracy_priority(self, ability_id.as_str(), 0, Some(pokemon_pos), None)'
  },

  // Fix AnyModifyAtk - takes source_pos, target_pos, move_id (in that order!)
  {
    find: /ability_callbacks::dispatch_on_any_modify_atk\(self, ability_id\.as_str\(\), 0, pokemon_pos, \(0, 0\), ""\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_atk(self, ability_id.as_str(), None, Some(pokemon_pos), "")'
  },

  // Fix multiline AnyModifyDamage, Def, SpA, SpD
  {
    find: /ability_callbacks::dispatch_on_any_modify_damage\(\s*self,\s*ability_id\.as_str\(\),\s*0,\s*pokemon_pos,\s*\(0,\s*0\),\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_damage(self, ability_id.as_str(), 0, None, Some(pokemon_pos), "")'
  },
  {
    find: /ability_callbacks::dispatch_on_any_modify_def\(\s*self,\s*ability_id\.as_str\(\),\s*0,\s*pokemon_pos,\s*\(0,\s*0\),\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_def(self, ability_id.as_str(), None, Some(pokemon_pos), "")'
  },
  {
    find: /ability_callbacks::dispatch_on_any_modify_sp_a\(\s*self,\s*ability_id\.as_str\(\),\s*0,\s*pokemon_pos,\s*\(0,\s*0\),\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_sp_a(self, ability_id.as_str(), None, Some(pokemon_pos), "")'
  },
  {
    find: /ability_callbacks::dispatch_on_any_modify_sp_d\(\s*self,\s*ability_id\.as_str\(\),\s*0,\s*pokemon_pos,\s*\(0,\s*0\),\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_modify_sp_d(self, ability_id.as_str(), None, Some(pokemon_pos), "")'
  },

  // Fix AnyRedirectTarget - takes target_pos, source_pos, move_id
  {
    find: /ability_callbacks::dispatch_on_any_redirect_target\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*None,\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_redirect_target(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },

  // Fix AnySetWeather - takes target_pos, source_pos (NO move_id)
  {
    find: /ability_callbacks::dispatch_on_any_set_weather\(self, ability_id\.as_str\(\), ""\)/g,
    replace: 'ability_callbacks::dispatch_on_any_set_weather(self, ability_id.as_str(), Some(pokemon_pos), None)'
  },

  // Fix AnyTryPrimaryHit - takes target_pos, source_pos, move_id
  {
    find: /ability_callbacks::dispatch_on_any_try_primary_hit\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*\(0,\s*0\),\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_try_primary_hit(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },

  // Fix AnyInvulnerability - takes target_pos, source_pos, move_id
  {
    find: /ability_callbacks::dispatch_on_any_invulnerability\(\s*self,\s*ability_id\.as_str\(\),\s*Some\(pokemon_pos\),\s*None,\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_invulnerability(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },
  {
    find: /ability_callbacks::dispatch_on_any_invulnerability_priority\(\s*self,\s*ability_id\.as_str\(\),\s*Some\(pokemon_pos\),\s*None,\s*""\s*\)/g,
    replace: 'ability_callbacks::dispatch_on_any_invulnerability_priority(self, ability_id.as_str(), Some(pokemon_pos), None, "")'
  },
];

fixes2.forEach(fix => {
  const before = battleContent.match(fix.find);
  battleContent = battleContent.replace(fix.find, fix.replace);
  const after = battleContent.match(fix.find);

  if (before && !after) {
    console.log('✓ Applied fix');
  }
});

fs.writeFileSync('src/battle.rs', battleContent);

console.log('Applied second batch of fixes');
