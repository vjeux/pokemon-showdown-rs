#!/bin/bash

# Read mod.rs
FILE="src/data/ability_callbacks/mod.rs"

# Fix sheerforce - add (0, 0) for pokemon_pos
sed -i '' 's/sheerforce::on_modify_move(battle, move_id)/sheerforce::on_modify_move(battle, move_id, (0, 0))/g' "$FILE"

# Fix unburden - remove source_pos
sed -i '' 's/unburden::on_take_item(battle, pokemon_pos, source_pos)/unburden::on_take_item(battle, pokemon_pos)/g' "$FILE"

# Fix ripen - add pokemon_pos
sed -i '' 's/ripen::on_try_eat_item(battle)/ripen::on_try_eat_item(battle, (0, 0))/g' "$FILE"

# Fix iceface - add None for source_pos
sed -i '' 's/iceface::on_weather_change(battle, pokemon_pos)/iceface::on_weather_change(battle, pokemon_pos, None)/g' "$FILE"

echo "Applied fixes"
