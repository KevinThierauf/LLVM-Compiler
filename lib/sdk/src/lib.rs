#[no_mangle]
pub extern "C" fn sdk_print(length: u32, pointer: u64) {
    unsafe {
        assert_ne!(pointer, 0);
        let pointer: *const u8 = std::mem::transmute(pointer);
        println!("{}", String::from_utf8_lossy(std::slice::from_raw_parts(pointer, length as usize)));
    }
}
