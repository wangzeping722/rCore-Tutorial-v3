use super::TaskContext;
use crate::syscall::SyscallInfo;

pub const MAX_SYSCALL_NUM: usize = 500;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    
    pub priority: usize,
    pub stride: usize,
    pub inititalized: bool
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}