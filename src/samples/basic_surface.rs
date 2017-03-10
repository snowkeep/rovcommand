use rov::{ROV, Vessel, Type};
use rov::basic::BasicROV;

pub struct BasicSurface {
    pub vessel: Vessel
}

impl ROV for BasicSurface {
    fn init() -> Self {
        let mut vessel: Vessel = BasicROV::new("Basic Surface Vessel", Type::Surface);
        BasicSurface {
            vessel: vessel
        }
    }

    fn run(&self) {
        loop {
            &self.vessel.forward(100);
            &self.vessel.gun_left(360);
            &self.vessel.backward(100);
            &self.vessel.gun_right(360);
        }
    }
}
