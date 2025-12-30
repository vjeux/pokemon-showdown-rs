use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Sort the queue by priority
    /// Order: order (lower first), priority (higher first), speed (higher first)
    //
    // 	sort() {
    // 		// this.log.push('SORT ' + this.debugQueue());
    // 		this.battle.speedSort(this.list);
    // 		return this;
    // 	}
    //
    pub fn sort(&mut self) {
        self.list.sort_by(|a, b| {
            // Order: lower first
            let order_cmp = a.order().cmp(&b.order());
            if order_cmp != std::cmp::Ordering::Equal {
                return order_cmp;
            }

            // Priority: higher first
            let priority_cmp = b.priority().cmp(&a.priority());
            if priority_cmp != std::cmp::Ordering::Equal {
                return priority_cmp;
            }

            // Fractional priority: higher first
            let frac_a = a.fractional_priority();
            let frac_b = b.fractional_priority();
            if frac_a != frac_b {
                return frac_b
                    .partial_cmp(&frac_a)
                    .unwrap_or(std::cmp::Ordering::Equal);
            }

            // Speed: higher first
            b.speed().cmp(&a.speed())
        });
    }
}
