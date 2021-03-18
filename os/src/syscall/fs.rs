use crate::mm::{
    translated_byte_buffer,
};
use crate::task::current_user_token;

const FD_STDOUT: usize = 1;
// use crate::task::get_app_address_space_current;
// use crate::task::get_task_current;
// use crate::task::get_user_stack_space_current;

/// TODO: 增加对syswrite的地址范围类型检查
/// 代码段 .text 不允许被修改；
/// 只读数据段 .rodata 不允许被修改，也不允许从它上面取指；
/// .data/.bss 均允许被读写，但是不允许从它上面取指。
/// 
/// 功能：将内存中缓冲区中的数据写入文件。
/// 参数：`fd` 表示待写入文件的文件描述符；
///      `buf` 表示内存中缓冲区的起始地址；
///      `len` 表示内存中缓冲区的长度。
/// 返回值：返回成功写入的长度。
/// syscall ID：64
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    // trace!("[kernel] now app {} is writing...",get_task_current());
    // trace!("call sys_write......");
    trace!("in sys_write...fd:{},buf:{:#x},len:{}",fd,buf as usize,len);

    // 下面就是对sys_write的检查
    // 在增加了映射规则后，就是只有权限为可写的地址才可以真的输出
    // 因此应该在这里进行一个地址转换

    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        },
        _ => {
            // return fd as isize;
            return -1 as isize;
            panic!("Unsupported fd in sys_write!");
        }
    }
}