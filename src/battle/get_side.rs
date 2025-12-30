use crate::*;
use crate::side::Side;

impl Battle {

    /// Get a side by ID
    //
    // 	getSide(sideid: SideID): Side {
    // 		return this.sides[parseInt(sideid[1]) - 1];
    // 	}
    //
    pub fn get_side(&self, side_id: SideID) -> Option<&Side> {
        self.sides.get(side_id.index())
    }
}
