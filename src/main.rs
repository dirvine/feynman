extern crate feynman;

use feynman::eyes;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    let eyes = eyes::Eyes::new().unwrap();
    loop {
        eyes.blue().unwrap();
        sleep(Duration::from_millis(500));
        eyes.green().unwrap();
        sleep(Duration::from_millis(500));
        eyes.red().unwrap();
        sleep(Duration::from_millis(500));
        eyes.closed().unwrap();
        sleep(Duration::from_millis(500));
    }
}
