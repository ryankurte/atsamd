# Adafruit NeoTrellis M4 Express Board Support Crate

This crate provides a type-safe Rust API for working with the
[RoboticExplortationLab/Sprite].

![RoboticExplortationLab/Sprite](https://github.com/roboticexplorationlab/sprite)

## Board Features

- Microchip [ATSAMD51G19A] Cortex-M4 microcontroller @ 120 MHz (32-bit, 3.3V logic and power)
  - 512kB Flash
  - 192kB SRAM
- 8 MB SPI Flash chip
- USB device controller (for e.g. MIDI)
- 4-JST hacking port with 3.3V power, ground, and two GPIO (can be I2C/ADC/UART)

## Examples?

Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/kicksat/examples

[ATSAMD51G19A]: https://www.microchip.com/wwwproducts/en/ATSAMD51G19A
