#!/usr/bin/env node

const fs = require('fs');

// Mapping of event names to dispatcher calls with correct parameters
// Based on the extracted signatures
const fixes = {
    '"AfterBoost"': 'ability_callbacks::dispatch_on_after_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)',
    '"AfterEachBoost"': 'ability_callbacks::dispatch_on_after_each_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)',
    '"AfterMoveSecondary"': 'ability_callbacks::dispatch_on_after_move_secondary(self, ability_id.as_str(), pokemon_pos, (0, 0), "")',
    '"AfterMoveSecondarySelf"': 'ability_callbacks::dispatch_on_after_move_secondary_self(self, ability_id.as_str(), pokemon_pos, (0, 0), "")',
    '"AfterSetStatus"': 'ability_callbacks::dispatch_on_after_set_status(self, ability_id.as_str(), None, Some(pokemon_pos), None, None)',
    '"AllySetStatus"': 'ability_callbacks::dispatch_on_ally_set_status(self, ability_id.as_str(), "", pokemon_pos, None, None)',
    '"AllyTryAddVolatile"': 'ability_callbacks::dispatch_on_ally_try_add_volatile(self, ability_id.as_str(), None, Some(pokemon_pos), None, None)',
    '"AllyTryBoost"': 'ability_callbacks::dispatch_on_ally_try_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)',
    '"AllyTryHitSide"': 'ability_callbacks::dispatch_on_ally_try_hit_side(self, ability_id.as_str(), Some(pokemon_pos), None, "")',
    '"AllyFaint"': 'ability_callbacks::dispatch_on_ally_faint(self, ability_id.as_str(), Some(pokemon_pos))',
    '"AnyAfterSetStatus"': 'ability_callbacks::dispatch_on_any_after_set_status(self, ability_id.as_str(), None, Some(pokemon_pos), None, None)',
    '"AnyAccuracy"': 'ability_callbacks::dispatch_on_any_accuracy(self, ability_id.as_str(), 100, Some(pokemon_pos), None, "")',
    '"AnyDamage"': 'ability_callbacks::dispatch_on_any_damage(self, ability_id.as_str(), 0, Some(pokemon_pos), None, None)',
    '"AnyFaint"': 'ability_callbacks::dispatch_on_any_faint(self, ability_id.as_str(), Some(pokemon_pos))',
    '"AnyModifyBoost"': 'ability_callbacks::dispatch_on_any_modify_boost(self, ability_id.as_str(), "", pokemon_pos)',
    '"Damage"': 'ability_callbacks::dispatch_on_damage(self, ability_id.as_str(), 0, pokemon_pos, None, None)',
    '"DamagePriority"': 'ability_callbacks::dispatch_on_damage_priority(self, ability_id.as_str(), 0, pokemon_pos, None, None)',
    '"SetStatus"': 'ability_callbacks::dispatch_on_set_status(self, ability_id.as_str(), "", pokemon_pos, None, None)',
    '"ChangeBoost"': 'ability_callbacks::dispatch_on_change_boost(self, ability_id.as_str(), "", Some(pokemon_pos), None, None)',
    '"CriticalHit"': 'ability_callbacks::dispatch_on_critical_hit(self, ability_id.as_str(), Some(pokemon_pos), None, "")',
    '"FoeAfterBoost"': 'ability_callbacks::dispatch_on_foe_after_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)',
    '"FoeMaybeTrapPokemon"': 'ability_callbacks::dispatch_on_foe_maybe_trap_pokemon(self, ability_id.as_str(), Some(pokemon_pos))',
    '"FoeTryEatItem"': 'ability_callbacks::dispatch_on_foe_try_eat_item(self, ability_id.as_str(), Some(pokemon_pos), None)',
    '"Immunity"': 'ability_callbacks::dispatch_on_immunity(self, ability_id.as_str(), "", pokemon_pos)',
    '"SourceAfterFaint"': 'ability_callbacks::dispatch_on_source_after_faint(self, ability_id.as_str(), pokemon_pos, (0, 0), None)',
    '"TakeItem"': 'ability_callbacks::dispatch_on_take_item(self, ability_id.as_str(), None, pokemon_pos, None)',
    '"TryAddVolatile"': 'ability_callbacks::dispatch_on_try_add_volatile(self, ability_id.as_str(), "", pokemon_pos, None, None)',
    '"TryBoost"': 'ability_callbacks::dispatch_on_try_boost(self, ability_id.as_str(), "", pokemon_pos, None, None)',
    '"TryBoostPriority"': 'ability_callbacks::dispatch_on_try_boost_priority(self, ability_id.as_str(), "", pokemon_pos, None, None)',
    '"TryEatItem"': 'ability_callbacks::dispatch_on_try_eat_item(self, ability_id.as_str(), Some(pokemon_pos), None)',
    '"TryEatItemPriority"': 'ability_callbacks::dispatch_on_try_eat_item_priority(self, ability_id.as_str(), Some(pokemon_pos), None)',
    '"TryHeal"': 'ability_callbacks::dispatch_on_try_heal(self, ability_id.as_str(), Some(pokemon_pos), None, None)',
    '"Weather"': 'ability_callbacks::dispatch_on_weather(self, ability_id.as_str(), "", pokemon_pos, None)',
    // Add source dispatchers
    '"SourceDamagingHit"': 'ability_callbacks::dispatch_on_source_damaging_hit(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")',
    '"SourceTryHeal"': 'ability_callbacks::dispatch_on_source_try_heal(self, ability_id.as_str(), 0, Some(pokemon_pos), None, None)',
    '"SourceTryPrimaryHit"': 'ability_callbacks::dispatch_on_source_try_primary_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), "")',
    '"DeductPP"': 'ability_callbacks::dispatch_on_deduct_p_p(self, ability_id.as_str(), pokemon_pos, (0, 0), "")',
};

// For callbacks that need multiple parameters but are on one line, we need more complex patterns
const multilineMatches = {
    'AfterMoveSecondary': {
        old: /ability_callbacks::dispatch_on_after_move_secondary\(\s*self,\s*ability_id\.as_str\(\),\s*pokemon_pos,\s*"",\s*\/\/ TODO: Wire through actual move_id\s*\)/g,
        new: 'ability_callbacks::dispatch_on_after_move_secondary(self, ability_id.as_str(), pokemon_pos, (0, 0), "")'
    }
};

console.log('Fixes to apply:');
Object.entries(fixes).forEach(([event, call]) => {
    console.log(`  ${event} => ${call}`);
});
