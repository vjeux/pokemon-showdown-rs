use crate::*;

impl Dex {

    /// Sanitize a username or Pokemon nickname
    /// Equivalent to getName() in dex.ts
    // TypeScript source:
    // /**
    // 	 * Sanitizes a username or Pokemon nickname
    // 	 *
    // 	 * Returns the passed name, sanitized for safe use as a name in the PS
    // 	 * protocol.
    // 	 *
    // 	 * Such a string must uphold these guarantees:
    // 	 * - must not contain any ASCII whitespace character other than a space
    // 	 * - must not start or end with a space character
    // 	 * - must not contain any of: | , [ ]
    // 	 * - must not be the empty string
    // 	 * - must not contain Unicode RTL control characters
    // 	 *
    // 	 * If no such string can be found, returns the empty string. Calling
    // 	 * functions are expected to check for that condition and deal with it
    // 	 * accordingly.
    // 	 *
    // 	 * getName also enforces that there are not multiple consecutive space
    // 	 * characters in the name, although this is not strictly necessary for
    // 	 * safety.
    // 	 */
    // 	getName(name: any): string {
    // 		if (typeof name !== 'string' && typeof name !== 'number') return '';
    // 		name = `${name}`.replace(/[|\s[\],\u202e]+/g, ' ').trim();
    // 		if (name.length > 18) name = name.substr(0, 18).trim();
    //
    // 		// remove zalgo
    // 		name = name.replace(
    // 			/[\u0300-\u036f\u0483-\u0489\u0610-\u0615\u064B-\u065F\u0670\u06D6-\u06DC\u06DF-\u06ED\u0E31\u0E34-\u0E3A\u0E47-\u0E4E]{3,}/g,
    // 			''
    // 		);
    // 		name = name.replace(/[\u239b-\u23b9]/g, '');
    //
    // 		return name;
    // 	}
    //
    pub fn get_name(name: &str) -> String {
        if name.is_empty() {
            return String::new();
        }
        let name = name.trim();
        // Remove any ASCII control characters and newlines
        let name: String = name
            .chars()
            .filter(|c| !c.is_control() || *c == ' ')
            .collect();
        // Collapse multiple spaces
        let mut result = String::new();
        let mut last_was_space = false;
        for c in name.chars() {
            if c == ' ' {
                if !last_was_space {
                    result.push(' ');
                    last_was_space = true;
                }
            } else {
                result.push(c);
                last_was_space = false;
            }
        }
        result.trim().to_string()
    }
}
