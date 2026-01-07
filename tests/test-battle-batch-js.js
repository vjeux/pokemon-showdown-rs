#!/usr/bin/env node

/**
 * Batch JavaScript Battle Test Runner
 *
 * Runs multiple battles in sequence without rebuilding
 *
 * Usage: node tests/test-battle-batch-js.js [start_seed] [end_seed]
 * Example: node tests/test-battle-batch-js.js 1 100
 */

const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const startSeed = parseInt(process.argv[2]) || 1;
const endSeed = parseInt(process.argv[3]) || 100;

console.error(`Running JS battles for seeds ${startSeed}-${endSeed}`);

for (let seedNum = startSeed; seedNum <= endSeed; seedNum++) {
    const teamsFile = `/tmp/teams-seed${seedNum}-js.json`;

    if (!fs.existsSync(teamsFile)) {
        console.error(`Seed ${seedNum}: SKIP - no team file`);
        continue;
    }

    try {
        // Load teams
        const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

        // Create battle
        const battle = new Battle({formatid: 'gen9randombattle'});
        battle.prng = new PRNG([0, 0, 0, seedNum]);

        // Track PRNG calls
        let totalPrngCalls = 0;
        const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
        battle.prng.rng.next = function() {
            totalPrngCalls++;
            return originalNext();
        };

        // Set up teams
        battle.setPlayer('p1', {team: teams.p1});
        battle.setPlayer('p2', {team: teams.p2});

        // Start battle
        battle.start();

        let iteration = 0;
        const MAX_TURNS = 100;

        // Open output file
        const outputFile = `/tmp/js-battle-seed${seedNum}.txt`;
        const output = [];

        while (!battle.ended && battle.turn < MAX_TURNS) {
            iteration++;
            const prngBefore = totalPrngCalls;

            // Make choices for both sides
            for (const side of battle.sides) {
                const request = side.getChoice();
                if (request && request.active) {
                    const pokemon = side.active[0];
                    if (pokemon && !pokemon.fainted) {
                        const moveIndex = (iteration % pokemon.moveSlots.length);
                        const choice = `move ${moveIndex + 1}`;
                        side.choose(choice);
                    }
                }
            }

            // Run the turn
            battle.makeChoices();

            const prngAfter = totalPrngCalls;

            // Format Pokemon state
            const formatTeam = (side) => {
                return side.pokemon
                    .filter(p => p.isActive)
                    .map(p => `${p.name}(${p.hp}/${p.maxhp})`)
                    .join(', ');
            };

            const p1State = formatTeam(battle.sides[0]);
            const p2State = formatTeam(battle.sides[1]);

            // Output line
            const line = `#${iteration}: turn=${battle.turn}, prng=${prngBefore}->${prngAfter}, P1=[${p1State}], P2=[${p2State}]`;
            output.push(line);
        }

        // Write output file
        fs.writeFileSync(outputFile, output.join('\n') + '\n');
        console.error(`Seed ${seedNum}: OK - ${iteration} iterations`);

    } catch (error) {
        console.error(`Seed ${seedNum}: ERROR - ${error.message}`);
    }
}

console.error('JS batch complete');
