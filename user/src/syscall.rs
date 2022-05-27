use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_ADD: usize = 1;


/// 系统调用
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        // asm! 可以嵌入到函数中
        // a0 存放函数调用返回值，也用来传递参数
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret, // a0
            in("x11") args[1],              // a1
            in("x12") args[2],              // a2
            in("x17") id                    // a7
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_add(num: usize) -> usize {
    syscall(SYSCALL_ADD, [num, 0, 0]) as usize
}