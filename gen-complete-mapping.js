#!/usr/bin/env node

const fs = require('fs');

// Read mod.rs to get all signatures
const modContent = fs.readFileSync('src/data/ability_callbacks/mod.rs', 'utf8');
const battleContent = fs.readFileSync('src/battle.rs', 'utf8');

// Extract all dispatcher signatures precisely
const dispatcherSignatures = new Map();
const lines = modContent.split('\n');

for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes('pub fn dispatch_')) {
    const match = lines[i].match(/pub fn (dispatch_\w+)\(/);
    if (match) {
      const funcName = match[1];

      // Collect full signature
      let sig = '';
      let j = i;
      while (j < lines.length && !lines[j].includes(') -> EventResult')) {
        sig += lines[j] + '\n';
        j++;
      }
      if (j < lines.length) {
        sig += lines[j];
      }

      // Extract parameter types after ability_id
      const paramMatch = sig.match(/ability_id: &str,\s*([^)]*)\) -> EventResult/s);
      if (paramMatch) {
        const params = paramMatch[1].trim();
        if (params) {
          dispatcherSignatures.set(funcName, params);
        } else {
          dispatcherSignatures.set(funcName, '');
        }
      } else {
        dispatcherSignatures.set(funcName, '');
      }
    }
  }
}

// Generate default args for each signature
function getDefaultArgs(paramStr) {
  if (!paramStr) return [];

  const args = [];
  let current = '';
  let depth = 0;

  for (let i = 0; i < paramStr.length; i++) {
    const char = paramStr[i];
    if (char === '<' || char === '(') depth++;
    if (char === '>' || char === ')') depth--;

    if (char === ',' && depth === 0) {
      args.push(current.trim());
      current = '';
    } else {
      current += char;
    }
  }
  if (current.trim()) args.push(current.trim());

  return args.map(arg => {
    const type = arg.split(':')[1].trim();

    if (type.includes('Option<(usize, usize)>')) {
      if (arg.includes('target_pos')) return 'Some(pokemon_pos)';
      return 'None';
    }
    if (type.includes('(usize, usize)')) return '(0, 0)';
    if (type.includes('Option<&str>')) return 'None';
    if (type.includes('&str')) return '""';
    if (type.includes('i32')) return '0';
    if (type.includes('f32')) return '0.0';
    if (type.includes('f64')) return '0.0';
    return 'todo!()';
  });
}

// Print comprehensive fix list
console.log('Comprehensive dispatcher signature reference:\\n');
console.log('='.repeat(80) + '\\n');

const sortedDispatchers = Array.from(dispatcherSignatures.entries()).sort((a, b) => a[0].localeCompare(b[0]));

sortedDispatchers.forEach(([name, params]) => {
  const args = getDefaultArgs(params);
  console.log(`${name}:`);
  if (args.length > 0) {
    console.log(`  Args: self, ability_id.as_str(), ${args.join(', ')}`);
  } else {
    console.log(`  Args: self, ability_id.as_str()`);
  }
  console.log('');
});

// Save mapping
const mapping = {};
sortedDispatchers.forEach(([name, params]) => {
  mapping[name] = getDefaultArgs(params);
});

fs.writeFileSync('complete-mapping.json', JSON.stringify(mapping, null, 2));
console.log(`\\nSaved complete mapping to complete-mapping.json (${sortedDispatchers.length} dispatchers)`);
