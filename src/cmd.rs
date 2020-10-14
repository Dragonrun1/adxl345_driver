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

use crate::{AdxlResult, Result};

/// Complete R/W register command set for the accelerometer.
pub trait Adxl345: Adxl345Reader + Adxl345Writer {}

/// Read register command set for accelerometer.
pub trait Adxl345Reader {
    /// Access the current activity control mode.
    fn activity_control(&self) -> AdxlResult<ActivityMode>;
    /// Access the cause of tap or activity event (interrupt).
    ///
    /// ___Note:___ _The register value should be read before clearing the
    /// interrupt._
    ///
    /// Disabling an axis from participation clears the corresponding source bit
    /// when the next activity or single tap/double tap event occurs.
    fn activity_tap_status(&self) -> AdxlResult<ATStatus>;
    /// Access the current data rate and power mode control mode.
    fn bandwidth_rate(&self) -> AdxlResult<BandwidthRateControl>;
    /// Access the device ID.
    fn device_id(&self) -> AdxlResult<u8>;
    /// Access the current power-saving features control mode.
    fn power_control(&self) -> AdxlResult<PowerControl>;
    /// Access to all non-control tap current values together as a structure.
    ///
    /// See [Tap] for more information.
    ///
    /// [Tap]: struct.Tap.html
    fn tap(&self) -> AdxlResult<Tap>;
    /// Access the current tap control mode.
    fn tap_control(&self) -> AdxlResult<TapMode>;
    /// Access the current duration value for tap interrupts.
    fn tap_duration(&self) -> AdxlResult<u8>;
    /// Access the current latency value for tap interrupts.
    fn tap_latency(&self) -> AdxlResult<u8>;
    /// Access the current threshold value for tap interrupts.
    fn tap_threshold(&self) -> AdxlResult<u8>;
    /// Access the current window value for tap interrupts.
    fn tap_window(&self) -> AdxlResult<u8>;
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
    /// Set the activity threshold.
    ///
    /// ## Arguments
    /// * `thresh` - Threshold value for detecting activity.
    /// The scale factor is 62.5 mg/LSB.
    /// ___Note:___ _that a value of 0 may result in undesirable behavior if
    /// the activity interrupt is enabled._
    fn set_activity(&mut self, thresh: u8) -> Result;
    /// Set activity/inactivity control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Activity mode bit flags.
    /// See [ActivityMode] bit flags for more info.
    ///
    /// [ActivityMode]: struct.ActivityMode.html
    fn set_activity_control<AM>(&mut self, mode: AM) -> Result
    where
        AM: Into<ActivityMode>;
    /// Set data rate and power mode control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Data rate and power mode bit flags.
    /// See [BandwidthRateControl] bit flags for more info.
    ///
    /// [BandwidthRateControl]: struct.BandwidthRateControl.html
    fn set_bandwidth_rate<BRC>(&self, mode: BRC) -> Result
    where
        BRC: Into<BandwidthRateControl>;
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
    /// Set power-saving features control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Power-saving features bit flags.
    /// See [PowerControl] bit flags for more info.
    ///
    /// [PowerControl]: struct.PowerControl.html
    fn set_power_control<PC>(&self, mode: PC) -> Result
    where
        PC: Into<PowerControl>;
    /// Set all non-control tap related values at the same time.
    ///
    /// ## Arguments
    /// * `mode` - Containing values for `threshold`, `duration`, `latency`, and
    /// `window` registers.
    fn set_tap<T>(&mut self, mode: T) -> Result
    where
        T: Into<Tap>;
    /// Set tap control mode options.
    ///
    /// ## Arguments
    /// * `mode` - Tab mode bit flags.
    /// See [TapMode] bit flags for more info.
    ///
    /// [TapMode]: struct.TapMode.html
    fn set_tap_control<TM>(&mut self, mode: TM) -> Result
    where
        TM: Into<TapMode>;
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
    bandwidth_control: [u8; 1],
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
    /// * `sleep` - (Bit 2) Puts part into sleep mode.
    /// * `wakeup` - (Bits 0-1) Control the frequency of readings in sleep mode
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
    power_control: [u8; 1],
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

impl From<&[u8; 4]> for Tap {
    fn from(tap: &[u8; 4]) -> Self {
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
