
/// ROV tactical status
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
    radar: Scan_Data,
    /// latest data for the nearest ROV scanned by active sonar
    active: Scan_Data,
    /// latest data for the nearest ROV scanned by passive sonar
    passive: Scan_Data,
    /// latest data for the nearest torpedo scanned by either sonar
    torpedo: Scan_data

}

// TODO: how do I get the ROV to recognize islands?
//  should islands be scanned, or should the ROV know the terrain at start?

// TODO: add in active and passive sonar results
// TODO: add in torpedo tracking

struct Scan_Data {
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

enum ROV {
    SurfaceVessel(
        tactical: Tactical,
    ),
    Submarine(
        tactical: Tactical,
        submerged: bool = false,
    )
}

impl JuniorROV for ROV {
/* 
    pub fn new() 
    pub fn forward(distance: i32, speed: i32)
    pub fn backward(distance: i32, speed: i32)
    pub fn left(angle: i32, speed: i32)
    pub fn right(angle: i32, speed: i32)
    pub fn forward_left(distance: i32, angle: i32, speed: i32)
    pub fn forward_right(distance: i32, angle: i32, speed: i32)
    pub fn backward_left(distance: i32, angle: i32, speed: i32)
    pub fn backward_right(distance: i32, angle: i32, speed: i32)
    pub fn turn_to(angle: i32, speed: i32)

    pub fn gun_left(angle: i32)
    pub fn gun_right(angle: i32)
    pub fn gun_bear_to(angle: i32)
    pub fn gun_to(angle: i32)
    pub fn fire(power: i32)

    pub fn torpedo(Vec<Vec<i32,i32>>, power: i32)
    pub fn depth_charge(power: i32)

    pub fn nop(turns)

    pub fn toggle_submerge()

    pub fn get_heading
    pub fn get_x
    put fn get_y

    fn on_hit_by_bullet()
    fn on_hit_robot()
    fn on_hit_wall()

    fn on_scanned_robot()

    fn run()

}
