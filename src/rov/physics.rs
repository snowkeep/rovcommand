
const ACCEL: f32 = 1.0;
const DECEL: f32 = 2.0;
const MAX_SPEED: f32 = 8.0;

const MAX_ROT_GUN: f32 = 20.0;
const MAX_ROT_RADAR: f32 = 45.0;

// min distance which will run for some time at MAXSPEED

pub fn dist_covered(t: f32, dist_tot: f32) -> f32 {
    let break_dist: f32 = (ACCEL/2.0) * (MAX_SPEED/ACCEL).powi(2) + (DECEL/2.0) * (MAX_SPEED/DECEL).powi(2);
    // does not reach max speed
    let x = if dist_tot <= break_dist {
        // time to switch from accel to decelerating
        let t_sp = ((2.0 * dist_tot) / (ACCEL * (1.0 + ACCEL/DECEL))).sqrt();
        // accelerating
        if t <= t_sp {
             (ACCEL / 2.0) * t.powi(2)
        }
        // decelerating
        else {
            dist_tot - (DECEL / 2.0) * (t - t_sp).powi(2)
        }
    }
    else {
        // time to max speed
        let t_sp1 = MAX_SPEED / ACCEL;
        // to to start slowing down
        let t_sp2 = t_sp1 + (dist_tot - break_dist)/MAX_SPEED - MAX_SPEED / DECEL;
        // accelerating
        if t <= t_sp1 {
             ACCEL / 2.0 * t.powi(2)
        }
        // constant speed
        else if t <= t_sp2 {
            MAX_SPEED * t - (ACCEL / 2.0) * t_sp1.powi(2)
        }
        // decelerating
        else {
            dist_tot - (DECEL / 2.0) * (t - t_sp2).powi(2)
        }
    };

    if x > dist_tot {
        dist_tot
    }
    else {
        x
    }
}
