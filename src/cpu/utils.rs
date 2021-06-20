// Verifies if hex address in a legal range.
pub fn check_hex_range(addr: u16) -> bool {
    return if addr < 0x0000 || addr > 0xFFFF {
        false
    } else {
        true
    };
}
