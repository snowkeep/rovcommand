#[macro_use] extern crate conrod;
#[macro_use] extern crate chan;

use std::thread;

pub mod gui;

fn main() {
    
    let (log_send, log_recv) = chan::sync(0);

    let ui = thread::spawn(|| {
        gui::main(log_recv);
    });

    let _ = ui.join();
    println!("Goodbye.");
}


