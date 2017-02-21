#[macro_use] extern crate conrod;
//#[macro_use] extern crate chan;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

pub mod gui;

fn main() {

    let (log_tx, log_rx) = mpsc::channel();

    let ui = thread::spawn(|| {
        gui::main(log_rx);
    });

    let _ = ui.join();
    println!("Goodbye.");
}
