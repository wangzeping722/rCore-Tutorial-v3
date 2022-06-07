use crate::config::PAGE_SIZE;
use crate::mm::translated_byte_buffer;
use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
    current_user_token, mmap, munmap
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

const MAX_MAP_SIZE: usize = 1 << 30;

pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    // 参数校验
    if len == 0 {
        return 0;
    }

    if len > MAX_MAP_SIZE {
        return -1;
    }

    // 地址未对齐
    if start & PAGE_SIZE - 1 != 0 {
        return -1
    }

    if ((port & !0x7) != 0) || ((port & 0x7) == 0) {
        return  -1;
    }

    // 长度向上对齐取整
    let len = (len+PAGE_SIZE-1)&(!(PAGE_SIZE-1));

    println!("start mmap @@@@@@@@@@@@@@@@@@@@@@@@@ {}", &len);
    let ret = mmap(start, len, port);
    println!("end mmap @@@@@@@@@@@@@@@@@@@@@@@@@");

    ret
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    // 参数校验
    if len == 0 {
        return 0;
    }

    if len > MAX_MAP_SIZE {
        return -1;
    }

    // 地址未对齐
    if start & PAGE_SIZE - 1 != 0 {
        return -1
    }

    // 长度向上对齐取整
    let len = (len+PAGE_SIZE-1)&(!(PAGE_SIZE-1));

    println!("start unmap ######################### {}", &len);
    let ret = munmap(start, len);
    println!("end unmap #########################");

    ret
}