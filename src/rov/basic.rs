///
/// This is the simplest ROV type, with a simplified model.
///
/// Instead of getters and setters, tactical information is read directly from the vessel struct.
///
/// All methods on are Basic ROV are blocking calls.  They do not return before their action has been
/// completed and take at least one turn to execute.
///
use rov;
use rov::Vessel;
use fringe::generator::Yielder;

pub trait BasicROV {
    fn new(&'static str, rov::Type, &Yielder<(), i32>) -> Self;
    fn forward(&self, i32);
    fn backward(&self, i32);
    fn gun_left(&self, i32);
    fn gun_right(&self, i32);
    fn log(&self);
}

impl BasicROV for Vessel {
    fn new(name: &'static str, rov_type: rov::Type, yielder: &Yielder<(), i32>) -> Self {
        Vessel { name: name.to_string(), rov_type: rov_type }
    }

    fn forward(&self, distance: i32) {

    }

    fn backward(&self, distance: i32) {

    }
//    pub fn left(&self, angle: i32)
//    pub fn right(&self, angle: i32)
//    pub fn forward_left(&self, distance: i32, angle: i32)
//    pub fn forward_right(&self, distance: i32, angle: i32)
//    pub fn backward_left(&self, distance: i32, angle: i32)
//    pub fn backward_right(&self, distance: i32, angle: i32)
//    pub fn turn_to(&self, angle: i32)

    fn gun_left(&self, angle: i32) {

    }
    fn gun_right(&self, angle: i32) {

    }
//    pub fn gun_bear_to(&self, angle: i32)
//    pub fn gun_to(&self, angle: i32)
//    pub fn fire(&self)

//    pub fn torpedo(&self, Vec<Vec<i32,i32>>)
//    pub fn depth_charge(&self)
//    pub fn flak(&self)

//    pub fn nop(&self, turns: i32)

//    pub fn toggle_submerge(&self)

    fn log(&self) {

    }
}
