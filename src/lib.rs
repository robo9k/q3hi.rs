#![crate_type = "cdylib"]

extern crate libc;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::ffi::CString;


pub type Syscall = extern "C" fn(arg: libc::intptr_t, ...) -> libc::intptr_t;

static mut SYSCALL: Option<Syscall> = None;


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn vmMain(command: libc::c_int,
                         arg0: libc::c_int,
                         arg1: libc::c_int,
                         arg2: libc::c_int,
                         arg3: libc::c_int,
                         arg4: libc::c_int,
                         arg5: libc::c_int,
                         arg6: libc::c_int,
                         arg7: libc::c_int,
                         arg8: libc::c_int,
                         arg9: libc::c_int,
                         arg10: libc::c_int,
                         arg11: libc::c_int)
                         -> libc::intptr_t {

    trace!("vmMain: command {}, arg0 {}, arg1 {}, arg2 {}, arg3 {}, arg4 {}, arg5 {}, arg6 {}, \
            arg7 {}, arg8 {}, arg9 {}, arg10 {}, arg11 {}",
           command,
           arg0,
           arg1,
           arg2,
           arg3,
           arg4,
           arg5,
           arg6,
           arg7,
           arg8,
           arg9,
           arg10,
           arg11);

    unsafe {
        if let Some(syscall) = SYSCALL {
            let msg = CString::new("Hello from Rust!").unwrap();
            syscall(1, msg.as_ptr());
        }
    }

    panic!("almost there..");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn dllEntry(syscall: Syscall) {
    env_logger::init().unwrap();

    debug!("dllEntry: syscall {:p}", syscall as *const ());
    unsafe {
        SYSCALL = Some(syscall);
    }
}
