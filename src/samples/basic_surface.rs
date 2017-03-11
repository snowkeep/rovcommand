use rov::{ROV, Vessel, Type};
use rov::basic::BasicROV;

pub const MYTYPE : Type = Type::BasicSurfaceROV;
pub const MYNAME : &'static str = "Basic Surface Vessel";

pub struct BasicSurface {
    pub vessel: Vessel
}

impl ROV for BasicSurface {
    fn init(vessel: Vessel) -> Self {
        BasicSurface { vessel: vessel }
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
