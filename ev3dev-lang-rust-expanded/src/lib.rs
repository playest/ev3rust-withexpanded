#![feature(prelude_import)]
#![feature(fmt_internals)]
#![feature(box_syntax)]
#![no_std]
#![allow(missing_docs)]

//! # Rust language bindings for ev3dev
//!
//! ```no_run
//! extern crate ev3dev_lang_rust;
//!
//! use ev3dev_lang_rust::prelude::*;
//! use ev3dev_lang_rust::Ev3Result;
//! use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
//! use ev3dev_lang_rust::sensors::ColorSensor;
//!
//! fn main() -> Ev3Result<()> {
//!
//!     // Get large motor on port outA.
//!     let large_motor = LargeMotor::get(MotorPort::OutA)?;
//!
//!     // Set command "run-direct".
//!     large_motor.run_direct()?;
//!
//!     // Run motor.
//!     large_motor.set_duty_cycle_sp(50)?;
//!
//!     // Find color sensor. Always returns the first recognised one.
//!     let color_sensor = ColorSensor::find()?;
//!
//!     // Switch to rgb mode.
//!     color_sensor.set_mode_rgb_raw()?;
//!
//!     // Get current rgb color tuple.
//!     println!("Current rgb color: {:?}", color_sensor.get_rgb()?);
//!
//!     Ok(())
//! }
//! ```
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;

#[macro_use]
extern crate ev3dev_lang_rust_derive;
extern crate libc;
extern crate alloc;

mod attriute {

    //! A wrapper to a attribute file in the `/sys/class/` directory.
    use crate::{Ev3Error, Ev3Result};
    use std::cell::RefCell;
    use std::error::Error;
    use std::fs::{self, File, OpenOptions};
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::io::{AsRawFd, RawFd};
    use std::rc::Rc;
    use std::string::String;
    /// The root driver path `/sys/class/`.
    const ROOT_PATH: &str = "/sys/class/";
    /// A wrapper to a attribute file in the `/sys/class/` directory.
    pub struct Attribute {
        file: Rc<RefCell<File>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Attribute {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Attribute {
                    file: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Attribute");
                    let _ = debug_trait_builder.field("file", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Attribute {
        #[inline]
        fn clone(&self) -> Attribute {
            match *self {
                Attribute {
                    file: ref __self_0_0,
                } => Attribute {
                    file: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl Attribute {
        /// Create a new `Attribute` instance that wrappes
        /// the file `/sys/class/{class_name}/{name}{attribute_name}`.
        pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Ev3Result<Attribute> {
            let filename = {
                let res = alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "", "/", "/"],
                    &match (&ROOT_PATH, &class_name, &name, &attribute_name) {
                        (arg0, arg1, arg2, arg3) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            };
            let stat = fs::metadata(&filename)?;
            let mode = stat.permissions().mode();
            let readable = mode & 256 == 256;
            let writeable = mode & 128 == 128;
            let file = OpenOptions::new()
                .read(readable)
                .write(writeable)
                .open(&filename)?;
            Ok(Attribute {
                file: Rc::new(RefCell::new(file)),
            })
        }
        /// Returns the current value of the wrapped file.
        fn get_str(&self) -> Ev3Result<String> {
            let mut value = String::new();
            let mut file = self.file.borrow_mut();
            file.seek(SeekFrom::Start(0))?;
            file.read_to_string(&mut value)?;
            Ok(value.trim_end().to_owned())
        }
        /// Sets the value of the wrapped file.
        /// Returns a `Ev3Result::InternalError` if the file is not writable.
        fn set_str(&self, value: &str) -> Ev3Result<()> {
            let mut file = self.file.borrow_mut();
            file.seek(SeekFrom::Start(0))?;
            file.write_all(value.as_bytes())?;
            Ok(())
        }
        /// Returns the current value of the wrapped file.
        /// The value is parsed to the type `T`.
        /// Returns a `Ev3Result::InternalError` if the current value is not parsable to type `T`.
        pub fn get<T>(&self) -> Ev3Result<T>
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: Error,
        {
            let value = self.get_str()?;
            match value.parse::<T>() {
                Ok(value) => Ok(value),
                Err(e) => Err(Ev3Error::InternalError {
                    msg: {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[""],
                            &match (&e,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    },
                }),
            }
        }
        /// Sets the value of the wrapped file.
        /// The value is parsed from the type `T`.
        /// Returns a `Ev3Result::InternalError` if the file is not writable.
        pub fn set<T>(&self, value: T) -> Ev3Result<()>
        where
            T: std::string::ToString,
        {
            self.set_str(&value.to_string())
        }
        #[inline]
        /// Sets the value of the wrapped file.
        /// This function skips the string parsing of the `self.set<T>()` function.
        /// Returns a `Ev3Result::InternalError` if the file is not writable.
        pub fn set_str_slice(&self, value: &str) -> Ev3Result<()> {
            self.set_str(value)
        }
        /// Returns a string vector representation of the wrapped file.
        /// The file value is splitet at whitespaces.
        pub fn get_vec(&self) -> Ev3Result<Vec<String>> {
            let value = self.get_str()?;
            let vec = value
                .split_whitespace()
                .map(|word| word.to_owned())
                .collect();
            Ok(vec)
        }
        /// Returns a C pointer to the wrapped file.
        pub fn get_raw_fd(&self) -> RawFd {
            self.file.borrow().as_raw_fd()
        }
    }
}
pub use attriute::Attribute;
mod driver {
    //! Helper struct that manages attributes.
    //! It creates an `Attribute` instance if it does not exists or uses a cached one.
    use crate::{utils::OrErr, Attribute, Ev3Error, Ev3Result, Port};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt::{self, Debug};
    use std::fs;
    use std::string::String;
    /// The root driver path `/sys/class/`.
    const ROOT_PATH: &str = "/sys/class/";
    /// Helper struct that manages attributes.
    /// It creates an `Attribute` instance if it does not exists or uses a cached one.
    pub struct Driver {
        class_name: String,
        name: String,
        attributes: RefCell<HashMap<String, Attribute>>,
    }
    impl Driver {
        /// Returns a new `Driver`.
        /// All attributes created by this driver will use the path `/sys/class/{class_name}/{name}`.
        pub fn new(class_name: &str, name: &str) -> Driver {
            Driver {
                class_name: class_name.to_owned(),
                name: name.to_owned(),
                attributes: RefCell::new(HashMap::new()),
            }
        }
        /// Returns the name of the device with the given `class_name`, `driver_name` and at the given `port`.
        ///
        /// Returns `Ev3Error::NotFound` if no such device exists.
        pub fn find_name_by_port_and_driver(
            class_name: &str,
            port: &dyn Port,
            driver_name: &str,
        ) -> Ev3Result<String> {
            let port_address = port.address();
            let paths = fs::read_dir({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&ROOT_PATH, &class_name) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            })?;
            for path in paths {
                let file_name = path?.file_name();
                let name = file_name.to_str().or_err()?;
                let address = Attribute::new(class_name, name, "address")?;
                if address.get::<String>()?.contains(&port_address) {
                    let driver = Attribute::new(class_name, name, "driver_name")?;
                    if driver.get::<String>()? == driver_name {
                        return Ok(name.to_owned());
                    }
                }
            }
            Err(Ev3Error::NotFound)
        }
        /// Returns the name of the device with the given `class_name` and at the given `port`.
        ///
        /// Returns `Ev3Error::NotFound` if no such device exists.
        /// Returns `Ev3Error::MultipleMatches` if more then one matching device exists.
        pub fn find_name_by_port(class_name: &str, port: &dyn Port) -> Ev3Result<String> {
            let port_address = port.address();
            let paths = fs::read_dir({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&ROOT_PATH, &class_name) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            })?;
            for path in paths {
                let file_name = path?.file_name();
                let name = file_name.to_str().or_err()?;
                let address = Attribute::new(class_name, name, "address")?;
                if address.get::<String>()?.contains(&port_address) {
                    return Ok(name.to_owned());
                }
            }
            Err(Ev3Error::NotFound)
        }
        /// Returns the name of the device with the given `class_name`.
        ///
        /// Returns `Ev3Error::NotFound` if no such device exists.
        /// Returns `Ev3Error::MultipleMatches` if more then one matching device exists.
        pub fn find_name_by_driver(class_name: &str, driver_name: &str) -> Ev3Result<String> {
            let mut names = Driver::find_names_by_driver(class_name, driver_name)?;
            match names.len() {
                0 => Err(Ev3Error::NotFound),
                1 => Ok(names
                    .pop()
                    .expect("Name vector contains exactly one element")),
                _ => Err(Ev3Error::MultipleMatches),
            }
        }
        /// Returns the names of the devices with the given `class_name`.
        pub fn find_names_by_driver(class_name: &str, driver_name: &str) -> Ev3Result<Vec<String>> {
            let paths = fs::read_dir({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&ROOT_PATH, &class_name) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            })?;
            let mut found_names = Vec::new();
            for path in paths {
                let file_name = path?.file_name();
                let name = file_name.to_str().or_err()?;
                let driver = Attribute::new(class_name, name, "driver_name")?;
                if driver.get::<String>()? == driver_name {
                    found_names.push(name.to_owned());
                }
            }
            Ok(found_names)
        }
        /// Return the `Attribute` wrapper for the given `attribute_name`.
        /// Creates a new one if it does not exist.
        pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
            let mut attributes = self.attributes.borrow_mut();
            if !attributes.contains_key(attribute_name) {
                if let Ok(v) =
                    Attribute::new(self.class_name.as_ref(), self.name.as_ref(), attribute_name)
                {
                    attributes.insert(attribute_name.to_owned(), v);
                };
            };
            attributes
                .get(attribute_name)
                .expect("Internal error in the attribute map")
                .clone()
        }
    }
    impl Clone for Driver {
        fn clone(&self) -> Self {
            Driver {
                class_name: self.class_name.clone(),
                name: self.name.clone(),
                attributes: RefCell::new(HashMap::new()),
            }
        }
    }
    impl Debug for Driver {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["Driver { class_name: ", ", name: ", " }"],
                &match (&self.class_name, &self.name) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ))
        }
    }
}
pub use driver::Driver;
mod device {
    use crate::{Attribute, Ev3Result};
    /// The ev3dev device base trait
    pub trait Device {
        /// Returns the attribute wrapper for an attribute name.
        fn get_attribute(&self, name: &str) -> Attribute;
        /// Returns the name of the port that the motor is connected to.
        fn get_address(&self) -> Ev3Result<String> {
            self.get_attribute("address").get()
        }
        /// Sends a command to the device controller.
        fn set_command(&self, command: &str) -> Ev3Result<()> {
            self.get_attribute("command").set_str_slice(command)
        }
        /// Returns a space separated list of commands that are supported by the device controller.
        fn get_commands(&self) -> Ev3Result<Vec<String>> {
            self.get_attribute("commands").get_vec()
        }
        /// Returns the name of the driver that provides this device.
        fn get_driver_name(&self) -> Ev3Result<String> {
            self.get_attribute("driver_name").get()
        }
    }
}
pub use device::Device;
mod findable {
    use crate::{Device, Ev3Result, Port};
    /// Helper trait to create a new `Device` instance.
    ///
    /// Can be automatically derived. Therefore are 3 parameters required:
    /// * `class_name: &str`
    /// * `driver_name: &str`
    /// * `port: dyn ev3dev_lang_rust::Motor`
    ///
    /// # Example:
    ///
    /// #[derive(Debug, Clone, Device, Findable, Motor, TachoMotor)]
    ///// #[class_name = "tacho-motor"]
    ///// #[driver_name = "lego-ev3-l-motor"]
    ///// #[port = "crate::motors::MotorPort"]
    /// pub struct LargeMotor {
    ///     driver: Driver,
    /// }
    pub trait Findable<PortType>
    where
        Self: std::marker::Sized,
        Self: Device,
        PortType: Port,
    {
        /// Extract list of connected 'Self'
        fn list() -> Ev3Result<Vec<Self>>;
        /// Try to get a `Self` on the given port. Returns `None` if port is not used or another device is connected.
        fn get(port: PortType) -> Ev3Result<Self>;
        /// Try to find a `Self`. Only returns a motor if their is exactly one connected, `Error::NotFound` otherwise.
        fn find() -> Ev3Result<Self>;
    }
}
pub use findable::Findable;
mod utils {
    //! Utility things.
    /// Helper `Result` type for easy access.
    pub type Ev3Result<T> = Result<T, Ev3Error>;
    /// Custom error type for internal errors.
    pub enum Ev3Error {
        /// Internal error with error `msg`.
        InternalError {
            /// Original error message.
            msg: String,
        },

        /// No matching device found.
        NotFound,

        /// More than one matching device found.
        MultipleMatches,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Ev3Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Ev3Error::InternalError { msg: ref __self_0 },) => {
                    let mut debug_trait_builder = f.debug_struct("InternalError");
                    let _ = debug_trait_builder.field("msg", &&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Ev3Error::NotFound,) => {
                    let mut debug_trait_builder = f.debug_tuple("NotFound");
                    debug_trait_builder.finish()
                }
                (&Ev3Error::MultipleMatches,) => {
                    let mut debug_trait_builder = f.debug_tuple("MultipleMatches");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl From<std::io::Error> for Ev3Error {
        fn from(err: std::io::Error) -> Self {
            Ev3Error::InternalError {
                msg: {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&err,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                },
            }
        }
    }
    impl From<std::string::FromUtf8Error> for Ev3Error {
        fn from(err: std::string::FromUtf8Error) -> Self {
            Ev3Error::InternalError {
                msg: {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&err,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                },
            }
        }
    }
    impl From<std::num::ParseIntError> for Ev3Error {
        fn from(err: std::num::ParseIntError) -> Self {
            Ev3Error::InternalError {
                msg: {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&err,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                },
            }
        }
    }
    /// EV3 ports
    pub trait Port {
        /// Returns the name of the port.
        fn address(&self) -> String;
    }
    /// Helper trait to convert an option to an error.
    /// Polyfill for the `Try` trait until it is stable.
    pub trait OrErr<T> {
        /// Consumes the `Option<T>` and returns an `Ev3Result<T>`.
        fn or_err(self) -> Ev3Result<T>;
    }
    impl<T> OrErr<T> for Option<T> {
        fn or_err(self) -> Ev3Result<T> {
            self.ok_or(Ev3Error::InternalError {
                msg: "Cannot unwrap option".to_owned(),
            })
        }
    }
}
pub use utils::{Ev3Error, Ev3Result, Port};
pub mod wait {
    //! Utility functions for cpu efficent `wait` commands.
    //! Uses the `libc::epoll_wait` that only works on linux systems.
    use libc;
    use std::os::unix::io::RawFd;
    use std::time::{Duration, Instant};
    /// Wait for until a condition `cond` is `true` or the `timeout` is reached.
    /// If the `timeout` is `None` it will wait an infinite time.
    /// The condition is checked when the `file` has changed.
    ///
    /// # Arguments
    /// * `file` - Listen to changes in this file
    /// * `cond` - Condition that should become true
    /// * `timeout` - Maximal timeout to wait for the condition or file changes
    ///
    /// # Example
    /// ```
    /// use std::fs::File;
    /// use std::os::unix::io::AsRawFd;
    /// use std::time::Duration;
    ///
    /// use ev3dev_lang_rust::wait;
    ///
    /// if let Ok(file) = File::open("...") {
    ///     let cond = || {
    ///         // ...
    ///         true
    ///     };
    ///     let timeout = Duration::from_millis(2000);
    ///
    ///     wait::wait(file.as_raw_fd(), cond, Some(timeout));
    /// }
    /// ```
    pub fn wait<F>(fd: RawFd, cond: F, timeout: Option<Duration>) -> bool
    where
        F: Fn() -> bool,
    {
        if cond() {
            return true;
        }
        let start = Instant::now();
        let mut t = timeout;
        loop {
            let wait_timeout = match t {
                Some(duration) => duration.as_millis() as i32,
                None => -1,
            };
            wait_file_changes(fd, wait_timeout);
            if let Some(duration) = timeout {
                let elapsed = start.elapsed();
                if elapsed >= duration {
                    return false;
                }
                t = Some(duration - elapsed);
            }
            if cond() {
                return true;
            }
        }
    }
    /// Wrapper for `libc::epoll_wait`
    fn wait_file_changes(fd: RawFd, timeout: i32) -> bool {
        let mut buf: [libc::epoll_event; 10] = [libc::epoll_event { events: 0, u64: 0 }; 10];
        let result = unsafe {
            libc::epoll_wait(
                fd,
                buf.as_mut_ptr() as *mut libc::epoll_event,
                buf.len() as i32,
                timeout,
            ) as i32
        };
        result > 0
    }
}
pub mod motors {
    //! # Container module for motor types
    pub mod dc_motor {
        //! The DcMotor trait provides a uniform interface for using
        //! regular DC motors with no fancy controls or feedback.
        //! This includes LEGO MINDSTORMS RCX motors and LEGO Power Functions motors.
        use super::Motor;
        use crate::Ev3Result;
        use std::time::Duration;
        /// Causes the motor to run until another command is sent.
        pub const COMMAND_RUN_FOREVER: &str = "run-forever";
        /// Run the motor for the amount of time specified in `time_sp`
        /// and then stops the motor using the command specified by `stop_action`.
        pub const COMMAND_RUN_TIMED: &str = "run-timed";
        /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
        /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
        pub const COMMAND_RUN_DIRECT: &str = "run-direct";
        /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
        pub const COMMAND_STOP: &str = "stop";
        /// A positive duty cycle will cause the motor to rotate clockwise.
        pub const POLARITY_NORMAL: &str = "normal";
        /// A positive duty cycle will cause the motor to rotate counter-clockwise.
        pub const POLARITY_INVERSED: &str = "inversed";
        /// Power is being sent to the motor.
        pub const STATE_RUNNING: &str = "running";
        /// The motor is ramping up or down and has not yet reached a pub constant output level.
        pub const STATE_RAMPING: &str = "ramping";
        /// Removes power from the motor. The motor will freely coast to a stop.
        pub const STOP_ACTION_COAST: &str = "coast";
        /// Removes power from the motor and creates a passive electrical load.
        /// This is usually done by shorting the motor terminals together.
        /// This load will absorb the energy from the rotation of the motors
        /// and cause the motor to stop more quickly than coasting.
        pub const STOP_ACTION_BRAKE: &str = "brake";
        /// The DcMotor trait provides a uniform interface for using
        /// regular DC motors with no fancy controls or feedback.
        /// This includes LEGO MINDSTORMS RCX motors and LEGO Power Functions motors.
        pub trait DcMotor: Motor {
            /// Returns the current duty cycle of the motor. Units are percent. Values are -100 to 100.
            fn get_duty_cycle(&self) -> Ev3Result<i32> {
                self.get_attribute("duty_cycle").get()
            }
            /// Returns the current duty cycle setpoint of the motor. Units are in percent.
            /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
            fn get_duty_cycle_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("duty_cycle_sp").get()
            }
            /// Sets the duty cycle setpoint of the motor. Units are in percent.
            /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
            fn set_duty_cycle_sp(&self, duty_cycle_sp: i32) -> Ev3Result<()> {
                self.get_attribute("duty_cycle_sp").set(duty_cycle_sp)
            }
            /// Returns the current polarity of the motor.
            fn get_polarity(&self) -> Ev3Result<String> {
                self.get_attribute("polarity").get()
            }
            /// Sets the polarity of the motor.
            fn set_polarity(&self, polarity: &str) -> Ev3Result<()> {
                self.get_attribute("polarity").set_str_slice(polarity)
            }
            /// Returns the current ramp up setpoint.
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
            fn get_ramp_up_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("ramp_up_sp").get()
            }
            /// Sets the ramp up setpoint.
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
            fn set_ramp_up_sp(&self, ramp_up_sp: i32) -> Ev3Result<()> {
                self.get_attribute("ramp_up_sp").set(ramp_up_sp)
            }
            /// Returns the current ramp down setpoint.
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
            fn get_ramp_down_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("ramp_down_sp").get()
            }
            /// Sets the ramp down setpoint.
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
            fn set_ramp_down_sp(&self, ramp_down_sp: i32) -> Ev3Result<()> {
                self.get_attribute("ramp_down_sp").set(ramp_down_sp)
            }
            /// Returns a list of state flags.
            fn get_state(&self) -> Ev3Result<Vec<String>> {
                self.get_attribute("state").get_vec()
            }
            /// Returns the current stop action.
            /// The value determines the motors behavior when command is set to stop.
            fn get_stop_action(&self) -> Ev3Result<String> {
                self.get_attribute("stop_action").get()
            }
            /// Sets the stop action.
            /// The value determines the motors behavior when command is set to stop.
            fn set_stop_action(&self, stop_action: &str) -> Ev3Result<()> {
                self.get_attribute("stop_action").set_str_slice(stop_action)
            }
            /// Returns the current amount of time the motor will run when using the run-timed command.
            /// Units are in milliseconds. Values must not be negative.
            fn get_time_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("time_sp").get()
            }
            /// Sets the amount of time the motor will run when using the run-timed command.
            /// Units are in milliseconds. Values must not be negative.
            fn set_time_sp(&self, time_sp: i32) -> Ev3Result<()> {
                self.get_attribute("time_sp").set(time_sp)
            }
            /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
            /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
            fn run_direct(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_RUN_DIRECT)
            }
            /// Causes the motor to run until another command is sent.
            fn run_forever(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_RUN_FOREVER)
            }
            /// Run the motor for the amount of time specified in `time_sp`
            /// and then stops the motor using the command specified by `stop_action`.
            fn run_timed(&self, time_sp: Option<Duration>) -> Ev3Result<()> {
                if let Some(duration) = time_sp {
                    let p = duration.as_millis() as i32;
                    self.set_time_sp(p)?;
                }
                self.set_command(COMMAND_RUN_TIMED)
            }
            /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
            fn stop(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_STOP)
            }
            /// Power is being sent to the motor.
            fn is_running(&self) -> Ev3Result<bool> {
                Ok(self.get_state()?.iter().any(|state| state == STATE_RUNNING))
            }
            /// The motor is ramping up or down and has not yet reached a pub constant output level.
            fn is_ramping(&self) -> Ev3Result<bool> {
                Ok(self.get_state()?.iter().any(|state| state == STATE_RAMPING))
            }
        }
    }
    mod large_motor {
        use super::{Motor, TachoMotor};
        use crate::{Attribute, Device, Driver, Ev3Result, Findable};
        /// EV3/NXT large servo motor
        //#[class_name = "tacho-motor"]
        //#[driver_name = "lego-ev3-l-motor"]
        //#[port = "crate::motors::MotorPort"]
        pub struct LargeMotor {
            driver: Driver,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for LargeMotor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    LargeMotor {
                        driver: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("LargeMotor");
                        let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for LargeMotor {
            #[inline]
            fn clone(&self) -> LargeMotor {
                match *self {
                    LargeMotor {
                        driver: ref __self_0_0,
                    } => LargeMotor {
                        driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Device for LargeMotor {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
        impl Findable<crate::motors::MotorPort> for LargeMotor {
            fn get(port: crate::motors::MotorPort) -> Ev3Result<Self> {
                let name =
                    Driver::find_name_by_port_and_driver("tacho-motor", &port, "lego-ev3-l-motor")?;
                Ok(LargeMotor {
                    driver: Driver::new("tacho-motor", &name),
                })
            }
            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver("tacho-motor", "lego-ev3-l-motor")?;
                Ok(LargeMotor {
                    driver: Driver::new("tacho-motor", &name),
                })
            }
            fn list() -> Ev3Result<Vec<Self>> {
                Ok(
                    Driver::find_names_by_driver("tacho-motor", "lego-ev3-l-motor")?
                        .iter()
                        .map(|name| LargeMotor {
                            driver: Driver::new("tacho-motor", name),
                        })
                        .collect(),
                )
            }
        }
        impl Motor for LargeMotor {}
        impl TachoMotor for LargeMotor {}
    }
    mod medium_motor {
        use super::{Motor, TachoMotor};
        use crate::{Attribute, Device, Driver, Ev3Result, Findable};
        /// EV3 medium servo motor
        //#[class_name = "tacho-motor"]
        //#[driver_name = "lego-ev3-m-motor"]
        //#[port = "crate::motors::MotorPort"]
        pub struct MediumMotor {
            driver: Driver,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for MediumMotor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    MediumMotor {
                        driver: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("MediumMotor");
                        let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for MediumMotor {
            #[inline]
            fn clone(&self) -> MediumMotor {
                match *self {
                    MediumMotor {
                        driver: ref __self_0_0,
                    } => MediumMotor {
                        driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Device for MediumMotor {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
        impl Findable<crate::motors::MotorPort> for MediumMotor {
            fn get(port: crate::motors::MotorPort) -> Ev3Result<Self> {
                let name =
                    Driver::find_name_by_port_and_driver("tacho-motor", &port, "lego-ev3-m-motor")?;
                Ok(MediumMotor {
                    driver: Driver::new("tacho-motor", &name),
                })
            }
            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver("tacho-motor", "lego-ev3-m-motor")?;
                Ok(MediumMotor {
                    driver: Driver::new("tacho-motor", &name),
                })
            }
            fn list() -> Ev3Result<Vec<Self>> {
                Ok(
                    Driver::find_names_by_driver("tacho-motor", "lego-ev3-m-motor")?
                        .iter()
                        .map(|name| MediumMotor {
                            driver: Driver::new("tacho-motor", name),
                        })
                        .collect(),
                )
            }
        }
        impl Motor for MediumMotor {}
        impl TachoMotor for MediumMotor {}
    }
    pub mod servo_motor {
        //! The ServoMotor trait provides a uniform interface for using hobby type servo motors.
        use super::Motor;
        use crate::Ev3Result;
        /// Remove power from the motor.
        pub const COMMAND_RUN: &str = "run";
        /// Drive servo to the position set in the position_sp attribute.
        pub const COMMAND_FLOAT: &str = "float";
        /// With normal polarity, a positive duty cycle will cause the motor to rotate clockwise.
        pub const POLARITY_NORMAL: &str = "normal";
        /// With inversed polarity, a positive duty cycle will cause the motor to rotate counter-clockwise.
        pub const POLARITY_INVERSED: &str = "inversed";
        /// Power is being sent to the motor.
        pub const STATE_RUNNING: &str = "running";
        /// The ServoMotor trait provides a uniform interface for using hobby type servo motors.
        pub trait ServoMotor: Motor {
            /// Returns the current polarity of the motor.
            fn get_polarity(&self) -> Ev3Result<String> {
                self.get_attribute("polarity").get()
            }
            /// Sets the polarity of the motor.
            fn set_polarity(&self, polarity: &str) -> Ev3Result<()> {
                self.get_attribute("polarity").set_str_slice(polarity)
            }
            /// Returns the current max pulse setpoint.
            /// Used to set the pulse size in milliseconds for the signal
            /// that tells the servo to drive to the maximum (clockwise) position_sp.
            /// Default value is 2400. Valid values are 2300 to 2700.
            /// You must write to the position_sp attribute for changes to this attribute to take effect.
            fn get_max_pulse_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("max_pulse_sp").get()
            }
            /// Sets the max pulse setpoint.
            /// Used to set the pulse size in milliseconds for the signal
            /// that tells the servo to drive to the maximum (clockwise) position_sp.
            /// Default value is 2400. Valid values are 2300 to 2700.
            /// You must write to the position_sp attribute for changes to this attribute to take effect.
            fn set_max_pulse_sp(&self, max_pulse_sp: i32) -> Ev3Result<()> {
                self.get_attribute("max_pulse_sp").set(max_pulse_sp)
            }
            /// Returns the current mid pulse setpoint.
            /// Used to set the pulse size in milliseconds for the signal
            /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
            /// Default value is 600.
            /// Valid values are 300 to 700.
            ///  You must write to the position_sp attribute for changes to this attribute to take effect.
            fn get_mid_pulse_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("mid_pulse_sp").get()
            }
            /// Sets the mid pulse setpoint.
            /// Used to set the pulse size in milliseconds for the signal
            /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
            /// Default value is 600.
            /// Valid values are 300 to 700.
            ///  You must write to the position_sp attribute for changes to this attribute to take effect.
            fn set_mid_pulse_sp(&self, max_pulse_sp: i32) -> Ev3Result<()> {
                self.get_attribute("mid_pulse_sp").set(max_pulse_sp)
            }
            /// Returns the current min pulse setpoint.
            /// Used to set the pulse size in milliseconds for the signal
            /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
            /// Default value is 600. Valid values are 300 to 700.
            /// You must write to the position_sp attribute for changes to this attribute to take effect.
            fn get_min_pulse_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("min_pulse_sp").get()
            }
            /// Sets the min pulse setpoint.
            /// Used to set the pulse size in milliseconds for the signal
            /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
            /// Default value is 600. Valid values are 300 to 700.
            /// You must write to the position_sp attribute for changes to this attribute to take effect.
            fn set_min_pulse_sp(&self, min_pulse_sp: i32) -> Ev3Result<()> {
                self.get_attribute("min_pulse_sp").set(min_pulse_sp)
            }
            /// Returns the current target position for the `run-to-abs-pos` and `run-to-rel-pos` commands. Units are in tacho counts.
            /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
            /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
            fn get_position_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("position_sp").get()
            }
            /// Sets the target position for the `run-to-abs-pos` and `run-to-rel-pos` commands.
            /// Units are in tacho counts.
            /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
            /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
            fn set_position_sp(&self, position_sp: i32) -> Ev3Result<()> {
                self.get_attribute("position_sp").set(position_sp)
            }
            /// Returns the current the rate_sp at which the servo travels from 0 to 100.0%
            /// (half of the full range of the servo).
            /// Units are in milliseconds.
            ///
            /// ## Example:
            ///
            /// Setting the rate_sp to 1000 means that it will take a 180
            /// degree servo 2 second to move from 0 to 180 degrees.
            ///
            /// ## Note:
            ///
            /// Some servo controllers may not support this in which case
            /// reading and writing will fail with -EOPNOTSUPP.
            /// In continuous rotation servos, this value will affect the
            /// rate_sp at which the speed ramps up or down.
            fn get_rate_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("rate_sp").get()
            }
            /// Sets the rate_sp at which the servo travels from 0 to 100.0%
            /// (half of the full range of the servo).
            /// Units are in milliseconds.
            ///
            /// ## Example:
            ///
            /// Setting the rate_sp to 1000 means that it will take a 180
            /// degree servo 2 second to move from 0 to 180 degrees.
            ///
            /// ## Note:
            ///
            /// Some servo controllers may not support this in which case
            /// reading and writing will fail with -EOPNOTSUPP.
            /// In continuous rotation servos, this value will affect the
            /// rate_sp at which the speed ramps up or down.
            fn set_rate_sp(&self, rate_sp: i32) -> Ev3Result<()> {
                self.get_attribute("rate_sp").set(rate_sp)
            }
            /// Returns a list of state flags.
            fn get_state(&self) -> Ev3Result<Vec<String>> {
                self.get_attribute("state").get_vec()
            }
            /// Power is being sent to the motor.
            fn is_running(&self) -> Ev3Result<bool> {
                Ok(self.get_state()?.iter().any(|state| state == STATE_RUNNING))
            }
            /// Drive servo to the position set in the `position_sp` attribute.
            fn run(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_RUN)
            }
            /// Remove power from the motor.
            fn float(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_FLOAT)
            }
        }
    }
    pub mod tacho_motor {
        //! The TachoMotor trait provides a uniform interface for using motors with positional
        //! and directional feedback such as the EV3 and NXT motors.
        //! This feedback allows for precise control of the motors.
        use super::Motor;
        use crate::{wait, Ev3Result};
        use std::time::Duration;
        /// Causes the motor to run until another command is sent.
        pub const COMMAND_RUN_FOREVER: &str = "run-forever";
        /// Runs the motor to an absolute position specified by `position_sp`
        /// and then stops the motor using the command specified in `stop_action`.
        pub const COMMAND_RUN_TO_ABS_POS: &str = "run-to-abs-pos";
        /// Runs the motor to a position relative to the current position value.
        /// The new position will be current `position` + `position_sp`.
        /// When the new position is reached, the motor will stop using the command specified by `stop_action`.
        pub const COMMAND_RUN_TO_REL_POS: &str = "run-to-rel-pos";
        /// Run the motor for the amount of time specified in `time_sp`
        /// and then stops the motor using the command specified by `stop_action`.
        pub const COMMAND_RUN_TIMED: &str = "run-timed";
        /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
        /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
        pub const COMMAND_RUN_DIRECT: &str = "run-direct";
        /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
        pub const COMMAND_STOP: &str = "stop";
        /// Resets all of the motor parameter attributes to their default values.
        /// This will also have the effect of stopping the motor.
        pub const COMMAND_RESET: &str = "reset";
        /// A positive duty cycle will cause the motor to rotate clockwise.
        pub const POLARITY_NORMAL: &str = "normal";
        /// A positive duty cycle will cause the motor to rotate counter-clockwise.
        pub const POLARITY_INVERSED: &str = "inversed";
        /// Power is being sent to the motor.
        pub const STATE_RUNNING: &str = "running";
        /// The motor is ramping up or down and has not yet reached a pub constant output level.
        pub const STATE_RAMPING: &str = "ramping";
        /// The motor is not turning, but rather attempting to hold a fixed position.
        pub const STATE_HOLDING: &str = "holding";
        /// The motor is turning as fast as possible, but cannot reach its `speed_sp`.
        pub const STATE_OVERLOADED: &str = "overloaded";
        /// The motor is trying to run but is not turning at all.
        pub const STATE_STALLED: &str = "stalled";
        /// Removes power from the motor. The motor will freely coast to a stop.
        pub const STOP_ACTION_COAST: &str = "coast";
        /// Removes power from the motor and creates a passive electrical load.
        /// This is usually done by shorting the motor terminals together.
        /// This load will absorb the energy from the rotation of the motors
        /// and cause the motor to stop more quickly than coasting.
        pub const STOP_ACTION_BRAKE: &str = "brake";
        /// Causes the motor to actively try to hold the current position.
        /// If an external force tries to turn the motor, the motor will “push back” to maintain its position.
        pub const STOP_ACTION_HOLD: &str = "hold";
        /// The TachoMotor trait provides a uniform interface for using motors with positional
        /// and directional feedback such as the EV3 and NXT motors.
        /// This feedback allows for precise control of the motors.
        pub trait TachoMotor: Motor {
            /// Returns the number of tacho counts in one rotation of the motor.
            ///
            /// Tacho counts are used by the position and speed attributes,
            /// so you can use this value to convert from rotations or degrees to tacho counts.
            /// (rotation motors only)
            ///
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// // Get position and count_per_rot as f32.
            /// let position = motor.get_position()? as f32;
            /// let count_per_rot = motor.get_count_per_rot()? as f32;
            ///
            /// // Calculate the rotation count.
            /// let rotations = position / count_per_rot;
            ///
            /// println!("The motor did {:.2} rotations", rotations);
            /// # Ok(())
            /// # }
            /// ```
            fn get_count_per_rot(&self) -> Ev3Result<i32> {
                self.get_attribute("count_per_rot").get()
            }
            /// Returns the number of tacho counts in one meter of travel of the motor.
            ///
            /// Tacho counts are used by the position and speed attributes,
            /// so you can use this value to convert from distance to tacho counts.
            /// (linear motors only)
            fn get_count_per_m(&self) -> Ev3Result<i32> {
                self.get_attribute("count_per_m").get()
            }
            /// Returns the number of tacho counts in the full travel of the motor.
            ///
            /// When combined with the count_per_m atribute,
            /// you can use this value to calculate the maximum travel distance of the motor.
            /// (linear motors only)
            fn get_full_travel_count(&self) -> Ev3Result<i32> {
                self.get_attribute("full_travel_count").get()
            }
            /// Returns the current duty cycle of the motor. Units are percent.
            ///
            /// Values are -100 to 100.
            ///
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use std::thread;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// // Set the motor command `run-direct` to start rotation.
            /// motor.run_direct()?;
            ///
            /// // Rotate motor forward and wait 5 seconds.
            /// motor.set_duty_cycle_sp(50)?;
            /// thread::sleep(Duration::from_secs(5));
            ///
            /// assert_eq!(motor.get_duty_cycle()?, 50);
            /// # Ok(())
            /// # }
            fn get_duty_cycle(&self) -> Ev3Result<i32> {
                self.get_attribute("duty_cycle").get()
            }
            /// Returns the current duty cycle setpoint of the motor.
            ///
            /// Units are in percent.
            /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
            ///
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use std::thread;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// // Rotate motor forward and wait 5 seconds.
            /// motor.set_duty_cycle_sp(50)?;
            /// thread::sleep(Duration::from_secs(5));
            ///
            /// assert_eq!(motor.get_duty_cycle()?, 50);
            /// # Ok(())
            /// # }
            fn get_duty_cycle_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("duty_cycle_sp").get()
            }
            /// Sets the duty cycle setpoint of the motor.
            ///
            /// Units are in percent.
            /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
            ///    
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use std::thread;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// // Set the motor command `run-direct` to start rotation.
            /// motor.run_direct()?;
            ///
            /// // Rotate motor forward and wait 5 seconds.
            /// motor.set_duty_cycle_sp(50)?;
            /// thread::sleep(Duration::from_secs(5));
            ///
            /// // Rotate motor backward and wait 5 seconds.
            /// motor.set_duty_cycle_sp(-50)?;
            /// thread::sleep(Duration::from_secs(5));
            /// # Ok(())
            /// # }
            fn set_duty_cycle_sp(&self, duty_cycle: i32) -> Ev3Result<()> {
                self.get_attribute("duty_cycle_sp").set(duty_cycle)
            }
            /// Returns the current polarity of the motor.
            fn get_polarity(&self) -> Ev3Result<String> {
                self.get_attribute("polarity").get()
            }
            /// Sets the polarity of the motor.
            fn set_polarity(&self, polarity: &str) -> Ev3Result<()> {
                self.get_attribute("polarity").set_str_slice(polarity)
            }
            /// Returns the current position of the motor in pulses of the rotary encoder.
            ///
            /// When the motor rotates clockwise, the position will increase.
            /// Likewise, rotating counter-clockwise causes the position to decrease.
            /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
            ///
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// // Get position and count_per_rot as f32.
            /// let position = motor.get_position()? as f32;
            /// let count_per_rot = motor.get_count_per_rot()? as f32;
            ///
            /// // Calculate the rotation count.
            /// let rotations: f32 = position / count_per_rot;
            ///
            /// println!("The motor did {:.2} rotations", rotations);
            /// # Ok(())
            /// # }
            /// ```
            fn get_position(&self) -> Ev3Result<i32> {
                self.get_attribute("position").get()
            }
            /// Sets the current position of the motor in pulses of the rotary encoder.
            ///
            /// When the motor rotates clockwise, the position will increase.
            /// Likewise, rotating counter-clockwise causes the position to decrease.
            /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
            ///
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// motor.set_position(0)?;
            /// let position = motor.get_position()?;
            ///
            /// // If the motor is not moving, the position value
            /// // should not change between set and get operation.
            /// assert_eq!(position, 0);
            /// # Ok(())
            /// # }
            /// ```
            fn set_position(&self, position: i32) -> Ev3Result<()> {
                self.get_attribute("position").set(position)
            }
            /// Returns the proportional pub constant for the position PID.
            fn get_hold_pid_kp(&self) -> Ev3Result<f32> {
                self.get_attribute("hold_pid_kp").get()
            }
            /// Sets the proportional pub constant for the position PID.
            fn set_hold_pid_kp(&self, kp: f32) -> Ev3Result<()> {
                self.get_attribute("hold_pid_kp").set(kp)
            }
            /// Returns the integral pub constant for the position PID.
            fn get_hold_pid_ki(&self) -> Ev3Result<f32> {
                self.get_attribute("hold_pid_ki").get()
            }
            /// Sets the integral pub constant for the position PID.
            fn set_hold_pid_ki(&self, ki: f32) -> Ev3Result<()> {
                self.get_attribute("hold_pid_ki").set(ki)
            }
            /// Returns the derivative pub constant for the position PID.
            fn get_hold_pid_kd(&self) -> Ev3Result<f32> {
                self.get_attribute("hold_pid_kd").get()
            }
            /// Sets the derivative pub constant for the position PID.
            fn set_hold_pid_kd(&self, kd: f32) -> Ev3Result<()> {
                self.get_attribute("hold_pid_kd").set(kd)
            }
            /// Returns the maximum value that is accepted by the `speed_sp` attribute.
            ///
            /// This value is the speed of the motor at 9V with no load.
            /// Note: The actual maximum obtainable speed will be less than this
            /// and will depend on battery voltage and mechanical load on the motor.
            fn get_max_speed(&self) -> Ev3Result<i32> {
                self.get_attribute("max_speed").get()
            }
            /// Returns the current target position for the `run-to-abs-pos` and `run-to-rel-pos` commands.
            ///
            /// Units are in tacho counts.
            /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
            ///
            /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
            fn get_position_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("position_sp").get()
            }
            /// Sets the target position for the `run-to-abs-pos` and `run-to-rel-pos` commands.
            ///
            /// Units are in tacho counts.
            /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
            ///
            /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
            ///
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use std::thread;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// // Save the current position.
            /// let old_position = motor.get_position()?;
            ///
            /// // Rotate by 100 ticks
            /// let position = motor.set_position_sp(100)?;
            /// motor.run_to_rel_pos(None)?;
            ///
            /// // Wait till rotation is finished.
            /// motor.wait_until_not_moving(None);
            ///
            /// // The new position should be 100 ticks larger.
            /// let new_position = motor.get_position()?;
            /// assert_eq!(old_position + 100, new_position);
            /// # Ok(())
            /// # }
            /// ```
            fn set_position_sp(&self, position_sp: i32) -> Ev3Result<()> {
                self.get_attribute("position_sp").set(position_sp)
            }
            /// Returns the current motor speed in tacho counts per second.
            ///
            /// Note, this is not necessarily degrees (although it is for LEGO motors).
            /// Use the `count_per_rot` attribute to convert this value to RPM or deg/sec.
            fn get_speed(&self) -> Ev3Result<i32> {
                self.get_attribute("speed").get()
            }
            /// Returns the target speed in tacho counts per second used for all run-* commands except run-direct.
            ///
            /// A negative value causes the motor to rotate in reverse
            /// with the exception of run-to-*-pos commands where the sign is ignored.
            /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
            /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
            fn get_speed_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("speed_sp").get()
            }
            /// Sets the target speed in tacho counts per second used for all run-* commands except run-direct.
            ///
            /// A negative value causes the motor to rotate in reverse
            /// with the exception of run-to-*-pos commands where the sign is ignored.
            /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
            /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
            fn set_speed_sp(&self, speed_sp: i32) -> Ev3Result<()> {
                self.get_attribute("speed_sp").set(speed_sp)
            }
            /// Returns the current ramp up setpoint.
            ///
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
            fn get_ramp_up_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("ramp_up_sp").get()
            }
            /// Sets the ramp up setpoint.
            ///
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
            fn set_ramp_up_sp(&self, ramp_up_sp: i32) -> Ev3Result<()> {
                self.get_attribute("ramp_up_sp").set(ramp_up_sp)
            }
            /// Returns the current ramp down setpoint.
            ///
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
            fn get_ramp_down_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("ramp_down_sp").get()
            }
            /// Sets the ramp down setpoint.
            ///
            /// Units are in milliseconds and must be positive. When set to a non-zero value,
            /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
            /// The actual ramp time is the ratio of the difference between the speed_sp
            /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
            fn set_ramp_down_sp(&self, ramp_down_sp: i32) -> Ev3Result<()> {
                self.get_attribute("ramp_down_sp").set(ramp_down_sp)
            }
            /// Returns the proportional pub constant for the speed regulation PID.
            fn get_speed_pid_kp(&self) -> Ev3Result<f32> {
                self.get_attribute("speed_pid_kp").get()
            }
            /// Sets the proportional pub constant for the speed regulation PID.
            fn set_speed_pid_kp(&self, kp: f32) -> Ev3Result<()> {
                self.get_attribute("speed_pid_kp").set(kp)
            }
            /// Returns the integral pub constant for the speed regulation PID.
            fn get_speed_pid_ki(&self) -> Ev3Result<f32> {
                self.get_attribute("speed_pid_ki").get()
            }
            /// Sets the integral pub constant for the speed regulation PID.
            fn set_speed_pid_ki(&self, ki: f32) -> Ev3Result<()> {
                self.get_attribute("speed_pid_ki").set(ki)
            }
            /// Returns the derivative pub constant for the speed regulation PID.
            fn get_speed_pid_kd(&self) -> Ev3Result<f32> {
                self.get_attribute("speed_pid_kd").get()
            }
            /// Sets the derivative pub constant for the speed regulation PID.
            fn set_speed_pid_kd(&self, kd: f32) -> Ev3Result<()> {
                self.get_attribute("speed_pid_kd").set(kd)
            }
            /// Returns a list of state flags.
            fn get_state(&self) -> Ev3Result<Vec<String>> {
                self.get_attribute("state").get_vec()
            }
            /// Returns the current stop action.
            ///
            /// The value determines the motors behavior when command is set to stop.
            fn get_stop_action(&self) -> Ev3Result<String> {
                self.get_attribute("stop_action").get()
            }
            /// Sets the stop action.
            ///
            /// The value determines the motors behavior when command is set to stop.
            fn set_stop_action(&self, stop_action: &str) -> Ev3Result<()> {
                self.get_attribute("stop_action").set_str_slice(stop_action)
            }
            /// Returns a list of stop actions supported by the motor controller.
            fn get_stop_actions(&self) -> Ev3Result<Vec<String>> {
                self.get_attribute("stop_actions").get_vec()
            }
            /// Returns the current amount of time the motor will run when using the run-timed command.
            ///
            /// Units are in milliseconds. Values must not be negative.
            fn get_time_sp(&self) -> Ev3Result<i32> {
                self.get_attribute("time_sp").get()
            }
            /// Sets the amount of time the motor will run when using the run-timed command.
            ///
            /// Units are in milliseconds. Values must not be negative.
            fn set_time_sp(&self, time_sp: i32) -> Ev3Result<()> {
                self.get_attribute("time_sp").set(time_sp)
            }
            /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
            ///
            /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
            fn run_direct(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_RUN_DIRECT)
            }
            /// Causes the motor to run until another command is sent.
            fn run_forever(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_RUN_FOREVER)
            }
            /// Runs the motor to an absolute position specified by `position_sp`
            ///
            /// and then stops the motor using the command specified in `stop_action`.
            fn run_to_abs_pos(&self, position_sp: Option<i32>) -> Ev3Result<()> {
                if let Some(p) = position_sp {
                    self.set_position_sp(p)?;
                }
                self.set_command(COMMAND_RUN_TO_ABS_POS)
            }
            /// Runs the motor to a position relative to the current position value.
            ///
            /// The new position will be current `position` + `position_sp`.
            /// When the new position is reached, the motor will stop using the command specified by `stop_action`.
            fn run_to_rel_pos(&self, position_sp: Option<i32>) -> Ev3Result<()> {
                if let Some(p) = position_sp {
                    self.set_position_sp(p)?;
                }
                self.set_command(COMMAND_RUN_TO_REL_POS)
            }
            /// Run the motor for the amount of time specified in `time_sp`
            ///
            /// and then stops the motor using the command specified by `stop_action`.
            fn run_timed(&self, time_sp: Option<Duration>) -> Ev3Result<()> {
                if let Some(duration) = time_sp {
                    let p = duration.as_millis() as i32;
                    self.set_time_sp(p)?;
                }
                self.set_command(COMMAND_RUN_TIMED)
            }
            /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
            fn stop(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_STOP)
            }
            /// Resets all of the motor parameter attributes to their default values.
            /// This will also have the effect of stopping the motor.
            fn reset(&self) -> Ev3Result<()> {
                self.set_command(COMMAND_RESET)
            }
            /// Power is being sent to the motor.
            fn is_running(&self) -> Ev3Result<bool> {
                Ok(self.get_state()?.iter().any(|state| state == STATE_RUNNING))
            }
            /// The motor is ramping up or down and has not yet reached a pub constant output level.
            fn is_ramping(&self) -> Ev3Result<bool> {
                Ok(self.get_state()?.iter().any(|state| state == STATE_RAMPING))
            }
            /// The motor is not turning, but rather attempting to hold a fixed position.
            fn is_holding(&self) -> Ev3Result<bool> {
                Ok(self.get_state()?.iter().any(|state| state == STATE_HOLDING))
            }
            /// The motor is turning as fast as possible, but cannot reach its `speed_sp`.
            fn is_overloaded(&self) -> Ev3Result<bool> {
                Ok(self
                    .get_state()?
                    .iter()
                    .any(|state| state == STATE_OVERLOADED))
            }
            /// The motor is trying to run but is not turning at all.
            fn is_stalled(&self) -> Ev3Result<bool> {
                Ok(self.get_state()?.iter().any(|state| state == STATE_STALLED))
            }
            /// Wait until condition `cond` returns true or the `timeout` is reached.
            ///
            /// The condition is checked when to the `state` attribute has changed.
            /// If the `timeout` is `None` it will wait an infinite time.
            ///
            /// # Examples
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use ev3dev_lang_rust::motors::tacho_motor;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// motor.run_timed(Some(Duration::from_secs(5)))?;
            ///
            /// let cond = || {
            ///     motor.get_state()
            ///         .unwrap_or_else(|_| vec![])
            ///         .iter()
            ///         .all(|s| s != tacho_motor::STATE_RUNNING)
            /// };
            /// motor.wait(cond, None);
            ///
            /// println!("Motor has stopped!");
            /// # Ok(())
            /// # }
            /// ```
            fn wait<F>(&self, cond: F, timeout: Option<Duration>) -> bool
            where
                F: Fn() -> bool,
            {
                let fd = self.get_attribute("state").get_raw_fd();
                wait::wait(fd, cond, timeout)
            }
            /// Wait while the `state` is in the vector `self.get_state()` or the `timeout` is reached.
            ///
            /// If the `timeout` is `None` it will wait an infinite time.
            ///
            /// # Example
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use ev3dev_lang_rust::motors::tacho_motor;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// motor.run_timed(Some(Duration::from_secs(5)))?;
            ///
            /// motor.wait_while(tacho_motor::STATE_RUNNING, None);
            ///
            /// println!("Motor has stopped!");
            /// # Ok(())
            /// # }
            /// ```
            fn wait_while(&self, state: &str, timeout: Option<Duration>) -> bool {
                let cond = || {
                    self.get_state()
                        .unwrap_or_else(|_| <[_]>::into_vec(box []))
                        .iter()
                        .all(|s| s != state)
                };
                self.wait(cond, timeout)
            }
            /// Wait until the `state` is in the vector `self.get_state()` or the `timeout` is reached.
            ///
            /// If the `timeout` is `None` it will wait an infinite time.
            ///
            /// # Example
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use ev3dev_lang_rust::motors::tacho_motor;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// motor.run_timed(Some(Duration::from_secs(5)))?;
            ///
            /// motor.wait_until(tacho_motor::STATE_RUNNING, None);
            ///
            /// println!("Motor has started!");
            /// # Ok(())
            /// # }
            /// ```
            fn wait_until(&self, state: &str, timeout: Option<Duration>) -> bool {
                let cond = || {
                    self.get_state()
                        .unwrap_or_else(|_| <[_]>::into_vec(box []))
                        .iter()
                        .any(|s| s == state)
                };
                self.wait(cond, timeout)
            }
            /// Wait until the motor is not moving or the timeout is reached.
            ///
            /// This is euqal to `wait_while(STATE_RUNNING, timeout)`.
            /// If the `timeout` is `None` it will wait an infinite time.
            ///
            /// # Example
            ///
            /// ```no_run
            /// use ev3dev_lang_rust::prelude::*;
            /// use ev3dev_lang_rust::motors::LargeMotor;
            /// use std::time::Duration;
            ///
            /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
            /// // Init a tacho motor.
            /// let motor = LargeMotor::find()?;
            ///
            /// motor.run_timed(Some(Duration::from_secs(5)))?;
            ///
            /// motor.wait_until_not_moving(None);
            ///
            /// println!("Motor has stopped!");
            /// # Ok(())
            /// # }
            /// ```
            fn wait_until_not_moving(&self, timeout: Option<Duration>) -> bool {
                self.wait_while(STATE_RUNNING, timeout)
            }
        }
    }
    pub use self::dc_motor::DcMotor;
    pub use self::large_motor::LargeMotor;
    pub use self::medium_motor::MediumMotor;
    pub use self::servo_motor::ServoMotor;
    pub use self::tacho_motor::TachoMotor;
    use crate::{Device, Port};
    /// Container trait to indicate something is a motor
    pub trait Motor: Device {}
    /// EV3 ports `outA` to `outD`
    pub enum MotorPort {
        /// EV3 `outA` port
        OutA,

        /// EV3 `outB` port
        OutB,

        /// EV3 `outC` port
        OutC,

        /// EV3 `outD` port
        OutD,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MotorPort {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&MotorPort::OutA,) => {
                    let mut debug_trait_builder = f.debug_tuple("OutA");
                    debug_trait_builder.finish()
                }
                (&MotorPort::OutB,) => {
                    let mut debug_trait_builder = f.debug_tuple("OutB");
                    debug_trait_builder.finish()
                }
                (&MotorPort::OutC,) => {
                    let mut debug_trait_builder = f.debug_tuple("OutC");
                    debug_trait_builder.finish()
                }
                (&MotorPort::OutD,) => {
                    let mut debug_trait_builder = f.debug_tuple("OutD");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for MotorPort {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MotorPort {
        #[inline]
        fn clone(&self) -> MotorPort {
            {
                *self
            }
        }
    }
    impl Port for MotorPort {
        fn address(&self) -> String {
            match self {
                MotorPort::OutA => "outA".to_owned(),
                MotorPort::OutB => "outB".to_owned(),
                MotorPort::OutC => "outC".to_owned(),
                MotorPort::OutD => "outD".to_owned(),
            }
        }
    }
}
pub mod sensors {
    //! # Container module for sensor types
    pub mod color_sensor {
        //! LEGO EV3 color sensor.
        use super::Sensor;
        use crate::{Attribute, Device, Driver, Ev3Result, Findable};
        /// Reflected light - sets LED color to red
        pub const MODE_COL_REFLECT: &str = "COL-REFLECT";
        /// Ambient light - sets LED color to blue (dimly lit)
        pub const MODE_COL_AMBIENT: &str = "COL-AMBIENT";
        /// Color - sets LED color to white (all LEDs rapidly cycling)
        pub const MODE_COL_COLOR: &str = "COL-COLOR";
        /// Raw Reflected - sets LED color to red
        pub const MODE_REF_RAW: &str = "REF-RAW";
        /// Raw Color Components - sets LED color to white (all LEDs rapidly cycling)
        pub const MODE_RGB_RAW: &str = "RGB-RAW";
        /// Calibration ??? - sets LED color to red, flashing every 4 seconds, then goes continuous
        pub const MODE_COL_CAL: &str = "COL-CAL";
        /// LEGO EV3 color sensor.
        //#[class_name = "lego-sensor"]
        //#[driver_name = "lego-ev3-color"]
        //#[port = "crate::sensors::SensorPort"]
        pub struct ColorSensor {
            driver: Driver,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ColorSensor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ColorSensor {
                        driver: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("ColorSensor");
                        let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ColorSensor {
            #[inline]
            fn clone(&self) -> ColorSensor {
                match *self {
                    ColorSensor {
                        driver: ref __self_0_0,
                    } => ColorSensor {
                        driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Device for ColorSensor {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
        impl Sensor for ColorSensor {}
        impl Findable<crate::sensors::SensorPort> for ColorSensor {
            fn get(port: crate::sensors::SensorPort) -> Ev3Result<Self> {
                let name =
                    Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-color")?;
                Ok(ColorSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-color")?;
                Ok(ColorSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn list() -> Ev3Result<Vec<Self>> {
                Ok(
                    Driver::find_names_by_driver("lego-sensor", "lego-ev3-color")?
                        .iter()
                        .map(|name| ColorSensor {
                            driver: Driver::new("lego-sensor", name),
                        })
                        .collect(),
                )
            }
        }
        impl ColorSensor {
            /// Reflected light - sets LED color to red
            pub fn set_mode_col_reflect(&self) -> Ev3Result<()> {
                self.set_mode(MODE_COL_REFLECT)
            }
            /// Ambient light - sets LED color to blue (dimly lit)
            pub fn set_mode_col_ambient(&self) -> Ev3Result<()> {
                self.set_mode(MODE_COL_AMBIENT)
            }
            /// Color - sets LED color to white (all LEDs rapidly cycling)
            pub fn set_mode_col_color(&self) -> Ev3Result<()> {
                self.set_mode(MODE_COL_COLOR)
            }
            /// Raw Reflected - sets LED color to red
            pub fn set_mode_ref_raw(&self) -> Ev3Result<()> {
                self.set_mode(MODE_REF_RAW)
            }
            /// Raw Color Components - sets LED color to white (all LEDs rapidly cycling)
            pub fn set_mode_rgb_raw(&self) -> Ev3Result<()> {
                self.set_mode(MODE_RGB_RAW)
            }
            /// Calibration ??? - sets LED color to red, flashing every 4 seconds, then goes continuous
            pub fn set_mode_col_cal(&self) -> Ev3Result<()> {
                self.set_mode(MODE_COL_CAL)
            }
            /// Red component of the detected color, in the range 0-1020.
            pub fn get_red(&self) -> Ev3Result<i32> {
                self.get_value0()
            }
            /// Green component of the detected color, in the range 0-1020.
            pub fn get_green(&self) -> Ev3Result<i32> {
                self.get_value1()
            }
            /// Blue component of the detected color, in the range 0-1020.
            pub fn get_blue(&self) -> Ev3Result<i32> {
                self.get_value2()
            }
            /// Red, green and blue componets of the detected color, each in the range 0-1020
            pub fn get_rgb(&self) -> Ev3Result<(i32, i32, i32)> {
                let red = self.get_red()?;
                let green = self.get_green()?;
                let blue = self.get_blue()?;
                Ok((red, green, blue))
            }
        }
    }
    pub use self::color_sensor::ColorSensor;
    pub mod gyro_sensor {
        //! LEGO EV3 gyro sensor.
        use super::Sensor;
        use crate::{Attribute, Device, Driver, Ev3Result, Findable};
        /// Angle
        pub const MODE_GYRO_ANG: &str = "GYRO-ANG";
        /// Rotational Speed
        pub const MODE_GYRO_RATE: &str = "GYRO-RATE";
        /// Raw sensor value ???
        pub const MODE_GYRO_FAS: &str = "GYRO-FAS";
        /// Angle and Rotational Speed
        pub const MODE_GYRO_G_AND_A: &str = "GYRO-G&A";
        /// Calibration ???
        pub const MODE_GYRO_CAL: &str = "GYRO-CAL";
        /// LEGO EV3 gyro sensor.
        //#[class_name = "lego-sensor"]
        //#[driver_name = "lego-ev3-gyro"]
        //#[port = "crate::sensors::SensorPort"]
        pub struct GyroSensor {
            driver: Driver,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for GyroSensor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    GyroSensor {
                        driver: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("GyroSensor");
                        let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for GyroSensor {
            #[inline]
            fn clone(&self) -> GyroSensor {
                match *self {
                    GyroSensor {
                        driver: ref __self_0_0,
                    } => GyroSensor {
                        driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Device for GyroSensor {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
        impl Sensor for GyroSensor {}
        impl Findable<crate::sensors::SensorPort> for GyroSensor {
            fn get(port: crate::sensors::SensorPort) -> Ev3Result<Self> {
                let name =
                    Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-gyro")?;
                Ok(GyroSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-gyro")?;
                Ok(GyroSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn list() -> Ev3Result<Vec<Self>> {
                Ok(
                    Driver::find_names_by_driver("lego-sensor", "lego-ev3-gyro")?
                        .iter()
                        .map(|name| GyroSensor {
                            driver: Driver::new("lego-sensor", name),
                        })
                        .collect(),
                )
            }
        }
        impl GyroSensor {
            /// Angle
            pub fn set_mode_col_ang(&self) -> Ev3Result<()> {
                self.set_mode(MODE_GYRO_ANG)
            }
            /// Rotational Speed
            pub fn set_mode_col_rate(&self) -> Ev3Result<()> {
                self.set_mode(MODE_GYRO_RATE)
            }
            /// Raw sensor value ???
            pub fn set_mode_col_fas(&self) -> Ev3Result<()> {
                self.set_mode(MODE_GYRO_FAS)
            }
            /// Angle and Rotational Speed
            pub fn set_mode_gyro_g_and_a(&self) -> Ev3Result<()> {
                self.set_mode(MODE_GYRO_G_AND_A)
            }
            /// Calibration ???
            pub fn set_mode_gyro_cal(&self) -> Ev3Result<()> {
                self.set_mode(MODE_GYRO_CAL)
            }
        }
    }
    pub use self::gyro_sensor::GyroSensor;
    pub mod infrared_sensor {
        //! LEGO EV3 infrared sensor.
        use super::Sensor;
        use crate::{Attribute, Device, Driver, Ev3Result, Findable};
        /// Proximity
        pub const MODE_IR_PROX: &str = "IR-PROX";
        /// IR Seeker
        pub const MODE_IR_SEEK: &str = "IR-SEEK";
        /// IR Remote Control
        pub const MODE_IR_REMOTE: &str = "IR-REMOTE";
        /// IR Remote Control
        pub const MODE_IR_REM_A: &str = "IR-REM-A";
        /// Alternate IR Seeker ???
        pub const MODE_IR_S_ALT: &str = "IR-S-ALT";
        /// Calibration ???
        pub const MODE_IR_CAL: &str = "IR-CAL";
        /// LEGO EV3 infrared sensor.
        //#[class_name = "lego-sensor"]
        //#[driver_name = "lego-ev3-ir"]
        //#[port = "crate::sensors::SensorPort"]
        pub struct InfraredSensor {
            driver: Driver,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InfraredSensor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InfraredSensor {
                        driver: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("InfraredSensor");
                        let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for InfraredSensor {
            #[inline]
            fn clone(&self) -> InfraredSensor {
                match *self {
                    InfraredSensor {
                        driver: ref __self_0_0,
                    } => InfraredSensor {
                        driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Device for InfraredSensor {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
        impl Findable<crate::sensors::SensorPort> for InfraredSensor {
            fn get(port: crate::sensors::SensorPort) -> Ev3Result<Self> {
                let name =
                    Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-ir")?;
                Ok(InfraredSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-ir")?;
                Ok(InfraredSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn list() -> Ev3Result<Vec<Self>> {
                Ok(Driver::find_names_by_driver("lego-sensor", "lego-ev3-ir")?
                    .iter()
                    .map(|name| InfraredSensor {
                        driver: Driver::new("lego-sensor", name),
                    })
                    .collect())
            }
        }
        impl Sensor for InfraredSensor {}
        impl InfraredSensor {
            /// Proximity
            pub fn set_mode_ir_prox(&self) -> Ev3Result<()> {
                self.set_mode(MODE_IR_PROX)
            }
            /// IR Seeker
            pub fn set_mode_ir_seek(&self) -> Ev3Result<()> {
                self.set_mode(MODE_IR_SEEK)
            }
            /// IR Remote Control
            pub fn set_mode_ir_remote(&self) -> Ev3Result<()> {
                self.set_mode(MODE_IR_REMOTE)
            }
            /// IR Remote Control
            pub fn set_mode_ir_rem_a(&self) -> Ev3Result<()> {
                self.set_mode(MODE_IR_REM_A)
            }
            /// Alternate IR Seeker ???
            pub fn set_mode_ir_s_alt(&self) -> Ev3Result<()> {
                self.set_mode(MODE_IR_S_ALT)
            }
            /// Calibration ???
            pub fn set_mode_ir_cal(&self) -> Ev3Result<()> {
                self.set_mode(MODE_IR_CAL)
            }
        }
    }
    pub use self::infrared_sensor::InfraredSensor;
    pub mod touch_sensor {
        //! Touch Sensor
        use super::Sensor;
        use crate::{Attribute, Device, Driver, Ev3Result, Findable};
        /// Button state
        pub const MODE_TOUCH: &str = "TOUCH";
        /// Touch Sensor
        //#[class_name = "lego-sensor"]
        //#[driver_name = "lego-ev3-touch"]
        //#[port = "crate::sensors::SensorPort"]
        pub struct TouchSensor {
            driver: Driver,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for TouchSensor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    TouchSensor {
                        driver: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("TouchSensor");
                        let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for TouchSensor {
            #[inline]
            fn clone(&self) -> TouchSensor {
                match *self {
                    TouchSensor {
                        driver: ref __self_0_0,
                    } => TouchSensor {
                        driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Device for TouchSensor {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
        impl Sensor for TouchSensor {}
        impl Findable<crate::sensors::SensorPort> for TouchSensor {
            fn get(port: crate::sensors::SensorPort) -> Ev3Result<Self> {
                let name =
                    Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-touch")?;
                Ok(TouchSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-touch")?;
                Ok(TouchSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn list() -> Ev3Result<Vec<Self>> {
                Ok(
                    Driver::find_names_by_driver("lego-sensor", "lego-ev3-touch")?
                        .iter()
                        .map(|name| TouchSensor {
                            driver: Driver::new("lego-sensor", name),
                        })
                        .collect(),
                )
            }
        }
        impl TouchSensor {
            /// A boolean indicating whether the current touch sensor is being pressed.
            pub fn get_pressed_state(&self) -> Ev3Result<bool> {
                Ok(self.get_value0()? != 0)
            }
        }
    }
    pub use self::touch_sensor::TouchSensor;
    pub mod ultrasonic_sensor {
        //! LEGO EV3 ultrasonic sensor
        use super::Sensor;
        use crate::{Attribute, Device, Driver, Ev3Result, Findable};
        /// Continuous measurement - sets LEDs on, steady.
        /// Units in centimeters. Distance (0-2550)
        pub const MODE_US_DIST_CM: &str = "US-DIST-CM";
        /// Continuous measurement - sets LEDs on, steady.
        /// Units in inches. Distance (0-1003)
        pub const MODE_US_DIST_IN: &str = "US-DIST-IN";
        /// Listen - sets LEDs on, blinking. Presence (0-1)
        pub const MODE_US_LISTEN: &str = "US-LISTEN";
        /// Single measurement - LEDs on momentarily when mode is set, then off.
        /// Units in centimeters. Distance (0-2550)
        pub const MODE_US_SI_CM: &str = "US-SI-CM";
        /// Single measurement - LEDs on momentarily when mode is set, then off.
        /// Units in inches. Distance (0-1003)
        pub const MODE_US_SI_IN: &str = "US-SI-IN";
        /// ??? - sets LEDs on, steady.
        /// Units in centimeters. Distance (0-2550)
        pub const MODE_US_DC_CM: &str = "US-DC-CM";
        /// ??? - sets LEDs on, steady.
        /// Units in inches. Distance (0-1003)
        pub const MODE_US_DC_IN: &str = "US-DC-IN";
        /// LEGO EV3 ultrasonic sensor.
        //#[class_name = "lego-sensor"]
        //#[driver_name = "lego-ev3-us"]
        //#[port = "crate::sensors::SensorPort"]
        pub struct UltrasonicSensor {
            driver: Driver,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for UltrasonicSensor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    UltrasonicSensor {
                        driver: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("UltrasonicSensor");
                        let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for UltrasonicSensor {
            #[inline]
            fn clone(&self) -> UltrasonicSensor {
                match *self {
                    UltrasonicSensor {
                        driver: ref __self_0_0,
                    } => UltrasonicSensor {
                        driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Device for UltrasonicSensor {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
        impl Sensor for UltrasonicSensor {}
        impl Findable<crate::sensors::SensorPort> for UltrasonicSensor {
            fn get(port: crate::sensors::SensorPort) -> Ev3Result<Self> {
                let name =
                    Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-us")?;
                Ok(UltrasonicSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-us")?;
                Ok(UltrasonicSensor {
                    driver: Driver::new("lego-sensor", &name),
                })
            }
            fn list() -> Ev3Result<Vec<Self>> {
                Ok(Driver::find_names_by_driver("lego-sensor", "lego-ev3-us")?
                    .iter()
                    .map(|name| UltrasonicSensor {
                        driver: Driver::new("lego-sensor", name),
                    })
                    .collect())
            }
        }
        impl UltrasonicSensor {
            /// Continuous measurement - sets LEDs on, steady. Units in centimeters. Distance (0-2550)
            pub fn set_mode_us_dist_cm(&self) -> Ev3Result<()> {
                self.set_mode(MODE_US_DIST_CM)
            }
            /// Continuous measurement - sets LEDs on, steady. Units in inches. Distance (0-1003)
            pub fn set_mode_us_dist_in(&self) -> Ev3Result<()> {
                self.set_mode(MODE_US_DIST_IN)
            }
            /// Listen - sets LEDs on, blinking. Presence (0-1)
            pub fn set_mode_us_listen(&self) -> Ev3Result<()> {
                self.set_mode(MODE_US_LISTEN)
            }
            /// Single measurement - LEDs on momentarily when mode is set, then off. Units in centimeters. Distance (0-2550)
            pub fn set_mode_us_si_cm(&self) -> Ev3Result<()> {
                self.set_mode(MODE_US_SI_CM)
            }
            /// Single measurement - LEDs on momentarily when mode is set, then off. Units in inches. Distance (0-1003)
            pub fn set_mode_us_si_in(&self) -> Ev3Result<()> {
                self.set_mode(MODE_US_SI_IN)
            }
            /// ??? - sets LEDs on, steady . Units in centimeters. Distance (0-2550)
            pub fn set_mode_us_dc_cm(&self) -> Ev3Result<()> {
                self.set_mode(MODE_US_DC_CM)
            }
            /// ??? - sets LEDs on, steady . Units in inches. Distance (0-1003)
            pub fn set_mode_us_dc_in(&self) -> Ev3Result<()> {
                self.set_mode(MODE_US_DC_IN)
            }
            /// Measurement of the distance detected by the sensor
            pub fn get_distance(&self) -> Ev3Result<i32> {
                self.get_value0()
            }
        }
    }
    pub use self::ultrasonic_sensor::UltrasonicSensor;
    use crate::{Device, Ev3Result, Port};
    /// Container trait to indicate something is a sensor
    pub trait Sensor: Device {
        /// Reading the file will give the unscaled raw values in the `value<N>` attributes.
        /// Use `bin_data_format`, `num_values` and the individual sensor documentation to determine how to interpret the data.
        fn get_bin_data(&self) -> Ev3Result<String> {
            self.get_attribute("bin_data").get()
        }
        /// Returns the format of the values in `bin_data` for the current mode. Possible values are:
        fn get_bin_data_format(&self) -> Ev3Result<String> {
            self.get_attribute("bin_data_format").get()
        }
        /// Returns the number of decimal places for the values in the `value<N>` attributes of the current mode.
        fn get_decimals(&self) -> Ev3Result<i32> {
            self.get_attribute("decimals").get()
        }
        /// Returns the firmware version of the sensor if available.
        /// Currently only NXT/I2C sensors support this.
        fn get_fw_version(&self) -> Ev3Result<String> {
            self.get_attribute("fw_version").get()
        }
        /// Returns the current mode.
        /// See the individual sensor documentation for a description of the modes available for each type of sensor.
        fn get_mode(&self) -> Ev3Result<String> {
            self.get_attribute("mode").get()
        }
        /// Sets the sensor to that mode.
        /// See the individual sensor documentation for a description of the modes available for each type of sensor.
        fn set_mode(&self, mode: &str) -> Ev3Result<()> {
            self.get_attribute("mode").set_str_slice(mode)
        }
        /// Returns a list of the valid modes for the sensor.
        fn get_modes(&self) -> Ev3Result<Vec<String>> {
            self.get_attribute("modes").get_vec()
        }
        /// Returns the number of `value<N>` attributes that will return a valid value for the current mode.
        fn get_num_values(&self) -> Ev3Result<i32> {
            self.get_attribute("num_values").get()
        }
        /// Returns the polling period of the sensor in milliseconds.
        /// Returns `-EOPNOTSUPP` if changing polling is not supported.
        /// Note: Setting poll_ms too high can cause the input port autodetection to fail.
        /// If this happens, use the mode attribute of the port to force the port to `nxt-i2c mode`. Values must not be negative.
        fn get_poll_ms(&self) -> Ev3Result<i32> {
            self.get_attribute("poll_ms").get()
        }
        /// Sets the polling period of the sensor in milliseconds.
        /// Setting to 0 disables polling.
        /// Note: Setting poll_ms too high can cause the input port autodetection to fail.
        /// If this happens, use the mode attribute of the port to force the port to `nxt-i2c mode`. Values must not be negative.
        fn set_poll_ms(&self, poll_ms: i32) -> Ev3Result<()> {
            self.get_attribute("poll_ms").set(poll_ms)
        }
        /// Returns the units of the measured value for the current mode. May return empty string if units are unknown.
        fn get_units(&self) -> Ev3Result<String> {
            self.get_attribute("units").get()
        }
        /// Returns the current `value0` value if available.
        fn get_value0(&self) -> Ev3Result<i32> {
            self.get_attribute("value0").get()
        }
        /// Returns the current `value1` value if available.
        fn get_value1(&self) -> Ev3Result<i32> {
            self.get_attribute("value1").get()
        }
        /// Returns the current `value2` value if available.
        fn get_value2(&self) -> Ev3Result<i32> {
            self.get_attribute("value2").get()
        }
        /// Returns the current `value3` value if available.
        fn get_value3(&self) -> Ev3Result<i32> {
            self.get_attribute("value3").get()
        }
        /// Returns the current `value4` value if available.
        fn get_value4(&self) -> Ev3Result<i32> {
            self.get_attribute("value4").get()
        }
        /// Returns the current `value5` value if available.
        fn get_value5(&self) -> Ev3Result<i32> {
            self.get_attribute("value5").get()
        }
        /// Returns the current `value6` value if available.
        fn get_value6(&self) -> Ev3Result<i32> {
            self.get_attribute("value6").get()
        }
        /// Returns the current `value7` value if available.
        fn get_value7(&self) -> Ev3Result<i32> {
            self.get_attribute("value7").get()
        }
        /// Returns a space delimited string representing sensor-specific text values. Returns `-EOPNOTSUPP` if a sensor does not support text values.
        fn get_text_value(&self) -> Ev3Result<String> {
            self.get_attribute("text_value").get()
        }
    }
    /// EV3 ports `in1` to `in4`
    pub enum SensorPort {
        /// EV3 `in1` port
        In1,

        /// EV3 `in2` port
        In2,

        /// EV3 `in3` port
        In3,

        /// EV3 `in4` port
        In4,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SensorPort {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&SensorPort::In1,) => {
                    let mut debug_trait_builder = f.debug_tuple("In1");
                    debug_trait_builder.finish()
                }
                (&SensorPort::In2,) => {
                    let mut debug_trait_builder = f.debug_tuple("In2");
                    debug_trait_builder.finish()
                }
                (&SensorPort::In3,) => {
                    let mut debug_trait_builder = f.debug_tuple("In3");
                    debug_trait_builder.finish()
                }
                (&SensorPort::In4,) => {
                    let mut debug_trait_builder = f.debug_tuple("In4");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for SensorPort {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SensorPort {
        #[inline]
        fn clone(&self) -> SensorPort {
            {
                *self
            }
        }
    }
    impl Port for SensorPort {
        fn address(&self) -> String {
            match self {
                SensorPort::In1 => "in1".to_owned(),
                SensorPort::In2 => "in2".to_owned(),
                SensorPort::In3 => "in3".to_owned(),
                SensorPort::In4 => "in4".to_owned(),
            }
        }
    }
}
pub mod led {
    //! The leds on top of the EV3 brick.
    use crate::{utils::OrErr, Attribute, Ev3Result};
    use std::fs;
    /// Color type.
    pub type Color = (u8, u8);
    /// Led off.
    pub const COLOR_OFF: Color = (0, 0);
    /// Led color red
    pub const COLOR_RED: Color = (255, 0);
    /// Led color green.
    pub const COLOR_GREEN: Color = (0, 255);
    /// Led color amber.
    pub const COLOR_AMBER: Color = (255, 255);
    /// Led color orange.
    pub const COLOR_ORANGE: Color = (255, 128);
    /// LED color yellow.
    pub const COLOR_YELLOW: Color = (25, 255);
    /// The leds on top of the EV3 brick.
    pub struct Led {
        left_red: Attribute,
        left_green: Attribute,
        right_red: Attribute,
        right_green: Attribute,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Led {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Led {
                    left_red: ref __self_0_0,
                    left_green: ref __self_0_1,
                    right_red: ref __self_0_2,
                    right_green: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Led");
                    let _ = debug_trait_builder.field("left_red", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("left_green", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("right_red", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("right_green", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Led {
        #[inline]
        fn clone(&self) -> Led {
            match *self {
                Led {
                    left_red: ref __self_0_0,
                    left_green: ref __self_0_1,
                    right_red: ref __self_0_2,
                    right_green: ref __self_0_3,
                } => Led {
                    left_red: ::core::clone::Clone::clone(&(*__self_0_0)),
                    left_green: ::core::clone::Clone::clone(&(*__self_0_1)),
                    right_red: ::core::clone::Clone::clone(&(*__self_0_2)),
                    right_green: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    impl Led {
        /// Create a new instance of the `Led` struct.
        pub fn new() -> Ev3Result<Led> {
            let mut left_red_name = String::new();
            let mut left_green_name = String::new();
            let mut right_red_name = String::new();
            let mut right_green_name = String::new();
            let paths = fs::read_dir("/sys/class/leds")?;
            for path in paths {
                let file_name = path?.file_name();
                let name = file_name.to_str().or_err()?.to_owned();
                if name.contains(":brick-status") || name.contains(":ev3dev") {
                    if name.contains("led0:") || name.contains("left:") {
                        if name.contains("red:") {
                            left_red_name = name;
                        } else if name.contains("green:") {
                            left_green_name = name
                        }
                    } else if name.contains("led1:") || name.contains("right:") {
                        if name.contains("red:") {
                            right_red_name = name
                        } else if name.contains("green:") {
                            right_green_name = name
                        }
                    }
                }
            }
            let left_red = Attribute::new("leds", left_red_name.as_str(), "brightness")?;
            let left_green = Attribute::new("leds", left_green_name.as_str(), "brightness")?;
            let right_red = Attribute::new("leds", right_red_name.as_str(), "brightness")?;
            let right_green = Attribute::new("leds", right_green_name.as_str(), "brightness")?;
            Ok(Led {
                left_red,
                left_green,
                right_red,
                right_green,
            })
        }
        /// Returns the current red value of the left led.
        fn get_left_red(&self) -> Ev3Result<u8> {
            self.left_red.get()
        }
        /// Sets the red value of the left led.
        fn set_left_red(&self, brightness: u8) -> Ev3Result<()> {
            self.left_red.set(brightness)
        }
        /// Returns the current green value of the left led.
        fn get_left_green(&self) -> Ev3Result<u8> {
            self.left_green.get()
        }
        /// Sets the green value of the left led.
        fn set_left_green(&self, brightness: u8) -> Ev3Result<()> {
            self.left_green.set(brightness)
        }
        /// Returns the current red value of the right led.
        fn get_right_red(&self) -> Ev3Result<u8> {
            self.right_red.get()
        }
        /// Sets the red value of the right led.
        fn set_right_red(&self, brightness: u8) -> Ev3Result<()> {
            self.right_red.set(brightness)
        }
        /// Returns the current green value of the right led.
        fn get_right_green(&self) -> Ev3Result<u8> {
            self.right_green.get()
        }
        /// Sets the green value of the right led.
        fn set_right_green(&self, brightness: u8) -> Ev3Result<()> {
            self.right_green.set(brightness)
        }
        /// Returns the current color value of the left led.
        pub fn get_left_color(&self) -> Ev3Result<Color> {
            let red = self.get_left_red()?;
            let green = self.get_left_green()?;
            Ok((red, green))
        }
        /// Sets the color value of the left led.
        pub fn set_left_color(&self, color: Color) -> Ev3Result<()> {
            self.set_left_red(color.0)?;
            self.set_left_green(color.1)
        }
        /// Returns the current color value of the right led.
        pub fn get_right_color(&self) -> Ev3Result<Color> {
            let red = self.get_right_red()?;
            let green = self.get_right_green()?;
            Ok((red, green))
        }
        /// Sets the color value of the right led.
        pub fn set_right_color(&self, color: Color) -> Ev3Result<()> {
            self.set_right_red(color.0)?;
            self.set_right_green(color.1)
        }
        /// Returns the color value of both leds or `None` if they are different.
        pub fn get_color(&self) -> Ev3Result<Option<Color>> {
            let left = self.get_left_color()?;
            let right = self.get_right_color()?;
            if left.0 == right.0 && left.1 == right.1 {
                Ok(Some(left))
            } else {
                Ok(None)
            }
        }
        /// Sets the color value of both leds.
        pub fn set_color(&self, color: Color) -> Ev3Result<()> {
            self.set_left_color(color)?;
            self.set_right_color(color)
        }
    }
}
pub use led::Led;
pub mod sound {
    //! Sound-related functions. It can beep, play wav files, or convert text to
    //! speech.
    //!
    //! Note that all methods of the meodule spawn system processes and return
    //! `std::process::Child` objects. The methods are asynchronous (they return
    //! immediately after child process was spawned, without waiting for its
    //! completion), but you can call wait() on the returned result.
    //!
    //! # Examples
    //! ```no_run
    //! # use ev3dev_lang_rust::Ev3Result;
    //! use ev3dev_lang_rust::sound;
    //!
    //! # fn main() -> Ev3Result<()> {
    //! // Play "bark.wav", return immediately:
    //! sound::play("bark.wav")?;
    //!
    //! // Introduce yourself, wait for completion:
    //! sound::speak("Hello, I am Robot")?.wait()?;
    //! # Ok(())
    //! # }
    //! ```
    use crate::{Ev3Error, Ev3Result};
    use std::ffi::OsStr;
    use std::process::{Child, Command, Stdio};
    /// Call beep command.
    ///
    /// # Example
    /// ```no_run
    /// # use ev3dev_lang_rust::Ev3Result;
    /// use ev3dev_lang_rust::sound;
    ///
    /// # fn main() -> Ev3Result<()> {
    /// sound::beep()?.wait()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn beep() -> Ev3Result<Child> {
        Ok(Command::new("/usr/bin/beep")
            .stdout(Stdio::null())
            .spawn()?)
    }
    /// Call beep command with the provided arguments.
    ///
    /// See `beep man page`_ and google `linux beep music`_ for inspiration.
    /// * `beep man page`: https://linux.die.net/man/1/beep
    /// * `linux beep music`: https://www.google.com/search?q=linux+beep+music
    ///
    /// # Example
    /// ```no_run
    /// # use ev3dev_lang_rust::Ev3Result;
    /// use ev3dev_lang_rust::sound;
    ///
    /// # fn main() -> Ev3Result<()> {
    /// sound::beep_args(&[""])?.wait()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn beep_args<I, S>(args: I) -> Ev3Result<Child>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Ok(Command::new("/usr/bin/beep")
            .args(args)
            .stdout(Stdio::null())
            .spawn()?)
    }
    /// Play tone sequence. The tone_sequence parameter is a list of tuples,
    /// where each tuple contains up to three numbers. The first number is
    /// frequency in Hz, the second is duration in milliseconds, and the third
    /// is delay in milliseconds between this and the next tone in the
    /// sequence.
    ///
    /// # Example
    /// ```no_run
    /// # use ev3dev_lang_rust::Ev3Result;
    /// use ev3dev_lang_rust::sound;
    ///
    /// # fn main() -> Ev3Result<()> {
    /// sound::tone(466.0, 500)?.wait()?;
    /// # Ok(())
    /// # }
    pub fn tone(frequency: f32, duration: i32) -> Ev3Result<Child> {
        beep_args(<[_]>::into_vec(box [
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["-f "],
                    &match (&frequency,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            },
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["-l "],
                    &match (&duration,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            },
        ]))
    }
    /// Play tone sequence. The tone_sequence parameter is a list of tuples,
    /// where each tuple contains up to three numbers. The first number is
    /// frequency in Hz, the second is duration in milliseconds, and the third
    /// is delay in milliseconds between this and the next tone in the
    /// sequence.
    ///
    /// # Example
    /// ```no_run
    /// # use ev3dev_lang_rust::Ev3Result;
    /// use ev3dev_lang_rust::sound;
    ///
    /// # fn main() -> Ev3Result<()> {
    /// sound::tone_sequence(
    ///     &[
    ///         (392.00, 350, 100), (392.00, 350, 100), (392.00, 350, 100), (311.1, 250, 100),
    ///         (466.20, 025, 100), (392.00, 350, 100), (311.10, 250, 100), (466.2, 025, 100),
    ///         (392.00, 700, 100), (587.32, 350, 100), (587.32, 350, 100),
    ///         (587.32, 350, 100), (622.26, 250, 100), (466.20, 025, 100),
    ///         (369.99, 350, 100), (311.10, 250, 100), (466.20, 025, 100), (392.00, 700, 100),
    ///         (784.00, 350, 100), (392.00, 250, 100), (392.00, 025, 100), (784.00, 350, 100),
    ///         (739.98, 250, 100), (698.46, 025, 100), (659.26, 025, 100),
    ///         (622.26, 025, 100), (659.26, 050, 400), (415.30, 025, 200), (554.36, 350, 100),
    ///         (523.25, 250, 100), (493.88, 025, 100), (466.16, 025, 100), (440.00, 025, 100),
    ///         (466.16, 050, 400), (311.13, 025, 200), (369.99, 350, 100),
    ///         (311.13, 250, 100), (392.00, 025, 100), (466.16, 350, 100), (392.00, 250, 100),
    ///         (466.16, 025, 100), (587.32, 700, 100), (784.00, 350, 100), (392.00, 250, 100),
    ///         (392.00, 025, 100), (784.00, 350, 100), (739.98, 250, 100), (698.46, 025, 100),
    ///         (659.26, 025, 100), (622.26, 025, 100), (659.26, 050, 400), (415.30, 025, 200),
    ///         (554.36, 350, 100), (523.25, 250, 100), (493.88, 025, 100),
    ///         (466.16, 025, 100), (440.00, 025, 100), (466.16, 050, 400), (311.13, 025, 200),
    ///         (392.00, 350, 100), (311.13, 250, 100), (466.16, 025, 100),
    ///         (392.00, 300, 150), (311.13, 250, 100), (466.16, 025, 100), (392.00, 700, 0)
    ///     ]
    /// )?.wait()?;
    /// # Ok(())
    /// # }
    pub fn tone_sequence(sequence: &[(f32, i32, i32)]) -> Ev3Result<Child> {
        let tones: Vec<String> = sequence
            .iter()
            .map(|(frequency, duration, delay)| {
                <[_]>::into_vec(box [
                    {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["-f "],
                            &match (&frequency,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    },
                    {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["-l "],
                            &match (&duration,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    },
                    {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["-D "],
                            &match (&delay,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    },
                ])
            })
            .collect::<Vec<Vec<String>>>()
            .join(&["-n".to_owned()][..]);
        beep_args(tones)
    }
    /// Play wav file
    pub fn play(wav_file: &str) -> Ev3Result<Child> {
        Ok(Command::new("/usr/bin/aplay")
            .arg("-q")
            .arg(wav_file)
            .stdout(Stdio::null())
            .spawn()?)
    }
    /// Speak the given text aloud.
    pub fn speak(text: &str) -> Ev3Result<Child> {
        let espeak = Command::new("/usr/bin/espeak")
            .args(&["--stdout", "-a", "200", "-s", "130", text])
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(Command::new("/usr/bin/aplay")
            .arg("-q")
            .stdin(espeak.stdout.ok_or(Ev3Error::NotFound)?)
            .stdout(Stdio::null())
            .spawn()?)
    }
    /// Get the main channel name or 'Playback' if not available.
    fn get_channels() -> Ev3Result<Vec<String>> {
        let out = String::from_utf8(
            Command::new("/usr/bin/amixer")
                .arg("scontrols")
                .output()?
                .stdout,
        )?;
        let mut channels: Vec<String> = out
            .split('\n')
            .filter_map(|line| {
                let vol_start = line.find('\'').unwrap_or(0) + 1;
                let vol_end = line.rfind('\'').unwrap_or(1);
                if vol_start >= vol_end {
                    None
                } else {
                    Some(line[vol_start..vol_end].to_owned())
                }
            })
            .collect();
        if channels.is_empty() {
            channels.push("Playback".to_owned());
        }
        Ok(channels)
    }
    /// Sets the sound volume to the given percentage [0-100] by calling
    /// `amixer -q set <channel> <pct>%`.
    pub fn set_volume_channel(volume: i32, channel: &str) -> Ev3Result<()> {
        Command::new("/usr/bin/amixer")
            .args(&["-q", "set", &channel, &{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "%"],
                    &match (&volume,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            }])
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
        Ok(())
    }
    /// Sets the sound volume to the given percentage [0-100] by calling
    /// `amixer -q set <channel> <pct>%`.
    /// It tries to determine the default channel
    /// by running `amixer scontrols`. If that fails as well, it uses the
    /// `Playback` channel, as that is the only channel on the EV3.
    pub fn set_volume(volume: i32) -> Ev3Result<()> {
        for channel in get_channels()? {
            set_volume_channel(volume, &channel)?;
        }
        Ok(())
    }
    /// Gets the current sound volume by parsing the output of
    /// `amixer get <channel>`.
    pub fn get_volume_channel(channel: &str) -> Ev3Result<i32> {
        let out = String::from_utf8(
            Command::new("/usr/bin/amixer")
                .args(&["get", channel])
                .output()?
                .stdout,
        )?;
        let vol_start = out.find('[').unwrap_or(0) + 1;
        let vol_end = out.find("%]").unwrap_or(1);
        let vol = &out[vol_start..vol_end].parse::<i32>()?;
        Ok(*vol)
    }
    /// Gets the current sound volume by parsing the output of
    /// `amixer get <channel>`.
    /// It tries to determine the default channel
    /// by running `amixer scontrols`. If that fails as well, it uses the
    /// `Playback` channel, as that is the only channel on the EV3.
    pub fn get_volume() -> Ev3Result<i32> {
        get_volume_channel(&get_channels()?[0])
    }
}
mod buttons {
    //! EV3 Buttons
    //!
    //! ```no_run
    //! use ev3dev_lang_rust::Ev3Button;
    //! use std::thread;
    //! use std::time::Duration;
    //!
    //! # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    //! let button = Ev3Button::new()?;
    //!
    //! loop {
    //!     button.process();
    //!
    //!     println!("Is 'up' pressed: {}", button.is_up());
    //!     println!("Pressed buttons: {:?}", button.get_pressed_buttons());
    //!
    //!     thread::sleep(Duration::from_millis(100));
    //! }
    //! # }
    //! ```
    use crate::Ev3Result;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;
    use std::rc::Rc;
    const KEY_BUF_LEN: usize = 96;
    const EVIOCGKEY: u32 = 2_153_792_792;
    /// Helper struct for ButtonFileHandler.
    struct FileMapEntry {
        pub file: File,
        pub buffer_cache: [u8; KEY_BUF_LEN],
    }
    impl fmt::Debug for FileMapEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FileMapEntry")
                .field("file", &self.file)
                .finish()
        }
    }
    /// Helper struct for ButtonFileHandler.
    struct ButtonMapEntry {
        pub file_name: String,
        pub key_code: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ButtonMapEntry {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ButtonMapEntry {
                    file_name: ref __self_0_0,
                    key_code: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ButtonMapEntry");
                    let _ = debug_trait_builder.field("file_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("key_code", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// This implementation depends on the availability of the EVIOCGKEY ioctl
    /// to be able to read the button state buffer. See Linux kernel source
    /// in /include/uapi/linux/input.h for details.
    struct ButtonFileHandler {
        file_map: HashMap<String, FileMapEntry>,
        button_map: HashMap<String, ButtonMapEntry>,
        pressed_buttons: HashSet<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ButtonFileHandler {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ButtonFileHandler {
                    file_map: ref __self_0_0,
                    button_map: ref __self_0_1,
                    pressed_buttons: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ButtonFileHandler");
                    let _ = debug_trait_builder.field("file_map", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("button_map", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("pressed_buttons", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl ButtonFileHandler {
        /// Create a new instance.
        fn new() -> Self {
            ButtonFileHandler {
                file_map: HashMap::new(),
                button_map: HashMap::new(),
                pressed_buttons: HashSet::new(),
            }
        }
        /// Add a button the the file handler.
        fn add_button(&mut self, name: &str, file_name: &str, key_code: u32) -> Ev3Result<()> {
            if !self.file_map.contains_key(file_name) {
                let file = File::open(file_name)?;
                let buffer_cache = [0u8; KEY_BUF_LEN];
                self.file_map
                    .insert(file_name.to_owned(), FileMapEntry { file, buffer_cache });
            }
            self.button_map.insert(
                name.to_owned(),
                ButtonMapEntry {
                    file_name: file_name.to_owned(),
                    key_code,
                },
            );
            Ok(())
        }
        fn get_pressed_buttons(&self) -> HashSet<String> {
            self.pressed_buttons.clone()
        }
        /// Check if a button is pressed.
        fn get_button_state(&self, name: &str) -> bool {
            self.pressed_buttons.contains(name)
        }
        /// Check for currenly pressed buttons. If the new state differs from the
        /// old state, call the appropriate button event handlers.
        fn process(&mut self) {
            for entry in self.file_map.values_mut() {
                unsafe {
                    libc::ioctl(
                        entry.file.as_raw_fd(),
                        EVIOCGKEY.into(),
                        &mut entry.buffer_cache,
                    );
                }
            }
            self.pressed_buttons.clear();
            for (
                btn_name,
                ButtonMapEntry {
                    file_name,
                    key_code,
                },
            ) in self.button_map.iter()
            {
                let buffer = &self.file_map[file_name].buffer_cache;
                if (buffer[(key_code / 8) as usize] & 1 << (key_code % 8)) != 0 {
                    self.pressed_buttons.insert(btn_name.to_owned());
                }
            }
        }
    }
    /// Ev3 brick button handler. Opens the corresponding `/dev/input` file handlers.
    ///
    /// This implementation depends on the availability of the EVIOCGKEY ioctl
    /// to be able to read the button state buffer. See Linux kernel source
    /// in /include/uapi/linux/input.h for details.
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::Ev3Button;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// let button = Ev3Button::new()?;
    ///
    /// loop {
    ///     button.process();
    ///
    ///     println!("Is 'up' pressed: {}", button.is_up());
    ///     println!("Pressed buttons: {:?}", button.get_pressed_buttons());
    ///
    ///     thread::sleep(Duration::from_millis(100));
    /// }
    /// # }
    /// ```
    pub struct Ev3Button {
        button_handler: Rc<RefCell<ButtonFileHandler>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Ev3Button {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Ev3Button {
                    button_handler: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Ev3Button");
                    let _ = debug_trait_builder.field("button_handler", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Ev3Button {
        #[inline]
        fn clone(&self) -> Ev3Button {
            match *self {
                Ev3Button {
                    button_handler: ref __self_0_0,
                } => Ev3Button {
                    button_handler: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl Ev3Button {
        /// Ev3 brick button handler. Opens the corresponding `/dev/input` file handlers.
        pub fn new() -> Ev3Result<Self> {
            let mut handler = ButtonFileHandler::new();
            handler.add_button("up", "/dev/input/by-path/platform-gpio_keys-event", 103)?;
            handler.add_button("down", "/dev/input/by-path/platform-gpio_keys-event", 108)?;
            handler.add_button("left", "/dev/input/by-path/platform-gpio_keys-event", 105)?;
            handler.add_button("right", "/dev/input/by-path/platform-gpio_keys-event", 106)?;
            handler.add_button("enter", "/dev/input/by-path/platform-gpio_keys-event", 28)?;
            handler.add_button(
                "backspace",
                "/dev/input/by-path/platform-gpio_keys-event",
                14,
            )?;
            Ok(Self {
                button_handler: Rc::new(RefCell::new(handler)),
            })
        }
        /// Check for currenly pressed buttons. If the new state differs from the
        /// old state, call the appropriate button event handlers.
        pub fn process(&self) {
            self.button_handler.borrow_mut().process()
        }
        /// Get all pressed buttons by name.
        pub fn get_pressed_buttons(&self) -> HashSet<String> {
            self.button_handler.borrow().get_pressed_buttons()
        }
        /// Check if 'up' button is pressed.
        pub fn is_up(&self) -> bool {
            self.button_handler.borrow().get_button_state("up")
        }
        /// Check if 'down' button is pressed.
        pub fn is_down(&self) -> bool {
            self.button_handler.borrow().get_button_state("down")
        }
        /// Check if 'left' button is pressed.
        pub fn is_left(&self) -> bool {
            self.button_handler.borrow().get_button_state("left")
        }
        /// Check if 'right' button is pressed.
        pub fn is_right(&self) -> bool {
            self.button_handler.borrow().get_button_state("right")
        }
        /// Check if 'enter' button is pressed.
        pub fn is_enter(&self) -> bool {
            self.button_handler.borrow().get_button_state("enter")
        }
        /// Check if 'backspace' button is pressed.
        pub fn is_backspace(&self) -> bool {
            self.button_handler.borrow().get_button_state("backspace")
        }
    }
}
pub use buttons::Ev3Button;
mod power_supply {
    //! An interface to read data from the system’s power_supply class.
    //! Uses the built-in legoev3-battery if none is specified.
    use crate::{utils::OrErr, Attribute, Device, Driver, Ev3Error, Ev3Result};
    use std::fs;
    /// An interface to read data from the system’s power_supply class.
    /// Uses the built-in legoev3-battery if none is specified.
    pub struct PowerSupply {
        driver: Driver,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PowerSupply {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                PowerSupply {
                    driver: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PowerSupply");
                    let _ = debug_trait_builder.field("driver", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PowerSupply {
        #[inline]
        fn clone(&self) -> PowerSupply {
            match *self {
                PowerSupply {
                    driver: ref __self_0_0,
                } => PowerSupply {
                    driver: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl Device for PowerSupply {
        fn get_attribute(&self, name: &str) -> Attribute {
            self.driver.get_attribute(name)
        }
    }
    impl PowerSupply {
        /// Create a new instance of `PowerSupply`.
        pub fn new() -> Ev3Result<PowerSupply> {
            let paths = fs::read_dir("/sys/class/power_supply")?;
            for path in paths {
                let file_name = path?.file_name();
                let name = file_name.to_str().or_err()?;
                if name.contains("ev3-battery") {
                    return Ok(PowerSupply {
                        driver: Driver::new("power_supply", name),
                    });
                }
            }
            Err(Ev3Error::NotFound)
        }
        /// Returns the battery current in microamps
        pub fn get_current_now(&self) -> Ev3Result<i32> {
            self.get_attribute("current_now").get()
        }
        /// Always returns System.
        pub fn get_scope(&self) -> Ev3Result<String> {
            self.get_attribute("zscope").get()
        }
        /// Returns Unknown or Li-ion depending on if the rechargeable battery is present.
        pub fn get_technology(&self) -> Ev3Result<String> {
            self.get_attribute("technology").get()
        }
        /// Always returns Battery.
        pub fn get_type(&self) -> Ev3Result<String> {
            self.get_attribute("type").get()
        }
        /// Returns the nominal “full” battery voltage. The value returned depends on technology.
        pub fn get_voltage_max_design(&self) -> Ev3Result<i32> {
            self.get_attribute("voltage_max_design").get()
        }
        /// Returns the nominal “empty” battery voltage. The value returned depends on technology.
        pub fn get_voltage_min_design(&self) -> Ev3Result<i32> {
            self.get_attribute("voltage_min_design").get()
        }
        /// Returns the battery voltage in microvolts.
        pub fn get_voltage_now(&self) -> Ev3Result<i32> {
            self.get_attribute("voltage_now").get()
        }
    }
}
pub use power_supply::PowerSupply;
pub mod prelude {
    //! The purpose of this module is to alleviate imports of many common ev3dev traits.
    //!
    //! ```
    //! use ev3dev_lang_rust::prelude::*;
    //! ```
    pub use motors::{DcMotor, Motor, ServoMotor, TachoMotor};
    pub use sensors::Sensor;
    pub use Device;
    pub use Findable;
}
