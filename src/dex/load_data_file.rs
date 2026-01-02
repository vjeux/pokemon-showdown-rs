// TODO: Implement loadDataFile from JavaScript
//
// JS Source:
// 
// 	loadDataFile(basePath: string, dataType: DataType): AnyObject | void {
// 		try {
// 			const filePath = basePath + DATA_FILES[dataType];
// 			const dataObject = require(filePath);
// 			if (!dataObject || typeof dataObject !== 'object') {
// 				throw new TypeError(`${filePath}, if it exists, must export a non-null object`);
// 			}
// 			if (dataObject[dataType]?.constructor?.name !== 'Object') {
// 				throw new TypeError(`${filePath}, if it exists, must export an object whose '${dataType}' property is an Object`);
// 			}
// 			return dataObject[dataType];
// 		} catch (e: any) {
// 			if (e.code !== 'MODULE_NOT_FOUND' && e.code !== 'ENOENT') {
// 				throw e;
// 			}
// 		}
// 	}

use crate::*;

impl Dex {
    // TODO: Implement this method
}
