use std::os::unix::process::ExitStatusExt;
use std::process::{self, ExitStatus};

pub fn exit_with(status: ExitStatus) {
    if let Some(code) = status.code() {
        process::exit(code);
    } else if let Some(signal) = status.signal() {
        unsafe { libc::kill(libc::getpid(), signal) };
    } else {
        panic!("child process exited without exitcode or signal");
    }
}

// use std::time::Duration;
// use std::{io, thread};
// pub fn kill(pid: &sysinfo::Pid) -> io::Result<()> {
//     let pid = pid.as_u32() as libc::pid_t;
//     println!("{pid}");
//     if unsafe { libc::kill(pid, libc::SIGINT) } != 0 {
//         return Err(io::Error::last_os_error());
//     }
//     println!("signal sent");
//     loop {
//         match unsafe { libc::kill(pid, 0) } {
//             0 => {
//                 println!("process exists");
//                 thread::sleep(Duration::from_millis(1));
//                 continue;
//             }
//             -1 => match unsafe { *libc::__errno_location() } {
//                 libc::ESRCH => {
//                     println!("pid {pid} does not exist");
//                     return Ok(());
//                 },
//                 os_err @ _ => return Err(io::Error::from_raw_os_error(os_err)),
//             },
//             x @ _ => {
//                 println!("kill returned: {x}");
//                 return Err(io::Error::last_os_error());
//             }
//         }
//     }
// }
