use core::arch::asm;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

const STACK_SIZE : usize = 0x1000;

// 获取当前栈指针
unsafe fn r_sp() -> usize {
    let sp: usize;

    asm!("mv {}, sp", out(reg) sp);

    sp
}

