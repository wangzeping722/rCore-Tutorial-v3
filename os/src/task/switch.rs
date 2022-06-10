//!Wrap `switch.S` as a function
use super::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

// 保存当前进程的栈，然后恢复指定进程的栈
extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
