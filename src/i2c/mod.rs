// MIT License
//
// Copyright © 2020-present, Michael Cummings <mgcummings@yahoo.com>.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//! Contains the I²C driver for the device.

use rppal::i2c::I2c;

use crate::{Adxl345, Adxl345Init, Adxl345Reader, Adxl345Writer, AdxlResult, Result};

/// I²C driver structure for the device.
#[derive(Debug)]
pub struct Device {
    /// Holds the bus interface from the [RPPAL I²C] peripheral.
    ///
    /// [RPPAL I²C]: https://docs.golemparts.com/rppal/0.11.3/rppal/i2c/index.html
    bus: I2c,
}

impl Device {
    /// Constructor
    pub fn new() -> AdxlResult<Self> {
        Self::with_address(0x53)
    }
    /// Constructor with slave address.
    ///
    /// The device only has two addresses 0x53 or 0x1d depending on the low or
    /// high logic level on the `ALT ADDRESS` pin.
    ///
    /// ## Arguments
    /// * `slave` - Address of ADXL345 device.
    pub fn with_address(slave: u16) -> AdxlResult<Self> {
        let mut device = Device { bus: I2c::new()? };
        device.bus.set_slave_address(slave)?;
        device.init()?;
        Ok(device)
    }
}

impl Adxl345 for Device {}
impl Adxl345Init for Device {}

impl Adxl345Reader for Device {
    fn access(&self, register: u8) -> AdxlResult<u8> {
        let buf = &mut [0u8; 1];
        self.bus.block_read(register, buf)?;
        Ok(buf[0])
    }
    fn acceleration(&self) -> AdxlResult<(i16, i16, i16)> {
        let register = 0x32;
        let buf = &mut [0u8; 6];
        self.bus.block_read(register, buf)?;
        Ok((
            i16::from_le_bytes([buf[0], buf[1]]),
            i16::from_le_bytes([buf[2], buf[3]]),
            i16::from_le_bytes([buf[4], buf[5]]),
        ))
    }
}

impl Adxl345Writer for Device {
    fn command(&mut self, register: u8, byte: u8) -> Result {
        self.bus.block_write(register, &[byte])?;
        Ok(())
    }
    fn init(&mut self) -> Result {
        self.init_registers(false)
    }
}
