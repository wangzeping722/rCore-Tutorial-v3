//! RISC-V timer-related functionality

/// 平台的时钟频率，每秒钟多少次
use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
const MICRO_PER_SEC: usize = 1000000;

/// read the `mtime` register
pub fn get_time() -> usize {
    // 获取当前寄存器`mtime`的值
    time::read()
}

/// get current time in milliseconds
pub fn get_time_ms() -> usize {
    // 每多少次滴答算1ms，CLOCK_FREQ / MSEC_PER_SEC
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

/// set the next timer interrupt
/// 设置下一次触发时钟中断的时间
pub fn set_next_trigger() {
    // CLOCK_FREQ / TICKS_PER_SEC 每过多少次时钟滴答就触发中断
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
