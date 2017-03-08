use rov::{Vessel, Type};
use rov::basic::BasicROV;

pub struct BasicSurface {
    pub vessel: Vessel
}

impl BasicSurface {
    pub fn init() -> Self {
        let mut vessel: Vessel = BasicROV::new("Basic Surface Vessel", Type::Surface);
        BasicSurface {
            vessel: vessel
        }
    }

    pub fn run(&self) {
        loop {
            &self.vessel.forward(100);
            &self.vessel.gun_left(360);
            &self.vessel.backward(100);
            &self.vessel.gun_right(360);
        }
    }
}
