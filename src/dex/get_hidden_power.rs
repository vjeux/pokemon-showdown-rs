use crate::*;
use crate::dex_data::StatsTable;

impl Dex {

    /// Calculate Hidden Power type from IVs
    /// Equivalent to getHiddenPower() in dex.ts
    //
    // 	getHiddenPower(ivs: StatsTable) {
    // 		const hpTypes = [
    // 			'Fighting', 'Flying', 'Poison', 'Ground', 'Rock', 'Bug', 'Ghost', 'Steel',
    // 			'Fire', 'Water', 'Grass', 'Electric', 'Psychic', 'Ice', 'Dragon', 'Dark',
    // 		];
    // 		const tr = this.trunc;
    // 		const stats = { hp: 31, atk: 31, def: 31, spe: 31, spa: 31, spd: 31 };
    // 		if (this.gen <= 2) {
    // 			// Gen 2 specific Hidden Power check. IVs are still treated 0-31 so we get them 0-15
    // 			const atkDV = tr(ivs.atk / 2);
    // 			const defDV = tr(ivs.def / 2);
    // 			const speDV = tr(ivs.spe / 2);
    // 			const spcDV = tr(ivs.spa / 2);
    // 			return {
    // 				type: hpTypes[4 * (atkDV % 4) + (defDV % 4)],
    // 				power: tr(
    // 					(5 * ((spcDV >> 3) + (2 * (speDV >> 3)) + (4 * (defDV >> 3)) + (8 * (atkDV >> 3))) + (spcDV % 4)) / 2 + 31
    // 				),
    // 			};
    // 		} else {
    // 			// Hidden Power check for Gen 3 onwards
    // 			let hpTypeX = 0;
    // 			let hpPowerX = 0;
    // 			let i = 1;
    // 			for (const s in stats) {
    // 				hpTypeX += i * (ivs[s as StatID] % 2);
    // 				hpPowerX += i * (tr(ivs[s as StatID] / 2) % 2);
    // 				i *= 2;
    // 			}
    // 			return {
    // 				type: hpTypes[tr(hpTypeX * 15 / 63)],
    // 				// After Gen 6, Hidden Power is always 60 base power
    // 				power: (this.gen && this.gen < 6) ? tr(hpPowerX * 40 / 63) + 30 : 60,
    // 			};
    // 		}
    // 	}
    //
    pub fn get_hidden_power(ivs: &StatsTable) -> (&'static str, i32) {
        // Gen 3+ formula
        let hp_bits = ivs.hp & 1;
        let atk_bits = (ivs.atk & 1) << 1;
        let def_bits = (ivs.def & 1) << 2;
        let spe_bits = (ivs.spe & 1) << 3;
        let spa_bits = (ivs.spa & 1) << 4;
        let spd_bits = (ivs.spd & 1) << 5;

        let type_num = (hp_bits | atk_bits | def_bits | spe_bits | spa_bits | spd_bits) * 15 / 63;

        let types = [
            "Fighting", "Flying", "Poison", "Ground", "Rock", "Bug", "Ghost", "Steel", "Fire",
            "Water", "Grass", "Electric", "Psychic", "Ice", "Dragon", "Dark",
        ];

        let hp_type = types.get(type_num as usize).unwrap_or(&"Dark");

        // Calculate power (Gen 3-5: 30-70, Gen 6+: always 60)
        let hp2_bits = (ivs.hp >> 1) & 1;
        let atk2_bits = ((ivs.atk >> 1) & 1) << 1;
        let def2_bits = ((ivs.def >> 1) & 1) << 2;
        let spe2_bits = ((ivs.spe >> 1) & 1) << 3;
        let spa2_bits = ((ivs.spa >> 1) & 1) << 4;
        let spd2_bits = ((ivs.spd >> 1) & 1) << 5;

        let power =
            (hp2_bits | atk2_bits | def2_bits | spe2_bits | spa2_bits | spd2_bits) * 40 / 63 + 30;

        (hp_type, power)
    }
}
