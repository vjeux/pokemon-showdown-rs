use crate::*;
use super::CHOOSABLE_TARGETS;

impl<'a> BattleActions<'a> {

    /// Check if a target type allows choosing
    /// Equivalent to targetTypeChoices in battle-actions.ts
    //
    // 	targetTypeChoices(targetType: string) {
    // 		return CHOOSABLE_TARGETS.has(targetType);
    // 	}
    //
    pub fn target_type_choices(target_type: &str) -> bool {
        CHOOSABLE_TARGETS.contains(target_type)
    }
}
