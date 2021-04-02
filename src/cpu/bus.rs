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
        return 0x00;
    }

    // Write to RAM
    pub fn write(&self, address: u16, data: u8) -> () {
        if utils::check_hex_range(address){
            self.ram[address] = data;
        } else {
            panic!("Tried to write to an illegal hex address")
        }

    }
}

