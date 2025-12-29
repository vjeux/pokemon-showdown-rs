# Fix multiline AnyBeforeMove
/"AnyBeforeMove" => ability_callbacks::dispatch_on_any_before_move(/,/)/{ 
  s/pokemon_pos,\s*)/)/g
}

# Fix multiline AnyFaintPriority
/"AnyFaintPriority" => ability_callbacks::dispatch_on_any_faint_priority(/,/)/{ 
  s/pokemon_pos,\s*)/)/g
}

# Fix multiline AnySwitchInPriority
/"AnySwitchInPriority" => ability_callbacks::dispatch_on_any_switch_in_priority(/,/)/{ 
  s/pokemon_pos,\s*)/)/g
}
