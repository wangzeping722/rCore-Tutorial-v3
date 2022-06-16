mod context;
mod switch;
mod task;
mod manager;
mod processor;
mod pid;

use crate::{fs::{open_file, OpenFlags}, config::PAGE_SIZE, mm::{MapPermission, VirtPageNum, VirtAddr}};
// use crate::{loader::get_app_data_by_name, config::PAGE_SIZE, mm::{VirtPageNum, MapPermission, VirtAddr}};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
use alloc::sync::Arc;
use manager::fetch_task;
use lazy_static::*;
pub use context::TaskContext;

pub use processor::{
    run_tasks,
    current_task,
    current_user_token,
    current_trap_cx,
    take_current_task,
    schedule,
    spawn
};
pub use manager::add_task;
pub use pid::{PidHandle, pid_alloc, KernelStack};

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

pub fn mmap(start: usize, len: usize, port: usize) -> isize {
    let current_task = current_task().unwrap();
    let from: usize = start/PAGE_SIZE;
    let to: usize = (start+len)/PAGE_SIZE;

    let memory_set = &mut current_task.inner_exclusive_access().memory_set;
    for vpn in from..to {
        if true == memory_set.find_vpn(VirtPageNum::from(vpn)) {
            return -1;
        }
    }
    
    let permission = match port {
        1 => MapPermission::U | MapPermission::R,
        2 => MapPermission::U | MapPermission::W,
        3 => MapPermission::U | MapPermission::R | MapPermission::W,
        4 => MapPermission::U | MapPermission::X,
        5 => MapPermission::U | MapPermission::R | MapPermission::X,
        6 => MapPermission::U | MapPermission::X | MapPermission::W,
        _ => MapPermission::U | MapPermission::R | MapPermission::W | MapPermission::X,
    };

    memory_set.insert_framed_area(VirtAddr::from(start), VirtAddr::from(start+len), permission);

    for vpn in from..to {
        if false == memory_set.find_vpn(VirtPageNum::from(vpn)) {
            return -1;
        }
    }
    0
}

pub fn munmap(start: usize, len: usize) -> isize {
    let current_task = current_task().unwrap();
    let from: usize = start/PAGE_SIZE;
    let to: usize = (start+len)/PAGE_SIZE;

    let memory_set = &mut current_task.inner_exclusive_access().memory_set;

    for vpn in from..to {
        if false == memory_set.find_vpn(VirtPageNum::from(vpn)) {
            return -1;
        }
    }

    for vpn in from..to {
        memory_set.unmap(VirtPageNum::from(vpn));
    }

    for vpn in from..to {
        if true == memory_set.find_vpn(VirtPageNum::from(vpn)) {
            return -1;
        }
    }
    
    return 0
}