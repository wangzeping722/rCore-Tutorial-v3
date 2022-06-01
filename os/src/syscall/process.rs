use alloc::task;

use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next, get_current_task,
};
use crate::timer::get_time_us;

use super::TaskInfo;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// pub fn sys_get_time() -> isize {
//     get_time_ms() as isize
// }

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

pub fn sys_task_info(id: usize, task_info: &mut TaskInfo) -> isize {
    task_info.id = id;
    task_info.time = get_time_us() - get_current_task().startTime;
    task_info.status = get_current_task().task_status;
    task_info.call = get_current_task().call;
    0
}