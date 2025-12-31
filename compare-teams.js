#!/usr/bin/env node

const fs = require('fs');

const rustTeams = JSON.parse(fs.readFileSync('teams-rust.json', 'utf8'));
const jsTeams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

function comparePokemon(p1, p2, index, team) {
    console.log(`\n=== ${team} Pokemon #${index + 1}: ${p1.name} ===`);

    const fields = ['species', 'level', 'ability', 'item', 'nature', 'gender'];
    let allMatch = true;

    for (const field of fields) {
        const match = p1[field] === p2[field];
        const symbol = match ? '✅' : '❌';
        console.log(`${symbol} ${field}: Rust="${p1[field]}" JS="${p2[field]}"`);
        if (!match) allMatch = false;
    }

    // Compare moves
    const movesMatch = JSON.stringify(p1.moves) === JSON.stringify(p2.moves);
    const movesSymbol = movesMatch ? '✅' : '❌';
    console.log(`${movesSymbol} moves: ${movesMatch ? 'MATCH' : 'DIFFER'}`);
    if (!movesMatch) {
        console.log(`  Rust: ${p1.moves.join(', ')}`);
        console.log(`  JS:   ${p2.moves.join(', ')}`);
        allMatch = false;
    }

    // Compare EVs
    const evsMatch = JSON.stringify(p1.evs) === JSON.stringify(p2.evs);
    const evsSymbol = evsMatch ? '✅' : '❌';
    console.log(`${evsSymbol} evs: ${evsMatch ? 'MATCH' : 'DIFFER'}`);
    if (!evsMatch) {
        console.log(`  Rust: ${JSON.stringify(p1.evs)}`);
        console.log(`  JS:   ${JSON.stringify(p2.evs)}`);
        allMatch = false;
    }

    // Compare IVs
    const ivsMatch = JSON.stringify(p1.ivs) === JSON.stringify(p2.ivs);
    const ivsSymbol = ivsMatch ? '✅' : '❌';
    console.log(`${ivsSymbol} ivs: ${ivsMatch ? 'MATCH' : 'DIFFER'}`);
    if (!ivsMatch) {
        console.log(`  Rust: ${JSON.stringify(p1.ivs)}`);
        console.log(`  JS:   ${JSON.stringify(p2.ivs)}`);
        allMatch = false;
    }

    return allMatch;
}

console.log('========================================');
console.log('  RUST vs JAVASCRIPT TEAM COMPARISON');
console.log('  Seed: [0, 0, 0, 1]');
console.log('========================================');

let totalMatch = true;

// Compare P1 team
for (let i = 0; i < 6; i++) {
    const match = comparePokemon(rustTeams.p1[i], jsTeams.p1[i], i, 'P1');
    if (!match) totalMatch = false;
}

console.log('\n========================================');

// Compare P2 team
for (let i = 0; i < 6; i++) {
    const match = comparePokemon(rustTeams.p2[i], jsTeams.p2[i], i, 'P2');
    if (!match) totalMatch = false;
}

console.log('\n========================================');
console.log(totalMatch ? '✅ ALL TEAMS MATCH PERFECTLY!' : '❌ TEAMS HAVE DIFFERENCES');
console.log('========================================\n');
