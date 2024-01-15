use std::sync::{atomic::{AtomicU64, Ordering}, RwLock};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CURRENT_INSTRUCTION: AtomicU64 = AtomicU64::new(0);
    pub static ref TERMINATED: AtomicU64 = AtomicU64::new(0);
    pub static ref TEXT_IO_BUFFER: RwLock<String> = RwLock::new(String::new());
}


#[no_mangle]
pub extern "C" fn set_current_instruction(instruction: u64) {
    CURRENT_INSTRUCTION.store(instruction, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn putc(input: u64) {
    let mut buffer = TEXT_IO_BUFFER.write().unwrap();
    buffer.push(input as u8 as char);
}

#[no_mangle]
pub extern "C" fn putnumber(input: u64) {
    let mut buffer = TEXT_IO_BUFFER.write().unwrap();
    buffer.push_str(format!("{}", input).as_str());
}
