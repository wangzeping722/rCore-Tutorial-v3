use crate::config::PAGE_SIZE;
use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
    current_task,
    current_user_token,
    add_task, spawn, mmap, munmap,
};
use crate::timer::get_time_us;
use crate::mm::{
    translated_str,
    translated_refmut,
    translated_ref, translated_byte_buffer,
};
use crate::fs::{
    open_file,
    OpenFlags,
};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::string::String;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8, mut args: *const usize) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        let arg_str_ptr = *translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(translated_str(token, arg_str_ptr as *const u8));
        unsafe { args = args.add(1); }
    }
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let all_data = app_inode.read_all();
        let task = current_task().unwrap();
        let argc = args_vec.len();
        task.exec(all_data.as_slice(), args_vec);
        // return argc because cx.x[10] will be covered with it later
        argc as isize
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();
    // find a child process

    // ---- access current PCB exclusively
    let mut inner = task.inner_exclusive_access();
    if inner.children
        .iter()
        .find(|p| {pid == -1 || pid as usize == p.getpid()})
        .is_none() {
        return -1;
        // ---- release current PCB
    }
    let pair = inner.children
        .iter()
        .enumerate()
        .find(|(_, p)| {
            // ++++ temporarily access child PCB exclusively
            p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.getpid())
            // ++++ release child PCB
        });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after being removed from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily access child PCB exclusively
        let exit_code = child.inner_exclusive_access().exit_code;
        // ++++ release child PCB
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB lock automatically
}

pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
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

pub fn sys_spawn(path: *const u8) -> isize {
    // 获得当前进程用户态页表
    let token = current_user_token();
    let path = translated_str(token, path);

    if let Some(data) = get_app_data_by_name(path.as_str()) {
        // 创建新进程，拷贝代码段
        spawn(data)
    } else {
        -1
    }
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

    let ret = mmap(start, len, port);

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

    let ret = munmap(start, len);

    ret
}