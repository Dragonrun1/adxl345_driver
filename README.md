# adxl345_driver

This is an implementation of a hardware driver for a [ADXL345] type 3-Axis
Digital Accelerometer write in [Rust] using the [rppal] library.
It exposes a simple trait-based API for the command set which minimizes the
coupling between the hardware driver (I²C, etc) and the code that passes
commands and data to and from the accelerometer.

Through the name says ADXL345 the driver should also work with the [ADXL346]
device as well since the only difference between them is the physical packaging
and not the internal workings.

## Table Of Contents

  * [Getting Started](#getting-started)
  * [Using The Crate](#using-the-crate)
  * [Examples](#examples)
  * [Contributing](#contributing)
  * [Licenses](#licenses)

## Getting Started

You will need to have a recent version of [Rust] installed.
Any version of Rust that supports version 0.11.3 or later of [rppal] should
work but versions from 1.43 to 1.47 of Rust have been used during initial
development on both the nightly and release channels.
Earlier versions might work as well but have not been tested.

Development can be done on any OS that Rust supports but the only expected
output target is a Raspberry Pi running a Linux OS.
All initial development has been done with a combination of a laptop running
Windows 10 and a 4GB Raspberry Pi 4 running the Raspberry Pi OS (Raspbian).

### Using The Crate

To use the crate in your own project all you need to do is include it in
`[dependencies]` of you project like you would any other crate.
If you have [cargo-edit] install then on the command line you can use:

```shell script
cargo add adxl345_driver
```

Which should add something like this in your [Cargo.toml]:

```toml
[dependencies]
adxl345_driver = "0.0.5"
```

## Examples

You will find examples in the `examples` directory. The Raspberry Pi I²C
example was used for testing during initial development on a RPi 4.

To build the I²C example start by clone this project somewhere on your Raspberry
Pi:

```shell
git clone https://github.com/Dragonrun1/adxl345_driver
```

Next execute the follow to build the example:

```shell
cargo build --example i2c
```

And finally execute:

```shell
sudo ./target/debug/examples/i2c
```

You should see the series of x, y, z values displayed in the terminal if your
device has been hooked up using the primary I²C that the example expects.

Output example:
```console
axis: {'x': 1.6083, 'y': 0.0392, 'z': 8.7868} m/s²
axis: {'x': 1.6867, 'y': 0.1177, 'z': 8.7868} m/s²
axis: {'x': 1.6475, 'y': 0.1177, 'z': 8.8260} m/s²
...
```

You can find the latest version by go to [adxl345_driver] on the crates.io website.

## Contributing

Contributors are welcome.
Make sure you have read the [Contributor Covenant Code of Conduct].
All contributed code will be considered to also be contributed under a [MIT]
license.
Please include your information in a comment on all code files for the copyright
etc.

All contributed documentation or non-code text like this README etc. will be
consider to be under the same [CC-BY-SA] license.

## Licenses

All code is available under the [MIT] license.
You can find a copy of the license in the [LICENSE] file.

All documentation like this README is licensed under a
<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons Attribution-ShareAlike 4.0 International License</a>
(CC-BY-SA). 

[ADXL345]: https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL345.pdf
[ADXL346]: https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL346.pdf
[CC-BY-SA]: http://creativecommons.org/licenses/by-sa/4.0/
[Cargo.toml]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[Contributor Covenant Code of Conduct]: CODE_OF_CONDUCT.md
[LICENSE]: LICENSE
[MIT]: https://opensource.org/licenses/MIT
[Rust]: https://www.rust-lang.org/
[adxl345_driver]: https://crates.io/crates/adxl345_driver
[cargo-edit]: https://crates.io/crates/cargo-edit
[rppal]: https://github.com/golemparts/rppal

<hr>
<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">
<img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png" />
</a>
