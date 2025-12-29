# Fix FoeAfterBoost
s/ability_callbacks::dispatch_on_foe_after_boost(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_foe_after_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)/g

# Fix FoeMaybeTrapPokemon
s/ability_callbacks::dispatch_on_foe_maybe_trap_pokemon(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_foe_maybe_trap_pokemon(self, ability_id.as_str(), Some(pokemon_pos))/g

# Fix FoeTryEatItem
s/ability_callbacks::dispatch_on_foe_try_eat_item(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_foe_try_eat_item(self, ability_id.as_str(), Some(pokemon_pos), None)/g

# Fix AnyModifyBoost
s/ability_callbacks::dispatch_on_any_modify_boost(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_any_modify_boost(self, ability_id.as_str(), "", pokemon_pos)/g
