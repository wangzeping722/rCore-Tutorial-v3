//! App management syscalls
use crate::batch::run_next_app;
use log::info;
use crate::batch;


/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    let app_id = batch::get_current_app();
    info!("[kernel][{}] Application exited with code {}", app_id, exit_code);
    run_next_app()
}
