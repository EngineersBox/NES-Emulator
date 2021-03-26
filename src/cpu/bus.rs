use std::ptr::null;
use crate::cpu::utils;

pub struct Bus {
    ram: [u16;2048] // change this later
}
impl Bus {
    pub fn new() -> Self {
        Self {ram: [0x0000; 2048]}
    }

    // Read from RAM
    pub fn read(&self, address: u16) -> u16 {
        if utils::check_hex_range(address){
            return self.ram[address as usize];
        } else {
            panic!("Tried to read from an illegal hex address")
        }
    }
}

