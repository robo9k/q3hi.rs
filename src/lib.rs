extern crate libc;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::ffi::CString;


/// syscallptr from g_syscalls.c
pub type Syscall = extern "C" fn(arg: libc::intptr_t, ...) -> libc::intptr_t;

static mut SYSCALL: Option<Syscall> = None;

/// gameExport_t from g_public.h
enum GameExport {
    Init = 0,
    Shutdown = 1,
}

/// gameImport_t from g_public.h
enum GameImport {
    Error = 1,
}

/// trap_Error() from g_syscalls.c
fn error<T>(text: T)
    where T: Into<Vec<u8>>
{
    unsafe {
        if let Some(syscall) = SYSCALL {
            let msg = CString::new(text).unwrap();
            syscall(GameImport::Error as libc::intptr_t, msg.as_ptr());
        }
    }

    unreachable!();
}

/// G_InitGame() from g_main.c
fn init_game(level_time: i32, random_seed: i32, restart: bool) {
    debug!("init_game: level_time {}, random_seed {}, restart {}",
           level_time,
           random_seed,
           restart);

    error("Hello from Rust!");
}


/// G_ShutdownGame() from g_main.c
fn shutdown_game(restart: bool) {
    debug!("shutdown_game: restart {}", restart);
}

/// vmMain() from g_main.c
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

    match command {
        command if command == GameExport::Init as libc::c_int => {
            init_game(arg0 as i32, arg1 as i32, 0 != arg2);
            return 0;
        }
        command if command == GameExport::Shutdown as libc::c_int => {
            shutdown_game(0 != arg0);
            return 0;
        }
        _ => panic!("Not implemented"),
    }
}

/// dllEntry() from g_syscalls.c
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn dllEntry(syscall: Syscall) {
    env_logger::init().unwrap();

    debug!("dllEntry: syscall {:p}", syscall as *const ());

    unsafe {
        SYSCALL = Some(syscall);
    }
}
