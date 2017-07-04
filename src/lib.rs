#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate log;
extern crate log4rs;
#[cfg(feature = "file")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "file")]
extern crate serde;

#[cfg(target_family = "unix")]
#[cfg(feature = "file")]
mod file;
#[cfg(target_family = "unix")]
#[cfg(feature = "file")]
pub use file::*;

#[cfg(target_family = "unix")]
mod syslog;
#[cfg(target_family = "unix")]
pub use syslog::*;
