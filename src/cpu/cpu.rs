

#[derive(Debug)]
struct Registers {
    a: u8,      // Accumulator
    x: u8,      // X register
    y: u8,      // Y register
    pc: u16,     // Program Counter
    sp: u8,   // Stack pointer register
    status: u8, // Status register
}

// Main CPU object
pub struct CPU {
    pub registers: Registers
}
// CPU methods
impl CPU {
    // Constructor
    pub fn new() -> Self {
        // Initialise registers
        Self { registers: Registers { a: 0x00, x: 0x00, y: 0x00, pc: 0x0000, sp: 0x00, status: 0x00}}
    }

    // Reset CPU to default state
    // Registers set as per:
    // https://www.c64-wiki.com/wiki/Reset_(Process)

    pub fn reset(&mut self) -> (){

        // Reset registers (Cycle 0)
        self.registers.a = 0x00;
        self.registers.x = 0x00;
        self.registers.y = 0x00;
        self.registers.sp = 0xFD;


        // Set program counter by reading from reset vector (0xFFFD - 0xFFFC)
        self.registers.pc = (bus::read(0xFFFD) << 8) + bus::read(0xFFFC);

        // Set status flags







    }
}