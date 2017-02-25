use rov::Vessel;
use rov::basic::BasicROV;

pub struct BasicSurface {
    rov: Vessel
}

impl BasicSurface {
    pub fn init() -> Self {
        let rov: Vessel = BasicROV::new("Basic Surface Vessel", "surface");
        BasicSurface {
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
