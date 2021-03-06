#![allow(non_snake_case, dead_code)]
use crate::cpu::bus;
use crate::cpu::flags::StatusRegFlags;
use crate::cpu::registers::Registers;
use crate::ternary;

type Opcode = u32;

static PAGE_SIZE: u16 = 0x00FF;

// Main CPU object
pub struct CPU {
    pub registers: Registers,
    pub bus: bus::Bus,

    // Addressing variables
    pub addr_abs: u16,  // absolute address
    pub addr_rel: u8,   // relative address
    pub addr_temp: u16, // temporary address storage variable

    // Utility variables
    pub cycles: u8,     // instruction cycle counter
    pub cpu_cycles: u8, // overall global cycle counter
}

// CPU methods
impl CPU {
    // Constructor
    pub fn new() -> Self {
        // Initialise registers
        Self {
            registers: Registers {
                a: 0x00,
                x: 0x00,
                y: 0x00,
                pc: 0x0000,
                sp: 0x00,
                status: 0x00,
                fetched: 0,
            },
            bus: bus::Bus::new(),
            addr_abs: 0x0000,
            addr_rel: 0x00,
            addr_temp: 0x0000,
            cycles: 0,
            cpu_cycles: 0,
        }
    }
    pub fn read(&self, address: u16) -> u16 {
        return self.bus.read(address);
    }
    pub fn write(&mut self, address: u16, data: u8) -> () {
        self.bus.write(address, data);
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
        self.registers.status = 0x00 | self.registers.get_flag(StatusRegFlags::U);

        // Set program counter by reading from reset vector (0xFFFD - 0xFFFC)
        self.registers.pc = (self.bus.read(0xFFFD) << 8) + self.bus.read(0xFFFC);

        self.addr_abs = 0x0000;
        self.addr_rel = 0x0000;
        self.registers.fetched = 0x00;

        self.cycles = 8;
    }

    // Function fetches data from memory and sets the fetched register to the data
    fn fetch(&mut self) -> u8 {
        // if addrMode == "IMP" {
        //     self.registers.fetched = self.bus.read(self.addr_abs) as u8;
        // }
        return self.registers.fetched;
    }

    pub fn execute_instruction(&mut self, opcode: Opcode) {
        // TODO: Implement Base64 compressed opcode invocation to method
    }

    /*

    ADDRESSING MODE IMPLEMENTATIONS

    If addressing mode func and instruction return 1, then extra cycle required
    // TODO implement this in instructions


    */

    // Implied Addressing
    pub fn IMP(&mut self) -> u8 {
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
        let addr_l: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        let addr_h: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        self.addr_abs = addr_l + (addr_h << 8);

        return 0;
    }

    // Absolute with offset X addressing mode
    pub fn ABX(&mut self) -> u8 {
        // Convert low and high to u16
        let addr_l: u16 = self.read(self.registers.pc as u16);

        self.registers.pc += 1;
        let addr_h: u16 = self.read(self.registers.pc as u16);

        self.registers.pc += 1;
        self.addr_abs = addr_l + (addr_h << 8); // concat two u8 -> u16
        self.addr_abs += self.registers.x as u16; // offset by x register

        return ternary!((self.addr_abs & 0xFF00) != (addr_h << 8), 1, 0);
    }

    // Absolute with offset Y addressing mode
    pub fn ABY(&mut self) -> u8 {
        // Convert low and high to u16
        let addr_l: u16 = self.read(self.registers.pc as u16);

        self.registers.pc += 1;
        let addr_h: u16 = self.read(self.registers.pc as u16);

        self.registers.pc += 1;
        self.addr_abs = addr_l + (addr_h << 8);
        self.addr_abs += self.registers.y as u16;

        return ternary!((self.addr_abs & 0xFF00) != (addr_h << 8), 1, 0);
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
        self.addr_abs = self.read(self.registers.pc + self.registers.x as u16);
        self.registers.pc += 1;
        self.addr_abs &= PAGE_SIZE;
        return 0;
    }

    // Zero Page With Y Offset
    pub fn ZPY(&mut self) -> u8 {
        self.addr_abs = self.read(self.registers.pc + self.registers.y as u16);
        self.registers.pc += 1;
        self.addr_abs &= PAGE_SIZE;
        return 0;
    }

    // Relative Addressing (used for branching)
    // Can branch -128 to 128 away from pc
    pub fn REL(&mut self) -> u8 {
        self.addr_rel = self.read(self.registers.pc as u16) as u8;
        self.registers.pc += 1;
        // Checking GSB set to 1 (i.e. signed)
        if self.addr_rel & 0x80 == 0x10 {
            self.addr_rel = (self.addr_rel as u16 | 0xFF00) as u8;
        }
        return 0;
    }
    // Indirect Addressing (pointer)
    pub fn IND(&mut self) -> u8 {
        // Construct pointer from low / high byte in pc
        let ptr_l: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        let ptr_h: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        let ptr: u16 = ptr_l + (ptr_h << 8);

        // REPLICATE 6502 INDIRECT ADDRESSING BUG
        if ptr_l == 0xFF00 {
            self.addr_abs = (self.read(ptr & 0xFF00) << 8) | self.read(ptr + 0);
        } else {
            // Normal
            self.addr_abs = (self.read(ptr + 1) << 8) | self.read(ptr + 0);
        }
        return 0;
    }

    // Indirect with X offset Addressing (pointer)
    pub fn IZX(&mut self) -> u8 {
        // Obtain the data (= another address) at the pc address
        let temp: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        // Offset address and ensure it is in the zero page (with mask)
        let addr_l: u16 = self.read((temp + (self.registers.x as u16)) & 0x00FF);
        let addr_h: u16 = self.read((temp + (self.registers.x as u16) + 1) & 0x00FF);

        // Concat into full 16 bit address
        self.addr_abs = addr_h << 8 | addr_l;

        return 0;
    }

    // Indirect with Y offset Addressing (pointer)
    // Slightly different to X offset - offset after
    // 16 bit address is created (not during).
    pub fn IZY(&mut self) -> u8 {
        // Obtain the data (= another address) at the pc address
        let temp: u16 = self.read(self.registers.pc as u16);
        self.registers.pc += 1;
        // Offset address and ensure it is in the zero page (with mask)
        let addr_l: u16 = self.read(temp & 0x00FF);
        let addr_h: u16 = self.read((temp + 1) & 0x00FF);

        // Concat into full 16 bit address
        self.addr_abs = addr_h << 8 | addr_l;
        self.addr_abs += self.registers.y as u16;

        return ternary!(self.addr_abs & 0xFF00 != addr_h << 8, 1, 0);
    }

    /*

    INSTRUCTIONS IMPLEMENTATIONS

    */

    // Add with carry
    pub fn ADC(&mut self) -> u8 {
        let mut temp: u16 = (self.registers.a as u16)
            + (self.registers.fetched as u16)
            + (self.registers.get_flag(StatusRegFlags::C) as u16);
        // Carry flag is set when addition exceeds 255 (into bit 9)
        self.registers.set_flag(StatusRegFlags::C, temp > 255);
        // Zero flag is set when the result is 0
        self.registers
            .set_flag(StatusRegFlags::Z, (temp & 0x00FF) == 0);
        // Signed overflow is set on the condition if the carry is set and the result exceeds an XOR with the first value
        self.registers.set_flag(
            StatusRegFlags::V,
            ((((self.registers.a as u16) ^ (self.registers.fetched as u16))
                & ((self.registers.a as u16) ^ temp))
                & 0x0080)
                == 0,
        );
        // Negative flag is set when the most significant bit is set
        self.registers
            .set_flag(StatusRegFlags::N, temp & 0x0080 == 0);
        // Mask off the last 2 bytes into the 8-bit accumulator
        self.registers.a = (temp & 0x00FF) as u8;
        // Some variants of the ADC op have additional cycles
        return 1;
    }

    // Bitwise AND
    pub fn AND(&mut self) -> u8 {
        // Fetch data
        let fetched: u16 = self.registers.fetched as u16;
        // Peform AND with data and data in accumulator
        self.registers.a = self.registers.a & (fetched as u8);
        // Zero flag set if result equals 0
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.a == 0x00);
        // Negative flag is set if the most significant bit of the result is set
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.a & 0x80 != 0);
        return 1;
    }

    // Arithmetic Shift Left
    pub fn ASL(&mut self) -> u8 {
        // Fetch data
        let fetched: u16 = self.registers.fetched as u16;
        // Shift left 1
        let shifted = fetched << 1;
        // Set carry flag if bit 8 == 1 (old bit 7 == 1)
        self.registers
            .set_flag(StatusRegFlags::C, shifted & 0xFF00 != 0);
        // Zero flag set if result equals 0
        self.registers.set_flag(StatusRegFlags::Z, shifted == 0x00);
        // Negative flag is set if the most significant bit of the result is set
        self.registers
            .set_flag(StatusRegFlags::N, shifted & 0x80 != 0);

        // TODO: need address mode lookup
        return 1;
    }

    // Branch if carry clear
    pub fn BCC(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::C) == 0 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + (self.addr_rel as u16);
            // If over zero page?
            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Branch if carry set
    pub fn BCS(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::C) == 1 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Branch if equal
    pub fn BEQ(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::Z) == 1 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Test if one or more bits are set at the address (sets the zero flag)
    pub fn BIT(&mut self) -> u8 {
        self.fetch();
        self.addr_temp = (self.registers.a & self.registers.fetched) as u16;
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.fetched & (1 << 7) != 0);
        self.registers
            .set_flag(StatusRegFlags::V, self.registers.fetched & (1 << 7) != 0);
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp & 0x00FF == 0x00);
        return 0;
    }

    // Branch if minus
    // If negative flag set, set absolute address, check if zero page overflow
    // then set pc = addr_abs
    pub fn BMI(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::N) == 1 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + self.addr_rel as u16;
            // If over zero page?
            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Branch if positive
    pub fn BPL(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::N) == 0 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + self.addr_rel as u16;
            // If over zero page
            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Branch if not equal
    // Similar logic to BMI except with zero flag clear.
    pub fn BNE(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::Z) == 0 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + self.addr_rel as u16;

            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Break / force interrupt
    pub fn BRK(&mut self) -> u8 {
        self.registers.pc += 1;
        self.registers.set_flag(StatusRegFlags::I, true);
        self.write(
            0x0100 + self.registers.sp as u16,
            ((self.registers.pc >> 8) & 0x00FF) as u8,
        );
        self.registers.sp -= 1;
        self.write(
            0x0100 + self.registers.sp as u16,
            (self.registers.pc & 0x00FF) as u8,
        );
        self.registers.sp -= 1;

        self.registers.set_flag(StatusRegFlags::B, true);
        self.write(0x0100 + self.registers.sp as u16, self.registers.status);
        self.registers.sp -= 1;
        self.registers.set_flag(StatusRegFlags::B, false);

        // Load interrupt vector
        self.registers.pc = self.read(0xFFFE) | self.read(0xFFFF) << 8;
        return 0;
    }

    // Branch if overflow clear
    pub fn BVC(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::V) == 0 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + self.addr_rel as u16;

            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Branch if overflow set
    pub fn BVS(&mut self) -> u8 {
        if self.registers.get_flag(StatusRegFlags::V) == 1 {
            self.cycles += 1;
            self.addr_abs = self.registers.pc + self.addr_rel as u16;

            if (self.addr_abs & 0xFF00) != (self.registers.pc & 0xFF00) {
                self.cycles += 1
            }
            self.registers.pc = self.addr_abs;
        }
        return 0;
    }

    // Clear carry flag
    pub fn CLC(&mut self) -> u8 {
        self.registers.set_flag(StatusRegFlags::C, false);
        return 0
    }
    // Clear decimal flag
    pub fn CDC(&mut self) -> u8 {
        self.registers.set_flag(StatusRegFlags::D, false);
        return 0;
    }

    // Clear interrupt flag
    pub fn CLI(&mut self) -> u8 {
        self.registers.set_flag(StatusRegFlags::I, false);
        return 0;
    }

    // Clear overflow flag
    pub fn CLV(&mut self) -> u8 {
        self.registers.set_flag(StatusRegFlags::V, false);
        return 0;
    }

    // Compare accumulator
    pub fn CMP(&mut self) -> u8 {
        self.fetch();
        self.addr_temp = self.registers.a as u16 - self.registers.fetched as u16;
        self.registers.set_flag(
            StatusRegFlags::C,
            self.registers.a >= self.registers.fetched,
        );
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp & 0x00FF == 0x0000);
        self.registers
            .set_flag(StatusRegFlags::N, self.addr_temp & 0x0080 == 1);
        return 1;
    }
    // Compare X register
    pub fn CMX(&mut self) -> u8 {
        self.fetch();
        self.addr_temp = self.registers.x as u16 - self.registers.fetched as u16;
        self.registers.set_flag(
            StatusRegFlags::C,
            self.registers.x >= self.registers.fetched,
        );
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp & 0x00FF == 0x0000);
        self.registers
            .set_flag(StatusRegFlags::N, self.addr_temp & 0x0080 == 1);
        return 0;
    }
    // Compare Y register
    pub fn CMY(&mut self) -> u8 {
        self.fetch();
        self.addr_temp = self.registers.x as u16 - self.registers.fetched as u16;
        self.registers.set_flag(
            StatusRegFlags::C,
            self.registers.x >= self.registers.fetched,
        );
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp & 0x00FF == 0x0000);
        self.registers
            .set_flag(StatusRegFlags::N, self.addr_temp & 0x0080 == 1);
        return 0;
    }

    // Subtract 1 from value at memory location
    pub fn DEC(&mut self) -> u8 {
        return 0;
    }

    // Subtract 1 from X register
    pub fn DEX(&mut self) -> u8 {
        self.registers.x -= 1;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.x == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.x & 0x80 == 1);
        return 0;
    }

    // Subtract 1 from Y register
    pub fn DEY(&mut self) -> u8 {
        self.registers.y -= 1;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.y == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.y & 0x80 == 1);
        return 0;
    }

    // Exclusive OR (XOR)
    pub fn EOR(&mut self) -> u8 {
        self.fetch();
        self.registers.a = self.registers.a ^ self.registers.fetched;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.a == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.a & 0x80 == 1);
        return 1;
    }

    // Increment fetched memory
    pub fn INC(&mut self) -> u8 {
        self.fetch();
        self.addr_temp = (self.registers.fetched + 1) as u16;
        self.write(self.addr_abs, (self.addr_temp & 0x00FF) as u8);
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp & 0x00FF == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.addr_temp & 0x80 == 1);
        return 0;
    }

    // Increment X register
    pub fn INX(&mut self) -> u8 {
        self.registers.x += 1;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.x == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.x & 0x80 == 1);
        return 0;
    }

    // Increment Y register
    pub fn INY(&mut self) -> u8 {
        self.registers.y += 1;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.y == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.y & 0x80 == 1);
        return 0;
    }

    // Set PC to memory address specified
    pub fn JMP(&mut self) -> u8 {
        self.registers.pc = self.addr_abs;
        return 0;
    }

    // Jump to subroutine
    pub fn JSR(&mut self) -> u8 {
        self.registers.pc -= 1;
        self.write(
            0x0100 + self.registers.sp as u16,
            ((self.registers.pc >> 8) & 0x00FF) as u8,
        );
        self.registers.sp -= 1;
        self.write(
            0x0100 + self.registers.sp as u16,
            (self.registers.pc & 0x00FF) as u8,
        );
        self.registers.sp -= 1;

        self.registers.pc = self.addr_abs;
        return 0;
    }

    // Load Accumulator
    pub fn LDA(&mut self) -> u8 {
        self.fetch();
        self.registers.a = self.registers.fetched;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.a == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.a & 0x80 == 1);
        return 1;
    }

    // Load X Register
    pub fn LDX(&mut self) -> u8 {
        self.fetch();
        self.registers.x = self.registers.fetched;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.x == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.x & 0x80 == 1);
        return 1;
    }

    // Load Y Register
    pub fn LDY(&mut self) -> u8 {
        self.fetch();
        self.registers.y = self.registers.fetched;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.y == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.y & 0x80 == 1); // high byte set
        return 1;
    }

    // Logical Shift Left
    pub fn LSR(&mut self) -> u8 {
        self.fetch();
        self.registers
            .set_flag(StatusRegFlags::C, self.registers.fetched & 0x0001 == 1);
        self.addr_temp = self.registers.fetched as u16 >> 1;
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.addr_temp & 0x0080 == 1); // high byte set

        // TODO lookup address mode. If implied. set accumulator = temp & 0x00FF (i.e. bottom portion of addr_temp)
        return 0;
    }

    // No operation
    pub fn NOP(&mut self) -> u8 {
        // TODO Every repo i've seen does this differently
        return 0;
    }

    // Bitwise logical inclusive OR
    pub fn ORA(&mut self) -> u8 {
        self.fetch();
        self.registers.a = self.registers.a | self.registers.fetched;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.a == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.a & 0x80 == 1);
        return 1;
    }

    // Pushes a copy of accumulator to the stack
    pub fn PHA(&mut self) -> u8 {
        self.write(0x0100 + self.registers.sp as u16, self.registers.a);
        self.registers.sp -= 1;
        return 0;
    }

    // Pushes a copy of status flags onto the stack
    pub fn PHP(&mut self) -> u8 {
        self.write(
            0x0100 + self.registers.sp as u16,
            self.registers.status
                | self.registers.get_flag(StatusRegFlags::B)
                | self.registers.get_flag(StatusRegFlags::B),
        );
        self.registers.set_flag(StatusRegFlags::B, true);
        self.registers.set_flag(StatusRegFlags::U, true);
        self.registers.sp -= 1;

        return 0;
    }

    // Pulls an 8bit value from stack into accumulator
    pub fn PLA(&mut self) -> u8 {
        self.registers.sp += 1;
        self.registers.a = self.read(0x0100 + (self.registers.sp as u16)) as u8; // TODO why do all read functions start at 0100.
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.a & 0x80 == 1);
        return 0;
    }

    // Pull 8bit value from stack into status register flags
    pub fn PLP(&mut self) -> u8 {
        self.registers.sp += 1;
        self.registers.status = self.read(0x0100 + (self.registers.sp as u16)) as u8;
        self.registers.set_flag(StatusRegFlags::U, true);
        return 0;
    }

    // Rotate left

    pub fn ROL(&mut self) -> u8 {
        self.fetch();

        self.addr_temp =
            (self.registers.fetched << 1 | self.registers.get_flag(StatusRegFlags::C)) as u16;
        self.registers
            .set_flag(StatusRegFlags::C, self.addr_temp & 0xFF00 == 1);
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp & 0x00FF == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.addr_temp & 0x0080 == 1);

        // TODO address lookup required
        return 0;
    }

    // Rotate right
    pub fn ROR(&mut self) -> u8 {
        self.fetch();

        self.addr_temp = (self.registers.get_flag(StatusRegFlags::C) << 7
            | (self.registers.fetched >> 1)) as u16;
        self.registers
            .set_flag(StatusRegFlags::C, self.registers.fetched & 0x01 == 1);
        self.registers
            .set_flag(StatusRegFlags::Z, self.addr_temp & 0x00FF == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.addr_temp & 0x0080 == 1);

        // TODO WRITE REQUIRED HERE
        return 0;
    }

    // Return from interrupt
    pub fn RTI(&mut self) -> u8 {
        self.registers.sp += 1;
        self.registers.status = self.read(0x0100 + self.registers.sp as u16) as u8;
        self.registers.status &= !self.registers.get_flag(StatusRegFlags::B);
        self.registers.status &= !self.registers.get_flag(StatusRegFlags::U);

        self.registers.sp += 1;
        self.registers.pc = self.read(0x0100 + self.registers.sp as u16);
        self.registers.sp += 1;
        self.registers.pc |= self.read(0x0100 + self.registers.sp as u16) << 8;
        return 0;
    }

    // Return from subroutine
    pub fn RTS(&mut self) -> u8 {
        self.registers.sp += 1;
        self.registers.pc = self.read(0x0100 + self.registers.sp as u16);
        self.registers.sp += 1;
        self.registers.pc |= self.read(0x0100 + self.registers.sp as u16) << 8;
        self.registers.pc += 1;
        return 0;
    }

    // Subtract with carry
    pub fn SBC(&mut self) -> u8 {
        let inverted_fetched: u16 = (self.registers.fetched as u16) ^ 0x00FF;
        let mut temp: u16 = (self.registers.a as u16)
            + inverted_fetched
            + (self.registers.get_flag(StatusRegFlags::C) as u16);
        // Carry flag is set when upper bits (8-16) have a value
        self.registers
            .set_flag(StatusRegFlags::C, temp & 0xFF00 != 0);
        // Zero flag is set when result is 0
        self.registers
            .set_flag(StatusRegFlags::Z, (temp & 0x00FF) == 0);
        // Signed overflow is set on the condition if there is wrap around underflow
        self.registers.set_flag(
            StatusRegFlags::V,
            ((temp ^ (self.registers.a as u16)) & (temp ^ inverted_fetched)) == 0,
        );
        // Negative flag is set when the most significant bit is set
        self.registers
            .set_flag(StatusRegFlags::N, (temp & 0x0080) != 0);
        // Mask off the last 1 bytes into the 8-bit accumulator
        self.registers.a = (temp & 0x00FF) as u8;
        // Some variants of the SBC op have additional cycles
        return 1;
    }

    // Set carry flag
    pub fn SEC(&mut self) -> u8 {
        self.registers.set_flag(StatusRegFlags::C, true);
        return 0;
    }

    // Set decimal flag
    pub fn SED(&mut self) -> u8 {
        self.registers.set_flag(StatusRegFlags::D, true);
        return 0;
    }

    // Set interrupt flag
    pub fn SEI(&mut self) -> u8 {
        self.registers.set_flag(StatusRegFlags::I, true);
        return 0;
    }

    // Store accumulator data in memory
    pub fn STA(&mut self) -> u8 {
        self.write(self.addr_abs, self.registers.a);
        return 0;
    }

    // Store X register data in memory
    pub fn STX(&mut self) -> u8 {
        self.write(self.addr_abs, self.registers.x);
        return 0;
    }

    // Store Y register data in memory
    pub fn STY(&mut self) -> u8 {
        self.write(self.addr_abs, self.registers.y);
        return 0;
    }

    // Transfer accumulator content to x register
    pub fn TAX(&mut self) -> u8 {
        self.registers.x = self.registers.a;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.x & 0x00FF == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.x & 0x0080 == 1);
        return 0;
    }

    // Transfer accumulator content to y register
    pub fn TAY(&mut self) -> u8 {
        self.registers.y = self.registers.a;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.y & 0x00FF == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.y & 0x0080 == 1);
        return 0;
    }

    // Transfer X register content to the accumulator
    pub fn TXA(&mut self) -> u8 {
        self.registers.a = self.registers.x;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.a & 0x00FF == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.a & 0x0080 == 1);
        return 0;
    }

    // Transfer X register content to stack pointer
    pub fn TXS(&mut self) -> u8 {
        self.registers.sp = self.registers.x;
        return 0;
    }

    // Transfer Y register content to the accumulator
    pub fn TYA(&mut self) -> u8 {
        self.registers.a = self.registers.y;
        self.registers
            .set_flag(StatusRegFlags::Z, self.registers.a & 0x00FF == 0x00);
        self.registers
            .set_flag(StatusRegFlags::N, self.registers.a & 0x0080 == 1);
        return 0;
    }

    // Illegal opcodes
    pub fn XXX() -> u8 {
        return 0;
    }
}
