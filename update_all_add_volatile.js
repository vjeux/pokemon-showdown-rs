#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function updateFile(filePath) {
    let content = fs.readFileSync(filePath, 'utf8');
    let original = content;

    // Helper to add Pokemon import
    function ensurePokemonImport(text) {
        if (text.includes('use crate::pokemon::Pokemon;')) return text;
        if (text.includes('use crate::event::EventResult;')) {
            return text.replace(
                'use crate::event::EventResult;',
                'use crate::event::EventResult;\nuse crate::pokemon::Pokemon;'
            );
        }
        return text;
    }

    // Pattern 1: Simple pokemon_mut/pokemon in move callbacks with battle and positions
    // Context: function signature has battle, pokemon_pos, target_pos
    // Replace: pokemon_mut.add_volatile(id) -> Pokemon::add_volatile(battle, pokemon_pos, id, None)

    // Check if this is a move/ability callback with standard signature
    const hasBattleAndPositions = content.match(/battle: &mut Battle.*pokemon_pos: \(usize, usize\)/s);

    if (hasBattleAndPositions) {
        // Pattern: pokemon_mut.add_volatile(id) where pokemon_mut comes from pokemon_at_mut(pokemon_pos)
        content = content.replace(
            /let pokemon_mut = match battle\.pokemon_at_mut\(pokemon_pos\.0, pokemon_pos\.1\) \{\s*Some\(p\) => p,\s*None => return EventResult::Continue,\s*\};\s*pokemon_mut\.add_volatile\(([^)]+)\);/gs,
            'Pokemon::add_volatile(battle, pokemon_pos, $1, None);'
        );

        // Pattern: pokemon = battle.pokemon_at_mut(...); pokemon.add_volatile(...)
        content = content.replace(
            /let pokemon = match battle\.pokemon_at_mut\(pokemon_pos\.0, pokemon_pos\.1\) \{\s*Some\(p\) => p,\s*None => return EventResult::Continue,\s*\};\s*pokemon\.add_volatile\(([^)]+)\);/gs,
            'Pokemon::add_volatile(battle, pokemon_pos, $1, None);'
        );
    }

    if (content !== original) {
        content = ensurePokemonImport(content);
        fs.writeFileSync(filePath, content);
        console.log(`✓ Updated: ${path.relative(process.cwd(), filePath)}`);
        return true;
    }

    return false;
}

// Get files from command line or find all
const files = process.argv.slice(2);

if (files.length === 0) {
    console.log("Usage: node update_all_add_volatile.js <files...>");
    process.exit(1);
}

let count = 0;
files.forEach(file => {
    if (updateFile(file)) count++;
});

console.log(`\nUpdated ${count} file(s)`);
