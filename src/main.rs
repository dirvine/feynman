extern crate feynman;

use feynman::eyes;
use std::thread::sleep;
use std::time::Duration;
use std::io::prelude::*;
use std::io::stdout;


fn main() {
    let eyes = eyes::Eyes::new().unwrap();
    loop {
        eyes.blue();
        sleep(Duration::from_millis(500));
        eyes.green();
        sleep(Duration::from_millis(500));
        eyes.red();
        sleep(Duration::from_millis(500));
        eyes.closed();
        sleep(Duration::from_millis(500));
    }
}
