use crate::mm::translated_byte_buffer;
use crate::{mm, task};
use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
    current_user_token
};
use crate::timer::get_time_us;

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

pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    // 要把值写到用户空间中
    let _us = get_time_us();
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    
    let token = current_user_token();

    let mut buffers = translated_byte_buffer(token, _ts as *const u8, 16);
    let buffers = buffers[0].as_mut_ptr();
    let _ts = buffers as *mut TimeVal;
    unsafe {
        *_ts = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    0
}