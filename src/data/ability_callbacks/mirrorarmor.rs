//! Mirror Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryBoost(boost, target, source, effect) {
///     // Don't bounce self stat changes, or boosts that have already bounced
///     if (!source || target === source || !boost || effect.name === 'Mirror Armor') return;
///     let b: BoostID;
///     for (b in boost) {
///         if (boost[b]! < 0) {
///             if (target.boosts[b] === -6) continue;
///             const negativeBoost: SparseBoostsTable = {};
///             negativeBoost[b] = boost[b];
///             delete boost[b];
///             if (source.hp) {
///                 this.add('-ability', target, 'Mirror Armor');
///                 this.boost(negativeBoost, source, target, null, true);
///             }
///         }
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    boost: Option<&mut crate::dex_data::BoostsTable>, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, _effect_id: Option<&str>,
) -> EventResult {
    // if (!source || target === source) return;
    if source_pos.is_none() {
        return EventResult::Continue;
    }
    let source_pos = source_pos.unwrap();
    if source_pos == target_pos {
        return EventResult::Continue;
    }

    // if (!boost) return;
    let boost = match boost {
        Some(b) => b,
        None => return EventResult::Continue,
    };

    // if (effect.name === 'Mirror Armor') return;
    let is_mirror_armor = battle.current_event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|id| id.as_str() == "mirrorarmor")
        .unwrap_or(false);

    if is_mirror_armor {
        return EventResult::Continue;
    }

    // Check if any negative boosts exist and reflect them
    let mut has_negative = false;
    let mut ability_shown = false;

    // Helper to bounce a single boost
    let mut bounce_boost = |boost_val: &mut i8, boost_id: crate::dex_data::BoostID| {
        if *boost_val < 0 {
            // Check if target boost is already at minimum
            let target_boost = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return,
                };
                target.boosts.get(boost_id)
            };

            // if (target.boosts[b] === -6) continue;
            if target_boost == -6 {
                return;
            }

            has_negative = true;

            // Check if source has HP
            let source_hp = {
                let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                    Some(p) => p,
                    None => return,
                };
                source.hp
            };

            if source_hp > 0 {
                // this.add('-ability', target, 'Mirror Armor');
                if !ability_shown {
                    let target_slot = {
                        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                            Some(p) => p,
                            None => return,
                        };
                        target.get_slot()
                    };

                    battle.add(
                        "-ability",
                        &[
                            crate::battle::Arg::from(target_slot),
                            crate::battle::Arg::from("Mirror Armor"),
                        ],
                    );
                    ability_shown = true;
                }

                // Boost the source with the negative value
                let boost_amount = *boost_val;
                *boost_val = 0; // delete boost[b];

                // Convert BoostID to string for battle.boost call
                let boost_name = match boost_id {
                    crate::dex_data::BoostID::Atk => "atk",
                    crate::dex_data::BoostID::Def => "def",
                    crate::dex_data::BoostID::SpA => "spa",
                    crate::dex_data::BoostID::SpD => "spd",
                    crate::dex_data::BoostID::Spe => "spe",
                    crate::dex_data::BoostID::Accuracy => "accuracy",
                    crate::dex_data::BoostID::Evasion => "evasion",
                };

                // this.boost(negativeBoost, source, target, null, true);
                battle.boost(
                    &[(boost_name, boost_amount)],
                    source_pos,
                    Some(target_pos),
                    None,
                    true,
                    false,
                );
            }
        }
    };

    // Process each boost
    bounce_boost(&mut boost.atk, crate::dex_data::BoostID::Atk);
    bounce_boost(&mut boost.def, crate::dex_data::BoostID::Def);
    bounce_boost(&mut boost.spa, crate::dex_data::BoostID::SpA);
    bounce_boost(&mut boost.spd, crate::dex_data::BoostID::SpD);
    bounce_boost(&mut boost.spe, crate::dex_data::BoostID::Spe);
    bounce_boost(&mut boost.accuracy, crate::dex_data::BoostID::Accuracy);
    bounce_boost(&mut boost.evasion, crate::dex_data::BoostID::Evasion);

    EventResult::Continue
}

