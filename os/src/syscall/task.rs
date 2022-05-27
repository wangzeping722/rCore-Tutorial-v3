
use log::info;

use crate::batch::{get_current_app};

pub fn sys_get_task_info() -> isize {
    info!("[kernel] current app id: {}", get_current_app());
    0
}