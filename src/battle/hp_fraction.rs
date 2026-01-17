// Helper function for HP fraction calculations that matches JavaScript behavior
//
// In JavaScript, division produces floating-point results (e.g., 1/8 = 0.125).
// When passed to damage() or heal(), non-zero values get clamped to at least 1.
//
// In Rust, integer division truncates (e.g., 1/8 = 0), which would incorrectly
// result in 0 damage/healing when JavaScript would give 1.
//
// This function ensures that when the base HP is positive, the result is at least 1,
// matching JavaScript's behavior where any non-zero fractional value gets rounded up.

/// Calculate fractional HP value, ensuring minimum of 1 when base > 0.
/// This matches JavaScript behavior where floating-point division preserves
/// non-zero values that then get clamped to at least 1 in damage()/heal().
///
/// # Arguments
/// * `base` - The base HP value (e.g., pokemon.base_maxhp)
/// * `divisor` - The divisor for the fraction (e.g., 8 for 1/8 HP)
///
/// # Returns
/// * `base / divisor` if the result is non-zero
/// * `1` if `base > 0` but `base / divisor == 0`
/// * `0` if `base == 0`
///
/// # Examples
/// ```
/// // Shedinja with 1 HP:
/// assert_eq!(hp_fraction(1, 8), 1);   // JS: 0.125 -> clamped to 1
/// assert_eq!(hp_fraction(1, 16), 1);  // JS: 0.0625 -> clamped to 1
///
/// // Normal Pokemon:
/// assert_eq!(hp_fraction(352, 8), 44);   // JS: 44.0 -> 44
/// assert_eq!(hp_fraction(352, 16), 22);  // JS: 22.0 -> 22
/// ```
pub fn hp_fraction(base: i32, divisor: i32) -> i32 {
    let result = base / divisor;
    if result == 0 && base > 0 {
        1
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hp_fraction_shedinja() {
        // Shedinja has 1 HP
        assert_eq!(hp_fraction(1, 8), 1);   // Black Sludge damage
        assert_eq!(hp_fraction(1, 16), 1);  // Leftovers healing
    }

    #[test]
    fn test_hp_fraction_normal() {
        // Normal Pokemon with 352 HP (Espathra from test seed)
        assert_eq!(hp_fraction(352, 8), 44);
        assert_eq!(hp_fraction(352, 16), 22);
    }

    #[test]
    fn test_hp_fraction_zero_base() {
        // Edge case: base HP is 0 (shouldn't happen in practice)
        assert_eq!(hp_fraction(0, 8), 0);
        assert_eq!(hp_fraction(0, 16), 0);
    }

    #[test]
    fn test_hp_fraction_small_values() {
        // Values < divisor
        assert_eq!(hp_fraction(7, 8), 1);   // Would be 0.875 in JS -> 1
        assert_eq!(hp_fraction(8, 8), 1);   // Exactly 1.0 in JS -> 1
        assert_eq!(hp_fraction(15, 16), 1); // Would be 0.9375 in JS -> 1
    }
}
