// TODO: Implement _writeLine from JavaScript
//
// JS Source:
// 
// 	_writeLine(type: string, message: string) {
// 		switch (type) {
// 		case 'start':
// 			const options = JSON.parse(message);
// 			options.send = (t: string, data: any) => {
// 				if (Array.isArray(data)) data = data.join("\n");
// 				this.pushMessage(t, data);
// 				if (t === 'end' && !this.keepAlive) this.pushEnd();
// 			};
// 			if (this.debug) options.debug = true;
// 			this.battle = new Battle(options);
// 			break;
// 		case 'player':
// 			const [slot, optionsText] = splitFirst(message, ' ');
// 			this.battle!.setPlayer(slot as SideID, JSON.parse(optionsText));
// 			break;
// 		case 'p1':
// 		case 'p2':
// 		case 'p3':
// 		case 'p4':
// 			if (message === 'undo') {
// 				this.battle!.undoChoice(type);
// 			} else {
// 				this.battle!.choose(type, message);
// 			}
// 			break;
// 		case 'forcewin':
// 		case 'forcetie':
// 			this.battle!.win(type === 'forcewin' ? message as SideID : null);
// 			if (message) {
// 				this.battle!.inputLog.push(`>forcewin ${message}`);
// 			} else {
// 				this.battle!.inputLog.push(`>forcetie`);
// 			}
// 			break;
// 		case 'forcelose':
// 			this.battle!.lose(message as SideID);
// 			this.battle!.inputLog.push(`>forcelose ${message}`);
// 			break;
// 		case 'reseed':
// 			this.battle!.resetRNG(message as PRNGSeed);
// 			// could go inside resetRNG, but this makes using it in `eval` slightly less buggy
// 			this.battle!.inputLog.push(`>reseed ${this.battle!.prng.getSeed()}`);
// 			break;
// 		case 'tiebreak':
// 			this.battle!.tiebreak();
// 			break;
// 		case 'chat-inputlogonly':
// 			this.battle!.inputLog.push(`>chat ${message}`);
// 			break;
// 		case 'chat':
// 			this.battle!.inputLog.push(`>chat ${message}`);
// 			this.battle!.add('chat', `${message}`);
// 			break;
// 		case 'eval':
// 			const battle = this.battle!;
// 
// 			// n.b. this will usually but not always work - if you eval code that also affects the inputLog,
// 			// replaying the inputlog would double-play the change.
// 			battle.inputLog.push(`>${type} ${message}`);
// 
// 			message = message.replace(/\f/g, '\n');
// 			battle.add('', '>>> ' + message.replace(/\n/g, '\n||'));
// 			try {
// 				/* eslint-disable no-eval, @typescript-eslint/no-unused-vars */
// 				const p1 = battle.sides[0];
// 				const p2 = battle.sides[1];
// 				const p3 = battle.sides[2];
// 				const p4 = battle.sides[3];
// 				const p1active = p1?.active[0];
// 				const p2active = p2?.active[0];
// 				const p3active = p3?.active[0];
// 				const p4active = p4?.active[0];
// 				const toID = battle.toID;
// 				const player = (input: string) => {
// 					input = toID(input);
// 					if (/^p[1-9]$/.test(input)) return battle.sides[parseInt(input.slice(1)) - 1];
// 					if (/^[1-9]$/.test(input)) return battle.sides[parseInt(input) - 1];
// 					for (const side of battle.sides) {
// 						if (toID(side.name) === input) return side;
// 					}
// 					return null;
// 				};
// 				const pokemon = (side: string | Side, input: string) => {
// 					if (typeof side === 'string') side = player(side)!;
// 
// 					input = toID(input);
// 					if (/^[1-9]$/.test(input)) return side.pokemon[parseInt(input) - 1];
// 					return side.pokemon.find(p => p.baseSpecies.id === input || p.species.id === input);
// 				};
// 				let result = eval(message);
// 				/* eslint-enable no-eval, @typescript-eslint/no-unused-vars */
// 
// 				if (result?.then) {
// 					result.then((unwrappedResult: any) => {
// 						unwrappedResult = Utils.visualize(unwrappedResult);
// 						battle.add('', 'Promise -> ' + unwrappedResult);
// 						battle.sendUpdates();
// 					}, (error: Error) => {
// 						battle.add('', '<<< error: ' + error.message);
// 						battle.sendUpdates();
// 					});
// 				} else {
// 					result = Utils.visualize(result);
// 					result = result.replace(/\n/g, '\n||');
// 					battle.add('', '<<< ' + result);
// 				}
// 			} catch (e: any) {
// 				battle.add('', '<<< error: ' + e.message);
// 			}
// 			break;
// 		case 'requestlog':
// 			this.push(`requesteddata\n${this.battle!.inputLog.join('\n')}`);
// 			break;
// 		case 'requestexport':
// 			this.push(`requesteddata\n${this.battle!.prngSeed}\n${this.battle!.inputLog.join('\n')}`);
// 			break;
// 		case 'requestteam':
// 			message = message.trim();
// 			const slotNum = parseInt(message.slice(1)) - 1;
// 			if (isNaN(slotNum) || slotNum < 0) {
// 				throw new Error(`Team requested for slot ${message}, but that slot does not exist.`);
// 			}
// 			const side = this.battle!.sides[slotNum];
// 			const team = Teams.pack(side.team);
// 			this.push(`requesteddata\n${team}`);
// 			break;
// 		case 'show-openteamsheets':
// 			this.battle!.showOpenTeamSheets();
// 			break;
// 		case 'version':
// 		case 'version-origin':
// 			break;
// 		default:
// 			throw new Error(`Unrecognized command ">${type} ${message}"`);
// 		}
// 	}

use crate::*;

impl Battle_stream {
    // TODO: Implement this method
}
