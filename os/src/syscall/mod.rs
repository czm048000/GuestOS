const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
// const SYSCALL_GETTIMEOFDAY: usize = 169;
const SYSCALL_SET_PRIORITY: usize = 140;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_SPAWN: usize = 400;

mod fs;
mod process;
mod memory;
mod trap;

use fs::*;
use process::*;
use memory::*;
use trap::*;
use crate::timer::TimeVal;
//现在的问题就是TimeVal为什么地址不能用？照理来说应该在创建的时候自动修改了才对

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    debug!("in syscall...{},{},{},{}",syscall_id,args[0],args[1],args[2]);
    match syscall_id {
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),

        // SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),

        //[time as *const _ as usize, tz, 0]
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_SET_PRIORITY => sys_set_priority(args[0]),
        SYSCALL_SPAWN => sys_spawn(args[0] as *const u8),

        SYSCALL_MMAP => sys_mmap(args[0],args[1],args[2]),
        SYSCALL_MUNMAP => sys_munmap(args[0],args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}