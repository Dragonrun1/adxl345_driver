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

use crate::{Adxl345, Adxl345AccExtract, Adxl345Init, Adxl345Reader, Adxl345Writer, AdxlError, AdxlResult, Result};
use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};

/// I²C driver structure for the device.
#[derive(Debug)]
pub struct Device<B> {
    /// Any bus object implementing the `embedded_hal::i2c::blocking::I2c` trait.
    bus: B,
    /// The 7 bit address of the device.
    address: SevenBitAddress,
}

impl<B: I2c> Device<B> {
    /// Constructor with default slave address `0x53`.
    ///
    /// ## Arguments
    /// * `bus` - Any object implementing the `embedded_hal::i2c::blocking::I2c` trait.
    pub fn new(bus: B) -> AdxlResult<Self> {
        Self::with_address(bus, 0x53)
    }
    /// Constructor with explicit slave address.
    ///
    /// The device only has two addresses `0x53` or `0x1d` depending on the low or
    /// high logic level on the `ALT ADDRESS` pin.
    ///
    /// ## Arguments
    /// * `bus` - Any object implementing the `embedded_hal::i2c::blocking::I2c` trait.
    /// * `address` - Address of ADXL345 device.
    pub fn with_address(bus: B, address: u8) -> AdxlResult<Self> {
        let mut device = Device { bus, address };
        device.init()?;
        Ok(device)
    }
}

impl<B: I2c> Adxl345 for Device<B> {}
impl<B: I2c> Adxl345Init for Device<B> {}
impl<B: I2c> Adxl345AccExtract for Device<B> {}

impl<B: I2c> Adxl345Reader for Device<B> {
    fn access(&mut self, register: u8) -> AdxlResult<u8> {
        let mut buf = [0u8; 1];
        if let Err(e) = self.bus.write_read(self.address, &[register], &mut buf) {
            return Err(AdxlError::I2c(format!("{:?}", e)));
        }
        Ok(buf[0])
    }
    fn acceleration(&mut self) -> AdxlResult<(i16, i16, i16)> {
        let register = 0x32;
        let mut buf = [0u8; 6];
        if let Err(e) = self.bus.write_read(self.address, &[register], &mut buf) {
            return Err(AdxlError::I2c(format!("{:?}", e)));
        }
        Ok(self.extract_acceleration(&buf))
    }
}

impl<B: I2c> Adxl345Writer for Device<B> {
    fn command(&mut self, register: u8, byte: u8) -> Result {
        if let Err(e) = self.bus.write(self.address, &[register, byte]) {
            return Err(AdxlError::I2c(format!("{:?}", e)));
        }
        Ok(())
    }
    fn init(&mut self) -> Result {
        self.init_registers(false)
    }
}
