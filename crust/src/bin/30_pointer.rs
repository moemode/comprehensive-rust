fn main() {
    // Create aligned memory with known pattern
    let bytes: [u8; 8] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let ptr = bytes.as_ptr();

    unsafe {
        // Different size pointers to same address
        let p8 = ptr; // Points to: 11
        let p16 = ptr as *const u16; // Points to: 2211
        let p32 = ptr as *const u32; // Points to: 44332211
        let p64 = ptr as *const u64; // Points to: 8877665544332211

        println!("u8:  0x{:02x}", *p8);
        println!("u16: 0x{:04x}", *p16);
        println!("u32: 0x{:08x}", *p32);
        println!("u64: 0x{:016x}", *p64);
    }
}
