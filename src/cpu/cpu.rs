

#[derive(Debug)]
struct Registers {
    a: u8,      // Accumulator
    x: u8,      // X register
    y: u8,      // Y register
    pc: u16,     // Program Counter
    stkp: u8,   // Stack pointer register
    status: u8, // Status register
}

// Defines the 8 flags for the status register
pub enum StatusRegFlags{
    C = (1 << 0), // Carry bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable interrupt
    D = (1 << 3), // Decimal mode
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

// Main CPU object
pub struct CPU {
    registers: Registers
}

impl CPU {

    // Constructor
    fn new() -> Self {
        // Initialise registers
        Self { registers: Registers { a: 0x00, x: 0x00, y: 0x00, pc: 0x0000, stkp: 0x00, status: 0x00}}
    }

    // Resets registers to original state pre-boot

}



fn main() {}