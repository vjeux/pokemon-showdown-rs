#!/bin/bash

# Add Pokemon import to all item callback files that use cure_status
for file in /Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/item_callbacks/{aspearberry,burntberry,cheriberry,chestoberry,flameorb,iceberry,lumberry,mintberry,miracleberry,pechaberry,przcureberry,psncureberry,rawstberry,toxicorb}.rs; do
  if [ -f "$file" ] && ! grep -q "use crate::Pokemon;" "$file"; then
    echo "Adding Pokemon import to $file"
    sed -i '' '/use crate::event::EventResult;/a\
use crate::Pokemon;
' "$file"
  fi
done

# Fix cure_status calls in item callbacks
for file in /Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/item_callbacks/{aspearberry,burntberry,cheriberry,chestoberry,iceberry,lumberry,mintberry,miracleberry,pechaberry,przcureberry,psncureberry,rawstberry}.rs; do
  if [ -f "$file" ]; then
    echo "Fixing cure_status in $file"
    perl -i -0pe 's/        let pokemon_mut = match battle\.pokemon_at_mut\(pokemon_pos\.0, pokemon_pos\.1\) \{\n            Some\(p\) => p,\n            None => return EventResult::Continue,\n        \};\n\n        if let Some\(\(status, removed_nightmare, _silent\)\) = pokemon_mut\.cure_status\(false\)/        if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, pokemon_pos, false)/g' "$file"
  fi
done

# Fix try_set_status calls in flameorb and toxicorb
for file in /Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/item_callbacks/{flameorb,toxicorb}.rs; do
  if [ -f "$file" ]; then
    echo "Fixing try_set_status in $file"
    # This pattern may vary, need to check individual files
  fi
done

echo "Item callbacks updated!"
