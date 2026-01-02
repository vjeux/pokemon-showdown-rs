// NOTE: This method is NOT in JavaScript - Rust-specific implementation

// TODO: Implement while from JavaScript
//
// JS Source:
// 				while (true) {
// 					// If data ends with a number, treat it as a target location.
// 					// We need to special case 'Conversion 2' so it doesn't get
// 					// confused with 'Conversion' erroneously sent with the target
// 					// '2' (since Conversion targets 'self', targetLoc can't be 2).
// 					if (/\s(?:-|\+)?[1-3]$/.test(data) && toID(data) !== 'conversion2') {
// 						if (targetLoc !== undefined) return error();
// 						targetLoc = parseInt(data.slice(-2));
// 						data = data.slice(0, -2).trim();
// 					} else if (data.endsWith(' mega')) {
// 						if (event) return error();
// 						event = 'mega';
// 						data = data.slice(0, -5);
// 					} else if (data.endsWith(' megax')) {
// 						if (event) return error();
// 						event = 'megax';
// 						data = data.slice(0, -6);
// 					} else if (data.endsWith(' megay')) {
// 						if (event) return error();
// 						event = 'megay';
// 						data = data.slice(0, -6);
// 					} else if (data.endsWith(' zmove')) {
// 						if (event) return error();
// 						event = 'zmove';
// 						data = data.slice(0, -6);
// 					} else if (data.endsWith(' ultra')) {
// 						if (event) return error();
// 						event = 'ultra';
// 						data = data.slice(0, -6);
// 					} else if (data.endsWith(' dynamax')) {
// 						if (event) return error();
// 						event = 'dynamax';
// 						data = data.slice(0, -8);
// 					} else if (data.endsWith(' gigantamax')) {
// 						if (event) return error();
// 						event = 'dynamax';
// 						data = data.slice(0, -11);
// 					} else if (data.endsWith(' max')) {
// 						if (event) return error();
// 						event = 'dynamax';
// 						data = data.slice(0, -4);
// 					} else if (data.endsWith(' terastal')) {
// 						if (event) return error();
// 						event = 'terastallize';
// 						data = data.slice(0, -9);
// 					} else if (data.endsWith(' terastallize')) {
// 						if (event) return error();
// 						event = 'terastallize';
// 						data = data.slice(0, -13);
// 					} else {
// 						break;
// 					}
// 				}

use crate::*;

impl Side {
    // TODO: Implement this method
}
