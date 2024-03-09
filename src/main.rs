mod os_specific;

use std::{env, process};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

fn main() {
    let sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    let own_pid = Pid::from_u32(process::id());
    println!("own pid: {own_pid}");
    let us = sys.process(own_pid).expect("could not get own process");
    let own_cmd = us.cmd();
    let own_cwd = us.cwd();
    let own_user_id = us.user_id();

    for (pid, sibling) in sys.processes() {
        if pid == &own_pid {
            continue;
        }
        if sibling.cmd() == own_cmd {
            println!("pid {pid} matches args");
            if sibling.user_id() == own_user_id {
                println!("pid {pid} matches user_id");
                if sibling.cwd() == own_cwd {
                    println!("pid {pid} matches cwd, killing ...");
                    kill_tree::blocking::kill_tree(pid.as_u32()).unwrap();
                }
            }
        }
    }

    let mut args = env::args().skip(1);
    let command = match args.next() {
        None => panic!("nothing to run"),
        Some(cmd) => cmd,
    };
    let mut child = process::Command::new(command)
        .args(args)
        .spawn()
        .expect("could not spawn process");
    os_specific::exit_with(child.wait().unwrap());
}
