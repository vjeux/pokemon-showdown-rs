use crate::*;

impl<'a> BattleActions<'a> {

    /// Execute Terastallization
    /// Equivalent to terastallize in battle-actions.ts
    /// Returns the new tera type if successful
    pub fn terastallize_check(
        tera_type: Option<&str>,
        already_terastallized: bool,
        side_terastallize_used: bool,
    ) -> Option<String> {
        if already_terastallized || side_terastallize_used {
            return None;
        }
        tera_type.map(|s| s.to_string())
    }
}
