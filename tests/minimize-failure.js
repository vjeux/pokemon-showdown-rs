#!/usr/bin/env node

/**
 * Minimize a failing seed test case
 *
 * Takes a seed that fails and progressively removes:
 * - Pokemon from teams (down to 1 each)
 * - Items
 * - Abilities (replaces with simple ones)
 * - Moves (down to 1 each)
 *
 * Keeps changes only if the test still fails.
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

// Simple abilities that have minimal effects
const SIMPLE_ABILITIES = ['Run Away', 'Honey Gather', 'Illuminate', 'Ball Fetch'];

// Load the original team
const teamFile = `/tmp/teams-seed${seed}-js.json`;
if (!fs.existsSync(teamFile)) {
  console.log(`Generating teams for seed ${seed}...`);
  execSync(`node tests/generate-test-teams.js ${seed}`, { stdio: 'inherit' });
}

let teams = JSON.parse(fs.readFileSync(teamFile, 'utf8'));

// Save working team to a temp file and run comparison
function testTeam(testTeams) {
  const tempFile = `/tmp/teams-seed${seed}-test.json`;
  fs.writeFileSync(tempFile, JSON.stringify(testTeams, null, 2));

  // Copy to both JS and Rust expected locations
  fs.writeFileSync(`/tmp/teams-seed${seed}-js.json`, JSON.stringify(testTeams, null, 2));
  fs.writeFileSync(`/tmp/teams-seed${seed}-rust.json`, JSON.stringify(testTeams, null, 2));

  // Also copy to docker container
  try {
    execSync(`docker cp /tmp/teams-seed${seed}-rust.json pokemon-rust-dev:/tmp/teams-seed${seed}-rust.json`, { stdio: 'pipe' });
  } catch (e) {
    // Ignore docker copy errors
  }

  try {
    // Run JS battle
    execSync(`node tests/test-battle-js.js ${seed} > /tmp/js-battle-seed${seed}.txt 2>&1`, { stdio: 'pipe' });

    // Run Rust battle
    execSync(`docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --release --example test_battle_rust ${seed} 2>/dev/null" > /tmp/rust-battle-seed${seed}.txt`, { stdio: 'pipe' });

    // Compare outputs - filter to only #N: lines (not # comments)
    const jsOutput = fs.readFileSync(`/tmp/js-battle-seed${seed}.txt`, 'utf8')
      .split('\n').filter(l => /^#\d+:/.test(l)).join('\n');
    const rustOutput = fs.readFileSync(`/tmp/rust-battle-seed${seed}.txt`, 'utf8')
      .split('\n').filter(l => /^#\d+:/.test(l)).join('\n');

    const matches = jsOutput === rustOutput;

    if (!matches) {
      // Find first divergence
      const jsLines = jsOutput.split('\n');
      const rustLines = rustOutput.split('\n');
      for (let i = 0; i < Math.max(jsLines.length, rustLines.length); i++) {
        if (jsLines[i] !== rustLines[i]) {
          return { passes: false, divergeAt: i + 1, jsLine: jsLines[i], rustLine: rustLines[i] };
        }
      }
    }

    return { passes: matches };
  } catch (e) {
    return { passes: false, error: e.message };
  }
}

// Deep clone teams
function clone(obj) {
  return JSON.parse(JSON.stringify(obj));
}

console.log(`\n=== Minimizing seed ${seed} ===\n`);

// First, verify the seed actually fails
let result = testTeam(teams);
if (result.passes) {
  console.log('Seed already passes! Nothing to minimize.');
  process.exit(0);
}

console.log(`Initial failure at line ${result.divergeAt || 'unknown'}`);
console.log(`  JS:   ${result.jsLine || 'N/A'}`);
console.log(`  Rust: ${result.rustLine || 'N/A'}`);
console.log('');

let changed = true;
let iteration = 0;

while (changed) {
  changed = false;
  iteration++;
  console.log(`\n--- Iteration ${iteration} ---`);

  // Try removing Pokemon from P1 (keep at least 1)
  for (let i = teams.p1.length - 1; i >= 1; i--) {
    const testTeams = clone(teams);
    testTeams.p1.splice(i, 1);
    console.log(`Trying: Remove P1 pokemon ${i} (${teams.p1[i].name})...`);
    result = testTeam(testTeams);
    if (!result.passes) {
      console.log(`  -> Still fails! Keeping change.`);
      teams = testTeams;
      changed = true;
      break;
    } else {
      console.log(`  -> Now passes. Reverting.`);
    }
  }
  if (changed) continue;

  // Try removing Pokemon from P2 (keep at least 1)
  for (let i = teams.p2.length - 1; i >= 1; i--) {
    const testTeams = clone(teams);
    testTeams.p2.splice(i, 1);
    console.log(`Trying: Remove P2 pokemon ${i} (${teams.p2[i].name})...`);
    result = testTeam(testTeams);
    if (!result.passes) {
      console.log(`  -> Still fails! Keeping change.`);
      teams = testTeams;
      changed = true;
      break;
    } else {
      console.log(`  -> Now passes. Reverting.`);
    }
  }
  if (changed) continue;

  // Try removing items from P1
  for (let i = 0; i < teams.p1.length; i++) {
    if (teams.p1[i].item && teams.p1[i].item !== '') {
      const testTeams = clone(teams);
      testTeams.p1[i].item = '';
      console.log(`Trying: Remove P1[${i}] item (${teams.p1[i].item})...`);
      result = testTeam(testTeams);
      if (!result.passes) {
        console.log(`  -> Still fails! Keeping change.`);
        teams = testTeams;
        changed = true;
        break;
      } else {
        console.log(`  -> Now passes. Reverting.`);
      }
    }
  }
  if (changed) continue;

  // Try removing items from P2
  for (let i = 0; i < teams.p2.length; i++) {
    if (teams.p2[i].item && teams.p2[i].item !== '') {
      const testTeams = clone(teams);
      testTeams.p2[i].item = '';
      console.log(`Trying: Remove P2[${i}] item (${teams.p2[i].item})...`);
      result = testTeam(testTeams);
      if (!result.passes) {
        console.log(`  -> Still fails! Keeping change.`);
        teams = testTeams;
        changed = true;
        break;
      } else {
        console.log(`  -> Now passes. Reverting.`);
      }
    }
  }
  if (changed) continue;

  // Try simplifying abilities on P1
  for (let i = 0; i < teams.p1.length; i++) {
    if (!SIMPLE_ABILITIES.includes(teams.p1[i].ability)) {
      for (const simpleAbility of SIMPLE_ABILITIES) {
        const testTeams = clone(teams);
        testTeams.p1[i].ability = simpleAbility;
        console.log(`Trying: P1[${i}] ability ${teams.p1[i].ability} -> ${simpleAbility}...`);
        result = testTeam(testTeams);
        if (!result.passes) {
          console.log(`  -> Still fails! Keeping change.`);
          teams = testTeams;
          changed = true;
          break;
        } else {
          console.log(`  -> Now passes. Reverting.`);
        }
      }
      if (changed) break;
    }
  }
  if (changed) continue;

  // Try simplifying abilities on P2
  for (let i = 0; i < teams.p2.length; i++) {
    if (!SIMPLE_ABILITIES.includes(teams.p2[i].ability)) {
      for (const simpleAbility of SIMPLE_ABILITIES) {
        const testTeams = clone(teams);
        testTeams.p2[i].ability = simpleAbility;
        console.log(`Trying: P2[${i}] ability ${teams.p2[i].ability} -> ${simpleAbility}...`);
        result = testTeam(testTeams);
        if (!result.passes) {
          console.log(`  -> Still fails! Keeping change.`);
          teams = testTeams;
          changed = true;
          break;
        } else {
          console.log(`  -> Now passes. Reverting.`);
        }
      }
      if (changed) break;
    }
  }
  if (changed) continue;

  // Try removing moves from P1 (keep at least 1)
  for (let i = 0; i < teams.p1.length; i++) {
    for (let m = teams.p1[i].moves.length - 1; m >= 1; m--) {
      const testTeams = clone(teams);
      testTeams.p1[i].moves.splice(m, 1);
      console.log(`Trying: Remove P1[${i}] move ${m} (${teams.p1[i].moves[m]})...`);
      result = testTeam(testTeams);
      if (!result.passes) {
        console.log(`  -> Still fails! Keeping change.`);
        teams = testTeams;
        changed = true;
        break;
      } else {
        console.log(`  -> Now passes. Reverting.`);
      }
    }
    if (changed) break;
  }
  if (changed) continue;

  // Try removing moves from P2 (keep at least 1)
  for (let i = 0; i < teams.p2.length; i++) {
    for (let m = teams.p2[i].moves.length - 1; m >= 1; m--) {
      const testTeams = clone(teams);
      testTeams.p2[i].moves.splice(m, 1);
      console.log(`Trying: Remove P2[${i}] move ${m} (${teams.p2[i].moves[m]})...`);
      result = testTeam(testTeams);
      if (!result.passes) {
        console.log(`  -> Still fails! Keeping change.`);
        teams = testTeams;
        changed = true;
        break;
      } else {
        console.log(`  -> Now passes. Reverting.`);
      }
    }
    if (changed) break;
  }
  if (changed) continue;

  // Try replacing moves with Tackle (simple move)
  for (let i = 0; i < teams.p1.length; i++) {
    for (let m = 0; m < teams.p1[i].moves.length; m++) {
      if (teams.p1[i].moves[m] !== 'tackle') {
        const testTeams = clone(teams);
        testTeams.p1[i].moves[m] = 'tackle';
        console.log(`Trying: P1[${i}] move ${m} (${teams.p1[i].moves[m]}) -> tackle...`);
        result = testTeam(testTeams);
        if (!result.passes) {
          console.log(`  -> Still fails! Keeping change.`);
          teams = testTeams;
          changed = true;
          break;
        } else {
          console.log(`  -> Now passes. Reverting.`);
        }
      }
    }
    if (changed) break;
  }
  if (changed) continue;

  // Try replacing moves with Tackle for P2
  for (let i = 0; i < teams.p2.length; i++) {
    for (let m = 0; m < teams.p2[i].moves.length; m++) {
      if (teams.p2[i].moves[m] !== 'tackle') {
        const testTeams = clone(teams);
        testTeams.p2[i].moves[m] = 'tackle';
        console.log(`Trying: P2[${i}] move ${m} (${teams.p2[i].moves[m]}) -> tackle...`);
        result = testTeam(testTeams);
        if (!result.passes) {
          console.log(`  -> Still fails! Keeping change.`);
          teams = testTeams;
          changed = true;
          break;
        } else {
          console.log(`  -> Now passes. Reverting.`);
        }
      }
    }
    if (changed) break;
  }
}

console.log('\n=== Minimization complete ===\n');

// Save minimized team
const minFile = `/tmp/teams-seed${seed}-minimized.json`;
fs.writeFileSync(minFile, JSON.stringify(teams, null, 2));
console.log(`Minimized team saved to: ${minFile}`);

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

// Show final divergence
console.log('\n--- Final Divergence ---');
result = testTeam(teams);
console.log(`Diverges at line ${result.divergeAt || 'unknown'}`);
console.log(`  JS:   ${result.jsLine || 'N/A'}`);
console.log(`  Rust: ${result.rustLine || 'N/A'}`);

// Print the full battle outputs for debugging
console.log('\n--- JS Battle Output ---');
try {
  const jsOut = fs.readFileSync(`/tmp/js-battle-seed${seed}.txt`, 'utf8');
  console.log(jsOut.split('\n').filter(l => /^#\d+:/.test(l)).slice(0, 20).join('\n'));
  console.log('...');
} catch (e) {}

console.log('\n--- Rust Battle Output ---');
try {
  const rustOut = fs.readFileSync(`/tmp/rust-battle-seed${seed}.txt`, 'utf8');
  console.log(rustOut.split('\n').filter(l => /^#\d+:/.test(l)).slice(0, 20).join('\n'));
  console.log('...');
} catch (e) {}
