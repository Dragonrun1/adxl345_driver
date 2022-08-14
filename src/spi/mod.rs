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

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use crate::{Adxl345, Adxl345Init, Adxl345Reader, Adxl345Writer, AdxlError, AdxlResult, Result};

/// SPI driver structure for the device.
#[derive(Debug)]
pub struct Device {
    /// Holds the bus interface from the [RPPAL SPI] peripheral.
    ///
    /// [RPPAL SPI]: https://docs.golemparts.com/rppal/0.13.1/rppal/spi/index.html
    bus: Spi,
    /// true: SPI 3-wire mode; false: SPI 4-wire mode.
    three_wire: bool,
}

impl Device {
    /// Constructor with default bus parameters.
    ///
    /// bus = 0; slave_select = 0; clock_speed = 1 MHz; 4-wire-SPI.
    pub fn new() -> AdxlResult<Self> {
        Self::with_bus(0, 0, 1_000_000, false)
    }
    /// Constructor with bus index and slave-select index.
    ///
    /// ## Arguments
    /// * `bus` - SPI bus index (0-2).
    /// * `slave_select` - SPI slave-select index (0-2).
    /// * `clock_speed` - SPI clock speed in Hz.
    /// * `three_wire` - true: SPI 3-wire mode; false: SPI 4-wire mode.
    pub fn with_bus(
        bus: u8,
        slave_select: u8,
        clock_speed: u32,
        three_wire: bool,
    ) -> AdxlResult<Self> {
        let bus = match bus {
            0 => Bus::Spi0,
            1 => Bus::Spi1,
            2 => Bus::Spi2,
            /*
            3 => Bus::Spi3,
            4 => Bus::Spi4,
            5 => Bus::Spi5,
            6 => Bus::Spi6,
            */
            _ => return Err(AdxlError::InvalidBusParams),
        };
        let slave_select = match slave_select {
            0 => SlaveSelect::Ss0,
            1 => SlaveSelect::Ss1,
            2 => SlaveSelect::Ss2,
            /*
            3 => SlaveSelect::Ss3,
            4 => SlaveSelect::Ss4,
            5 => SlaveSelect::Ss5,
            6 => SlaveSelect::Ss6,
            7 => SlaveSelect::Ss7,
            8 => SlaveSelect::Ss8,
            9 => SlaveSelect::Ss9,
            10 => SlaveSelect::Ss10,
            11 => SlaveSelect::Ss11,
            12 => SlaveSelect::Ss12,
            13 => SlaveSelect::Ss13,
            14 => SlaveSelect::Ss14,
            15 => SlaveSelect::Ss15,
            */
            _ => return Err(AdxlError::InvalidBusParams),
        };
        let mut device = Device {
            bus: Spi::new(bus, slave_select, clock_speed, Mode::Mode3)?,
            three_wire,
        };
        device.init()?;
        Ok(device)
    }
}

impl Adxl345 for Device {}
impl Adxl345Init for Device {}

impl Adxl345Reader for Device {
    fn access(&self, register: u8) -> AdxlResult<u8> {
        let mut read_buf = [0u8, 0u8];
        debug_assert!(register <= 0x7F);
        let write_buf = [(register & 0x7Fu8) | 0x80u8, 0u8];
        self.bus.transfer(&mut read_buf, &write_buf)?;
        Ok(read_buf[1])
    }
    fn acceleration(&self) -> AdxlResult<(i16, i16, i16)> {
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
        self.bus.transfer(&mut read_buf, &write_buf)?;
        Ok((
            i16::from_le_bytes([read_buf[1], read_buf[2]]),
            i16::from_le_bytes([read_buf[3], read_buf[4]]),
            i16::from_le_bytes([read_buf[5], read_buf[6]]),
        ))
    }
}

impl Adxl345Writer for Device {
    fn command(&mut self, register: u8, byte: u8) -> Result {
        debug_assert!(register <= 0x7F);
        let write_buf = [(register & 0x7Fu8), byte];
        self.bus.write(&write_buf)?;
        Ok(())
    }
    fn init(&mut self) -> Result {
        self.init_registers(self.three_wire)
    }
}
