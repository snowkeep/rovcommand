use rov::{ROV, Vessel, Type};
use rov::basic::BasicROV;

pub const MYTYPE : Type = Type::BasicSurfaceROV;
pub const MYNAME : &'static str = "Basic Surface Vessel";

pub struct BasicSurface<'a> {
    pub vessel: Vessel<'a>
}

impl<'a> ROV for BasicSurface<'a> {
    fn init(vessel: Vessel) -> Self {
        BasicSurface { vessel: vessel }
    }

    fn run(&self) {
        loop {
            &self.vessel.forward(100.0);
            &self.vessel.gun_left(360.0);
            &self.vessel.backward(100.0);
            &self.vessel.gun_right(360.0);
        }
    }
}
