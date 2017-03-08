extern crate rand;
extern crate fringe;
// This is the main simulation engine for rovcommand

use message;
use samples::basic_surface::BasicSurface;

use std::sync::mpsc::Receiver;

use self::rand::Rng;
use self::fringe::{OwnedStack, Generator};

pub fn main(ctrl_rx: Receiver<message::Ctrl>) {
    // TODO: set up the playing field
    let area_w: usize = 1024;
    let area_h: usize = 768;

    let mut irng = rand::thread_rng();
    let mut frng = rand::thread_rng();

    // get the list of rovs
    let mut rovs = Vec::new();
    // initialize all ROVs
    rovs.push(BasicSurface::init());

    // randomize starting locations and headings for rovs
    for mut rov in &rovs {
        rov.vessel.tactical.x = irng.gen_range(0, area_w);
        rov.vessel.tactical.y = irng.gen_range(0, area_h);
        rov.vessel.tactical.heading = frng.gen_range(0.0, 360.0);
        rov.vessel.tactical.gun_bearing = 0.0;
        rov.vessel.tactical.gun_heading = rov.vessel.tactical.heading.clone();

        rov.vessel.tactical.others = rovs.len();
    }



    // run simulation
        // check ROV queue

//    let rov1stack = OwnedStack::new(1 << 16).unwrap();
 //   let mut rov1run = Generator::new(rov1stack, move yielder, ()| {
  //      rov1.run();
//    })
/*
    'main: loop {
        let ctrl = ctrl_rx.try_recv();
        if ctrl.is_ok() {
            match ctrl.unwrap() {
                message::Ctrl::Run     => println!("run"),
                message::Ctrl::Pause   => println!("pause"),
                message::Ctrl::Step    => println!("step"),
                message::Ctrl::Restart => println!("restart")
            }
        }

    }
*/

}
