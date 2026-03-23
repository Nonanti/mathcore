#![no_std]

extern crate alloc;

use alloc::{format, string::String, vec::Vec};
use core::{panic::PanicInfo, slice, str};
use mathcore::MathCore;

#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

fn pack_buffer(ptr: *mut u8, len: usize) -> u64 {
    ((len as u64) << 32) | (ptr as u32 as u64)
}

fn leak_string(value: String) -> u64 {
    let mut bytes = value.into_bytes();
    let ptr = bytes.as_mut_ptr();
    let len = bytes.len();
    core::mem::forget(bytes);
    pack_buffer(ptr, len)
}

#[no_mangle]
pub extern "C" fn alloc_buffer(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    ptr
}

#[no_mangle]
pub extern "C" fn free_buffer(ptr: *mut u8, len: usize) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        drop(Vec::from_raw_parts(ptr, len, len));
    }
}

#[no_mangle]
pub extern "C" fn evaluate_expression(ptr: *const u8, len: usize) -> u64 {
    if ptr.is_null() {
        return leak_string("Error: null input pointer".into());
    }

    let input = unsafe { slice::from_raw_parts(ptr, len) };
    let expression = match str::from_utf8(input) {
        Ok(value) => value,
        Err(_) => return leak_string("Error: input was not valid UTF-8".into()),
    };

    let output = match MathCore::new().calculate(expression) {
        Ok(result) => format!("{} = {}", expression, result),
        Err(error) => format!("Error: {}", error),
    };

    leak_string(output)
}