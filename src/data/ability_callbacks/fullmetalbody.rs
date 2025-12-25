//! Full Metal Body Ability - Prevents stat reduction

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryBoost(boost, target, source, effect)
/// Prevents stats from being lowered by opponents (same as White Smoke/Clear Body)
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, effect_id: &str, has_secondaries: bool) -> AbilityHandlerResult {
    // If source exists and target is the source, do nothing
    if let Some(src) = source {
        if (target.side_index, target.position) == (src.side_index, src.position) {
            return AbilityHandlerResult::Undefined;
        }
    }
    let mut show_msg = false;
    // Remove all negative boosts
    let stats: Vec<String> = boost.keys().cloned().collect();
    for stat in stats {
        if let Some(&val) = boost.get(&stat) {
            if val < 0 {
                boost.remove(&stat);
                show_msg = true;
            }
        }
    }
    if show_msg && !has_secondaries && effect_id != "octolock" {
        battle.add("-fail", &[Arg::Pokemon(target), Arg::Str("unboost"), Arg::Str("[from] ability: Full Metal Body"), Arg::Str(&format!("[of] {}", target.name))]);
    }
    AbilityHandlerResult::Undefined
}
