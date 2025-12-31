#!/usr/bin/env node

const fs = require('fs');

const rustTeams = JSON.parse(fs.readFileSync('teams-rust.json', 'utf8'));
const jsTeams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

console.log('========================================');
console.log('  PRNG SYNCHRONIZATION VERIFICATION');
console.log('  Seed: [0, 0, 0, 1]');
console.log('========================================\n');

function verifyPRNGFields(teams1, teams2, teamName) {
    let allMatch = true;

    for (let i = 0; i < 6; i++) {
        const p1 = teams1[i];
        const p2 = teams2[i];

        const prngFields = {
            'species': p1.species === p2.species,
            'level': p1.level === p2.level,
            'item': p1.item === p2.item,
            'nature': p1.nature === p2.nature,
            'gender': p1.gender === p2.gender,
            'moves': JSON.stringify(p1.moves) === JSON.stringify(p2.moves),
            'evs': JSON.stringify(p1.evs) === JSON.stringify(p2.evs),
            'ivs': JSON.stringify(p1.ivs) === JSON.stringify(p2.ivs)
        };

        const mismatch = Object.entries(prngFields).filter(([k, v]) => !v);

        if (mismatch.length > 0) {
            console.log(`❌ ${teamName} Pokemon #${i+1} (${p1.name}): ${mismatch.map(([k]) => k).join(', ')} differ`);
            allMatch = false;
        } else {
            console.log(`✅ ${teamName} Pokemon #${i+1} (${p1.name}): ALL PRNG fields match`);
        }
    }

    return allMatch;
}

const p1Match = verifyPRNGFields(rustTeams.p1, jsTeams.p1, 'P1');
console.log();
const p2Match = verifyPRNGFields(rustTeams.p2, jsTeams.p2, 'P2');

console.log('\n========================================');
if (p1Match && p2Match) {
    console.log('✅ PERFECT PRNG SYNCHRONIZATION!');
    console.log('All species, levels, items, natures, genders,');
    console.log('moves, EVs, and IVs match across both implementations.');
} else {
    console.log('❌ PRNG synchronization issues detected');
}
console.log('========================================\n');
