#!/usr/bin/env node

const fs = require('fs');

const files = [
    'src/data/move_callbacks/spiritshackle.rs',
    'src/data/move_callbacks/meanlook.rs',
    'src/data/move_callbacks/jawlock.rs',
    'src/data/move_callbacks/block.rs',
];

files.forEach(file => {
    const content = fs.readFileSync(file, 'utf8');

    // Pattern: target_pokemon.add_volatile(ID::from("trapped"))
    // Context: we have battle and target position
    const newContent = content
        .replace(
            /if let Some\(target_pokemon\) = battle\.pokemon_at_mut\(target\.0, target\.1\) \{\s*target_pokemon\.add_volatile\(ID::from\("trapped"\)\);\s*\}/g,
            'Pokemon::add_volatile(battle, target, ID::from("trapped"), Some(source_pos));'
        )
        .replace(
            /(use crate::event::EventResult;)/,
            '$1\nuse crate::pokemon::Pokemon;'
        );

    if (content !== newContent) {
        fs.writeFileSync(file, newContent);
        console.log(`Updated: ${file}`);
    }
});
