#[derive(Debug)]
struct Registers {
    a: u8,      // Accumulator
    x: u8,      // X register
    y: u8,      // Y register
    pc: u16,     // Program Counter
    stkp: u8,   // Stack pointer register
    status: u8, // Status register
}

// Main CPU object
pub struct CPU {
    pub registers: Registers
}
// CPU methods
impl CPU {
    // Constructor
    fn new() -> Self {
        // Initialise registers
        Self { registers: Registers { a: 0x00, x: 0x00, y: 0x00, pc: 0x0000, stkp: 0x00, status: 0x00}}
    }
}