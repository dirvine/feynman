use error::*;

use sysfs_gpio::{Direction, Pin};
use super::*;

/// guess
pub struct Eyes {
    eyes_blue: Pin,
    eyes_green: Pin,
    eyes_red: Pin,
}

impl Eyes {
    /// Set all eyes off and ready eyes
    pub fn new() -> Result<Eyes> {
        let eyes_blue = Pin::new(EYES_BLUE);
        try!(eyes_blue.export().chain_err(|| "could not export pin"));
        try!(eyes_blue.set_direction(Direction::Out).chain_err(|| "could not set pin direction"));
        let eyes_green = Pin::new(EYES_GREEN);
        try!(eyes_green.export().chain_err(|| "could not export pin"));
        try!(eyes_green.set_direction(Direction::Out).chain_err(|| "could not set pin direction"));
        let eyes_red = Pin::new(EYES_RED);
        try!(eyes_red.export().chain_err(|| "could not export pin"));
        try!(eyes_red.set_direction(Direction::Out).chain_err(|| "could not set pin direction"));
        Ok(Eyes {
            eyes_blue: eyes_blue,
            eyes_green: eyes_green,
            eyes_red: eyes_red,
        })
    }
    /// Set blue eye colour
    pub fn blue(&self) -> Result<()> {
        Ok(try!(self.eyes_blue
            .set_value(1)
            .and(self.eyes_green.set_value(0))
            .and(self.eyes_red.set_value(0))
            .chain_err(|| "couldn't set pin for blue eyes")))
    }
    /// Set green eye colour
    pub fn green(&self) -> Result<()> {
        Ok(try!(self.eyes_green
            .set_value(1)
            .and(self.eyes_blue.set_value(0))
            .and(self.eyes_red.set_value(0))
            .chain_err(|| "couldn't set pin for green eyes")))
    }
    /// Set red eye colour
    pub fn red(&self) -> Result<()> {
        Ok(try!(self.eyes_red
            .set_value(1)
            .and(self.eyes_green.set_value(0))
            .and(self.eyes_blue.set_value(0))
            .chain_err(|| "couldn't set pin for red eyes")))
    }
    /// Set red eye colour
    pub fn closed(&self) -> Result<()> {
        Ok(try!(self.eyes_red
            .set_value(0)
            .and(self.eyes_green.set_value(0))
            .and(self.eyes_blue.set_value(0))
            .chain_err(|| "couldn't set pins for eyes closed")))
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;

    #[test]
    fn eyes_basic() {
        let eyes = Eyes::new().unwrap();
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
