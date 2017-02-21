#[macro_use] extern crate conrod;

use std::thread;
use std::sync::mpsc;

pub mod gui;

fn main() {

    let (log_tx, log_rx) = mpsc::channel();

    let ui = thread::spawn(|| {
        gui::main(log_rx);
    });

    let _ = ui.join();
    println!("Goodbye.");
}
