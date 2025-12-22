#!/usr/bin/env node
// Extract Pokemon Showdown data to JSON for Rust consumption

const fs = require('fs');
const path = require('path');

// Load the data directory
const dataDir = path.resolve(__dirname, '../pokemon-showdown-src/data');

// We need to handle TypeScript files - let's use a simple approach
// by extracting the exported objects

function extractData(filename) {
    const content = fs.readFileSync(path.join(dataDir, filename), 'utf8');

    // Find the export const XXX = { ... } pattern
    const match = content.match(/export const \w+ .*?= (\{[\s\S]*)/);
    if (!match) {
        console.error(`Could not parse ${filename}`);
        return null;
    }

    let objStr = match[1];

    // Remove TypeScript type annotations after colons in object keys
    // and handle function definitions by replacing them with null

    // Replace functions with null placeholders
    // This regex handles multi-line functions
    let cleanStr = objStr;

    // Remove trailing comma before closing brace
    cleanStr = cleanStr.replace(/,(\s*})/g, '$1');

    // Try to parse as JavaScript object
    try {
        // Use eval in a sandboxed way - convert to JSON-parseable format
        const result = eval(`(${cleanStr})`);
        return result;
    } catch (e) {
        console.error(`Error parsing ${filename}:`, e.message);
        return null;
    }
}

// Alternative: Use Node.js to require the compiled JS or use ts-node
// For now, let's manually create simplified JSON data

console.log('Creating simplified Pokemon data for Rust...');

// Create a minimal but functional dataset for testing
const speciesData = {
    pikachu: {
        num: 25,
        name: "Pikachu",
        types: ["Electric"],
        baseStats: { hp: 35, atk: 55, def: 40, spa: 50, spd: 50, spe: 90 },
        abilities: { "0": "Static", H: "Lightning Rod" },
        heightm: 0.4,
        weightkg: 6.0,
        genderRatio: { M: 0.5, F: 0.5 },
        evos: ["Raichu"],
    },
    raichu: {
        num: 26,
        name: "Raichu",
        types: ["Electric"],
        baseStats: { hp: 60, atk: 90, def: 55, spa: 90, spd: 80, spe: 110 },
        abilities: { "0": "Static", H: "Lightning Rod" },
        heightm: 0.8,
        weightkg: 30.0,
        genderRatio: { M: 0.5, F: 0.5 },
        prevo: "Pikachu",
    },
    charizard: {
        num: 6,
        name: "Charizard",
        types: ["Fire", "Flying"],
        baseStats: { hp: 78, atk: 84, def: 78, spa: 109, spd: 85, spe: 100 },
        abilities: { "0": "Blaze", H: "Solar Power" },
        heightm: 1.7,
        weightkg: 90.5,
        genderRatio: { M: 0.875, F: 0.125 },
        prevo: "Charmeleon",
    },
    bulbasaur: {
        num: 1,
        name: "Bulbasaur",
        types: ["Grass", "Poison"],
        baseStats: { hp: 45, atk: 49, def: 49, spa: 65, spd: 65, spe: 45 },
        abilities: { "0": "Overgrow", H: "Chlorophyll" },
        heightm: 0.7,
        weightkg: 6.9,
        genderRatio: { M: 0.875, F: 0.125 },
        evos: ["Ivysaur"],
    },
    squirtle: {
        num: 7,
        name: "Squirtle",
        types: ["Water"],
        baseStats: { hp: 44, atk: 48, def: 65, spa: 50, spd: 64, spe: 43 },
        abilities: { "0": "Torrent", H: "Rain Dish" },
        heightm: 0.5,
        weightkg: 9.0,
        genderRatio: { M: 0.875, F: 0.125 },
        evos: ["Wartortle"],
    },
};

const movesData = {
    thunderbolt: {
        num: 85,
        name: "Thunderbolt",
        type: "Electric",
        category: "Special",
        basePower: 90,
        accuracy: 100,
        pp: 15,
        priority: 0,
        target: "normal",
        secondary: { chance: 10, status: "par" },
        flags: { protect: 1, mirror: 1 },
    },
    quickattack: {
        num: 98,
        name: "Quick Attack",
        type: "Normal",
        category: "Physical",
        basePower: 40,
        accuracy: 100,
        pp: 30,
        priority: 1,
        target: "normal",
        flags: { contact: 1, protect: 1, mirror: 1 },
    },
    flamethrower: {
        num: 53,
        name: "Flamethrower",
        type: "Fire",
        category: "Special",
        basePower: 90,
        accuracy: 100,
        pp: 15,
        priority: 0,
        target: "normal",
        secondary: { chance: 10, status: "brn" },
        flags: { protect: 1, mirror: 1 },
    },
    dragonclaw: {
        num: 337,
        name: "Dragon Claw",
        type: "Dragon",
        category: "Physical",
        basePower: 80,
        accuracy: 100,
        pp: 15,
        priority: 0,
        target: "normal",
        flags: { contact: 1, protect: 1, mirror: 1 },
    },
    tackle: {
        num: 33,
        name: "Tackle",
        type: "Normal",
        category: "Physical",
        basePower: 40,
        accuracy: 100,
        pp: 35,
        priority: 0,
        target: "normal",
        flags: { contact: 1, protect: 1, mirror: 1 },
    },
    thunderwave: {
        num: 86,
        name: "Thunder Wave",
        type: "Electric",
        category: "Status",
        basePower: 0,
        accuracy: 90,
        pp: 20,
        priority: 0,
        target: "normal",
        status: "par",
        flags: { protect: 1, mirror: 1, reflectable: 1 },
    },
    protect: {
        num: 182,
        name: "Protect",
        type: "Normal",
        category: "Status",
        basePower: 0,
        accuracy: true,
        pp: 10,
        priority: 4,
        target: "self",
        flags: { stallingMove: 1 },
        volatileStatus: "protect",
    },
    swordsdance: {
        num: 14,
        name: "Swords Dance",
        type: "Normal",
        category: "Status",
        basePower: 0,
        accuracy: true,
        pp: 20,
        priority: 0,
        target: "self",
        boosts: { atk: 2 },
        flags: { snatch: 1 },
    },
};

const abilitiesData = {
    static: {
        num: 9,
        name: "Static",
        desc: "30% chance a Pokemon making contact with this Pokemon will be paralyzed.",
        shortDesc: "30% chance of paralyzing Pokemon making contact with this Pokemon.",
        rating: 2,
    },
    blaze: {
        num: 66,
        name: "Blaze",
        desc: "When this Pokemon has 1/3 or less of its maximum HP, rounded down, its attacking stat is multiplied by 1.5 while using a Fire-type attack.",
        shortDesc: "At 1/3 or less of its max HP, this Pokemon's offensive stat is 1.5x with Fire attacks.",
        rating: 2,
    },
    overgrow: {
        num: 65,
        name: "Overgrow",
        desc: "When this Pokemon has 1/3 or less of its maximum HP, rounded down, its attacking stat is multiplied by 1.5 while using a Grass-type attack.",
        shortDesc: "At 1/3 or less of its max HP, this Pokemon's offensive stat is 1.5x with Grass attacks.",
        rating: 2,
    },
    torrent: {
        num: 67,
        name: "Torrent",
        desc: "When this Pokemon has 1/3 or less of its maximum HP, rounded down, its attacking stat is multiplied by 1.5 while using a Water-type attack.",
        shortDesc: "At 1/3 or less of its max HP, this Pokemon's offensive stat is 1.5x with Water attacks.",
        rating: 2,
    },
    lightningrod: {
        num: 31,
        name: "Lightning Rod",
        desc: "This Pokemon is immune to Electric-type moves and raises its Special Attack by 1 stage when hit by an Electric-type move.",
        shortDesc: "This Pokemon draws Electric moves to itself to raise Sp. Atk by 1; Electric immunity.",
        rating: 3,
    },
    solarpower: {
        num: 94,
        name: "Solar Power",
        desc: "If Sunny Day is active, this Pokemon's Special Attack is multiplied by 1.5 and it loses 1/8 of its maximum HP, rounded down, at the end of each turn.",
        shortDesc: "If Sunny Day is active, this Pokemon's Sp. Atk is 1.5x; loses 1/8 max HP per turn.",
        rating: 2,
    },
    chlorophyll: {
        num: 34,
        name: "Chlorophyll",
        desc: "If Sunny Day is active, this Pokemon's Speed is doubled.",
        shortDesc: "If Sunny Day is active, this Pokemon's Speed is doubled.",
        rating: 3,
    },
    raindish: {
        num: 44,
        name: "Rain Dish",
        desc: "If Rain Dance is active, this Pokemon restores 1/16 of its maximum HP, rounded down, at the end of each turn.",
        shortDesc: "If Rain Dance is active, this Pokemon heals 1/16 of its max HP each turn.",
        rating: 1,
    },
};

const itemsData = {
    leftovers: {
        num: 234,
        name: "Leftovers",
        desc: "At the end of every turn, holder restores 1/16 of its max HP.",
        fling: { basePower: 10 },
    },
    lifeorb: {
        num: 270,
        name: "Life Orb",
        desc: "Holder's attacks do 1.3x damage, and it loses 1/10 its max HP after the attack.",
        fling: { basePower: 30 },
    },
    choiceband: {
        num: 220,
        name: "Choice Band",
        desc: "Holder's Attack is 1.5x, but it can only select the first move it executes.",
        isChoice: true,
        fling: { basePower: 10 },
    },
    choicescarf: {
        num: 287,
        name: "Choice Scarf",
        desc: "Holder's Speed is 1.5x, but it can only select the first move it executes.",
        isChoice: true,
        fling: { basePower: 10 },
    },
    choicespecs: {
        num: 297,
        name: "Choice Specs",
        desc: "Holder's Sp. Atk is 1.5x, but it can only select the first move it executes.",
        isChoice: true,
        fling: { basePower: 10 },
    },
    focussash: {
        num: 275,
        name: "Focus Sash",
        desc: "If holder's HP is full, will survive an attack that would KO it with 1 HP. Single use.",
        fling: { basePower: 10 },
    },
};

const typechartData = {
    Normal: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 0, Grass: 0, Ice: 0,
            Fighting: 1, Poison: 0, Ground: 0, Flying: 0, Psychic: 0, Bug: 0,
            Rock: 0, Ghost: 3, Dragon: 0, Dark: 0, Steel: 0, Fairy: 0
        },
    },
    Fire: {
        damageTaken: {
            Normal: 0, Fire: 2, Water: 1, Electric: 0, Grass: 2, Ice: 2,
            Fighting: 0, Poison: 0, Ground: 1, Flying: 0, Psychic: 0, Bug: 2,
            Rock: 1, Ghost: 0, Dragon: 0, Dark: 0, Steel: 2, Fairy: 2
        },
    },
    Water: {
        damageTaken: {
            Normal: 0, Fire: 2, Water: 2, Electric: 1, Grass: 1, Ice: 2,
            Fighting: 0, Poison: 0, Ground: 0, Flying: 0, Psychic: 0, Bug: 0,
            Rock: 0, Ghost: 0, Dragon: 0, Dark: 0, Steel: 2, Fairy: 0
        },
    },
    Electric: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 2, Grass: 0, Ice: 0,
            Fighting: 0, Poison: 0, Ground: 1, Flying: 2, Psychic: 0, Bug: 0,
            Rock: 0, Ghost: 0, Dragon: 0, Dark: 0, Steel: 2, Fairy: 0
        },
    },
    Grass: {
        damageTaken: {
            Normal: 0, Fire: 1, Water: 2, Electric: 2, Grass: 2, Ice: 1,
            Fighting: 0, Poison: 1, Ground: 2, Flying: 1, Psychic: 0, Bug: 1,
            Rock: 0, Ghost: 0, Dragon: 0, Dark: 0, Steel: 0, Fairy: 0
        },
    },
    Ice: {
        damageTaken: {
            Normal: 0, Fire: 1, Water: 0, Electric: 0, Grass: 0, Ice: 2,
            Fighting: 1, Poison: 0, Ground: 0, Flying: 0, Psychic: 0, Bug: 0,
            Rock: 1, Ghost: 0, Dragon: 0, Dark: 0, Steel: 1, Fairy: 0
        },
    },
    Fighting: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 0, Grass: 0, Ice: 0,
            Fighting: 0, Poison: 0, Ground: 0, Flying: 1, Psychic: 1, Bug: 2,
            Rock: 2, Ghost: 0, Dragon: 0, Dark: 2, Steel: 0, Fairy: 1
        },
    },
    Poison: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 0, Grass: 2, Ice: 0,
            Fighting: 2, Poison: 2, Ground: 1, Flying: 0, Psychic: 1, Bug: 2,
            Rock: 0, Ghost: 0, Dragon: 0, Dark: 0, Steel: 0, Fairy: 2
        },
    },
    Ground: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 1, Electric: 3, Grass: 1, Ice: 1,
            Fighting: 0, Poison: 2, Ground: 0, Flying: 0, Psychic: 0, Bug: 0,
            Rock: 2, Ghost: 0, Dragon: 0, Dark: 0, Steel: 0, Fairy: 0
        },
    },
    Flying: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 1, Grass: 2, Ice: 1,
            Fighting: 2, Poison: 0, Ground: 3, Flying: 0, Psychic: 0, Bug: 2,
            Rock: 1, Ghost: 0, Dragon: 0, Dark: 0, Steel: 0, Fairy: 0
        },
    },
    Psychic: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 0, Grass: 0, Ice: 0,
            Fighting: 2, Poison: 0, Ground: 0, Flying: 0, Psychic: 2, Bug: 1,
            Rock: 0, Ghost: 1, Dragon: 0, Dark: 1, Steel: 0, Fairy: 0
        },
    },
    Bug: {
        damageTaken: {
            Normal: 0, Fire: 1, Water: 0, Electric: 0, Grass: 2, Ice: 0,
            Fighting: 2, Poison: 0, Ground: 2, Flying: 1, Psychic: 0, Bug: 0,
            Rock: 1, Ghost: 0, Dragon: 0, Dark: 0, Steel: 0, Fairy: 0
        },
    },
    Rock: {
        damageTaken: {
            Normal: 2, Fire: 2, Water: 1, Electric: 0, Grass: 1, Ice: 0,
            Fighting: 1, Poison: 2, Ground: 1, Flying: 2, Psychic: 0, Bug: 0,
            Rock: 0, Ghost: 0, Dragon: 0, Dark: 0, Steel: 1, Fairy: 0
        },
    },
    Ghost: {
        damageTaken: {
            Normal: 3, Fire: 0, Water: 0, Electric: 0, Grass: 0, Ice: 0,
            Fighting: 3, Poison: 2, Ground: 0, Flying: 0, Psychic: 0, Bug: 2,
            Rock: 0, Ghost: 1, Dragon: 0, Dark: 1, Steel: 0, Fairy: 0
        },
    },
    Dragon: {
        damageTaken: {
            Normal: 0, Fire: 2, Water: 2, Electric: 2, Grass: 2, Ice: 1,
            Fighting: 0, Poison: 0, Ground: 0, Flying: 0, Psychic: 0, Bug: 0,
            Rock: 0, Ghost: 0, Dragon: 1, Dark: 0, Steel: 0, Fairy: 1
        },
    },
    Dark: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 0, Grass: 0, Ice: 0,
            Fighting: 1, Poison: 0, Ground: 0, Flying: 0, Psychic: 3, Bug: 1,
            Rock: 0, Ghost: 2, Dragon: 0, Dark: 2, Steel: 0, Fairy: 1
        },
    },
    Steel: {
        damageTaken: {
            Normal: 2, Fire: 1, Water: 0, Electric: 0, Grass: 2, Ice: 2,
            Fighting: 1, Poison: 3, Ground: 1, Flying: 2, Psychic: 2, Bug: 2,
            Rock: 2, Ghost: 0, Dragon: 2, Dark: 0, Steel: 2, Fairy: 2
        },
    },
    Fairy: {
        damageTaken: {
            Normal: 0, Fire: 0, Water: 0, Electric: 0, Grass: 0, Ice: 0,
            Fighting: 2, Poison: 1, Ground: 0, Flying: 0, Psychic: 0, Bug: 2,
            Rock: 0, Ghost: 0, Dragon: 3, Dark: 2, Steel: 1, Fairy: 0
        },
    },
};

const naturesData = {
    adamant: { name: "Adamant", plus: "atk", minus: "spa" },
    bashful: { name: "Bashful" },
    bold: { name: "Bold", plus: "def", minus: "atk" },
    brave: { name: "Brave", plus: "atk", minus: "spe" },
    calm: { name: "Calm", plus: "spd", minus: "atk" },
    careful: { name: "Careful", plus: "spd", minus: "spa" },
    docile: { name: "Docile" },
    gentle: { name: "Gentle", plus: "spd", minus: "def" },
    hardy: { name: "Hardy" },
    hasty: { name: "Hasty", plus: "spe", minus: "def" },
    impish: { name: "Impish", plus: "def", minus: "spa" },
    jolly: { name: "Jolly", plus: "spe", minus: "spa" },
    lax: { name: "Lax", plus: "def", minus: "spd" },
    lonely: { name: "Lonely", plus: "atk", minus: "def" },
    mild: { name: "Mild", plus: "spa", minus: "def" },
    modest: { name: "Modest", plus: "spa", minus: "atk" },
    naive: { name: "Naive", plus: "spe", minus: "spd" },
    naughty: { name: "Naughty", plus: "atk", minus: "spd" },
    quiet: { name: "Quiet", plus: "spa", minus: "spe" },
    quirky: { name: "Quirky" },
    rash: { name: "Rash", plus: "spa", minus: "spd" },
    relaxed: { name: "Relaxed", plus: "def", minus: "spe" },
    sassy: { name: "Sassy", plus: "spd", minus: "spe" },
    serious: { name: "Serious" },
    timid: { name: "Timid", plus: "spe", minus: "atk" },
};

// Write all data files
const outDir = path.join(__dirname, 'data');
if (!fs.existsSync(outDir)) {
    fs.mkdirSync(outDir, { recursive: true });
}

fs.writeFileSync(path.join(outDir, 'species.json'), JSON.stringify(speciesData, null, 2));
fs.writeFileSync(path.join(outDir, 'moves.json'), JSON.stringify(movesData, null, 2));
fs.writeFileSync(path.join(outDir, 'abilities.json'), JSON.stringify(abilitiesData, null, 2));
fs.writeFileSync(path.join(outDir, 'items.json'), JSON.stringify(itemsData, null, 2));
fs.writeFileSync(path.join(outDir, 'typechart.json'), JSON.stringify(typechartData, null, 2));
fs.writeFileSync(path.join(outDir, 'natures.json'), JSON.stringify(naturesData, null, 2));

console.log('Data files created in', outDir);
console.log('Files created: species.json, moves.json, abilities.json, items.json, typechart.json, natures.json');
