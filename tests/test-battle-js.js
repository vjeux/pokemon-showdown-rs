#!/usr/bin/env node

/**
 * JavaScript Battle Test Runner
 *
 * Runs a random battle with a specific seed and outputs:
 * - Turn number
 * - PRNG call count before/after each turn
 * - HP of all active Pokemon
 *
 * Output format: #<iteration>: turn=<turn>, prng=<before>-><after>, P1=[...], P2=[...]
 *
 * Usage: node tests/test-battle-js.js [seed_number]
 */

const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const seedNum = parseInt(process.argv[2]) || 1;
const teamsFile = `/tmp/teams-seed${seedNum}-js.json`;

if (!fs.existsSync(teamsFile)) {
    console.error(`ERROR: Team file not found: ${teamsFile}`);
    console.error('Run: node tests/generate-test-teams.js [seed_number] first');
    process.exit(1);
}

// Load teams from JSON file
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

// Create battle with specific seed
const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, seedNum]);

battle.setPlayer('p1', {
    name: 'Player 1',
    team: teams.p1.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: teams.p2.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

// Wrap PRNG to count calls
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    const result = originalNext();
    // Log PRNG calls on turn 17 to debug the missing call
    if (battle.turn === 17 && totalPrngCalls >= 86 && totalPrngCalls <= 91) {
        const stack = new Error().stack;
        const lines = stack.split('\n').slice(1, 10); // Get first 10 frames
        console.error(`[PRNG_JS] call #${totalPrngCalls}, result=${result}`);
        lines.forEach((line, i) => console.error(`  Frame ${i}: ${line.trim()}`));
    }
    return result;
};

console.log(`# JavaScript Battle Test - Seed ${seedNum}`);
console.log(`# P1: ${teams.p1[0].name} vs P2: ${teams.p2[0].name}`);

// Helper to format Pokemon detail
function formatPokemonDetail(pokemon, side) {
    if (!pokemon) return null;

    const boosts = [];
    for (const [stat, boost] of Object.entries(pokemon.boosts || {})) {
        if (boost !== 0) {
            boosts.push(`${stat}:${boost > 0 ? '+' : ''}${boost}`);
        }
    }

    const volatiles = Object.keys(pokemon.volatiles || {}).filter(v => v !== 'lockedmove');

    const moves = (pokemon.moveSlots || []).map(m => `${m.id}(${m.pp}/${m.maxpp})`).join(', ');

    const statsStr = pokemon.stats
        ? `Atk:${pokemon.stats.atk} Def:${pokemon.stats.def} SpA:${pokemon.stats.spa} SpD:${pokemon.stats.spd} Spe:${pokemon.stats.spe}`
        : 'not initialized';

    return {
        name: pokemon.name,
        species: pokemon.species?.name || 'unknown',
        hp: `${pokemon.hp}/${pokemon.maxhp}`,
        hpPercent: Math.floor((pokemon.hp / pokemon.maxhp) * 100),
        status: pokemon.status || 'none',
        item: pokemon.item || 'none',
        ability: pokemon.ability || 'none',
        stats: statsStr,
        boosts: boosts.length > 0 ? boosts.join(', ') : 'none',
        volatiles: volatiles.length > 0 ? volatiles.join(', ') : 'none',
        moves: moves
    };
}

function printBattleState(battle, iteration) {
    console.error('');
    console.error(`========== Turn ${battle.turn} (Iteration ${iteration}) ==========`);

    // Field conditions
    const weather = battle.field.weather || 'none';
    const terrain = battle.field.terrain || 'none';
    console.error(`Field: Weather=${weather}, Terrain=${terrain}, PRNG calls=${totalPrngCalls}`);

    // Player 1 state
    console.error('');
    console.error('--- Player 1 ---');
    battle.sides[0].active.forEach((pokemon, i) => {
        if (pokemon) {
            const detail = formatPokemonDetail(pokemon, battle.sides[0]);
            console.error(`  Active[${i}]: ${detail.name} (${detail.species})`);
            console.error(`    HP: ${detail.hp} (${detail.hpPercent}%) | Status: ${detail.status}`);
            console.error(`    Item: ${detail.item} | Ability: ${detail.ability}`);
            console.error(`    Stats: ${detail.stats}`);
            console.error(`    Boosts: ${detail.boosts}`);
            console.error(`    Volatiles: ${detail.volatiles}`);
            console.error(`    Moves: ${detail.moves}`);
        }
    });

    // Show side conditions for P1
    const p1SideConditions = Object.keys(battle.sides[0].sideConditions || {});
    if (p1SideConditions.length > 0) {
        console.error(`  Side Conditions: ${p1SideConditions.join(', ')}`);
    }

    // Player 2 state
    console.error('');
    console.error('--- Player 2 ---');
    battle.sides[1].active.forEach((pokemon, i) => {
        if (pokemon) {
            const detail = formatPokemonDetail(pokemon, battle.sides[1]);
            console.error(`  Active[${i}]: ${detail.name} (${detail.species})`);
            console.error(`    HP: ${detail.hp} (${detail.hpPercent}%) | Status: ${detail.status}`);
            console.error(`    Item: ${detail.item} | Ability: ${detail.ability}`);
            console.error(`    Stats: ${detail.stats}`);
            console.error(`    Boosts: ${detail.boosts}`);
            console.error(`    Volatiles: ${detail.volatiles}`);
            console.error(`    Moves: ${detail.moves}`);
        }
    });

    // Show side conditions for P2
    const p2SideConditions = Object.keys(battle.sides[1].sideConditions || {});
    if (p2SideConditions.length > 0) {
        console.error(`  Side Conditions: ${p2SideConditions.join(', ')}`);
    }

    console.error('');
}

// Run battle for up to 100 turns
for (let i = 1; i <= 100; i++) {
    const prngBefore = totalPrngCalls;

    // Print detailed state before turn
    printBattleState(battle, i);

    console.error(`>>> Making choices for turn ${battle.turn}...`);
    battle.makeChoices('default', 'default');

    const prngAfter = totalPrngCalls;

    // Get active Pokemon HP
    const p1Active = battle.sides[0].active
        .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
        .join(', ');
    const p2Active = battle.sides[1].active
        .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
        .join(', ');

    console.log(`#${i}: turn=${battle.turn}, prng=${prngBefore}->${prngAfter}, P1=[${p1Active}], P2=[${p2Active}]`);
    console.error(`>>> Turn ${battle.turn} completed. PRNG: ${prngBefore}->${prngAfter} (+${prngAfter - prngBefore} calls)`);

    if (battle.ended || i >= 100) {
        console.error('');
        console.error('========================================');
        console.error(`Battle ended: ${battle.ended}`);
        console.error(`Final turn: ${battle.turn}`);
        console.error(`Total PRNG calls: ${totalPrngCalls}`);
        console.error('========================================');
        console.log(`# Battle ended: ${battle.ended}, Turn: ${battle.turn}, Total PRNG: ${totalPrngCalls}`);
        break;
    }
}
