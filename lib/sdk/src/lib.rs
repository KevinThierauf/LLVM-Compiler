use std::ptr::null;
use std::str::FromStr;

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

#[no_mangle]
pub extern "C" fn sdk_read_int() -> u32 {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).expect("failed to read from stdin");
        if let Ok(input) = u32::from_str(input.trim()) {
            return input;
        } else {
            println!("Invalid integer \"{input}\", try again.");
        }
    }
}

