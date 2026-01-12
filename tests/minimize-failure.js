#!/usr/bin/env node

/**
 * Minimize a failing seed test case - Improved version
 *
 * Strategy:
 * 1. Run battle and find exact divergence point
 * 2. Identify which Pokemon are active at divergence
 * 3. Create minimal test with those Pokemon as leads
 * 4. Simplify stats (EVs to 85, IVs to 31)
 * 5. Try simplifying abilities and moves
 *
 * Usage: node tests/minimize-failure.js <seed>
 */

const fs = require('fs');
const { execSync } = require('child_process');
const path = require('path');

const seed = parseInt(process.argv[2]);
if (!seed) {
  console.error('Usage: node tests/minimize-failure.js <seed>');
  process.exit(1);
}

// Output directory for minimized cases
const MINIMIZED_DIR = path.join(__dirname, 'minimized');
if (!fs.existsSync(MINIMIZED_DIR)) {
  fs.mkdirSync(MINIMIZED_DIR, { recursive: true });
}

// Build Rust binary once upfront for faster iteration
console.log('Building Rust binary (one-time)...');
try {
  execSync(`docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_battle_rust 2>&1"`, { stdio: 'pipe' });
  console.log('Rust binary built successfully.\n');
} catch (e) {
  console.error('Failed to build Rust binary:', e.message);
  process.exit(1);
}

// Path to pre-built binary
const RUST_BINARY = '/home/builder/workspace/target/release/examples/test_battle_rust';

// Simple abilities that have minimal effects
const SIMPLE_ABILITIES = ['Run Away', 'Honey Gather', 'Illuminate', 'Ball Fetch'];

// Standard stats for simplification
const STANDARD_EVS = { hp: 85, atk: 85, def: 85, spa: 85, spd: 85, spe: 85 };
const STANDARD_IVS = { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 };

// Run a battle and return the output lines
function runBattle(teams) {
  const tempFile = `/tmp/teams-seed${seed}-test.json`;
  fs.writeFileSync(tempFile, JSON.stringify(teams, null, 2));
  fs.writeFileSync(`/tmp/teams-seed${seed}-js.json`, JSON.stringify(teams, null, 2));
  fs.writeFileSync(`/tmp/teams-seed${seed}-rust.json`, JSON.stringify(teams, null, 2));

  try {
    execSync(`docker cp /tmp/teams-seed${seed}-rust.json pokemon-rust-dev:/tmp/teams-seed${seed}-rust.json`, { stdio: 'pipe' });
  } catch (e) {}

  try {
    execSync(`node tests/test-battle-js.js ${seed} > /tmp/js-battle-seed${seed}.txt 2>&1`, { stdio: 'pipe' });
    execSync(`docker exec pokemon-rust-dev bash -c "${RUST_BINARY} ${seed} 2>/dev/null" > /tmp/rust-battle-seed${seed}.txt`, { stdio: 'pipe' });

    const jsLines = fs.readFileSync(`/tmp/js-battle-seed${seed}.txt`, 'utf8')
      .split('\n').filter(l => /^#\d+:/.test(l));
    const rustLines = fs.readFileSync(`/tmp/rust-battle-seed${seed}.txt`, 'utf8')
      .split('\n').filter(l => /^#\d+:/.test(l));

    return { jsLines, rustLines };
  } catch (e) {
    return { jsLines: [], rustLines: [], error: e.message };
  }
}

// Find the divergence point and extract Pokemon names
function findDivergence(jsLines, rustLines) {
  for (let i = 0; i < Math.max(jsLines.length, rustLines.length); i++) {
    if (jsLines[i] !== rustLines[i]) {
      // Extract Pokemon names from the line
      const jsLine = jsLines[i] || '';
      const rustLine = rustLines[i] || '';

      // Parse Pokemon names from format: P1=[Name(hp/max)], P2=[Name(hp/max)]
      const p1Match = jsLine.match(/P1=\[([^\(]+)\(/);
      const p2Match = jsLine.match(/P2=\[([^\(]+)\(/);

      return {
        index: i,
        jsLine,
        rustLine,
        p1Pokemon: p1Match ? p1Match[1] : null,
        p2Pokemon: p2Match ? p2Match[1] : null
      };
    }
  }
  return null;
}

// Find a Pokemon in the team by name
function findPokemonByName(team, name) {
  return team.find(p => p.name === name || p.species === name);
}

// Deep clone
function clone(obj) {
  return JSON.parse(JSON.stringify(obj));
}

// Simplify a Pokemon's stats
function simplifyStats(pokemon) {
  pokemon.evs = { ...STANDARD_EVS };
  pokemon.ivs = { ...STANDARD_IVS };
  pokemon.nature = 'docile';
  return pokemon;
}

// Test if teams still fail
function testFails(teams) {
  const { jsLines, rustLines } = runBattle(teams);
  if (jsLines.length === 0) return false;

  for (let i = 0; i < Math.max(jsLines.length, rustLines.length); i++) {
    if (jsLines[i] !== rustLines[i]) {
      return true;
    }
  }
  return false;
}

console.log(`\n=== Minimizing seed ${seed} ===\n`);

// Load the original team
const teamFile = `/tmp/teams-seed${seed}-js.json`;
if (!fs.existsSync(teamFile)) {
  console.log(`Generating teams for seed ${seed}...`);
  execSync(`node tests/generate-test-teams.js ${seed}`, { stdio: 'inherit' });
}

let teams = JSON.parse(fs.readFileSync(teamFile, 'utf8'));

// Step 1: Run initial battle and find divergence
console.log('Step 1: Finding divergence point...');
let { jsLines, rustLines } = runBattle(teams);

if (jsLines.length === 0) {
  console.log('Error: No JS output');
  process.exit(1);
}

let divergence = findDivergence(jsLines, rustLines);
if (!divergence) {
  console.log('Seed already passes! Nothing to minimize.');
  process.exit(0);
}

console.log(`  Divergence at line ${divergence.index + 1}`);
console.log(`  JS:   ${divergence.jsLine}`);
console.log(`  Rust: ${divergence.rustLine}`);
console.log(`  P1 active: ${divergence.p1Pokemon}`);
console.log(`  P2 active: ${divergence.p2Pokemon}`);

// Step 2: Try creating minimal team with just the active Pokemon
console.log('\nStep 2: Creating minimal team with active Pokemon...');

const p1Active = findPokemonByName(teams.p1, divergence.p1Pokemon);
const p2Active = findPokemonByName(teams.p2, divergence.p2Pokemon);

if (p1Active && p2Active) {
  const minimalTeams = {
    p1: [clone(p1Active)],
    p2: [clone(p2Active)]
  };

  if (testFails(minimalTeams)) {
    console.log('  Success! Minimal 1v1 team still fails.');
    teams = minimalTeams;
  } else {
    console.log('  Minimal team passes. Need to find required setup...');

    // Try to find what's needed - maybe the lead Pokemon that sets up the state
    // Add back Pokemon one by one from the start
    for (let i = 0; i < teams.p1.length; i++) {
      const testTeams = {
        p1: teams.p1.slice(0, i + 1).map(clone),
        p2: [clone(p2Active)]
      };
      // Make sure active Pokemon is included
      if (!testTeams.p1.find(p => p.name === p1Active.name)) {
        testTeams.p1.push(clone(p1Active));
      }

      if (testFails(testTeams)) {
        console.log(`  Found: Need P1[0..${i}] + active`);
        teams = testTeams;
        break;
      }
    }

    // Similarly for P2
    if (!testFails(teams)) {
      for (let i = 0; i < teams.p2.length; i++) {
        const testTeams = {
          p1: teams.p1,
          p2: teams.p2.slice(0, i + 1).map(clone)
        };
        if (!testTeams.p2.find(p => p.name === p2Active.name)) {
          testTeams.p2.push(clone(p2Active));
        }

        if (testFails(testTeams)) {
          console.log(`  Found: Need P2[0..${i}] + active`);
          teams = testTeams;
          break;
        }
      }
    }
  }
}

// Step 3: Simplify stats
console.log('\nStep 3: Simplifying stats...');
for (let i = 0; i < teams.p1.length; i++) {
  const testTeams = clone(teams);
  simplifyStats(testTeams.p1[i]);
  if (testFails(testTeams)) {
    console.log(`  P1[${i}] stats simplified`);
    teams = testTeams;
  }
}
for (let i = 0; i < teams.p2.length; i++) {
  const testTeams = clone(teams);
  simplifyStats(testTeams.p2[i]);
  if (testFails(testTeams)) {
    console.log(`  P2[${i}] stats simplified`);
    teams = testTeams;
  }
}

// Step 4: Remove items
console.log('\nStep 4: Removing items...');
for (let i = 0; i < teams.p1.length; i++) {
  if (teams.p1[i].item) {
    const testTeams = clone(teams);
    testTeams.p1[i].item = '';
    if (testFails(testTeams)) {
      console.log(`  P1[${i}] item removed`);
      teams = testTeams;
    }
  }
}
for (let i = 0; i < teams.p2.length; i++) {
  if (teams.p2[i].item) {
    const testTeams = clone(teams);
    testTeams.p2[i].item = '';
    if (testFails(testTeams)) {
      console.log(`  P2[${i}] item removed`);
      teams = testTeams;
    }
  }
}

// Step 5: Simplify abilities
console.log('\nStep 5: Simplifying abilities...');
for (let i = 0; i < teams.p1.length; i++) {
  if (!SIMPLE_ABILITIES.includes(teams.p1[i].ability)) {
    for (const ability of SIMPLE_ABILITIES) {
      const testTeams = clone(teams);
      testTeams.p1[i].ability = ability;
      if (testFails(testTeams)) {
        console.log(`  P1[${i}] ability -> ${ability}`);
        teams = testTeams;
        break;
      }
    }
  }
}
for (let i = 0; i < teams.p2.length; i++) {
  if (!SIMPLE_ABILITIES.includes(teams.p2[i].ability)) {
    for (const ability of SIMPLE_ABILITIES) {
      const testTeams = clone(teams);
      testTeams.p2[i].ability = ability;
      if (testFails(testTeams)) {
        console.log(`  P2[${i}] ability -> ${ability}`);
        teams = testTeams;
        break;
      }
    }
  }
}

// Step 6: Reduce moves
console.log('\nStep 6: Reducing moves...');
for (let i = 0; i < teams.p1.length; i++) {
  while (teams.p1[i].moves.length > 1) {
    const testTeams = clone(teams);
    testTeams.p1[i].moves.pop();
    if (testFails(testTeams)) {
      teams = testTeams;
    } else {
      break;
    }
  }
  console.log(`  P1[${i}] moves: ${teams.p1[i].moves.join(', ')}`);
}
for (let i = 0; i < teams.p2.length; i++) {
  while (teams.p2[i].moves.length > 1) {
    const testTeams = clone(teams);
    testTeams.p2[i].moves.pop();
    if (testFails(testTeams)) {
      teams = testTeams;
    } else {
      break;
    }
  }
  console.log(`  P2[${i}] moves: ${teams.p2[i].moves.join(', ')}`);
}

// Step 7: Try replacing moves with tackle
console.log('\nStep 7: Simplifying moves to tackle...');
for (let i = 0; i < teams.p1.length; i++) {
  for (let m = 0; m < teams.p1[i].moves.length; m++) {
    if (teams.p1[i].moves[m] !== 'tackle') {
      const testTeams = clone(teams);
      testTeams.p1[i].moves[m] = 'tackle';
      if (testFails(testTeams)) {
        console.log(`  P1[${i}] move ${m} -> tackle`);
        teams = testTeams;
      }
    }
  }
}
for (let i = 0; i < teams.p2.length; i++) {
  for (let m = 0; m < teams.p2[i].moves.length; m++) {
    if (teams.p2[i].moves[m] !== 'tackle') {
      const testTeams = clone(teams);
      testTeams.p2[i].moves[m] = 'tackle';
      if (testFails(testTeams)) {
        console.log(`  P2[${i}] move ${m} -> tackle`);
        teams = testTeams;
      }
    }
  }
}

// Final verification and get divergence info
console.log('\n=== Final verification ===');
const { jsLines: finalJs, rustLines: finalRust } = runBattle(teams);
const finalDiv = findDivergence(finalJs, finalRust);

if (finalDiv) {
  // Add bug description
  teams._bug = `Diverges at line ${finalDiv.index + 1}: JS="${finalDiv.jsLine}" vs Rust="${finalDiv.rustLine}"`;
}

// Save minimized team
const minFile = path.join(MINIMIZED_DIR, `seed${seed}.json`);
fs.writeFileSync(minFile, JSON.stringify(teams, null, 2));
console.log(`\nMinimized team saved to: ${minFile}`);

// Print summary
console.log('\n--- Minimized Team ---');
console.log(`P1: ${teams.p1.length} Pokemon`);
for (const p of teams.p1) {
  console.log(`  - ${p.name} @ ${p.item || '(no item)'}`);
  console.log(`    Ability: ${p.ability}`);
  console.log(`    Moves: ${p.moves.join(', ')}`);
}
console.log(`P2: ${teams.p2.length} Pokemon`);
for (const p of teams.p2) {
  console.log(`  - ${p.name} @ ${p.item || '(no item)'}`);
  console.log(`    Ability: ${p.ability}`);
  console.log(`    Moves: ${p.moves.join(', ')}`);
}

if (finalDiv) {
  console.log('\n--- Divergence ---');
  console.log(`Line ${finalDiv.index + 1}:`);
  console.log(`  JS:   ${finalDiv.jsLine}`);
  console.log(`  Rust: ${finalDiv.rustLine}`);
}
