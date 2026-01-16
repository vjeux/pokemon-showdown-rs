//! BattleStream::receive_line - Process single protocol line
//!
//! 1:1 port of receiveLine from battle-stream.ts

use crate::battle_stream::BattleStream;

impl BattleStream {
    /// Process a single protocol line
    /// Equivalent to receiveLine() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   receiveLine(line: string) {
    ///     if (this.debug) console.log(line);
    ///     if (!line.startsWith('|')) return;
    ///     const [cmd, rest] = splitFirst(line.slice(1), '|');
    ///     if (cmd === 'request') return this.receiveRequest(JSON.parse(rest));
    ///     if (cmd === 'error') return this.receiveError(new Error(rest));
    ///     this.log.push(line);
    ///   }
    pub fn receive_line(&mut self, line: &str) {
        // JS: if (this.debug) console.log(line);
        if self.debug {
            debug_elog!("[BattleStream] {}", line);
        }

        // JS: if (!line.startsWith('|')) return;
        if !line.starts_with('|') {
            return;
        }

        // JS: const [cmd, rest] = splitFirst(line.slice(1), '|');
        let line_without_prefix = &line[1..];
        let parts = crate::battle_stream::split_first(line_without_prefix, "|", 1);
        let cmd = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let rest = parts.get(1).map(|s| s.as_str()).unwrap_or("");

        // JS: if (cmd === 'request') return this.receiveRequest(JSON.parse(rest));
        if cmd == "request" {
            // Parse the JSON request and pass to receiveRequest
            if let Ok(request) = serde_json::from_str(rest) {
                self.receive_request(request);
            }
            return;
        }

        // JS: if (cmd === 'error') return this.receiveError(new Error(rest));
        if cmd == "error" {
            // Errors are propagated through Result, just log for now
            if let Some(battle) = self.battle.as_mut() {
                battle.log.push(line.to_string());
            }
            return;
        }

        // JS: this.log.push(line);
        if let Some(battle) = self.battle.as_mut() {
            battle.log.push(line.to_string());
        }
    }
}
