use std::sync::atomic::{AtomicU64, Ordering};

pub static CURRENT_INSTRUCTION: AtomicU64 = AtomicU64::new(0);
pub static TERMINATED: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
extern "C" fn set_current_instruction(instruction: u64) {
    CURRENT_INSTRUCTION.store(instruction, Ordering::SeqCst);
}
