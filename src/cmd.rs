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
//! Contains the Analog Device ADXL345 3-Axis Digital Accelerometer register
//! command set traits and associated parameter types.
//!
//! Control set traits are based on the [ADXL345 Datasheet] information.
//!
//! This is quick reference table for register address, datasheet name, and
//! method names which may be helpful during application/driver development.
//!
//! | Register    | Datasheet Name  | Reader Method        | Writer Method            |
//! | ----------: | :-------------: | -------------------- | ------------------------ |
//! |        0x00 | DEVID           | device_id            | ___READ-ONLY REGISTER___ |
//! | 0x01 - 0x1c | ___RESERVED___  | ___n/a___            | ___n/a___                |
//! |        0x1d | THRESH_TAP      | tap_threshold        | set_tap_threshold        |
//! |        0x1e | OFSX            | x_offset             | set_x_offset             |
//! |        0x1f | OFSY            | y_offset             | set_y_offset             |
//! |        0x20 | OFSZ            | z_offset             | set_z_offset             |
//! |        0x21 | DUR             | tap_duration         | set_tap_duration         |
//! |        0x22 | Latent          | tap_latent           | set_tap_latent           |
//! |        0x23 | Window          | tap_window           | set_tap_window           |
//! |        0x24 | THRESH_ACT      | activity_threshold   | set_activity_threshold   |
//! |        0x25 | THRESH_INACT    | inactivity_threshold | set_inactivity_threshold |
//! |        0x26 | TIME_INACT      | tap_inactivity_time  | set_inactivity_time      |
//! |        0x27 | ACT_INACT_CTL   | activity_control     | set_activity_control     |
//! |        0x28 | THRESH_FF       | free_fall_threshold  | set_free_fall_threshold  |
//! |        0x29 | TIME_FF         | free_fall_time       | set_free_fall_time       |
//! |        0x2a | TAP_AXES        | tap_control          | set_tap_control          |
//! |        0x2b | ACT_TAP_STATUS  | activity_tap_status  | ___READ-ONLY REGISTER___ |
//! |        0x2c | BW_RATE         | bandwidth_rate       | set_bandwidth_rate       |
//! |        0x2d | POWER_CTL       | power_control        | set_power_control        |
//! |        0x2e | INT_ENABLE      | interrupt_control    | set_interrupt_control    |
//! |        0x2f | INT_MAP         | interrupt_map        | set_interrupt_map        |
//! |        0x30 | INT_SOURCE      | interrupt_source     | ___READ-ONLY REGISTER___ |
//! |        0x31 | DATA_FORMAT     | data_format          | set_data_format          |
//! | 0x32 - 0x37 | DATAX0 - DATAZ1 | acceleration         | ___READ-ONLY REGISTER___ |
//! |        0x38 | FIFO_CTL        | fifo_control         | set_fifo_control         |
//! |        0x39 | FIFO_STATUS     | fifo_status          | ___READ-ONLY REGISTER___ |
//!
//! [ADXL345 Datasheet]: https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL345.pdf

use crate::{AdxlError, AdxlResult, Result};
use core::convert::{TryFrom, TryInto};

/// Complete R/W register command set for the accelerometer.
pub trait Adxl345: Adxl345Reader + Adxl345Writer {}

/// Read register command set for accelerometer.
pub trait Adxl345Reader {
    //
    // ## Per driver required stuff ##
    //
    /// Provides access to individual register values.
    ///
    /// This is __NOT__ part of the actual ADXL345 command register set but a
    /// necessary method to interface with all drivers.
    ///
    /// Provides a place for drivers to do any needed shared command processing.
    ///
    /// The implementation of most other methods in the command set will use
    /// this trait after doing any per command processing.
    ///
    /// ## Arguments
    /// * `register` - Register address to be accessed (read).
    ///
    fn access(&self, register: u8) -> AdxlResult<u8>;
    /// Access the 3-axis of acceleration data together.
    fn acceleration(&self) -> AdxlResult<(i16, i16, i16)>;
    //
    // ## Shouldn't be a need to change these methods in driver implementations. ##
    //
    // ### Convenience methods which allow accessing registers in related sets.
    //
    /// Access the current free-fall threshold and time values.
    fn free_fall(&self) -> AdxlResult<(u8, u8)> {
        Ok((self.free_fall_threshold()?, self.free_fall_time()?))
    }
    /// Access all 3-axis of the offset adjustments.
    fn offset_adjustment(&self) -> AdxlResult<(i8, i8, i8)> {
        Ok((self.x_offset()?, self.y_offset()?, self.z_offset()?))
    }
    /// Access to all non-control tap current values together as a structure.
    ///
    /// See [Tap] for more information.
    ///
    /// [Tap]: struct.Tap.html
    fn tap(&self) -> AdxlResult<Tap> {
        let values = [
            self.tap_threshold()?,
            self.tap_duration()?,
            self.tap_latency()?,
            self.tap_window()?,
        ];
        Ok(values.into())
    }
    //
    // ### Per register access methods.
    //
    /// Access the current axis activity/inactivity control mode.
    fn activity_control(&self) -> AdxlResult<ActivityMode> {
        let register = 0x27;
        let data = self.access(register)?;
        let result = ActivityMode::from_bits(data).ok_or(AdxlError::UnknownModeBit(data))?;
        Ok(result)
    }
    /// Access the current activity threshold value.
    fn activity_threshold(&self) -> AdxlResult<u8> {
        let register = 0x24;
        self.access(register)
    }
    /// Access the cause of tap or activity event (interrupt).
    ///
    /// ___Note:___ _The register value should be read before clearing the
    /// interrupt._
    ///
    /// Disabling an axis from participation clears the corresponding source bit
    /// when the next activity or single tap/double tap event occurs.
    fn activity_tap_status(&self) -> AdxlResult<ATStatus> {
        let register = 0x2b;
        let data = self.access(register)?;
        let result = ATStatus::from_bits(data).ok_or(AdxlError::UnknownModeBit(data))?;
        Ok(result)
    }
    /// Access the current data rate and power mode control mode.
    fn bandwidth_rate(&self) -> AdxlResult<BandwidthRateControl> {
        let register = 0x2c;
        self.access(register)?.try_into()
    }
    /// Access the current data format mode.
    fn data_format(&self) -> AdxlResult<DataFormat> {
        let register = 0x31;
        self.access(register)?.try_into()
    }
    /// Access the device ID.
    fn device_id(&self) -> AdxlResult<u8> {
        let register = 0x00;
        self.access(register)
    }
    /// Access the current free-fall threshold value.
    fn free_fall_threshold(&self) -> AdxlResult<u8> {
        let register = 0x28;
        self.access(register)
    }
    /// Access the current free-fall threshold value.
    fn free_fall_time(&self) -> AdxlResult<u8> {
        let register = 0x29;
        self.access(register)
    }
    /// Access the current fifo control mode.
    fn fifo_control(&self) -> AdxlResult<FifoControl> {
        let register = 0x38;
        Ok(self.access(register)?.into())
    }
    /// Access the current fifo status.
    fn fifo_status(&self) -> AdxlResult<FifoStatus> {
        let register = 0x39;
        self.access(register)?.try_into()
    }
    /// Access the current inactivity threshold value.
    fn inactivity_threshold(&self) -> AdxlResult<u8> {
        let register = 0x25;
        self.access(register)
    }
    /// Access the current inactivity time value.
    fn inactivity_time(&self) -> AdxlResult<u8> {
        let register = 0x26;
        self.access(register)
    }
    /// Access the current interrupt control mode.
    fn interrupt_control(&self) -> AdxlResult<IntControlMode> {
        let register = 0x2e;
        let data = self.access(register)?;
        let result = IntControlMode::from_bits(data).ok_or(AdxlError::UnknownModeBit(data))?;
        Ok(result)
    }
    /// Access the current interrupt mapping mode.
    fn interrupt_map(&self) -> AdxlResult<IntMapMode> {
        let register = 0x2f;
        let data = self.access(register)?;
        let result = IntMapMode::from_bits(data).ok_or(AdxlError::UnknownModeBit(data))?;
        Ok(result)
    }
    /// Access the current interrupt source.
    fn interrupt_source(&self) -> AdxlResult<IntSource> {
        let register = 0x30;
        let data = self.access(register)?;
        let result = IntSource::from_bits(data).ok_or(AdxlError::UnknownModeBit(data))?;
        Ok(result)
    }
    /// Access the current power-saving features control mode.
    fn power_control(&self) -> AdxlResult<PowerControl> {
        let register = 0x2d;
        self.access(register)?.try_into()
    }
    /// Access the current tap control mode.
    fn tap_control(&self) -> AdxlResult<TapMode> {
        let register = 0x2a;
        let data = self.access(register)?;
        let result = TapMode::from_bits(data).ok_or(AdxlError::UnknownModeBit(data))?;
        Ok(result)
    }
    /// Access the current duration value for tap interrupts.
    fn tap_duration(&self) -> AdxlResult<u8> {
        let register = 0x21;
        self.access(register)
    }
    /// Access the current latency value for tap interrupts.
    fn tap_latency(&self) -> AdxlResult<u8> {
        let register = 0x22;
        self.access(register)
    }
    /// Access the current threshold value for tap interrupts.
    fn tap_threshold(&self) -> AdxlResult<u8> {
        let register = 0x1d;
        self.access(register)
    }
    /// Access the current window value for tap interrupts.
    fn tap_window(&self) -> AdxlResult<u8> {
        let register = 0x23;
        self.access(register)
    }
    /// Access the current x-axis offset adjustment value.
    fn x_offset(&self) -> AdxlResult<i8> {
        let register = 0x1e;
        Ok(self.access(register)? as i8)
    }
    /// Access the current y-axis offset adjustment value.
    fn y_offset(&self) -> AdxlResult<i8> {
        let register = 0x1f;
        Ok(self.access(register)? as i8)
    }
    /// Access the current z-axis offset adjustment value.
    fn z_offset(&self) -> AdxlResult<i8> {
        let register = 0x20;
        Ok(self.access(register)? as i8)
    }
}

/// Write register command set for accelerometer.
pub trait Adxl345Writer {
    //
    // ## Per driver required stuff ##
    //
    /// Provides an interface to send commands through the ADXL345 driver.
    ///
    /// This is __NOT__ part of the actual ADXL345 command register set but a
    /// necessary method to interface with all drivers.
    ///
    /// Provides a place for drivers to do any needed shared command processing.
    ///
    /// The implementation of most other methods in the command set will use
    /// this trait after doing any per command processing.
    ///
    /// ## Arguments
    /// * `register` - Register address to be written.
    /// * `byte` - Byte of data to be written into the given register.
    fn command(&mut self, register: u8, byte: u8) -> Result;
    /// Used to initialize the accelerometer into a know state.
    ///
    /// Typically this will be only be called from a `new()` method of the
    /// implementation but there can be rare times when the library user needs
    /// access after instance has been created.
    /// Think of it as a soft/warm reset.
    fn init(&mut self) -> Result;
    //
    // ## Shouldn't be a need to change these methods in driver implementations. ##
    //
    // ### Convenience methods which allow setting registers in related sets.
    //
    /// Used to set the threshold for detecting activity.
    ///
    /// ## Arguments
    /// * `thresh` - Threshold value for detecting activity.
    /// The scale factor is 62.5 mg/LSB.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the inactivity interrupt is enabled._
    /// * `time` - Time value representing the amount of time that acceleration
    /// must be less than the value in `thresh` for inactivity to be declared.
    /// The scale factor is 1 sec/LSB.
    fn set_inactivity(&mut self, thresh: u8, time: u8) -> Result {
        self.set_inactivity_threshold(thresh)?;
        self.set_inactivity_time(time)
    }
    /// Used to set threshold and time values for free-fall detection.
    ///
    /// ## Arguments
    /// * `thresh` -  The threshold value for free-fall detection.
    /// The scale factor is 62.5 mg/LSB.
    /// Recommended values between 300mg and 600mg (0x14 - 0x46).
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the free-fall interrupt is enabled._
    /// * `time` - Time value representing the minimum time that the value of
    /// all axes must be less than `thresh` to generate a free-fall interrupt.
    /// The scale factor is 5 ms/LSB.
    /// Recommended values between 100ms and 350ms (0x14 - 0x46).
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the free-fall interrupt is enabled._
    fn set_free_fall(&mut self, thresh: u8, time: u8) -> Result {
        self.set_free_fall_threshold(thresh)?;
        self.set_free_fall_time(time)
    }
    //
    // ### Per register access methods.
    //
    /// Set activity/inactivity control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Activity mode bit flags.
    /// See [ActivityMode] bit flags for more info.
    ///
    /// [ActivityMode]: struct.ActivityMode.html
    fn set_activity_control<AM>(&mut self, mode: AM) -> Result
    where
        AM: Into<ActivityMode>,
    {
        let register = 0x27;
        self.command(register, mode.into().bits())
    }
    /// Set the activity threshold.
    ///
    /// ## Arguments
    /// * `thresh` - Threshold value for detecting activity.
    /// The scale factor is 62.5 mg/LSB.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the activity interrupt is enabled._
    fn set_activity_threshold(&mut self, thresh: u8) -> Result {
        let register = 0x24;
        self.command(register, thresh)
    }
    /// Set data rate and power mode control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Data rate and power mode bit flags.
    /// See [BandwidthRateControl] bit flags for more info.
    ///
    /// [BandwidthRateControl]: struct.BandwidthRateControl.html
    fn set_bandwidth_rate<BRC>(&mut self, mode: BRC) -> Result
    where
        BRC: TryInto<BandwidthRateControl, Error = AdxlError>,
    {
        let register = 0x2c;
        self.command(register, mode.try_into()?.byte[0])
    }
    /// Set data format mode options.
    ///
    /// ## Arguments
    /// * `mode` - Data format mode bit flags.
    /// See [DataFormat] bit flags for more info.
    ///
    /// [DataFormat]: struct.DataFormat.html
    fn set_data_format<DF>(&mut self, mode: DF) -> Result
    where
        DF: TryInto<DataFormat, Error = AdxlError>,
    {
        let register = 0x31;
        self.command(register, mode.try_into()?.byte[0])
    }
    /// Set the free-fall threshold.
    ///
    /// ## Arguments
    /// * `thresh` - Threshold value for detecting activity.
    /// The scale factor is 62.5 mg/LSB.
    /// Values between 300 mg and 600 mg(0x05 to 0x09) are recommended.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the free-fall interrupt is enabled._
    fn set_free_fall_threshold(&mut self, thresh: u8) -> Result {
        let register = 0x28;
        self.command(register, thresh)
    }
    /// Set the free-fall time.
    ///
    /// ## Arguments
    /// * `time` - Time value representing the minimum amount of time that
    /// acceleration must be less than the value in the free-fall threshold
    /// register for a free-fall interrupt to be generated.
    /// The scale factor is 5 ms/LSB.
    /// Values between 100 ms and 350 ms (0x14 to 0x46) are recommended.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the free-fall interrupt is enabled._
    fn set_free_fall_time(&mut self, time: u8) -> Result {
        let register = 0x29;
        self.command(register, time)
    }
    /// Set fifo control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Fifo control mode bit flags.
    /// See [FifoControl] bit flags for more info.
    ///
    /// [FifoControl]: struct.FifoControl.html
    fn set_fifo_control<FC>(&mut self, mode: FC) -> Result
    where
        FC: Into<FifoControl>,
    {
        let register = 0x38;
        self.command(register, mode.into().byte[0])
    }
    /// Set the inactivity threshold.
    ///
    /// ## Arguments
    /// * `thresh` - Threshold value for detecting activity.
    /// The scale factor is 62.5 mg/LSB.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the inactivity interrupt is enabled._
    fn set_inactivity_threshold(&mut self, thresh: u8) -> Result {
        let register = 0x25;
        self.command(register, thresh)
    }
    /// Set the inactivity time.
    ///
    /// ## Arguments
    /// * `time` - Time value representing the amount of time that acceleration
    /// must be less than the value in the inactivity threshold register for
    /// inactivity to be declared.
    /// The scale factor is 1 sec/LSB.
    /// ___Note:___ _that a value of 0 results in an interrupt when the output
    /// data is less than the threshold._
    fn set_inactivity_time(&mut self, time: u8) -> Result {
        let register = 0x26;
        self.command(register, time)
    }
    /// Set interrupt control enable options.
    ///
    /// ## Arguments
    /// * `mode` - Interrupt control mode bit flags.
    /// See [IntControlMode] bit flags for more info.
    ///
    /// [IntControlMode]: struct.IntControlMode.html
    fn set_interrupt_control<IC>(&mut self, mode: IC) -> Result
    where
        IC: Into<IntControlMode>,
    {
        let register = 0x2e;
        self.command(register, mode.into().bits())
    }
    /// Set interrupt mapping mode options.
    ///
    /// ## Arguments
    /// * `mode` - Interrupt mapping mode bit flags.
    /// See [IntMapMode] bit flags for more info.
    ///
    /// [IntMapMode]: struct.IntMapMode.html
    fn set_interrupt_map<IM>(&mut self, mode: IM) -> Result
    where
        IM: Into<IntMapMode>,
    {
        let register = 0x2f;
        self.command(register, mode.into().bits())
    }
    /// Use to set one or more axis offset adjustments.
    ///
    /// ## Arguments
    /// * `x` - X-axis offset adjustment value in twos complement format
    /// with a scale factor of 15.6 mg/LSB.
    /// Automatically added to the acceleration data before storing in the data
    /// register.
    /// A `None` value leaves the existing offset adjustment unchanged.
    /// * `y` - Y-axis offset adjustment value in twos complement format
    /// with a scale factor of 15.6 mg/LSB.
    /// Automatically added to the acceleration data before storing in the data
    /// register.
    /// A `None` value leaves the existing offset adjustment unchanged.
    /// * `z` - Z-axis offset adjustment value in twos complement format
    /// with a scale factor of 15.6 mg/LSB.
    /// Automatically added to the acceleration data before storing in the data
    /// register.
    /// A `None` value leaves the existing offset adjustment unchanged.
    fn set_offset_adjustment<X, Y, Z>(&mut self, x: X, y: Y, z: Z) -> Result
    where
        X: Into<Option<i8>>,
        Y: Into<Option<i8>>,
        Z: Into<Option<i8>>,
    {
        let x = x.into();
        let y = y.into();
        let z = z.into();
        if let Some(x) = x {
            self.set_x_offset(x)?
        };
        if let Some(y) = y {
            self.set_x_offset(y)?
        };
        if let Some(z) = z {
            self.set_x_offset(z)?
        };
        Ok(())
    }
    /// Set power-saving features control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Power-saving features bit flags.
    /// See [PowerControl] bit flags for more info.
    ///
    /// [PowerControl]: struct.PowerControl.html
    fn set_power_control<PC>(&mut self, mode: PC) -> Result
    where
        PC: TryInto<PowerControl, Error = AdxlError>,
    {
        let register = 0x2d;
        self.command(register, mode.try_into()?.byte[0])
    }
    /// Set all non-control tap related values at the same time.
    ///
    /// ## Arguments
    /// * `tap` - Containing values for `threshold`, `duration`, `latency`, and
    /// `window` registers.
    fn set_tap<T>(&mut self, tap: T) -> Result
    where
        T: Into<Tap>,
    {
        let tap = tap.into();
        self.set_tap_threshold(tap.threshold)?;
        self.set_tap_duration(tap.duration)?;
        self.set_tap_latency(tap.latency)?;
        self.set_tap_window(tap.window)
    }
    /// Set tap control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Tab mode bit flags.
    /// See [TapMode] bit flags for more info.
    ///
    /// [TapMode]: struct.TapMode.html
    fn set_tap_control<TM>(&mut self, mode: TM) -> Result
    where
        TM: Into<TapMode>,
    {
        let register = 0x2a;
        self.command(register, mode.into().bits())
    }
    /// Set required duration required to qualify a tap event vs double tap event.
    ///
    /// ## Arguments
    /// `duration` -  Time value representing the maximum time that an event
    /// must be above the threshold to qualify as a tap event.
    /// The scale factor is 625 μs/LSB.
    /// A value of 0 disables the single/double tap functions.
    fn set_tap_duration(&mut self, duration: u8) -> Result {
        let register = 0x21;
        self.command(register, duration)
    }
    /// Set latency for double tap events.
    ///
    /// ## Arguments
    /// `latency` -  Time value representing the wait time from the detection of
    /// a tap event to the start of the time window during which a possible
    /// second tap event can be detected.
    ///
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    fn set_tap_latency(&mut self, latency: u8) -> Result {
        let register = 0x22;
        self.command(register, latency)
    }
    /// Set threshold for tap events.
    ///
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the single tap/double tap interrupt(s) are enabled._
    ///
    /// ## Arguments
    /// `threshold` -  Threshold value for tap interrupts.
    /// The scale factor is 62.5 mg/LSB.
    fn set_tap_threshold(&mut self, threshold: u8) -> Result {
        let register = 0x1d;
        self.command(register, threshold)
    }
    /// Set window for double tap events.
    ///
    /// ## Arguments
    /// `window` -  Time value representing the amount of time after the
    /// expiration of the latency time during which a second valid tap can begin.
    ///
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    fn set_tap_window(&mut self, window: u8) -> Result {
        let register = 0x23;
        self.command(register, window)
    }
    /// Set the x-axis offset adjustment.
    ///
    /// ## Arguments
    /// `x` - Offset adjustment in two's complement format.
    /// The scale factor is 15.6 mg/LSB.
    fn set_x_offset(&mut self, x: i8) -> Result {
        let register = 0x1e;
        self.command(register, x as u8)
    }
    /// Set the y-axis offset adjustment.
    ///
    /// ## Arguments
    /// `y` - Offset adjustment in two's complement format.
    /// The scale factor is 15.6 mg/LSB.
    fn set_y_offset(&mut self, y: i8) -> Result {
        let register = 0x1f;
        self.command(register, y as u8)
    }
    /// Set the z-axis offset adjustment.
    ///
    /// ## Arguments
    /// `z` - Offset adjustment in two's complement format.
    /// The scale factor is 15.6 mg/LSB.
    fn set_z_offset(&mut self, z: i8) -> Result {
        let register = 0x20;
        self.command(register, z as u8)
    }
}

pub(crate) trait Adxl345Init: Adxl345Writer {
    fn init_registers(&mut self, spi_3wire: bool) -> Result {
        let register = 0x31;
        self.command(register, if spi_3wire { 1 << 6 } else { 0 })?;
        for register in 0x1du8..=0x2a {
            self.command(register, 0)?;
        }
        let register = 0x2c;
        self.command(register, 0x0a)?;
        for register in 0x2du8..=0x2f {
            self.command(register, 0)?;
        }
        let register = 0x38;
        self.command(register, 0)?;
        Ok(())
    }
}

// Activity/Inactivity control mode.
bitflags! {
    /// Activity mode bit flags used in [activity_control()] and
    /// [set_activity_control()] methods.
    ///
    /// [activity_control()]: trait.Adxl345Reader.html#method.activity_control
    /// [set_activity_control()]: trait.Adxl345Writer.html#method.set_activity_control
    #[derive(Default)]
    pub struct ActivityMode: u8 {
        /// Select activity AC-coupled operation.
        const ACT_AC = 0x80;
        /// Select activity DC-coupled operation.
        const ACT_DC = 0x00;
        /// Enable X-axis in detecting activity.
        const ACT_X_ENABLE = 0x40;
        /// Disable X-axis in detecting activity.
        const ACT_X_DISABLE = 0x00;
        /// Enable Y-axis in detecting activity.
        const ACT_Y_ENABLE = 0x20;
        /// Disable Y-axis in detecting activity.
        const ACT_Y_DISABLE = 0x00;
        /// Enable Z-axis in detecting activity.
        const ACT_Z_ENABLE = 0x10;
        /// Disable Z-axis in detecting activity.
        const ACT_Z_DISABLE = 0x00;
        /// Select inactivity AC-coupled operation.
        const INACT_AC = 0x08;
        /// Select inactivity DC-coupled operation.
        const INACT_DC = 0x00;
        /// Enable X-axis in detecting inactivity.
        const INACT_X_ENABLE = 0x04;
        /// Disable X-axis in detecting inactivity.
        const INACT_X_DISABLE = 0x00;
        /// Enable Y-axis in detecting inactivity.
        const INACT_Y_ENABLE = 0x02;
        /// Disable Y-axis in detecting inactivity.
        const INACT_Y_DISABLE = 0x00;
        /// Enable Z-axis in detecting inactivity.
        const INACT_Z_ENABLE = 0x01;
        /// Disable Z-axis in detecting inactivity.
        const INACT_Z_DISABLE = 0x00;
    }
}

// Activity/tap status.
bitflags! {
    /// Activity/Tap Status bit flags returned by [activity_tap_status()] method.
    ///
    /// The register should be read before clearing the interrupt.
    ///
    /// [activity_tap_status()]: trait.Adxl345Reader.html#method.activity_tap_status
    pub struct ATStatus: u8 {
        /// Indicate the X-axis is involved in the activity event.
        const ACT_X = 0x40;
        /// Indicate the Y-axis is involved in the activity event.
        const ACT_Y = 0x20;
        /// Indicate the Z-axis is involved in the activity event.
        const ACT_Z = 0x10;
        /// Indicates if the part is asleep or not.
        const ASLEEP = 0x08;
        /// Indicate the X-axis is involved in the tap event.
        const TAP_X = 0x04;
        /// Indicate the Y-axis is involved in the tap event.
        const TAP_Y = 0x02;
        /// Indicate the Z-axis is involved in the tap event.
        const TAP_Z = 0x01;
    }
}

/// Bandwidth rate control bitfields used in [bandwidth_rate()] and
/// [set_bandwidth_rate()] methods.
///
/// [bandwidth_rate()]: trait.Adxl345Reader.html#method.bandwidth_rate
/// [set_bandwidth_rate()]: trait.Adxl345Writer.html#method.set_bandwidth_rate
#[repr(C, align(1))]
#[derive(BitfieldStruct, Clone, Copy)]
pub struct BandwidthRateControl {
    /// Bit fields:
    /// * `low_power` - (Bit 4) Selects reduced power operation, which has
    /// somewhat higher noise level.
    /// * `rate` - (Bits 0-3) Select the device bandwidth and output data rate.
    /// See the table below for more information.
    ///
    /// Power mode table:
    ///
    /// | Data  Rate (Hz) | Bandwidth (Hz) | Rate Code (bin) | Low Power? |
    /// | --------------: | -------------: | --------------: | :--------: |
    /// |         3200    |        1600    |            1111 | No         |
    /// |         1600    |         800    |            1110 | No         |
    /// |          800    |         400    |            1101 | No         |
    /// |          400    |         200    |            1100 | __Yes__    |
    /// |          200    |         100    |            1011 | __Yes__    |
    /// |          100    |          50    |            1010 | __Yes__    |
    /// |           50    |          25    |            1001 | __Yes__    |
    /// |           25    |          12.5  |            1000 | __Yes__    |
    /// |           12.5  |           6.25 |            0111 | __Yes__    |
    /// |           6.25  |           3.13 |            0110 | No         |
    /// |           3.13  |           1.56 |            0101 | No         |
    /// |           1.56  |           0.78 |            0100 | No         |
    /// |           0.78  |           0.39 |            0011 | No         |
    /// |           0.39  |           0.20 |            0010 | No         |
    /// |           0.20  |           0.10 |            0001 | No         |
    /// |           0.10  |           0.05 |            0000 | No         |
    ///
    /// Data rates marked ___`Yes`___ for low power will show reduced power use
    /// when the `low_power` bit is set. It has no effect on other data rates.
    ///
    #[bitfield(name = "low_power", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "rate", ty = "u8", bits = "0..=3")]
    byte: [u8; 1],
}

impl TryFrom<u8> for BandwidthRateControl {
    type Error = AdxlError;
    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        // Bit-wise AND with negative mask of allowed bitfields.
        if value & !0x1f == 0 {
            Ok(Self { byte: [value; 1] })
        } else {
            Err(AdxlError::UnknownModeBit(value))
        }
    }
}

/// Data format bitfields used in [data_format()] and [set_data_format()]
/// methods.
///
/// The data format register controls the format of the data registers values.
/// All returned data, except for the ±16 g range, will be clipped to avoid
/// rollover.
///
/// [data_format()]: trait.Adxl345Reader.html#method.data_format
/// [set_data_format()]: trait.Adxl345Writer.html#method.set_data_format
#[repr(C, align(1))]
#[derive(BitfieldStruct, Clone, Copy)]
pub struct DataFormat {
    /// Bit fields:
    /// * `self_test` - (Bit 7) A `true` applies a self-test force to the sensor,
    /// causing a shift in the output data.
    /// A `false` disable the self-test force.
    /// * `spi` - (Bit 6) A `true` sets 3-wire SPI mode and `false` 4-wire mode.
    /// * `int_invert` - (Bit 5) A `true` switches the interrupt pins to active
    /// low, while `false` sets them to active high.
    /// * `full_res` - (Bit 3) When `true` puts the device into full resolution
    /// mode where the output resolution increases with the g-force range while
    /// maintaining a 3.9mg/LSB scale factor.
    /// When `false` the device is in 10-bit mode where the `range` bitfield
    /// determines the maximum g-force range and scale factor.
    /// See the table below for more information.
    /// * `justify` - (Bit 2) When `true` selects left-justified (MSB) mode.
    /// A `false` selects right-justified mode with sign extension.
    /// * `range` - (Bits 0-1) Controls the g-force range and scale factor of
    /// readings as described in table.
    ///
    /// g-force range table:
    ///
    /// |  g Range | mg/LSB | `range` bits |
    /// | -------: | -----: | -----------: |
    /// |  &pm; 2g |  3.9mg |           00 |
    /// |  &pm; 4g |  7.8mg |           01 |
    /// |  &pm; 8g | 15.6mg |           10 |
    /// | &pm; 16g | 31.2mg |           11 |
    ///
    #[bitfield(name = "self_test", ty = "bool", bits = "7..=7")]
    #[bitfield(name = "spi", ty = "bool", bits = "6..=6")]
    #[bitfield(name = "int_invert", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "full_res", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "justify", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "range", ty = "u8", bits = "0..=1")]
    byte: [u8; 1],
}

impl TryFrom<u8> for DataFormat {
    type Error = AdxlError;
    //noinspection DuplicatedCode
    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        // Bit-wise AND with negative mask of allowed bitfields.
        if value & !0xef == 0 {
            Ok(Self { byte: [value; 1] })
        } else {
            Err(AdxlError::UnknownModeBit(value))
        }
    }
}

/// Fifo buffer control bitfields used in [fifo_control()] and
/// [set_fifo_control()] methods.
///
/// [fifo_control()]: trait.Adxl345Reader.html#method.fifo_control
/// [set_fifo_control()]: trait.Adxl345Writer.html#method.set_fifo_control
#[repr(C, align(1))]
#[derive(BitfieldStruct, Clone, Copy)]
pub struct FifoControl {
    /// Bit fields:
    /// * `fifo_mode` - (Bits 6-7) One of the fifo modes:
    ///
    /// | Mode    |`fifo_mode` bits | Function          |
    /// | ------- | --------------: | ----------------- |
    /// | Bypass  |              00 | FIFO is bypassed. |
    /// | FIFO    |              01 | FIFO collects up to 32 values and then waits until room is available in FIFO to write again. |
    /// | Stream  |              10 | FIFO holds the last 32 data values. Oldest data is overwritten if not read quickly enough. |
    /// | Trigger |              11 | Retains the preceding `samples` worth of FIFO entries then continues filling FIFO with entries until full. |
    ///
    /// * `trigger` - (Bit 5) When `true` links trigger event to INT2 else to INT1.
    /// * `samples` - (Bits 0-4) The function of these bits depends on the `fifo_mode`.
    /// See the table below for `fifo_mode` vs `samples` function.
    /// A 0 value will immediately set the `watermark` bit in the interrupt
    /// source register regardless of the FIFO mode.
    /// ___Note:___ _A 0 value should never be used when `fifo_mode` is set to
    /// trigger mode._
    ///
    /// Samples bits functions:
    ///
    /// | `fifo_mode` | `samples` function |
    /// | ----------- | ------------------ |
    /// | Bypass      | None.              |
    /// | FIFO        | Specifies number of FIFO entries needed before `watermark` interrupt triggers. |
    /// | Stream      | Specifies number of FIFO entries needed before `watermark` interrupt triggers. |
    /// | Trigger     | Specifies how many FIFO entries to retain before trigger event. |
    ///
    #[bitfield(name = "fifo_mode", ty = "u8", bits = "6..=7")]
    #[bitfield(name = "trigger", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "samples", ty = "u8", bits = "0..=4")]
    byte: [u8; 1],
}

impl From<u8> for FifoControl {
    fn from(value: u8) -> Self {
        Self { byte: [value; 1] }
    }
}

/// Fifo buffer status bitfields used in [fifo_status()] method.
///
/// [fifo_status()]: trait.Adxl345Reader.html#method.fifo_status
#[repr(C, align(1))]
#[derive(BitfieldStruct, Clone, Copy)]
pub struct FifoStatus {
    /// Bit fields:
    /// * `fifo_trigger` - (Bit 7) Is `true` if trigger event occurred.
    /// * `entries` - (Bits 0-5) Reports how many entries are available in FIFO.
    /// ___Note:___ _Maximum value is 33 since FIFO can store 32 entries plus
    /// the one available from the output filter of the device._
    #[bitfield(name = "fifo_trigger", ty = "bool", bits = "7..=7")]
    #[bitfield(name = "entries", ty = "u8", bits = "0..=5")]
    byte: [u8; 1],
}

impl TryFrom<u8> for FifoStatus {
    type Error = AdxlError;
    //noinspection DuplicatedCode
    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        // Bit-wise AND with negative mask of allowed bitfields.
        if value & !0xbf == 0 {
            Ok(Self { byte: [value; 1] })
        } else {
            Err(AdxlError::UnknownModeBit(value))
        }
    }
}

// Interrupt control mode.
bitflags! {
    /// Interrupt enable control bit flags use by [interrupt_control()] and
    /// [set_interrupt_control()] methods.
    ///
    /// [interrupt_control()]: trait.Adxl345Reader.html#method.interrupt_control
    /// [set_interrupt_control()]: trait.Adxl345Writer.html#method.set_interrupt_control
    #[derive(Default)]
    pub struct IntControlMode: u8 {
        /// Disable DATA_READY interrupt.
        ///
        /// Function is always enabled.
        const DATA_READY_DISABLE = 0x00;
        /// Enable DATA_READY interrupt.
        const DATA_READY_ENABLE = 0x80;
        /// Disable SINGLE_TAP interrupt and function.
        const SINGLE_TAP_DISABLE = 0x00;
        /// Enable SINGLE_TAP interrupt and function.
        const SINGLE_TAP_ENABLE = 0x40;
        /// Disable DOUBLE_TAP interrupt and function.
        const DOUBLE_TAP_DISABLE = 0x00;
        /// Enable DOUBLE_TAP interrupt and function.
        const DOUBLE_TAP_ENABLE = 0x20;
        /// Disable ACTIVITY interrupt and function.
        const ACTIVITY_DISABLE = 0x00;
        /// Enable ACTIVITY interrupt and function.
        const ACTIVITY_ENABLE = 0x10;
        /// Disable INACTIVITY interrupt and function.
        const INACTIVITY_DISABLE = 0x00;
        /// Enable INACTIVITY interrupt and function.
        const INACTIVITY_ENABLE = 0x08;
        /// Disable FREE_FALL interrupt and function.
        const FREE_FALL_DISABLE = 0x00;
        /// Enable FREE_FALL interrupt and function.
        const FREE_FALL_ENABLE = 0x04;
        /// Disable WATERMARK interrupt.
        ///
        /// Function is always enabled.
        const WATERMARK_DISABLE = 0x00;
        /// Enable WATERMARK interrupt.
        const WATERMARK_ENABLE = 0x02;
        /// Disable OVERRUN interrupt.
        ///
        /// Function is always enabled.
        const OVERRUN_DISABLE = 0x00;
        /// Enable OVERRUN interrupt.
        const OVERRUN_ENABLE = 0x01;
    }
}

// Interrupt map mode.
bitflags! {
    /// Interrupt map bit flags use by [interrupt_map()] and [set_interrupt_map()] methods.
    ///
    /// [interrupt_map()]: trait.Adxl345Reader.html#method.interrupt_map
    /// [set_interrupt_map()]: trait.Adxl345Writer.html#method.set_interrupt_map
    #[derive(Default)]
    pub struct IntMapMode: u8 {
        /// Map DATA_READY interrupt to `INT1` pin.
        const DATA_READY_INT1 = 0x00;
        /// Map DATA_READY interrupt to `INT2` pin.
        const DATA_READY_INT2 = 0x80;
        /// Map SINGLE_TAP interrupt to `INT1` pin.
        const SINGLE_TAP_INT1 = 0x00;
        /// Map SINGLE_TAP interrupt to `INT2` pin.
        const SINGLE_TAP_INT2 = 0x40;
        /// Map DOUBLE_TAP interrupt to `INT1` pin.
        const DOUBLE_TAP_INT1 = 0x00;
        /// Map DOUBLE_TAP interrupt to `INT2` pin.
        const DOUBLE_TAP_INT2 = 0x20;
        /// Map ACTIVITY interrupt to `INT1` pin.
        const ACTIVITY_INT1 = 0x00;
        /// Map ACTIVITY interrupt to `INT2` pin.
        const ACTIVITY_INT2 = 0x10;
        /// Map INACTIVITY interrupt to `INT1` pin.
        const INACTIVITY_INT1 = 0x00;
        /// Map INACTIVITY interrupt to `INT2` pin.
        const INACTIVITY_INT2 = 0x08;
        /// Map FREE_FALL interrupt to `INT1` pin.
        const FREE_FALL_INT1 = 0x00;
        /// Map FREE_FALL interrupt to `INT2` pin.
        const FREE_FALL_INT2 = 0x04;
        /// Map WATERMARK interrupt to `INT1` pin.
        const WATERMARK_INT1 = 0x00;
        /// Map WATERMARK interrupt to `INT2` pin.
        const WATERMARK_INT2 = 0x02;
        /// Map OVERRUN interrupt to `INT1` pin.
        const OVERRUN_INT1 = 0x00;
        /// Map OVERRUN interrupt to `INT2` pin.
        const OVERRUN_INT2 = 0x01;
    }
}

// Interrupt source.
bitflags! {
    /// Interrupt source bit flags use by [interrupt_source()] method.
    ///
    /// [interrupt_source()]: trait.Adxl345Reader.html#method.interrupt_source
    #[derive(Default)]
    pub struct IntSource: u8 {
        /// Function triggered DATA_READY event.
        ///
        /// Event always visible here.
        ///
        /// Cleared by reading data from the data registers.
        /// May require multiple reads.
        const DATA_READY = 0x80;
        /// Function triggered SINGLE_TAP event.
        ///
        /// Interrupt must be enabled to see here.
        const SINGLE_TAP = 0x40;
        /// Function triggered DOUBLE_TAP event.
        ///
        /// Interrupt must be enabled to see here.
        const DOUBLE_TAP = 0x20;
        /// Function triggered ACTIVITY event.
        ///
        /// Interrupt must be enabled to see here.
        const ACTIVITY = 0x10;
        /// Function triggered INACTIVITY event.
        ///
        /// Interrupt must be enabled to see here.
        const INACTIVITY = 0x08;
        /// Function triggered FREE_FALL event.
        ///
        /// Interrupt must be enabled to see here.
        const FREE_FALL = 0x04;
        /// Function triggered WATERMARK event.
        ///
        /// Event always visible here.
        ///
        /// Cleared by reading data from the data registers.
        /// May require multiple reads.
        const WATERMARK = 0x02;
        /// Function triggered OVERRUN event.
        ///
        /// Event always visible here.
        ///
        /// Cleared by reading data from the data registers.
        const OVERRUN = 0x01;
    }
}

/// Power control bitfields used in [power_control()] and [set_power_control()]
/// methods.
///
/// [power_control()]: trait.Adxl345Reader.html#method.power_control
/// [set_power_control()]: trait.Adxl345Writer.html#method.set_power_control
#[repr(C, align(1))]
#[derive(BitfieldStruct, Clone, Copy)]
pub struct PowerControl {
    /// Bit fields:
    /// * `link` - (Bit 5) This bit serially links the activity and inactivity
    /// functions.
    /// When the bit is 0 the inactivity and activity functions are concurrent.
    /// * `auto_sleep` - (Bit 4) If the `link` bit is set, a setting of 1 in the
    /// `auto_sleep` bit enables the auto-sleep functionality.
    /// When bit is 0 then the activity/inactivity settings are ignored.
    /// * `measure` - (Bit 3) Standby/measurement mode.
    /// The ADXL345 is in standby mode at power up.
    /// * `sleep` - (Bit 2) Puts the part into sleep mode.
    /// * `wakeup` - (Bits 0-1) Controls the frequency of readings in sleep mode
    /// as described in table.
    ///
    /// ___Note:___ _It is recommended that the `measure` bit be placed into
    /// standby mode and then set back to measurement mode with a subsequent
    /// write when changing any of the other power mode bit fields._
    ///
    /// Wakeup frequency table:
    ///
    /// | Frequency (Hz) | Wakeup (bin) |
    /// | -------------: | -----------: |
    /// |              8 |           00 |
    /// |              4 |           01 |
    /// |              2 |           10 |
    /// |              1 |           11 |
    ///
    #[bitfield(name = "link", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "auto_sleep", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "measure", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "sleep", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "wakeup", ty = "u8", bits = "0..=1")]
    byte: [u8; 1],
}

impl TryFrom<u8> for PowerControl {
    type Error = AdxlError;
    //noinspection DuplicatedCode
    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        // Bit-wise AND with negative mask of allowed bitfields.
        if value & !0x3f == 0 {
            Ok(Self { byte: [value; 1] })
        } else {
            Err(AdxlError::UnknownModeBit(value))
        }
    }
}

/// Hold a collection of single/double tap non-control related values.
///
/// Structure is used by the [tap()] and [set_tap()] methods.
///
/// [tap()]: trait.Adxl345Reader.html#method.tap
/// [set_tap()]: trait.Adxl345Writer.html#method.set_tap
#[derive(Debug, Clone, Copy)]
pub struct Tap {
    /// Threshold value required to trigger a tap interrupt.
    ///
    /// The scale factor is 62.5 mg/LSB.
    ///
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the single tap/double tap interrupt(s) are enabled._
    threshold: u8,
    /// Time value representing the maximum time that an event must be above the
    /// threshold to qualify as a tap event.
    ///
    /// The scale factor is 625 μs/LSB.
    /// A value of 0 disables the single/double tap functions.
    duration: u8,
    /// Time value representing the wait time from the detection of a tap event
    /// to the start of the time window during which a possible second tap event
    /// can be detected.
    ///
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    latency: u8,
    /// Time value representing the amount of time after the expiration of the
    /// latency time during which a second valid tap can begin.
    ///
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    window: u8,
}

impl Tap {
    /// Tap constructor.
    ///
    /// ## Arguments
    /// * `threshold` - Threshold value required to trigger a tap interrupt.
    /// The scale factor is 62.5 mg/LSB.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the single tap/double tap interrupt(s) are enabled._
    /// * `duration` - Time value representing the maximum time that an event
    /// must be above the threshold to qualify as a tap event.
    /// The scale factor is 625 μs/LSB.
    /// A value of 0 disables the single tap/double tap functions.
    /// * `latency` - Time value representing the wait time from the detection
    /// of a tap event to the start of the time window during which a possible
    /// second tap event can be detected.
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    /// * `window` - Time value representing the amount of time after the
    /// expiration of the latency time during which a second valid tap can begin.
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    pub fn new(threshold: u8, duration: u8, latency: u8, window: u8) -> Self {
        Tap {
            threshold,
            duration,
            latency,
            window,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Tap {
    fn from(tap: (u8, u8, u8, u8)) -> Self {
        Tap {
            threshold: tap.0,
            duration: tap.1,
            latency: tap.2,
            window: tap.3,
        }
    }
}

impl From<[u8; 4]> for Tap {
    fn from(tap: [u8; 4]) -> Self {
        Tap {
            threshold: tap[0],
            duration: tap[1],
            latency: tap[2],
            window: tap[3],
        }
    }
}

// Tap Axis control mode.
bitflags! {
    /// Tap axis mode bit flags used in [tap_control()] and [set_tap_control()]
    /// methods.
    ///
    /// [tap_control()]: trait.Adxl345Reader.html#method.tap_control
    /// [set_tap_control()]: trait.Adxl345Writer.html#method.set_tap_control
    #[derive(Default)]
    pub struct TapMode: u8 {
        /// Disable (suppress) double tap detection if acceleration is greater
        /// than tap threshold between taps.
        const DT_DISABLE = 0x08;
        /// Enable (ignore) double tap detection while ignore any acceleration
        /// that is greater than tap threshold between taps.
        const DT_ENABLE = 0x00;
        /// Disable X-axis for tap detection.
        const X_DISABLE = 0x00;
        /// Enable X-axis for tap detection.
        const X_ENABLE = 0x04;
        /// Disable Y-axis for tap detection.
        const Y_DISABLE = 0x00;
        /// Enable Y-axis for tap detection.
        const Y_ENABLE = 0x02;
        /// Disable Z-axis for tap detection.
        const Z_DISABLE = 0x00;
        /// Enable Z-axis for tap detection.
        const Z_ENABLE = 0x01;
    }
}
