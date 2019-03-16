#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

#[cfg(feature = "keypad-unproven")]
#[macro_use]
pub extern crate keypad;

pub mod pins;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use hal::{*, atsamd51g19a::*};

use gpio::{Floating, Input, Port};

use hal::prelude::*;
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster4, PadPin};
use hal::time::Hertz;
#[cfg(feature = "keypad-unproven")]
use hal::gpio::{OpenDrain, Output, PullUp};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd51g19a,

    // I2C
    pin sda = a16,
    pin scl = a17,

    pin led = b3,

);
