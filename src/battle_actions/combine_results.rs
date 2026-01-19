//! BattleActions::combineResults - Combine two result values
//!
//! 1:1 port of combineResults from battle-actions.ts

use crate::battle_actions::DamageResult;

/// Combine two result values based on JavaScript's combineResults logic
/// JavaScript equivalent: combineResults() in battle-actions.ts
///
/// JavaScript signature:
/// combineResults<T extends number | boolean | null | '' | undefined,
///     U extends number | boolean | null | '' | undefined>(
///     left: T, right: U
/// ): T | U
///
/// Priority order (lowest to highest):
/// - undefined (lowest)
/// - '' (NOT_FAIL string)
/// - null (NULL object)
/// - boolean
/// - number (highest)
//
// combineResults<T extends number | boolean | null | '' | undefined,
//     U extends number | boolean | null | '' | undefined>(
//     left: T, right: U
// ): T | U {
//     const NOT_FAILURE = 'string';
//     const NULL = 'object';
//     const resultsPriorities = ['undefined', NOT_FAILURE, NULL, 'boolean', 'number'];
//     if (resultsPriorities.indexOf(typeof left) > resultsPriorities.indexOf(typeof right)) {
//         return left;
//     } else if (left && !right && right !== 0) {
//         return left;
//     } else if (typeof left === 'number' && typeof right === 'number') {
//         return (left + right) as T;
//     } else {
//         return right;
//     }
// }
pub fn combine_results(left: DamageResult, right: DamageResult) -> DamageResult {
    use DamageResult::*;

    // Type priority mapping (matches JavaScript resultsPriorities array)
    // const resultsPriorities = ['undefined', NOT_FAILURE, NULL, 'boolean', 'number'];
    // Indices: undefined=0, ''=1, null=2, boolean=3, number=4
    let get_priority = |r: &DamageResult| match r {
        Undefined => 0,         // 'undefined'
        NotFail => 1,           // 'string' (NOT_FAIL = '')
        Null => 2,              // 'object' (null)
        Failed | Success => 3,  // 'boolean' (false | true)
        Damage(_) | HitSubstitute => 4, // 'number'
    };

    let left_priority = get_priority(&left);
    let right_priority = get_priority(&right);

    // if (resultsPriorities.indexOf(typeof left) > resultsPriorities.indexOf(typeof right))
    //     return left;
    if left_priority > right_priority {
        return left;
    }

    // else if (left && !right && right !== 0)
    //     return left;
    // In JavaScript: truthy left, falsy right (but not 0)
    // For DamageResult: Success/Damage are truthy, Failed/Undefined/Null are falsy
    // NotFail is '' (empty string) in JS, which is FALSY
    // HitSubstitute is 0 so falsy but equals 0
    let is_truthy = |r: &DamageResult| matches!(r, Success | Damage(_));

    if is_truthy(&left) && !is_truthy(&right) && !matches!(right, HitSubstitute | Damage(0)) {
        return left;
    }

    // else if (typeof left === 'number' && typeof right === 'number')
    //     return (left + right) as T;
    if let (Damage(l), Damage(r)) = (left, right) {
        return Damage(l + r);
    }

    // Handle HitSubstitute (treated as 0) + number
    if let (Damage(l), HitSubstitute) = (left, right) {
        return Damage(l);
    }
    if let (HitSubstitute, Damage(r)) = (left, right) {
        return Damage(r);
    }

    // else
    //     return right;
    right
}
