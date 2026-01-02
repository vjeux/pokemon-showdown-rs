#!/bin/bash

# Script to fix cure_status calls in ability callback files

for file in /Users/vjeux/random/showdown/pokemon-showdown-rs/src/data/ability_callbacks/{limber,vitalspirit,magmaarmor,waterveil,immunity,insomnia}.rs; do
  echo "Processing $file..."

  # Replace the pattern:
  # From:
  #   let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
  #       Some(p) => p,
  #       None => return EventResult::Continue,
  #   };
  #
  #   if let Some((status, removed_nightmare, _silent)) = pokemon_mut.cure_status(false) {
  #
  # To:
  #   if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, pokemon_pos, false) {

  perl -i -0pe 's/        let pokemon_mut = match battle\.pokemon_at_mut\(pokemon_pos\.0, pokemon_pos\.1\) \{\n            Some\(p\) => p,\n            None => return EventResult::Continue,\n        \};\n\n        if let Some\(\(status, removed_nightmare, _silent\)\) = pokemon_mut\.cure_status\(false\)/        if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, pokemon_pos, false)/g' "$file"

done

echo "Done!"
