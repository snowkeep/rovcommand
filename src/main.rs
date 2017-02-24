#[macro_use] extern crate conrod;

use std::thread;
use std::sync::mpsc;

mod gui;
mod mcp;
mod message;

fn main() {

    let (log_tx, log_rx) = mpsc::channel();
    let (ctrl_tx, ctrl_rx) = mpsc::channel();

    let ui = thread::spawn(|| {
        gui::main(log_rx, ctrl_tx);
    });

    let sim = thread::spawn(|| {
        mcp::main(ctrl_rx);
    });

    let _ = ui.join();
    let _ = sim.join();

    println!("Goodbye.");
}
