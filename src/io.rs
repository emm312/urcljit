use std::sync::atomic::{AtomicU64, Ordering};

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CURRENT_INSTRUCTION: AtomicU64 = AtomicU64::new(0);
    pub static ref TERMINATED: AtomicU64 = AtomicU64::new(0);
    pub static ref TEXT_IO_BUFFER: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
extern "C" fn set_current_instruction(instruction: u64) {
    CURRENT_INSTRUCTION.store(instruction, Ordering::SeqCst);
}
