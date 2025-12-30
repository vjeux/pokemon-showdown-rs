use crate::*;

impl<'a> BattleActions<'a> {

    /// Calculate the effective stat value with boost applied
    pub fn calculate_stat_with_boost(base_stat: i32, boost: i8) -> i32 {
        let (num, denom) = Self::get_boost_modifier(boost);
        (base_stat * num / denom).max(1)
    }
}
