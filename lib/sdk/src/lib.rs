use std::ptr::null;

#[no_mangle]
pub extern "C" fn sdk_print_string(pointer: *const u8, length: u32) {
    unsafe {
        if pointer == null() {
            println!();
        } else {
            println!("{}", String::from_utf8_lossy(std::slice::from_raw_parts(pointer, length as usize)));
        }
    }
}

#[no_mangle]
pub extern "C" fn sdk_print_int(value: i32) {
    println!("{value}");
}

#[no_mangle]
pub extern "C" fn sdk_print_float(value: f32) {
    println!("{value}");
}

