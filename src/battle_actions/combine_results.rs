use crate::*;
use crate::battle_actions::DamageValue;

impl<'a> BattleActions<'a> {

    /// Combine results for damage/effect calculations
    /// Equivalent to combineResults in battle-actions.ts
    //
    // 	combineResults<T extends number | boolean | null | '' | undefined,
    // 		U extends number | boolean | null | '' | undefined>(
    // 		left: T, right: U
    // 	): T | U {
    // 		const NOT_FAILURE = 'string';
    // 		const NULL = 'object';
    // 		const resultsPriorities = ['undefined', NOT_FAILURE, NULL, 'boolean', 'number'];
    // 		if (resultsPriorities.indexOf(typeof left) > resultsPriorities.indexOf(typeof right)) {
    // 			return left;
    // 		} else if (left && !right && right !== 0) {
    // 			return left;
    // 		} else if (typeof left === 'number' && typeof right === 'number') {
    // 			return (left + right) as T;
    // 		} else {
    // 			return right;
    // 		}
    // 	}
    //
    pub fn combine_results(
        left: Option<DamageValue>,
        right: Option<DamageValue>,
    ) -> Option<DamageValue> {
        // Priority: undefined < NOT_FAIL < null < boolean < number
        match (&left, &right) {
            (None, r) => r.clone(),
            (l, None) => l.clone(),
            (Some(DamageValue::Damage(l)), Some(DamageValue::Damage(r))) => {
                Some(DamageValue::Damage(l + r))
            }
            (_, r) => r.clone(),
        }
    }
}
