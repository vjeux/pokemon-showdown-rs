#!/usr/bin/env node

/**
 * Batch Battle Test Runner for Minimized Seeds (JavaScript)
 *
 * Runs all minimized seeds in a single process for efficiency.
 *
 * Usage: node tests/test-minimized-batch-js.js
 * Output: Writes /tmp/js-battle-seed{N}.txt for each seed
 */

const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');
const path = require('path');

const MINIMIZED_DIR = path.join(__dirname, 'minimized');

// Get all seed files
const seedFiles = fs.readdirSync(MINIMIZED_DIR)
    .filter(f => f.endsWith('.json') && f.startsWith('seed'))
    .sort((a, b) => {
        const numA = parseInt(a.replace('seed', '').replace('.json', ''));
        const numB = parseInt(b.replace('seed', '').replace('.json', ''));
        return numA - numB;
    });

console.log(`Running ${seedFiles.length} minimized seeds...`);

let successCount = 0;
let errorCount = 0;

for (const seedFile of seedFiles) {
    const seedNum = parseInt(seedFile.replace('seed', '').replace('.json', ''));
    const teamsFile = path.join(MINIMIZED_DIR, seedFile);

    try {
        const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

        const battle = new Battle({formatid: 'gen9randombattle'});
        battle.prng = new PRNG([0, 0, 0, seedNum]);

        // Track PRNG calls
        let totalPrngCalls = 0;
        const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
        battle.prng.rng.next = function() {
            totalPrngCalls++;
            return originalNext();
        };

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

        // Collect turn results
        const results = [];

        for (let i = 1; i <= 100; i++) {
            const prngBefore = totalPrngCalls;
            battle.makeChoices('default', 'default');
            const prngAfter = totalPrngCalls;

            const p1Active = battle.sides[0].active
                .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
                .join(', ');
            const p2Active = battle.sides[1].active
                .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
                .join(', ');

            results.push(`#${i}: turn=${battle.turn}, prng=${prngBefore}->${prngAfter}, P1=[${p1Active}], P2=[${p2Active}]`);

            if (battle.ended || i >= 100) break;
        }

        // Write results to file
        fs.writeFileSync(`/tmp/js-battle-seed${seedNum}.txt`, results.join('\n') + '\n');
        successCount++;

    } catch (e) {
        console.error(`Seed ${seedNum}: ERROR - ${e.message}`);
        errorCount++;
    }
}

console.log(`Done: ${successCount} successful, ${errorCount} errors`);
