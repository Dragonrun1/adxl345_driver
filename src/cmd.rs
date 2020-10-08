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

use crate::{AdxlResult, Result};

/// Complete R/W command set for Analog Device ADXL345 3-Axis Digital Accelerometer.
pub trait Adxl345: Adxl345Reader + Adxl345Writer {}

/// Read command set for accelerometer.
pub trait Adxl345Reader {
    /// Access the device ID.
    fn device_id(&self) -> AdxlResult<u8>;
    fn tab_threshold(&self) -> AdxlResult<u8>;
}

/// Write command set for accelerometer.
pub trait Adxl345Writer {
    //
    // ## Per driver required stuff ##
    //
    /// Provides an interface to send commands through the ADXL345 driver.
    ///
    /// This is __NOT__ part of the actual ADXL345 command register set but a
    /// necessary method to interface with all drivers.
    ///
    /// Provides a place for drivers to do any needed common command processing.
    ///
    /// The implementation of the command set in this trait will use this method
    /// after doing any needed per command processing.
    ///
    /// ## Arguments
    /// * `addr` - Register address (offset) to be written.
    /// * `byte` - Byte of data to be written into the given register.
    fn command(&mut self, addr: u8, byte: u8) -> Result;
    /// Used to initialize the accelerometer into a know state.
    fn init(&mut self) -> Result;
    //
    // ## Shouldn't be a need to change these in driver implementations. ##
    //
    /// Tap related settings
    ///
    /// ## Arguments
    /// * `thresh` - Threshold value for tap interrupts.
    /// The scale factor is 62.5 mg/LSB.
    /// A value of 0 may result in undesirable behavior if single tap/double tap
    /// interrupts are enabled.
    /// * `duration` - Time value representing the maximum time that an event
    /// must be above the threshold to qualify as a tap event.
    /// The scale factor is 625 μs/LSB.
    /// A value of 0 disables the single tap/ double tap functions.
    /// * `latency` - Time value representing the wait time from the detection
    /// of a tap event to the start of the time window during which a possible
    /// second tap event can be detected.
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    /// * `window` - Time value representing the amount of time after the
    /// expiration of the latency time during which a second valid tap can begin.
    /// The scale factor is 1.25 ms/LSB.
    /// A value of 0 disables the double tap function.
    fn set_tap(&mut self, thresh: u8, duration: u8, latency: u8, window: u8) -> Result;
    /// Set tap control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Tab mode bit flags.
    /// See [TapMode] bit flags for more info.
    ///
    /// [TapMode]: struct.TapMode.html
    fn tap_control<TM>(&mut self, mode: TM) -> Result
    where
        TM: Into<TapMode>;
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
        Z: Into<Option<i8>>;
    /// Used to set the threshold for detecting activity.
    ///
    /// ## Arguments
    /// * `thresh` - Threshold value for detecting activity.
    /// The scale factor is 62.5 mg/LSB.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the activity interrupt is enabled._
    fn set_activity(&mut self, thresh: u8) -> Result;
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
    fn set_inactivity(&mut self, thresh: u8, time: u8) -> Result;
    /// Set activity/inactivity control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Activity mode bit flags.
    /// See [ActivityMode] bit flags for more info.
    ///
    /// [ActivityMode]: struct.ActivityMode.html
    fn activity_control<AM>(&mut self, mode: AM) -> Result
    where
        AM: Into<ActivityMode>;
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
    fn set_free_fall(&mut self, thresh: u8, time: u8) -> Result;
}

// Activity/Inactivity control mode.
bitflags! {
    /// Activity mode bit flags used in [activity_control()] command.
    ///
    /// [activity_control()]: trait.ADXL345Writer.html#method.activity_control
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
// Tap Axis control mode.
bitflags! {
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
