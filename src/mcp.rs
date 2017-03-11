extern crate rand;
// This is the main simulation engine for rovcommand

use message;
use samples::basic_surface::{self, BasicSurface};

use std::sync::mpsc::Receiver;

use self::rand::Rng;
use fringe::{OsStack, Generator};

use rov;
use rov::ROV;
use rov::Type::*;
use rov::basic::BasicROV;

struct Rov<ROV> {
    vessel: ROV,
    tactical: rov::Tactical
}

pub fn main(ctrl_rx: Receiver<message::Ctrl>) {
    // TODO: set up the playing field
    let area_w: usize = 1024;
    let area_h: usize = 768;


    // get the list of rovs

    // initialize all ROVs into separate co-routines
    // randomize starting locations and headings for rovs
 //   for rov in &rovs {
    let stack = OsStack::new(1 << 16).unwrap();
    let mut gen = Generator::new(stack, move |yielder, ()| {
        let vessel = match basic_surface::MYTYPE {
            BasicSurfaceROV => BasicSurface::init(BasicROV::new(basic_surface::MYNAME, basic_surface::MYTYPE, yielder)),
            _ => panic!("This ROV type not yet implemented")
        };
        let mut irng = rand::thread_rng();
        let mut frng = rand::thread_rng();
        let myRov = Rov {
            vessel: vessel,
            tactical: rov::Tactical {
                x: irng.gen_range(0, area_w),
                y: irng.gen_range(0, area_h),
                heading: frng.gen_range(0.0, 360.0),
                gun_bearing: 0.0,
                ..Default::default()
            }
        };
    });
  //  }


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
