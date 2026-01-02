// TODO: Implement getDescs from JavaScript
//
// JS Source:
// 
// 	getDescs(table: keyof TextTableData, id: ID, dataEntry: AnyObject) {
// 		if (dataEntry.shortDesc) {
// 			return {
// 				desc: dataEntry.desc,
// 				shortDesc: dataEntry.shortDesc,
// 			};
// 		}
// 		const entry = this.loadTextData()[table][id];
// 		if (!entry) return null;
// 		const descs = {
// 			desc: '',
// 			shortDesc: '',
// 		};
// 		for (let i = this.gen; i < dexes['base'].gen; i++) {
// 			const curDesc = entry[`gen${i}` as keyof typeof entry]?.desc;
// 			const curShortDesc = entry[`gen${i}` as keyof typeof entry]?.shortDesc;
// 			if (!descs.desc && curDesc) {
// 				descs.desc = curDesc;
// 			}
// 			if (!descs.shortDesc && curShortDesc) {
// 				descs.shortDesc = curShortDesc;
// 			}
// 			if (descs.desc && descs.shortDesc) break;
// 		}
// 		if (!descs.shortDesc) descs.shortDesc = entry.shortDesc || '';
// 		if (!descs.desc) descs.desc = entry.desc || descs.shortDesc;
// 		return descs;
// 	}

use crate::*;

impl Dex {
    // TODO: Implement this method
}
