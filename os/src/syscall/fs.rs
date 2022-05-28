use crate::batch::{get_current_app_address, UserStack, USER_STACK};

const FD_STDOUT: usize = 1;
const USER_STACK_SIZE: usize = 4096;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let (start, end) = get_current_app_address();
            // 检查用户栈
            let is_user_stack = ((buf as usize) >= USER_STACK.get_sp() - USER_STACK_SIZE)
                && ((buf as usize)+len <= USER_STACK.get_sp());
            // 检查代码段
            let is_code_space = (buf as usize) >= start && (buf as usize) + len <= end;

            if is_user_stack || is_code_space {
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            } else {
                -1
            }
        }
        _ => {
            // panic!("Unsupported fd[{}] in sys_write!", fd);
            return -1
        }
    }
}
