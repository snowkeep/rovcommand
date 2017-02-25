use rov;
use rov::{Vessel, ROV};
use rov::basic;
use rov::basic::BasicROV;

pub struct Basic_Surface {
    rov: Vessel
}

impl Basic_Surface {
    pub fn init() -> Self {
        let rov: Vessel = BasicROV::new("Basic Surface Vessel", "surface");
        Basic_Surface {
            rov: rov
        }
    }

    fn run(&self) {
        &self.rov.forward(100);
        &self.rov.gun_left(360);
        &self.rov.backward(100);
        &self.rov.gun_right(360);
    }
}
