mod rov;

///
/// This is the simplest ROV type, with a simplified model.
///
/// Instead of getters and setters, tactical information is read directly from the vessel struct.
///
/// All methods on are Basic ROV are blocking calls.  They do not return before their action has been
/// completed and take at least one turn to execute.
///
impl BasicROV for Vessel {
/* 
    pub fn new() 
    pub fn forward(&self, distance: i32, speed: i32)
    pub fn backward(&self, distance: i32, speed: i32)
    pub fn left(&self, angle: i32, speed: i32)
    pub fn right(&self, angle: i32, speed: i32)
    pub fn forward_left(&self, distance: i32, angle: i32, speed: i32)
    pub fn forward_right(&self, distance: i32, angle: i32, speed: i32)
    pub fn backward_left(&self, distance: i32, angle: i32, speed: i32)
    pub fn backward_right(&self, distance: i32, angle: i32, speed: i32)
    pub fn turn_to(&self, angle: i32, speed: i32)

    pub fn gun_left(&self, angle: i32)
    pub fn gun_right(&self, angle: i32)
    pub fn gun_bear_to(&self, angle: i32)
    pub fn gun_to(&self, angle: i32)
    pub fn fire(&self, power: i32)

    pub fn torpedo(&self, Vec<Vec<i32,i32>>, power: i32)
    pub fn depth_charge(&self, power: i32)
    pub fn flak(&self, )

    pub fn nop(&self, turns: i32)

    pub fn toggle_submerge(&self)

    pub fn log(&self)

*/
}
