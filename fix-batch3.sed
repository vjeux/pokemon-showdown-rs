# Fix AnyAfterTerastallization (takes 2, currently 3)
s/ability_callbacks::dispatch_on_any_after_terastallization(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_any_after_terastallization(self, ability_id.as_str())/g

# Fix AnyBeforeMove (takes 2, currently 3)
s/ability_callbacks::dispatch_on_any_before_move(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_any_before_move(self, ability_id.as_str())/g

# Fix AnyFaintPriority (takes 2, currently 3)
s/ability_callbacks::dispatch_on_any_faint_priority(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_any_faint_priority(self, ability_id.as_str())/g

# Fix AnySwitchIn (takes 2)
s/ability_callbacks::dispatch_on_any_switch_in(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_any_switch_in(self, ability_id.as_str())/g

# Fix AnySwitchInPriority (takes 2)
s/ability_callbacks::dispatch_on_any_switch_in_priority(\s*self,\s*ability_id\.as_str(),\s*pokemon_pos,\s*)/ability_callbacks::dispatch_on_any_switch_in_priority(self, ability_id.as_str())/g

# Fix AnyTryMove (takes 2)
s/ability_callbacks::dispatch_on_any_try_move(self, ability_id\.as_str(), pokemon_pos)/ability_callbacks::dispatch_on_any_try_move(self, ability_id.as_str())/g
