
/// Vessel tactical status
struct Tactical {
    /// Current energy of the ROV, where 100 means full energy and 0 means no energy (dead)
    energy: i32 = 100,
    /// Current flak (torpedo stopper count)
    flak: i32 = 100,
    /// Current heading angle of this ROV (in degrees)
    heading: i32,
    /// Current horizontal location of this ROV
    x: i32,
    /// Current vertical location of this ROV
    y: i32,

    /// current gun heading of this ROV (in degrees), relative to the body
    gun_bearing: i32,
    /// current gun heading of this ROV
    gun_heading: i32,
    /// flag specifying if the gun is ready to fire
    gun_ready: bool

    /// latest angle from where this ROV was hit by a bullet (in degrees)
    hit_bullet_angle: i32,
    /// latest angle from where this ROV was hit by a bullet (in degrees), relative to the body
    hit_bullet_bearing: i32,
    /// latest angle where this ROV was hit by another ROV (in degrees)
    hit_robot_angle: i32,
    /// latest angle where this ROV was hit by another ROV (in degrees), relative to the body
    hit_robot_bearing: i32,
    /// latest angle where this ROV hit a non-ROV obstruction (in degrees)
    hit_obs_angle: i32,
    /// latest angle where this ROV hit a non-ROV obstruction (in degrees), relative to the body
    hit_obs_bearing: i32,

    /// current number of other ROVs in the battle
    others: i32,
    /// latest data for the nearest ROV scanned by radar
    radar: Sensor,
    /// latest data for the nearest ROV scanned by active sonar
    active: Sensor,
    /// latest data for the nearest ROV scanned by passive sonar
    passive: Sensor,
    /// latest data for the nearest torpedo scanned by either sonar
    torpedo: Sensor

}

// TODO: how do I get the ROV to recognize islands?
//  should islands be scanned, or should the ROV know the terrain at start?

// TODO: add in active and passive sonar results
// TODO: add in torpedo tracking

/// Sensor data
struct Sensor_Data {
    /// current angle to the object (in degrees)
    angle: i32,
    /// current angle to the object (in degrees), relative to the body
    bearing: i32,
    /// current distance to the object
    distance: i32,
    /// current heading of the object
    heading: i32,
    /// current velocity of the object
    velocity: i32
}

/// ROV - either a surface vessel or a submarine
enum Vessel {
    Surface(
        name: String,
        tactical: Tactical,
    ),
    Submarine(
        name: String,
        tactical: Tactical,
        submerged: bool = false,
    )
}

/// trait to implement the ROV code
trait ROV {
    /// set up ROV
    fn init() {};
    /// Main ROV method
    fn run();
    /// Called by MCP when radar senses a ROV
    fn on_radar_rov() {};
    /// Called by MCP when either active or passive sonar senses a ROV
    fn on_sonar_rov() {};
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

}
