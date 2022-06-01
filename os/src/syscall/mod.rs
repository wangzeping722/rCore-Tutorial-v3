const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

mod fs;
mod process;

use fs::*;
use process::*;
use crate::task::{TaskStatus, get_current_task};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize,
}


const MAX_SYSCALL_NUM: usize = 500;

#[repr(C)]
#[derive(Debug)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
}


pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    {
        get_current_task().call[syscall_id].times += 1;
    }
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

