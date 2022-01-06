// MIT License
//
// Copyright © 2022, Michael Büsch <m@bues.ch>
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
//! Contains the SPI driver for the device.

use crate::{Adxl345, Adxl345AccExtract, Adxl345Init, Adxl345Reader, Adxl345Writer, AdxlError, AdxlResult, Result};
use embedded_hal::spi::blocking::{Transfer, Write};

/// SPI driver structure for the device.
#[derive(Debug)]
pub struct Device<B> {
    /// Any bus object implementing the `embedded_hal::spi::blocking::{Transfer, Write}` traits.
    bus: B,
    /// true: SPI 3-wire mode; false: SPI 4-wire mode.
    three_wire: bool,
}

impl<B: Transfer + Write> Device<B> {
    /// Constructor.
    ///
    /// ## Arguments
    /// * `bus` - Any bus object implementing the `embedded_hal::spi::blocking::{Transfer, Write}` traits.
    /// * `three_wire` - true: SPI 3-wire mode; false: SPI 4-wire mode.
    pub fn new(bus: B, three_wire: bool) -> AdxlResult<Self> {
        let mut device = Device { bus, three_wire };
        device.init()?;
        Ok(device)
    }
}

impl<B: Transfer + Write> Adxl345 for Device<B> {}
impl<B: Transfer + Write> Adxl345Init for Device<B> {}
impl<B: Transfer + Write> Adxl345AccExtract for Device<B> {}

impl<B: Transfer + Write> Adxl345Reader for Device<B> {
    fn access(&mut self, register: u8) -> AdxlResult<u8> {
        let mut read_buf = [0u8, 0u8];
        debug_assert!(register <= 0x7F);
        let write_buf = [(register & 0x7Fu8) | 0x80u8, 0u8];
        if let Err(e) = self.bus.transfer(&mut read_buf, &write_buf) {
            return Err(AdxlError::Spi(format!("{:?}", e)));
        }
        Ok(read_buf[1])
    }
    fn acceleration(&mut self) -> AdxlResult<(i16, i16, i16)> {
        let register = 0x32;
        let mut read_buf = [0u8; 7];
        debug_assert!(register <= 0x7F);
        let write_buf = [
            (register & 0x7Fu8) | 0x80u8 | 0x40u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
        ];
        if let Err(e) = self.bus.transfer(&mut read_buf, &write_buf) {
            return Err(AdxlError::Spi(format!("{:?}", e)));
        }
        Ok(self.extract_acceleration(&read_buf[1..7]))
    }
}

impl<B: Transfer + Write> Adxl345Writer for Device<B> {
    fn command(&mut self, register: u8, byte: u8) -> Result {
        debug_assert!(register <= 0x7F);
        let write_buf = [(register & 0x7Fu8), byte];
        if let Err(e) = self.bus.write(&write_buf) {
            return Err(AdxlError::Spi(format!("{:?}", e)));
        }
        Ok(())
    }
    fn init(&mut self) -> Result {
        self.init_registers(self.three_wire)
    }
}
