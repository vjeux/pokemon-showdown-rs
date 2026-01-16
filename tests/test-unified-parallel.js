#!/usr/bin/env node

/**
 * Unified Battle Test Runner (JavaScript) - Parallel Version
 * Uses worker threads for parallel execution.
 *
 * Usage: node tests/test-unified-parallel.js <start_seed> <end_seed>
 */

const {Worker, isMainThread, parentPort, workerData} = require('worker_threads');
const os = require('os');

if (isMainThread) {
    const startSeed = parseInt(process.argv[2]) || 1;
    const endSeed = parseInt(process.argv[3]) || startSeed;
    const numWorkers = Math.min(os.cpus().length, endSeed - startSeed + 1);

    const results = new Map();
    let completed = 0;
    const totalSeeds = endSeed - startSeed + 1;

    // Divide work among workers
    const seedsPerWorker = Math.ceil(totalSeeds / numWorkers);
    const workers = [];

    for (let i = 0; i < numWorkers; i++) {
        const workerStart = startSeed + (i * seedsPerWorker);
        const workerEnd = Math.min(workerStart + seedsPerWorker - 1, endSeed);

        if (workerStart > endSeed) break;

        const worker = new Worker(__filename, {
            workerData: {startSeed: workerStart, endSeed: workerEnd}
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
            for (let seed = startSeed; seed <= endSeed; seed++) {
                console.log(results.get(seed));
            }
        });
} else {
    // Worker thread
    const {Dex} = require('./../../pokemon-showdown-ts/dist/sim/dex');
    const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
    const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');

    const {startSeed, endSeed} = workerData;

    // Pre-load data once
    const allSpecies = Object.values(Dex.data.Pokedex).sort((a, b) => a.name.localeCompare(b.name));
    const allMoves = Object.keys(Dex.data.Moves).sort();
    const allItems = Object.keys(Dex.data.Items).sort();
    const allNatures = Object.keys(Dex.data.Natures).sort();

    for (let seedNum = startSeed; seedNum <= endSeed; seedNum++) {
        try {
            const prng = new PRNG([0, 0, 0, seedNum]);
            const team1 = generateRandomTeam(prng, allSpecies, allMoves, allItems, allNatures, Dex);
            const team2 = generateRandomTeam(prng, allSpecies, allMoves, allItems, allNatures, Dex);

            const battle = new Battle({formatid: 'gen9randombattle'});
            battle.prng = new PRNG([0, 0, 0, seedNum]);

            let totalPrngCalls = 0;
            const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
            battle.prng.rng.next = function() {
                totalPrngCalls++;
                return originalNext();
            };

            battle.setPlayer('p1', {name: 'Player 1', team: team1});
            battle.setPlayer('p2', {name: 'Player 2', team: team2});

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

    function generateRandomTeam(prng, allSpecies, allMoves, allItems, allNatures, Dex) {
        const team = [];
        const usedSpecies = [];
        const usedItems = [];

        for (let i = 0; i < 6; i++) {
            let availableSpecies = allSpecies.filter(s => !usedSpecies.includes(s.name));
            if (availableSpecies.length === 0) availableSpecies = allSpecies;
            const species = prng.sample(availableSpecies);
            usedSpecies.push(species.name);

            const level = Math.floor(prng.random() * (101 - 50)) + 50;

            let ability = '';
            if (species.abilities && species.abilities['0']) {
                ability = species.abilities['0'];
            } else if (species.baseSpecies) {
                const baseSpecies = Dex.data.Pokedex[species.baseSpecies.toLowerCase().replace(/[^a-z0-9]/g, '')];
                if (baseSpecies && baseSpecies.abilities && baseSpecies.abilities['0']) {
                    ability = baseSpecies.abilities['0'];
                }
            }

            let item = '';
            if (allItems.length > 0) {
                let availableItems = allItems.filter(i => !usedItems.includes(i) || i === '');
                if (availableItems.length === 0) availableItems = [''];
                item = prng.sample(availableItems) || '';
                if (item !== '') usedItems.push(item);
            }

            const nature = allNatures.length > 0 ? prng.sample(allNatures) : 'hardy';

            let gender = '';
            if (species.genderRatio) {
                if (species.genderRatio.M > 0 && species.genderRatio.F > 0) {
                    gender = prng.randomChance(1, 2) ? 'M' : 'F';
                } else if (species.genderRatio.M > 0) {
                    gender = 'M';
                } else if (species.genderRatio.F > 0) {
                    gender = 'F';
                }
            } else if (species.gender) {
                if (species.gender === 'M') gender = 'M';
                else if (species.gender === 'F') gender = 'F';
                else if (species.gender === 'N') gender = '';
                else gender = prng.randomChance(1, 2) ? 'M' : 'F';
            } else {
                gender = prng.randomChance(1, 2) ? 'M' : 'F';
            }

            const moves = [];
            for (let j = 0; j < 4; j++) {
                if (allMoves.length > 0) {
                    const move = prng.sample(allMoves);
                    if (!moves.includes(move)) moves.push(move);
                }
            }
            while (moves.length < 4) moves.push('tackle');

            const evs = generateRandomEVs(prng);
            const ivs = {
                hp: Math.floor(prng.random() * 32),
                atk: Math.floor(prng.random() * 32),
                def: Math.floor(prng.random() * 32),
                spa: Math.floor(prng.random() * 32),
                spd: Math.floor(prng.random() * 32),
                spe: Math.floor(prng.random() * 32),
            };

            team.push({
                name: species.name,
                species: species.name,
                level,
                ability,
                item,
                nature,
                gender,
                moves,
                evs,
                ivs,
            });
        }

        return team;
    }

    function generateRandomEVs(prng) {
        const evs = [0, 0, 0, 0, 0, 0];
        let totalEvs = 0;

        while (totalEvs < 510) {
            const availableStats = [];
            for (let i = 0; i < 6; i++) {
                if (evs[i] < 252) availableStats.push(i);
            }
            if (availableStats.length === 0) break;

            const statIdx = prng.sample(availableStats);
            const amount = Math.min(
                Math.floor(prng.random() * 4) + 1,
                252 - evs[statIdx],
                510 - totalEvs
            );
            evs[statIdx] += amount;
            totalEvs += amount;
        }

        return {
            hp: evs[0], atk: evs[1], def: evs[2],
            spa: evs[3], spd: evs[4], spe: evs[5],
        };
    }
}
