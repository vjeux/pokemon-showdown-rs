# Fix AllyFaint
s/ability_callbacks::dispatch_on_ally_faint(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_ally_faint(self, ability_id.as_str(), Some(pokemon_pos))/g

# Fix AnyDamage  
s/ability_callbacks::dispatch_on_any_damage(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_any_damage(self, ability_id.as_str(), 0, Some(pokemon_pos), None, None)/g

# Fix AnyFaint
s/ability_callbacks::dispatch_on_any_faint(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_any_faint(self, ability_id.as_str(), Some(pokemon_pos))/g

# Fix Damage
s/ability_callbacks::dispatch_on_damage(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_damage(self, ability_id.as_str(), 0, pokemon_pos, None, None)/g

# Fix SetStatus
s/ability_callbacks::dispatch_on_set_status(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_set_status(self, ability_id.as_str(), "", pokemon_pos, None, None)/g

# Fix ChangeBoost
s/ability_callbacks::dispatch_on_change_boost(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_change_boost(self, ability_id.as_str(), "", Some(pokemon_pos), None, None)/g

# Fix Immunity
s/ability_callbacks::dispatch_on_immunity(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_immunity(self, ability_id.as_str(), "", pokemon_pos)/g
