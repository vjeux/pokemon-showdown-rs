// NOTE: This method is NOT in JavaScript - Rust-specific implementation


/// Split first utility function
/// Equivalent to splitFirst() in battle-stream.ts
// TypeScript source:
// /**
//  * Like string.split(delimiter), but only recognizes the first `limit`
//  * delimiters (default 1).
//  *
//  * `"1 2 3 4".split(" ", 2) => ["1", "2"]`
//  *
//  * `Utils.splitFirst("1 2 3 4", " ", 1) => ["1", "2 3 4"]`
//  *
//  * Returns an array of length exactly limit + 1.
//  */
// function splitFirst(str: string, delimiter: string, limit = 1) {
// 	const splitStr: string[] = [];
// 	while (splitStr.length < limit) {
// 		const delimiterIndex = str.indexOf(delimiter);
// 		if (delimiterIndex >= 0) {
// 			splitStr.push(str.slice(0, delimiterIndex));
// 			str = str.slice(delimiterIndex + delimiter.length);
// 		} else {
// 			splitStr.push(str);
// 			str = '';
// 		}
// 	}
// 	splitStr.push(str);
// 	return splitStr;
// }
//
pub fn split_first(s: &str, delimiter: &str, limit: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut remaining = s;

    for _ in 0..limit {
        if let Some(idx) = remaining.find(delimiter) {
            result.push(remaining[..idx].to_string());
            remaining = &remaining[idx + delimiter.len()..];
        } else {
            result.push(remaining.to_string());
            remaining = "";
            break;
        }
    }
    result.push(remaining.to_string());
    result
}
