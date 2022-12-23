use crate::excla::process;

mod excla;

extern "C" {
    fn get_length() -> u32;
    fn write_to_buffer(pointer: *const u8);
    fn result_buffer(pointer: *const u8, length: u32);
}

#[no_mangle]
pub unsafe extern fn run() {
    let length = get_length();
    println!("length: {}", length);
    let mut buffer = Vec::with_capacity(length as usize);
    let pointer = buffer.as_mut_ptr();

    write_to_buffer(pointer);
    buffer.set_len(length as usize);
    let str = std::str::from_utf8(&buffer).unwrap();
    println!("str: {}", str);
    let result = process(str);
    println!("result: {}", result);
    result_buffer(result.as_bytes().as_ptr(), result.len() as u32);
}
