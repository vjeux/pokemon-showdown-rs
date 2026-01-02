// TODO: Implement receiveLine from JavaScript
//
// JS Source:
// 
// 	receiveLine(line: string) {
// 		if (this.debug) console.log(line);
// 		if (!line.startsWith('|')) return;
// 		const [cmd, rest] = splitFirst(line.slice(1), '|');
// 		if (cmd === 'request') return this.receiveRequest(JSON.parse(rest));
// 		if (cmd === 'error') return this.receiveError(new Error(rest));
// 		this.log.push(line);
// 	}

use crate::*;

impl Battle_stream {
    // TODO: Implement this method
}
