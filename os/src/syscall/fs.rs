//! File and filesystem-related syscalls

use crate::batch::get_current_app_address;

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let (start, end) = get_current_app_address();
    if (buf as usize) < start || (buf as usize) + len > end {
        panic!("try print address:[0x{:016x}] without permit, start: 0x{:016x}, end: 0x{:016x}", buf as usize, start, end);
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd[{}] in sys_write!", fd);
        }
    }
}
