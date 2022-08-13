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
//! This is a simple example of how to use library.
//!
//! The example was written assuming Raspberry Pi OS but should work with other
//! Linux distros with little or no change.
//!
//! ## Examples
//! To build the example use:
//! ```sh, no_run
//! cargo build --example spi
//! ```
//! Then to run use:
//! ```sh, no_run
//! sudo ./target/debug/examples/spi
//! ```
//!
//! Output example:
//! ```sh, no_run
//! axis: {'x': 1.6083, 'y': 0.0392, 'z': 8.7868} m/s²
//! axis: {'x': 1.6867, 'y': 0.1177, 'z': 8.7868} m/s²
//! axis: {'x': 1.6475, 'y': 0.1177, 'z': 8.8260} m/s²
//! ...
//! ```

use adxl345_driver::{spi::Device, Adxl345Reader, Adxl345Writer};
use anyhow::{Context, Result};
use rppal::system::DeviceInfo;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread::sleep,
    time::Duration,
};

/// Output scale is 4mg/LSB.
const SCALE_MULTIPLIER: f64 = 0.004;
/// Average Earth gravity in m/s²
const EARTH_GRAVITY_MS2: f64 = 9.80665;

/// Entry point of example.
fn main() -> Result<()> {
    println!(
        "SPI example started on a {}",
        DeviceInfo::new()
            .context("Failed to get new DeviceInfo")?
            .model()
    );
    let mut adxl345 = Device::new().context("Failed to get instance")?;
    let id = adxl345.device_id().context("Failed to get device id")?;
    println!("Device id = {}", id);
    // Set full scale output and range to 2G.
    adxl345
        .set_data_format(8)
        .context("Failed to set data format")?;
    // Set measurement mode on.
    adxl345
        .set_power_control(8)
        .context("Failed to turn on measurement mode")?;
    // Stuff needed to nicely handle Ctrl-C from user.
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .context("Error setting Ctrl-C handler")?;
    // Loop until Ctrl-C is received.
    while running.load(Ordering::SeqCst) {
        let (x, y, z) = adxl345
            .acceleration()
            .context("Failed to get acceleration data")?;
        let x = x as f64 * SCALE_MULTIPLIER * EARTH_GRAVITY_MS2;
        let y = y as f64 * SCALE_MULTIPLIER * EARTH_GRAVITY_MS2;
        let z = z as f64 * SCALE_MULTIPLIER * EARTH_GRAVITY_MS2;
        println!(
            "axis: {{'x': {:1.4}, 'y': {:1.4}, 'z': {:1.4}}} m/s²",
            x, y, z
        );
        sleep(Duration::from_millis(100));
    }
    // Set measurement mode off.
    adxl345
        .set_power_control(0)
        .context("Failed to turn on measurement mode")?;
    println!("\nSPI example stopped");
    Ok(())
}
