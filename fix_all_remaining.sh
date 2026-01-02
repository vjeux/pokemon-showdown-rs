#!/bin/bash

# Final comprehensive fix for all remaining cure_status and try_set_status calls

# Fix all move callback files
for file in /Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/move_callbacks/*.rs; do
  if [ -f "$file" ]; then
    # Fix pokemon_mut.cure_status(false) pattern
    perl -i -pe 's/pokemon_mut\.cure_status\(false\)/Pokemon::cure_status(battle, pokemon_pos, false)/g' "$file"
    # Fix target_mut.cure_status(false) pattern
    perl -i -pe 's/target_mut\.cure_status\(false\)/Pokemon::cure_status(battle, target_pos, false)/g' "$file"
    # Fix source_mut.cure_status(false) pattern
    perl -i -pe 's/source_mut\.cure_status\(false\)/Pokemon::cure_status(battle, source_pos, false)/g' "$file"

    # Fix pokemon_mut.try_set_status pattern
    perl -i -pe 's/pokemon_mut\.try_set_status\(([^,]+),\s*([^)]+)\)/Pokemon::try_set_status(battle, pokemon_pos, $1, $2)/g' "$file"
    # Fix target_mut.try_set_status pattern
    perl -i -pe 's/target_mut\.try_set_status\(([^,]+),\s*([^)]+)\)/Pokemon::try_set_status(battle, target_pos, $1, $2)/g' "$file"
    # Fix source_mut.try_set_status pattern
    perl -i -pe 's/source_mut\.try_set_status\(([^,]+),\s*([^)]+)\)/Pokemon::try_set_status(battle, source_pos, $1, $2)/g' "$file"
  fi
done

echo "All move callbacks fixed!"
