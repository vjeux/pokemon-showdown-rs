#!/bin/bash

# Fix berry item callbacks that have simple cure_status calls without result capture

BERRIES="aspearberry burntberry cheriberry chestoberry iceberry lumberry mintberry miracleberry pechaberry przcureberry psncureberry rawstberry"

for berry in $BERRIES; do
  file="/Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/item_callbacks/${berry}.rs"
  if [ -f "$file" ]; then
    echo "Fixing $berry.rs..."
    # Replace pokemon_mut.cure_status(false); with Pokemon::cure_status(battle, pokemon_pos, false);
    sed -i '' 's/pokemon_mut\.cure_status(false);/Pokemon::cure_status(battle, pokemon_pos, false);/g' "$file"
    # Also replace pokemon_mut.try_set_status if present
    sed -i '' 's/pokemon_mut\.try_set_status(/Pokemon::try_set_status(battle, pokemon_pos, /g' "$file"
  fi
done

echo "Berry items fixed!"
