const fs = require('fs');
const path = require('path');

// Read all Rust files and update add_volatile calls
function updateFile(filePath) {
    let content = fs.readFileSync(filePath, 'utf8');
    let modified = false;

    // Pattern 1: battle.sides[side_idx].pokemon[poke_idx].add_volatile(id)
    // Replace with: Pokemon::add_volatile(battle, (side_idx, poke_idx), id, None)
    const pattern1 = /battle\.sides\[(\w+)\]\.pokemon\[(\w+)\]\.add_volatile\(([^)]+)\)/g;
    if (pattern1.test(content)) {
        content = content.replace(pattern1, 'Pokemon::add_volatile(battle, ($1, $2), $3, None)');
        modified = true;

        // Add import if not present
        if (!content.includes('use crate::pokemon::Pokemon;')) {
            content = content.replace(
                /(use crate::dex_data::ID;)/,
                '$1\n    use crate::pokemon::Pokemon;'
            );
        }
    }

    if (modified) {
        fs.writeFileSync(filePath, content);
        console.log(`Updated: ${filePath}`);
    }
}

// Get all Rust files
const files = process.argv.slice(2);
files.forEach(updateFile);
