use crate::cpu::flags::StatusRegFlags;
use crate::ternary;

#[derive(Debug)]
pub struct Registers {
    pub a: u8,       // Accumulator
    pub x: u8,       // X register
    pub y: u8,       // Y register
    pub pc: u16,     // Program Counter
    pub sp: u8,      // Stack pointer register
    pub status: u8,  // Status register
    pub fetched: u8  // Result from the fetch in FDE cycle
}

impl Registers {
    pub fn get_flag(&self, flag: StatusRegFlags) -> u8 {
        return ternary!(((self.status & (flag as u8)) > 0), 1, 0);
    }

    pub fn set_flag(&mut self, flag: StatusRegFlags, condition: bool) {
        ternary!(condition, self.status |= (flag as u8), self.status &= (flag as u8));
    }
}