#!/usr/bin/env node

const http = require('http');
const { exec } = require('child_process');
const path = require('path');

const PORT = 3456;
const REPO_DIR = __dirname;

const server = http.createServer((req, res) => {
  // CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }

  if (req.method === 'POST' && req.url === '/git') {
    let body = '';

    req.on('data', chunk => {
      body += chunk.toString();
    });

    req.on('end', () => {
      try {
        const { command } = JSON.parse(body);

        if (!command || !command.startsWith('git ')) {
          res.writeHead(400);
          res.end(JSON.stringify({ error: 'Command must start with "git "' }));
          return;
        }

        console.log(`Executing: ${command}`);

        exec(command, { cwd: REPO_DIR }, (error, stdout, stderr) => {
          const result = {
            stdout: stdout,
            stderr: stderr,
            exitCode: error ? error.code : 0
          };

          console.log(`Exit code: ${result.exitCode}`);
          if (stdout) console.log('stdout:', stdout);
          if (stderr) console.log('stderr:', stderr);

          res.writeHead(200, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify(result));
        });
      } catch (e) {
        res.writeHead(400);
        res.end(JSON.stringify({ error: e.message }));
      }
    });
  } else if (req.method === 'GET' && req.url === '/ping') {
    res.writeHead(200);
    res.end('pong');
  } else {
    res.writeHead(404);
    res.end('Not found');
  }
});

server.listen(PORT, '127.0.0.1', () => {
  console.log(`Git HTTP server running on http://127.0.0.1:${PORT}`);
  console.log(`Repository: ${REPO_DIR}`);
  console.log('Ready to accept git commands via POST to /git');
  console.log('Example: curl -X POST http://127.0.0.1:3456/git -H "Content-Type: application/json" -d \'{"command":"git status"}\'');
});
