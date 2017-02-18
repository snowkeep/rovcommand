#[macro_use] extern crate conrod;

use std::thread;

pub mod gui;

fn main() {
    let ui = thread::spawn(|| {
        gui::main();
    });

    ui.join();
    println!("Goodbye.");
}


