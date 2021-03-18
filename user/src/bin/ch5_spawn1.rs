#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{spawn, wait, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    let cpid = spawn("ch5_exit0");
    assert!(cpid >= 0, "child pid invalid");
    println!("new child {}", cpid);
    let mut exit_code: i32 = 0;
    let exit_pid = wait(&mut exit_code);
    assert_eq!(exit_pid, cpid, "error exit pid");
    assert_eq!(exit_code, 66778, "error exit code");
    println!("TEST wait OK!");
    let (cpid0, cpid1) = (spawn("ch5_exit0"), spawn("ch5_exit1"));
    let exit_pid = waitpid(cpid1 as usize, &mut exit_code);
    assert_eq!(exit_pid, cpid1, "error exit pid");
    assert_eq!(exit_code, -233, "error exit code");
    let exit_pid = wait(&mut exit_code);
    assert_eq!(exit_pid, cpid0, "error exit pid");
    assert_eq!(exit_code, 66778, "error exit code");
    println!("TEST waitpid OK!");
    0
}