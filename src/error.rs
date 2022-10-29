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
//! A common set of error and result type used in the library.

use thiserror::Error;

/// Provides a shared set of error types.
#[derive(Error, Debug)]
pub enum AdxlError {
    /// Used when given address (offset) is read-only, reserved, or unknown.
    #[error("Attempted illegal write to address {0}")]
    IllegalWriteAddress(u8),
    /// Used to pass through any underlying I²C errors.
    #[error("I²C interface access failed")]
    I2c(String),
    /// Used to pass through any underlying SPI errors.
    #[error("SPI interface access failed")]
    Spi(String),
    /// Used when given an un-excepted value for a mode.
    #[error("Received one or more set unknown mode bit(s) in value: {0}")]
    UnknownModeBit(u8),
}

/// Result type used when return value is needed from methods in library.
pub type AdxlResult<T> = std::result::Result<T, AdxlError>;

/// Result type used when return value is _NOT_ needed from methods in library.
pub type Result = std::result::Result<(), AdxlError>;
