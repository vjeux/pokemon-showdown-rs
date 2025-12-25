//! Item Callback Handlers
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the actual callback implementations for each item,
//! translated from data/items.ts in the JavaScript codebase.
//!
//! Handlers receive a mutable reference to Battle and Pokemon indices,
//! calling methods on Battle like add_log(), heal(), damage(), boost(), etc.

use crate::battle::Battle;
use crate::dex_data::ID;

// =============================================================================
// HANDLER RESULT - what the handler returns
// =============================================================================

/// Result from a handler - can be None (undefined in JS), or a specific value
#[derive(Debug, Clone)]
pub enum HandlerResult {
    /// No return value (undefined in JS)
    Undefined,
    /// Return false
    False,
    /// Return true
    True,
    /// Return 0 (for onEffectiveness)
    Zero,
    /// Return a number
    Number(i32),
    /// Chain modifier applied (numerator, denominator)
    ChainModify(u32, u32),
}

// =============================================================================
// HANDLER CONTEXT - information passed to handlers
// =============================================================================

/// Context for item handlers
#[derive(Debug, Clone)]
pub struct ItemContext {
    /// The Pokemon holding the item (side_index, pokemon_index)
    pub holder: (usize, usize),
    /// The source of the effect (if any)
    pub source: Option<(usize, usize)>,
    /// The target of the effect (if any)
    pub target: Option<(usize, usize)>,
    /// Move type (if applicable)
    pub move_type: Option<String>,
    /// Move category (if applicable)
    pub move_category: Option<String>,
    /// Move flags (if applicable)
    pub move_flags: MoveFlags,
    /// Damage amount (if applicable)
    pub damage: Option<u32>,
    /// Type modifier from move hit data
    pub type_mod: i32,
    /// Effect ID (if applicable)
    pub effect_id: Option<String>,
    /// Effect type (if applicable)
    pub effect_type: Option<String>,
}

impl Default for ItemContext {
    fn default() -> Self {
        Self {
            holder: (0, 0),
            source: None,
            target: None,
            move_type: None,
            move_category: None,
            move_flags: MoveFlags::default(),
            damage: None,
            type_mod: 0,
            effect_id: None,
            effect_type: None,
        }
    }
}

/// Move flags relevant to item callbacks
#[derive(Debug, Clone, Default)]
pub struct MoveFlags {
    pub bypasssub: bool,
    pub contact: bool,
}

// =============================================================================
// ITEM HANDLERS
// =============================================================================

// -----------------------------------------------------------------------------
// LEFTOVERS
// -----------------------------------------------------------------------------

pub mod leftovers {
    use super::*;

    /// onResidualOrder: 5, onResidualSubOrder: 5
    /// onResidual(pokemon) { this.heal(pokemon.baseMaxhp / 16); }
    pub fn on_residual(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            let amount = pokemon.base_maxhp / 16;
            battle.heal(amount, ctx.holder, None, Some("item: Leftovers"));
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BLACK SLUDGE
// -----------------------------------------------------------------------------

pub mod blacksludge {
    use super::*;

    /// onResidual(pokemon) {
    ///     if (this.field.isTerrain('grassyterrain')) return;
    ///     if (pokemon.hasType('Poison')) {
    ///         this.heal(pokemon.baseMaxhp / 16);
    ///     } else {
    ///         this.damage(pokemon.baseMaxhp / 8);
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        // Check for grassy terrain
        if battle.field.terrain.as_str() == "grassyterrain" {
            return HandlerResult::Undefined;
        }

        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            let has_poison = pokemon.types.iter().any(|t| t == "Poison");
            if has_poison {
                let amount = pokemon.base_maxhp / 16;
                battle.heal(amount, ctx.holder, None, Some("item: Black Sludge"));
            } else {
                let amount = pokemon.base_maxhp / 8;
                battle.damage(amount, ctx.holder, None, Some("item: Black Sludge"));
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIFE ORB
// -----------------------------------------------------------------------------

pub mod lifeorb {
    use super::*;

    /// onModifyDamage(damage, source, target, move) {
    ///     return this.chainModify([5324, 4096]);
    /// }
    pub fn on_modify_damage(_battle: &mut Battle, _ctx: &ItemContext) -> HandlerResult {
        HandlerResult::ChainModify(5324, 4096) // ~1.3x
    }

    /// onAfterMoveSecondarySelf - deals recoil damage
    pub fn on_after_move_secondary_self(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        if let Some(target) = ctx.target {
            if ctx.holder != target && ctx.move_category.as_deref() != Some("Status") {
                let (side_idx, poke_idx) = ctx.holder;
                if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                    let amount = pokemon.base_maxhp / 10;
                    battle.damage(amount, ctx.holder, Some(ctx.holder), Some("item: Life Orb"));
                }
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// CHOICE BAND / SPECS / SCARF
// -----------------------------------------------------------------------------

pub mod choiceband {
    use super::*;

    /// onModifyAtk - 1.5x Attack if not Dynamaxed
    pub fn on_modify_atk(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.volatiles.contains_key(&ID::new("dynamax")) {
                return HandlerResult::Undefined;
            }
        }
        HandlerResult::ChainModify(6144, 4096) // 1.5x
    }
}

pub mod choicespecs {
    use super::*;

    /// onModifySpA - 1.5x Sp. Atk if not Dynamaxed
    pub fn on_modify_spa(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.volatiles.contains_key(&ID::new("dynamax")) {
                return HandlerResult::Undefined;
            }
        }
        HandlerResult::ChainModify(6144, 4096) // 1.5x
    }
}

pub mod choicescarf {
    use super::*;

    /// onModifySpe - 1.5x Speed if not Dynamaxed
    pub fn on_modify_spe(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.volatiles.contains_key(&ID::new("dynamax")) {
                return HandlerResult::Undefined;
            }
        }
        HandlerResult::ChainModify(6144, 4096) // 1.5x
    }
}

// -----------------------------------------------------------------------------
// ASSAULT VEST
// -----------------------------------------------------------------------------

pub mod assaultvest {
    use super::*;

    /// onModifySpD - 1.5x Sp. Def
    pub fn on_modify_spd(_battle: &mut Battle, _ctx: &ItemContext) -> HandlerResult {
        HandlerResult::ChainModify(6144, 4096) // 1.5x
    }
}

// -----------------------------------------------------------------------------
// EVIOLITE
// -----------------------------------------------------------------------------

pub mod eviolite {
    use super::*;

    /// onModifyDef - 1.5x Def if NFE
    /// Note: In the real implementation, we'd check species.nfe
    pub fn on_modify_def(_battle: &mut Battle, _ctx: &ItemContext) -> HandlerResult {
        // Would check pokemon.species.nfe here in full implementation
        // For now, just return the modifier (caller should check NFE status)
        HandlerResult::ChainModify(6144, 4096) // 1.5x
    }

    /// onModifySpD - 1.5x Sp. Def if NFE
    pub fn on_modify_spd(_battle: &mut Battle, _ctx: &ItemContext) -> HandlerResult {
        HandlerResult::ChainModify(6144, 4096) // 1.5x
    }
}

// -----------------------------------------------------------------------------
// FOCUS SASH
// -----------------------------------------------------------------------------

pub mod focussash {
    use super::*;

    /// onDamage - survive fatal hit at full HP
    pub fn on_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        let damage = ctx.damage.unwrap_or(0);
        let is_move = ctx.effect_type.as_deref() == Some("Move");

        // Get values we need before mutating
        let (should_activate, position, hp_minus_1) = {
            if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let activate = pokemon.hp == pokemon.maxhp && damage >= pokemon.hp && is_move;
                (activate, pokemon.position, pokemon.hp.saturating_sub(1) as i32)
            } else {
                (false, 0, 0)
            }
        };

        if should_activate {
            battle.add_log("-enditem", &[&format!("{}", position), "Focus Sash"]);
            return HandlerResult::Number(hp_minus_1);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SITRUS BERRY
// -----------------------------------------------------------------------------

pub mod sitrusberry {
    use super::*;

    /// onUpdate - eat at 50% HP
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.hp <= pokemon.maxhp / 2 {
                // Would trigger eat_item here
                let amount = pokemon.base_maxhp / 4;
                battle.heal(amount, ctx.holder, None, Some("item: Sitrus Berry"));
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// TYPE-BOOSTING ITEMS
// -----------------------------------------------------------------------------

/// Generic type boost handler - 1.2x damage for moves of the specified type
fn type_boost_on_base_power(ctx: &ItemContext, boost_type: &str) -> HandlerResult {
    if ctx.move_type.as_deref() == Some(boost_type) {
        return HandlerResult::ChainModify(4915, 4096); // ~1.2x
    }
    HandlerResult::Undefined
}

pub mod charcoal {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Fire")
    }
}

pub mod mysticwater {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Water")
    }
}

pub mod magnet {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Electric")
    }
}

pub mod miracleseed {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Grass")
    }
}

pub mod nevermeltice {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Ice")
    }
}

pub mod blackbelt {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Fighting")
    }
}

pub mod poisonbarb {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Poison")
    }
}

pub mod softsand {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Ground")
    }
}

pub mod sharpbeak {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Flying")
    }
}

pub mod twistedspoon {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Psychic")
    }
}

pub mod silverpowder {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Bug")
    }
}

pub mod hardstone {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Rock")
    }
}

pub mod spelltag {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Ghost")
    }
}

pub mod dragonfang {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Dragon")
    }
}

pub mod blackglasses {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Dark")
    }
}

pub mod metalcoat {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Steel")
    }
}

pub mod silkscarf {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Normal")
    }
}

pub mod fairyfeather {
    use super::*;
    pub fn on_base_power(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_boost_on_base_power(ctx, "Fairy")
    }
}

// -----------------------------------------------------------------------------
// TYPE RESIST BERRIES
// -----------------------------------------------------------------------------

/// Generic type resist berry handler - 0.5x damage from super effective move of type
fn type_resist_berry_on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext, resist_type: &str) -> HandlerResult {
    if ctx.move_type.as_deref() == Some(resist_type) && ctx.type_mod > 0 {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            // Check for substitute blocking
            let has_sub = pokemon.volatiles.contains_key(&ID::new("substitute"));
            if has_sub && !ctx.move_flags.bypasssub {
                return HandlerResult::Undefined;
            }
            battle.add_log("-enditem", &[&format!("{}", pokemon.position), &format!("{} Berry", resist_type), "[weaken]"]);
            return HandlerResult::ChainModify(2048, 4096); // 0.5x
        }
    }
    HandlerResult::Undefined
}

pub mod occaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Fire")
    }
}

pub mod passhoberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Water")
    }
}

pub mod wacanberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Electric")
    }
}

pub mod rindoberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Grass")
    }
}

pub mod yacheberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Ice")
    }
}

pub mod chopleberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Fighting")
    }
}

pub mod kebiaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Poison")
    }
}

pub mod shucaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Ground")
    }
}

pub mod cobaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Flying")
    }
}

pub mod payapaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Psychic")
    }
}

pub mod tangaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Bug")
    }
}

pub mod chartiberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Rock")
    }
}

pub mod kasibberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Ghost")
    }
}

pub mod habanberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Dragon")
    }
}

pub mod colburberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Dark")
    }
}

pub mod babiriberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Steel")
    }
}

pub mod roseliberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, ctx, "Fairy")
    }
}

pub mod chilanberry {
    use super::*;
    /// Chilan Berry resists Normal without requiring super effective
    pub fn on_source_modify_damage(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        if ctx.move_type.as_deref() == Some("Normal") {
            let (side_idx, poke_idx) = ctx.holder;
            if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let has_sub = pokemon.volatiles.contains_key(&ID::new("substitute"));
                if has_sub && !ctx.move_flags.bypasssub {
                    return HandlerResult::Undefined;
                }
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Chilan Berry", "[weaken]"]);
                return HandlerResult::ChainModify(2048, 4096); // 0.5x
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// EXPERT BELT
// -----------------------------------------------------------------------------

pub mod expertbelt {
    use super::*;

    /// onModifyDamage - 1.2x if super effective
    pub fn on_modify_damage(_battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        if ctx.type_mod > 0 {
            return HandlerResult::ChainModify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FLAME ORB / TOXIC ORB
// -----------------------------------------------------------------------------

pub mod flameorb {
    use super::*;

    /// onResidual - inflict burn on holder
    pub fn on_residual(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        // Would use battle.try_set_status in full implementation
        battle.add_log("-status", &[&format!("{:?}", ctx.holder), "brn", "[from] item: Flame Orb"]);
        HandlerResult::Undefined
    }
}

pub mod toxicorb {
    use super::*;

    /// onResidual - inflict toxic on holder
    pub fn on_residual(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        battle.add_log("-status", &[&format!("{:?}", ctx.holder), "tox", "[from] item: Toxic Orb"]);
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// STATUS CURE BERRIES
// -----------------------------------------------------------------------------

pub mod lumberry {
    use super::*;

    /// onUpdate - cure any status or confusion
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            let has_status = !pokemon.status.is_empty();
            let has_confusion = pokemon.volatiles.contains_key(&ID::new("confusion"));
            if has_status || has_confusion {
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Lum Berry", "[eat]"]);
                // Would cure status and remove confusion here
            }
        }
        HandlerResult::Undefined
    }
}

pub mod cheriberry {
    use super::*;
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.status.as_str() == "par" {
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Cheri Berry", "[eat]"]);
            }
        }
        HandlerResult::Undefined
    }
}

pub mod chestoberry {
    use super::*;
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.status.as_str() == "slp" {
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Chesto Berry", "[eat]"]);
            }
        }
        HandlerResult::Undefined
    }
}

pub mod pechaberry {
    use super::*;
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            let status = pokemon.status.as_str();
            if status == "psn" || status == "tox" {
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Pecha Berry", "[eat]"]);
            }
        }
        HandlerResult::Undefined
    }
}

pub mod rawstberry {
    use super::*;
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.status.as_str() == "brn" {
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Rawst Berry", "[eat]"]);
            }
        }
        HandlerResult::Undefined
    }
}

pub mod aspearberry {
    use super::*;
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.status.as_str() == "frz" {
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Aspear Berry", "[eat]"]);
            }
        }
        HandlerResult::Undefined
    }
}

pub mod persimberry {
    use super::*;
    pub fn on_update(battle: &mut Battle, ctx: &ItemContext) -> HandlerResult {
        let (side_idx, poke_idx) = ctx.holder;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.volatiles.contains_key(&ID::new("confusion")) {
                battle.add_log("-enditem", &[&format!("{}", pokemon.position), "Persim Berry", "[eat]"]);
            }
        }
        HandlerResult::Undefined
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_result_chain_modify() {
        let result = HandlerResult::ChainModify(6144, 4096);
        match result {
            HandlerResult::ChainModify(num, denom) => {
                assert_eq!(num, 6144);
                assert_eq!(denom, 4096);
            }
            _ => panic!("Expected ChainModify"),
        }
    }

    #[test]
    fn test_type_boost() {
        let mut ctx = ItemContext::default();
        ctx.move_type = Some("Fire".to_string());

        let result = type_boost_on_base_power(&ctx, "Fire");
        assert!(matches!(result, HandlerResult::ChainModify(4915, 4096)));

        ctx.move_type = Some("Water".to_string());
        let result2 = type_boost_on_base_power(&ctx, "Fire");
        assert!(matches!(result2, HandlerResult::Undefined));
    }
}
