
// This is the main simulation engine for rovcommand

use message;

use std::sync::mpsc::Receiver;

pub fn main(ctrl_rx: Receiver<message::Ctrl>) {

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

}
