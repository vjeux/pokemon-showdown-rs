//! BattleStream::_writeLines - Process multiple protocol command lines
//!
//! 1:1 port of _writeLines from battle-stream.ts
//
// JS Source:
// 
// 	_writeLines(chunk: string) {
// 		for (const line of chunk.split('\n')) {
// 			if (line.startsWith('>')) {
// 				const [type, message] = splitFirst(line.slice(1), ' ');
// 				this._writeLine(type, message);
// 			}
// 		}
// 	}

use crate::*;
use crate::battle_stream::BattleStream;

impl BattleStream {
    /// Process multiple protocol command lines
    /// Equivalent to _writeLines() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   _writeLines(chunk: string) {
    ///     for (const line of chunk.split('\n')) {
    ///       if (line.startsWith('>')) {
    ///         const [type, message] = splitFirst(line.slice(1), ' ');
    ///         this._writeLine(type, message);
    ///       }
    ///     }
    ///   }
    pub fn _write_lines(&mut self, chunk: &str) -> Result<(), String> {
        // for (const line of chunk.split('\n'))
        for line in chunk.split('\n') {
            // if (line.startsWith('>'))
            if line.starts_with('>') {
                // const [type, message] = splitFirst(line.slice(1), ' ');
                let (cmd_type, message) = crate::battle_stream::split_first(&line[1..], ' ');

                // this._writeLine(type, message);
                self._write_line(cmd_type, message)?;
            }
        }
        Ok(())
    }
}
