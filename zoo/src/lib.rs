#![feature(cstr_memory)]
#![cfg_attr(test, deny(warnings))]
// #![deny(missing_docs)]

extern crate libc;
extern crate zoo_sys;
extern crate errno;

#[macro_use]
extern crate log;

pub use handler::ZooKeeper;

mod ffi;
mod handler;

use libc::c_int;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    SystemError,
    RuntimeInconsistency,
    DataInconsistency,
    ConnectionLoss,
    MarshallingError,
    Unimplemented,
    OperationTimeout,
    BadArguments,
    InvalidState,
    ApiError,
    NoNode,
    NoAuth,
    BadVersion,
    NochildrenForEphemerals,
    NodeExists,
    NotEmpty,
    SessionExpired,
    InvalidCallback,
    InvalidACL,
    AuthFailed,
    Closing,
    Nothing,
    SessionMoved
}

impl Error {
    pub fn from_raw(error: c_int) -> Option<Error> {
        use self::Error::*;

        match error {
            -1 => Some(SystemError),
            -2 => Some(RuntimeInconsistency),
            -3 => Some(DataInconsistency),
            -4 => Some(ConnectionLoss),
            -5 => Some(MarshallingError),
            -6 => Some(Unimplemented),
            -7 => Some(OperationTimeout),
            -8 => Some(BadArguments),
            -9 => Some(InvalidState),
            -100 => Some(ApiError),
            -101 => Some(NoNode),
            -102 => Some(NoAuth),
            -103 => Some(BadVersion),
            -108 => Some(NochildrenForEphemerals),
            -110 => Some(NodeExists),
            -111 => Some(NotEmpty),
            -112 => Some(SessionExpired),
            -113 => Some(InvalidCallback),
            -114 => Some(InvalidACL),
            -115 => Some(AuthFailed),
            -116 => Some(Closing),
            -117 => Some(Nothing),
            -118 => Some(SessionMoved),
            _ => None,
        }
    }

    pub fn from_errno() -> Option<Error> {
        Error::from_raw(errno::errno().0)
    }

    pub fn raw_into_result(error: c_int) -> Result<()> {
        match Error::from_raw(error) {
            Some(e) => Err(e),
            None => Ok(())
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
         fmt::Debug::fmt(self, fmt)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "ZooKeeper Error"
    }
}

