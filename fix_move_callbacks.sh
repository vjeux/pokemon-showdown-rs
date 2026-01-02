#!/bin/bash

# List of move callback files that use cure_status
CURE_STATUS_MOVES="aromatherapy healbell junglehealing lunarblessing purify refresh sparklingaria sparklyswirl takeheart"

# List of move callback files that use try_set_status
TRY_SET_STATUS_MOVES="banefulbunker beakblast burningbulwark burningjealousy direclaw gmaxbefuddle gmaxmalodor gmaxstunshock gmaxsweetness gmaxvoltcrash smellingsalts toxicspikes triattack uproar wakeupslap yawn"

# Add Pokemon import
for move in $CURE_STATUS_MOVES $TRY_SET_STATUS_MOVES; do
  file="/Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/move_callbacks/${move}.rs"
  if [ -f "$file" ] && ! grep -q "use crate::Pokemon;" "$file"; then
    echo "Adding Pokemon import to $move.rs"
    sed -i '' '/use crate::event::EventResult;/a\
use crate::Pokemon;
' "$file"
  fi
done

# Fix cure_status pattern in moves
for move in $CURE_STATUS_MOVES; do
  file="/Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/move_callbacks/${move}.rs"
  if [ -f "$file" ]; then
    echo "Fixing cure_status in $move.rs"
    # Pattern 1: pokemon_mut.cure_status -> Pokemon::cure_status
    perl -i -pe 's/pokemon_mut\.cure_status\(/Pokemon::cure_status(battle, pokemon_pos, /g' "$file"
    # Pattern 2: ally_mut.cure_status -> Pokemon::cure_status (for allies iteration)
    perl -i -pe 's/ally_mut\.cure_status\(/Pokemon::cure_status(battle, ally_pos, /g' "$file"
    # Pattern 3: pokemon.cure_status -> Pokemon::cure_status
    perl -i -pe 's/(\s+)pokemon\.cure_status\(/$1Pokemon::cure_status(battle, pokemon_pos, /g' "$file"
  fi
done

echo "Move callbacks updated!"
