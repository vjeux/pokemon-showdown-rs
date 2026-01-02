#!/bin/bash

# Final fix for all remaining method call patterns

# Item callbacks with pokemon.cure_status pattern
for file in iceberry przcureberry psncureberry rawstberry; do
  filepath="/Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/item_callbacks/${file}.rs"
  if [ -f "$filepath" ]; then
    echo "Fixing $file.rs..."
    perl -i -pe 's/pokemon\.cure_status\(false\)/Pokemon::cure_status(battle, pokemon_pos, false)/g' "$filepath"
  fi
done

# Move callbacks with various patterns
for file in banefulbunker beakblast burningbulwark burningjealousy direclaw gmaxbefuddle gmaxmalodor gmaxstunshock gmaxsweetness gmaxvoltcrash wakeupslap; do
  filepath="/Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/move_callbacks/${file}.rs"
  if [ -f "$filepath" ]; then
    echo "Fixing $file.rs..."
    # Fix any remaining pokemon.try_set_status
    perl -i -pe 's/pokemon\.try_set_status\(([^,]+),\s*([^)]+)\)/Pokemon::try_set_status(battle, pokemon_pos, $1, $2)/g' "$filepath"
    # Fix source.try_set_status
    perl -i -pe 's/source\.try_set_status\(([^,]+),\s*([^)]+)\)/Pokemon::try_set_status(battle, source_pos, $1, $2)/g' "$filepath"
    # Fix target.try_set_status
    perl -i -pe 's/target\.try_set_status\(([^,]+),\s*([^)]+)\)/Pokemon::try_set_status(battle, target_pos, $1, $2)/g' "$filepath"
  fi
done

echo "All remaining files fixed!"
