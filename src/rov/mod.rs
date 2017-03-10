pub mod basic;

/// Vessel tactical status
#[derive(Default)]
pub struct Tactical {
    /// Current energy of the ROV, where 100 means full energy and 0 means no energy (dead)
    pub energy: i32,
    /// Current flak (torpedo stopper count)
    pub flak: i32,
    /// Current heading angle of this ROV (in degrees)
    pub heading: f32,
    /// Current horizontal location of this ROV
    pub x: usize,
    /// Current vertical location of this ROV
    pub y: usize,

    /// current gun heading of this ROV (in degrees), relative to the body
    pub gun_bearing: f32,
    /// current gun heading of this ROV
    pub gun_heading: f32,
    /// flag specifying if the gun is ready to fire
    pub gun_ready: bool,

    /// latest angle from where this ROV was hit by a bullet (in degrees)
    pub hit_bullet_angle: f32,
    /// latest angle from where this ROV was hit by a bullet (in degrees), relative to the body
    pub hit_bullet_bearing: f32,
    /// latest angle where this ROV was hit by another ROV (in degrees)
    pub hit_robot_angle: f32,
    /// latest angle where this ROV was hit by another ROV (in degrees), relative to the body
    pub hit_robot_bearing: f32,
    /// latest angle where this ROV hit a non-ROV obstruction (in degrees)
    pub hit_obs_angle: f32,
    /// latest angle where this ROV hit a non-ROV obstruction (in degrees), relative to the body
    pub hit_obs_bearing: f32,

    /// current number of other ROVs in the battle
    pub others: usize,
    /// latest data for the nearest ROV scanned by radar
    pub radar: SensorData,
    /// latest data for the nearest ROV scanned by active sonar
    pub active: SensorData,
    /// latest data for the nearest ROV scanned by passive sonar
    pub passive: SensorData,
    /// latest data for the nearest torpedo scanned by either sonar
    pub torpedo: SensorData

}

// TODO: how do I get the ROV to recognize islands?
//  should islands be scanned, or should the ROV know the terrain at start?

// TODO: add in active and passive sonar results
// TODO: add in torpedo tracking

/// Sensor data
#[derive(Default)]
pub struct SensorData {
    /// current angle to the object (in degrees)
    pub angle: f32,
    /// current angle to the object (in degrees), relative to the body
    pub bearing: f32,
    /// current distance to the object
    pub distance: f32,
    /// current heading of the object
    pub heading: f32,
    /// current velocity of the object
    pub velocity: f32
}

/// ROV - either a surface vessel or a submarine
pub enum Type {
    Surface,
    Submarine
}

pub struct Vessel {
    name: String,
    rov_type: Type,
}

/// trait to implement the ROV code
pub trait ROV {
    /// set up ROV
    fn init() -> Self;
    /// Main ROV method
    fn run(&self) -> ();
/*    /// Called by MCP when radar senses a ROV
    fn on_radar_rov() {_};
    /// Called by MCP when either active or passive sonar senses a ROV
/   fn on_sonar_rov() -> () {}
    /// Called by MCP when either active or passive sonar senses a torpedo
    fn on_sonar_torpedo() {};
    /// Called by MCP when ROV is hit by a bullet
    fn on_hit_bullet() {};
    /// Called by MCP when ROV is hit by torpedo blast
    fn on_hit_torpedo() {};
    /// Called by MCP when ROV is hit by depth charge blast
    fn on_hit_charge() {};
    /// Called by MCP when ROV collides with another ROV
    fn on_hit_rov() {};
    /// Called by MCP when ROV collides with wall
    fn on_hit_wall() {};
    /// Called by MCP when ROV collides with a rock
    fn on_hit_rock() {};
*/
}
