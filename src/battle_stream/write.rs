// JS Source:
// 			write(data: string) {
// 				void stream.write(data);
// 			}


use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Write input to the battle
    /// Equivalent to _write() and _writeLines() in battle-stream.ts (lines 68-80)
    /// Processes input commands sent to the battle stream
    pub fn write(&mut self, input: &str) {
        for line in input.lines() {
            let line = line.trim();
            if let Some(command_line) = line.strip_prefix('>') {
                let parts: Vec<&str> = command_line.splitn(2, ' ').collect();
                let cmd = parts.first().copied().unwrap_or("");
                let args = parts.get(1).copied().unwrap_or("");
                self.write_line(cmd, args);
            }
        }
    }
}
