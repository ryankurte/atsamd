//! NeoTrellis M4 Express pins

use gpio::{Floating, Input, Port};
use hal::gpio::*;

/// Sets of pins split apart by category
pub struct Sets {
    /// I2C pins
    pub i2c: I2C,

    /// Port
    pub port: Port,
}


/// I2C pins
pub struct I2C {
    pub sda: Pb8<Input<Floating>>,
    pub scl: Pb9<Input<Floating>>,
}
