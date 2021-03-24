use crate::cpu::flags::StatusRegFlags;
use crate::cpu::registers::Registers;
use test::TestFn::StaticBenchFn;

type Opcode = u32;

// Main CPU object
pub struct CPU {
    pub registers: Registers
}

// CPU methods
impl CPU {
    // Constructor
    pub fn new() -> Self {
        // Initialise registers
        Self {
            registers: Registers { a: 0x00, x: 0x00, y: 0x00, pc: 0x0000, sp: 0x00, status: 0x00, fetched: 0},
        }
    }

    // Reset CPU to default state
    // Registers set as per:
    // https://www.c64-wiki.com/wiki/Reset_(Process)
    pub fn reset(&mut self) {

        // Reset registers (Cycle 0)
        self.registers.a = 0x00;
        self.registers.x = 0x00;
        self.registers.y = 0x00;
        self.registers.sp = 0xFD;

        // Set program counter by reading from reset vector (0xFFFD - 0xFFFC)
        self.registers.pc = (bus::read(0xFFFD) << 8) + bus::read(0xFFFC);

        // Set status flags
    }

    fn fetch(&mut self) -> u8 {
        // TODO: Implement this method for the FDE cycle
        0
    }

    pub fn execute_instruction(&mut self, opcode: Opcode) {
        // TODO: Implement Base64 compressed opcode invocation to method
    }

    // NOTE: There will be no cpu::fetch() calls in the operations as we
    //       can use multiple calls in a closure so it would be better to
    //       pair it there rather than here.
    //
    // For example:
    // |_| -> () { fetch(); self.operations.ADC(); }

    // Add with carry
    pub fn ADC(&mut self) -> u8 {
        let mut temp: u16 = (self.registers.a as u16) + (self.registers.fetched as u16) + (self.registers.get_flag(StatusRegFlags::C) as u16);
        // Carry flag is set when addition exceeds 255 (into bit 9)
        self.registers.set_flag(StatusRegFlags::C, temp > 255);
        // Zero flag is set when the result is 0
        self.registers.set_flag(StatusRegFlags::Z, (temp & 0x00FF) == 0);
        // Signed overflow is set on the condition if the carry is set and the result exceeds an XOR with the first value
        self.registers.set_flag(StatusRegFlags::V, ((((self.registers.a as u16) ^ (self.registers.fetched as u16)) & ((self.registers.a as u16) ^ temp)) & 0x0080) == 0);
        // Negative flag is set when the most significant bit is set
        self.registers.set_flag(StatusRegFlags::N, temp & 0x0080 == 0);
        // Mask off the last 2 bytes into the 8-bit accumulator
        self.registers.a = (temp & 0x00FF) as u8;
        // Some variants of the ADC op have additional cycles
        return 1;
    }

    // Subtract with carry
    pub fn SBC(&mut self) -> u8 {
        let inverted_fetched: u16 = (self.registers.fetched as u16) ^ 0x00FF;
        let mut temp: u16 = (self.registers.a as u16) + inverted_fetched + (self.registers.get_flag(StatusRegFlags::C) as u16);
        // Carry flag is set when upper bits (8-16) have a value
        self.registers.set_flag(StatusRegFlags::C, temp & 0xFF00 != 0);
        // Zero flag is set when result is 0
        self.registers.set_flag(StatusRegFlags::Z, (temp & 0x00FF) == 0);
        // Signed overflow is set on the condition if there is wrap around underflow
        self.registers.set_flag(StatusRegFlags::V, ((temp ^ (self.registers.a as u16)) & (temp ^ inverted_fetched)) == 0);
        // Negative flag is set when the most significant bit is set
        self.registers.set_flag(StatusRegFlags::N, (temp & 0x0080) != 0);
        // Mask off the last 1 bytes into the 8-bit accumulator
        self.registers.a = (temp & 0x00FF) as u8;
        // Some variants of the SBC op have additional cycles
        return 1;
    }

    // Bitwise AND
    pub fn AND(&mut self) -> u8 {
        // Fetch data
        let fetched: u16 = self.registers.fetched as u16;
        // Peform AND with data and data in accumulator
        self.registers.a = a & fetched;
        // Zero flag set if result equals 0
        self.registers.set_flag(StatusRegFlags::Z, self.registers.a == 0x00);
        // Negative flag is set if the most significant bit of the result is set
        self.registers.set_flag(StatusRegFlags::N, self.registers.a & 0x80 != 0);
        return 1
    }
}





