
// This is the main simulation engine for rovcommand

use message;
use samples;
use samples::basic_surface::Basic_Surface;

use std::sync::mpsc::Receiver;

pub fn main(ctrl_rx: Receiver<message::Ctrl>) {
    // get the list of rovs
    // initialize all ROVs
    // randomize starting locations for rovs
    // run simulation
        // check ROV queue
    let rov1 = samples::basic_surface::Basic_Surface::init();

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
