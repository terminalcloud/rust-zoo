use libc::{c_void, c_int, c_char};

use std::net::SocketAddr;
use std::ffi::CString;
use std::path::Path;
use std::{mem, ptr, str};

use ffi::{
    zookeeper_init,
    zookeeper_close,
    client_id,
    zhandle_t,
    zoo_client_id,
    zoo_recv_timeout,
    zoo_get_context,
    zoo_set_context,
    zoo_set_watcher,
    zoo_state,
};

use {Error, Result};

pub struct ZooKeeper {
    raw: *mut zhandle_t
}

pub type Type = c_int;
pub type State = c_int;

impl ZooKeeper {
    pub fn new<F>(hosts: &[SocketAddr], watcher: F, recv_timeout: i32) -> Result<Self>
    where F: Fn(&ZooKeeper, Type, State, &Path) {
        // Comma delimineted set of addresses.
        let raw_hosts = CString::new(hosts.iter()
            .map(|sock| sock.to_string())
            .collect::<Vec<_>>().connect(",")).unwrap();

        let watcher_context: *mut c_void = unsafe { mem::transmute(Box::new(watcher)) };

        extern fn raw_watcher<F>(zh: *mut zhandle_t, _type: c_int, state: c_int,
                                 path: *const c_char, context: *mut c_void)
        where F: Fn(&ZooKeeper, Type, State, &Path) {
            let watcher: &F = unsafe { mem::transmute(context) };
            let cstring = unsafe { CString::from_ptr(path) };
            let path = Path::new(str::from_utf8(cstring.as_bytes()).unwrap());
            let zh = ZooKeeper { raw: zh };

            watcher(&zh, _type, state, path);

            mem::forget(zh);
        }

        let raw = unsafe { zookeeper_init(
            raw_hosts.as_bytes_with_nul().as_ptr() as *const c_char,
            Some(raw_watcher::<F>),
            recv_timeout,
            ptr::null(),
            watcher_context,
            0,
        ) };

        if raw == ptr::null_mut() {
            Err(Error::from_errno().unwrap())
        } else {
            Ok(ZooKeeper {
                raw: raw
            })
        }

    }

    pub fn close(mut self) -> Result<()> {
        unsafe { self.close_inner() }
    }

    unsafe fn close_inner(&mut self) -> Result<()> {
        let raw = mem::replace(&mut self.raw, ptr::null_mut());
        Error::raw_into_result(zookeeper_close(raw))
    }
}

impl Drop for ZooKeeper {
    fn drop(&mut self) {
        if self.raw == ptr::null_mut() { return; }

        if let Err(e) = unsafe { self.close_inner() } {
            error!("Failed to close ZooKeeper: {}", e);
        }
    }
}

