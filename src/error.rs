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

/// Provides a shared set of error types.
#[derive(Debug)]
pub enum AdxlError {
    /// Used when given address (offset) is read-only, reserved, or unknown.
    IllegalWriteAddress(u8),
    /// Underlying I²C error.
    I2c(),
    /// Underlying SPI error.
    Spi(),
    /// Invalid bus parameters.
    InvalidBusParams,
    /// Used when given an un-excepted value for a mode.
    UnknownModeBit(u8),
}

impl core::fmt::Display for AdxlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AdxlError::IllegalWriteAddress(addr) =>
                write!(f, "Attempted illegal write to address {}", addr),
            AdxlError::I2c() =>
                write!(f, "I²C interface access failed"),
            AdxlError::Spi() =>
                write!(f, "SPI interface access failed"),
            AdxlError::InvalidBusParams =>
                write!(f, "Invalid bus parameters"),
            AdxlError::UnknownModeBit(value) =>
                write!(f, "Received one or more set unknown mode bit(s) in value: {}", value),
        }
    }
}

#[cfg(not(feature="no_std"))]
impl std::error::Error for AdxlError {}

impl From<rppal::i2c::Error> for AdxlError {
    fn from(_: rppal::i2c::Error) -> AdxlError {
        AdxlError::I2c()
    }
}

impl From<rppal::spi::Error> for AdxlError {
    fn from(_: rppal::spi::Error) -> AdxlError {
        AdxlError::Spi()
    }
}

/// Result type used when return value is needed from methods in library.
pub type AdxlResult<T> = core::result::Result<T, AdxlError>;

/// Result type used when return value is _NOT_ needed from methods in library.
pub type Result = core::result::Result<(), AdxlError>;
