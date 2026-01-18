#!/usr/bin/env node

/**
 * Battle Test Runner for Minimized Seeds (JavaScript)
 * Uses worker threads for parallel execution.
 *
 * Usage: node tests/test-minimized.js
 * Output: One line per seed in format: SEED <n>: turns=<t>, prng=<p>, winner=<w>
 */

const {Worker, isMainThread, parentPort, workerData} = require('worker_threads');
const os = require('os');
const fs = require('fs');
const path = require('path');

if (isMainThread) {
    const MINIMIZED_DIR = path.join(__dirname, 'minimized');

    // Get all seed files
    const seedFiles = fs.readdirSync(MINIMIZED_DIR)
        .filter(f => f.endsWith('.json') && f.startsWith('seed'))
        .map(f => parseInt(f.replace('seed', '').replace('.json', '')))
        .sort((a, b) => a - b);

    const numWorkers = Math.min(os.cpus().length, seedFiles.length);
    const results = new Map();
    let completed = 0;

    // Divide work among workers
    const seedsPerWorker = Math.ceil(seedFiles.length / numWorkers);
    const workers = [];

    for (let i = 0; i < numWorkers; i++) {
        const startIdx = i * seedsPerWorker;
        const endIdx = Math.min(startIdx + seedsPerWorker, seedFiles.length);

        if (startIdx >= seedFiles.length) break;

        const workerSeeds = seedFiles.slice(startIdx, endIdx);

        const worker = new Worker(__filename, {
            workerData: {seeds: workerSeeds, minimizedDir: MINIMIZED_DIR}
        });

        worker.on('message', (msg) => {
            results.set(msg.seed, msg.line);
            completed++;
        });

        worker.on('error', (err) => {
            console.error('Worker error:', err);
        });

        workers.push(worker);
    }

    // Wait for all workers to complete
    Promise.all(workers.map(w => new Promise(resolve => w.on('exit', resolve))))
        .then(() => {
            // Output results in order
            for (const seed of seedFiles) {
                console.log(results.get(seed));
            }
        });
} else {
    // Worker thread
    const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
    const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');

    const {seeds, minimizedDir} = workerData;

    for (const seedNum of seeds) {
        try {
            const teamsFile = path.join(minimizedDir, `seed${seedNum}.json`);
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

            for (let i = 1; i <= 100; i++) {
                battle.makeChoices('default', 'default');
                if (battle.ended || i >= 100) break;
            }

            let winner = 'none';
            if (battle.winner === 'Player 1') winner = 'p1';
            else if (battle.winner === 'Player 2') winner = 'p2';
            else if (battle.winner === '') winner = 'tie';

            parentPort.postMessage({
                seed: seedNum,
                line: `SEED ${seedNum}: turns=${battle.turn}, prng=${totalPrngCalls}, winner=${winner}`
            });
        } catch (e) {
            parentPort.postMessage({
                seed: seedNum,
                line: `SEED ${seedNum}: ERROR - ${e.message}`
            });
        }
    }
}
