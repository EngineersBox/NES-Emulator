use crate::cpu::flags::StatusRegFlags;
use crate::cpu::registers::Registers;
use crate::cpu::bus::Bus;
use test::TestFn::StaticBenchFn;

type Opcode = u32;

static PAGE_SIZE: u8 = 0xFF;

// Main CPU object
pub struct CPU {
    pub registers: Registers,
    pub bus: Bus,

    // Addressing variables
    pub addr_abs: u16, // absolute address
    pub addr_rel: u8, // relative address

    // Utility variables
    pub cycles: u8, // instruction cycle counter
    pub cpu_cycles: u8 // overall global cycle counter
}

// CPU methods
impl CPU {
    // Constructor
    pub fn new() -> Self {
        // Initialise registers
        Self {
            registers: Registers { a: 0x00, x: 0x00, y: 0x00, pc: 0x0000, sp: 0x00, status: 0x00, fetched: 0},
            bus: Bus::new(),
            addr_abs: 0x0000,
            addr_rel: 0x00,
            cycles: 0,
            cpu_cycles: 0,

        }
    }

   pub fn read(&self, address: u16) -> u16 {
        return self.bus.read(address);
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
        self.registers.fetched = 0x00;
        // Set program counter by reading from reset vector (0xFFFD - 0xFFFC)
        self.registers.pc = (bus::read(0xFFFD) << 8) + bus::read(0xFFFC);

        // TODO Set status flags on reset
    }


    // Function fetches data from memory and sets the fetched register to the data
    fn fetch(&mut self, &addrMode: String) -> () {
        if addrMode == "IMP" {
            self.registers.fetched = self.bus.read(self.addr_abs) as u8;
        }
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


    /*

     ADDRESSING MODE IMPLEMENTATIONS

     If addressing mode func and instruction return 1, then extra cycle required
     // TODO implement this in instructions


     */


    // Implied Addressing
    pub fn ACC(&mut self) -> u8 {
        self.registers.fetched = self.registers.a;
        return 1;
    }

    // Immediate Addressing
    pub fn IMM(&mut self) -> u8 {
        self.registers.pc += 1;
        self.addr_abs = self.registers.pc;
        return 0;
    }

    // Absolute Addressing
    pub fn ABS(&mut self) -> u8 {
        addrL: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        addrH: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        self.addr_abs = addrL + (addrH << 8);

        return 0;
    }

    // Zero Page Addressing
    pub fn ZP0(&mut self) -> u8 {
        self.addr_abs = self.read(self.registers.pc);
        self.registers.pc += 1;
        self.addr_abs &= PAGE_SIZE;
        return 0;
    }

    // Zero Page With X Offset
    pub fn ZPX(&mut self) -> u8 {
        self.addr_abs = self.read(self.registers.pc + self.registers.x);
        self.registers.pc += 1;
        self.addr_abs &= PAGE_SIZE;
        return 0;
    }

    // Zero Page With Y Offset
    pub fn ZPY(&mut self) -> u8 {
        self.addr_abs = self.read(self.registers.pc + self.registers.y);
        self.registers.pc += 1;
        self.addr_abs &= PAGE_SIZE;
        return 0;
    }



    /*

    INSTRUCTIONS IMPLEMENTATIONS

    */

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

    // Arithmetic Shift Left
    pub fn ASL(&mut self) -> u8 {
        // Fetch data
        let fetched: u16 = self.registers.fetched as u16;
        // Shift left 1
        let shifted = fetched << 1;
        // Set carry flag if bit 8 == 1 (old bit 7 == 1)
        self.registers.set_flag(StatusRegFlags::C, shifted & 0xFF00 != 0);
        // Zero flag set if result equals 0
        self.registers.set_flag(StatusRegFlags::Z, shifted == 0x00);
        // Negative flag is set if the most significant bit of the result is set
        self.registers.set_flag(StatusRegFlags::N, shifted & 0x80 != 0);

        // TODO: need to build the address mode functions
        //
        // If ASL address mode == implied, then set a = shifted & 0x00FF
        // else write shifted & 0x00FF to the abs address chosen with
        return 1
    }

    // Branch if carry clear
    pub fn BCC(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::C) == 0 {
            self.addr_abs = self.registers.pc + self.addr_rel;
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }
    // Branch if carry set
    pub fn BCS(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::C) == 1 {
            self.addr_abs = self.registers.pc + self.addr_rel;
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

}





