#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Configuration
const JS_FILES = {
  'battle.ts': 'src/battle',
  'battle-actions.ts': 'src/battle_actions',
  'battle-queue.ts': 'src/battle_queue',
  'battle-stream.ts': 'src/battle_stream',
  'dex.ts': 'src/dex',
  'pokemon.ts': 'src/pokemon',
  'side.ts': 'src/side'
};

const SHOWDOWN_PATH = '../pokemon-showdown-ts/sim';

// Extract methods from a TypeScript file
function extractMethods(filePath) {
  const content = fs.readFileSync(filePath, 'utf-8');
  const methods = [];

  // Skip keywords that aren't actual methods
  const KEYWORDS = new Set(['for', 'if', 'while', 'switch', 'catch', 'try', 'else', 'do', 'return', 'const', 'let', 'var']);

  // Match method definitions in classes
  // Matches: methodName(...) { or async methodName(...) {
  const methodRegex = /^\s*(async\s+)?(\w+)\s*\([^)]*\)\s*(?::\s*[^{]+)?\s*\{/gm;

  let match;
  while ((match = methodRegex.exec(content)) !== null) {
    const methodName = match[2];
    const startPos = match.index;

    // Skip constructors, type definitions, and keywords
    if (methodName === 'constructor' ||
        methodName === 'interface' ||
        methodName === 'type' ||
        methodName === 'function' ||
        KEYWORDS.has(methodName)) {
      continue;
    }

    // Find the end of the method by counting braces
    let braceCount = 1;
    let pos = content.indexOf('{', startPos) + 1;
    let endPos = pos;

    while (braceCount > 0 && pos < content.length) {
      const char = content[pos];
      if (char === '{') braceCount++;
      if (char === '}') braceCount--;
      if (braceCount === 0) {
        endPos = pos;
        break;
      }
      pos++;
    }

    const methodSource = content.substring(startPos, endPos + 1);

    methods.push({
      name: methodName,
      source: methodSource,
      fullSource: methodSource
    });
  }

  return methods;
}

// Convert camelCase to snake_case
function toSnakeCase(str) {
  return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

// Check if a Rust file exists and has JS source comment
function checkRustFile(rustDir, methodName) {
  const snakeName = toSnakeCase(methodName);
  const rustPath = path.join(rustDir, `${snakeName}.rs`);

  if (!fs.existsSync(rustPath)) {
    return { exists: false, hasComment: false, path: rustPath };
  }

  const content = fs.readFileSync(rustPath, 'utf-8');

  // Check if it has JS source code as comment
  // Look for patterns like: /// JS Source, // JS:, /// ```text, etc.
  const hasComment = content.includes('JS Source') ||
                     content.includes('// JS:') ||
                     content.includes('///     ') ||
                     content.match(/\/\/\/?\s*\w+\([^)]*\)\s*\{/);

  return { exists: true, hasComment, path: rustPath, content };
}

// Create a new Rust file with TODO and JS source
function createRustFile(rustPath, methodName, jsSource) {
  const snakeName = toSnakeCase(methodName);
  const moduleName = path.basename(path.dirname(rustPath));

  const content = `// TODO: Implement ${methodName} from JavaScript
//
// JS Source:
// ${jsSource.split('\n').join('\n// ')}

use crate::*;

impl ${capitalize(moduleName)} {
    // TODO: Implement this method
}
`;

  fs.writeFileSync(rustPath, content);
  console.log(`✅ Created: ${rustPath}`);
}

// Add JS source comment to existing Rust file
function addJsSourceComment(rustPath, jsSource) {
  const content = fs.readFileSync(rustPath, 'utf-8');

  // Add JS source at the top of the file after initial comments
  const lines = content.split('\n');
  let insertPos = 0;

  // Skip initial doc comments and find first use/impl
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    if (line.startsWith('use ') || line.startsWith('impl ') || line.startsWith('pub ')) {
      insertPos = i;
      break;
    }
  }

  const jsComment = `// JS Source:\n// ${jsSource.split('\n').join('\n// ')}\n\n`;
  lines.splice(insertPos, 0, jsComment);

  fs.writeFileSync(rustPath, lines.join('\n'));
  console.log(`📝 Added JS source to: ${rustPath}`);
}

// Find Rust files without JS equivalent
function findRustOnlyFiles(rustDir, jsMethods) {
  if (!fs.existsSync(rustDir)) {
    return [];
  }

  const files = fs.readdirSync(rustDir)
    .filter(f => f.endsWith('.rs') && f !== 'mod.rs');

  const jsMethodNames = new Set(jsMethods.map(m => toSnakeCase(m.name) + '.rs'));
  const rustOnlyFiles = [];

  for (const file of files) {
    if (!jsMethodNames.has(file)) {
      rustOnlyFiles.push(path.join(rustDir, file));
    }
  }

  return rustOnlyFiles;
}

// Add "NOT in JS" comment to Rust file
function addNotInJsComment(rustPath) {
  const content = fs.readFileSync(rustPath, 'utf-8');

  // Check if comment already exists
  if (content.includes('NOT in JavaScript') || content.includes('NOT in JS')) {
    return;
  }

  // Add at the top
  const newContent = `// NOTE: This method is NOT in JavaScript - Rust-specific implementation\n\n${content}`;
  fs.writeFileSync(rustPath, newContent);
  console.log(`🔧 Marked as Rust-only: ${rustPath}`);
}

function capitalize(str) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

// Main processing
function processFile(jsFileName, rustDir) {
  console.log(`\n${'='.repeat(60)}`);
  console.log(`Processing: ${jsFileName} -> ${rustDir}`);
  console.log('='.repeat(60));

  const jsPath = path.join(SHOWDOWN_PATH, jsFileName);

  if (!fs.existsSync(jsPath)) {
    console.log(`⚠️  JS file not found: ${jsPath}`);
    return;
  }

  // Extract methods from JS
  const jsMethods = extractMethods(jsPath);
  console.log(`Found ${jsMethods.length} methods in ${jsFileName}`);

  // Ensure Rust directory exists
  if (!fs.existsSync(rustDir)) {
    fs.mkdirSync(rustDir, { recursive: true });
  }

  // Process each JS method
  for (const method of jsMethods) {
    const rustCheck = checkRustFile(rustDir, method.name);

    if (!rustCheck.exists) {
      console.log(`❌ Missing: ${method.name} -> creating ${rustCheck.path}`);
      createRustFile(rustCheck.path, method.name, method.source);
    } else if (!rustCheck.hasComment) {
      console.log(`⚠️  No JS source: ${method.name} -> adding to ${rustCheck.path}`);
      addJsSourceComment(rustCheck.path, method.source);
    } else {
      // Method exists with comment - do nothing
      // console.log(`✓ OK: ${method.name}`);
    }
  }

  // Find Rust-only files
  const rustOnlyFiles = findRustOnlyFiles(rustDir, jsMethods);
  for (const rustPath of rustOnlyFiles) {
    addNotInJsComment(rustPath);
  }

  console.log(`\n✓ Completed ${jsFileName}`);
}

// Main execution
console.log('JavaScript to Rust Method Sync');
console.log('==============================\n');

for (const [jsFile, rustDir] of Object.entries(JS_FILES)) {
  processFile(jsFile, rustDir);
}

console.log('\n' + '='.repeat(60));
console.log('Sync complete!');
console.log('='.repeat(60));
