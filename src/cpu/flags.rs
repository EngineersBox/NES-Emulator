type Flag = u8;

#[derive(Debug)]
pub struct Flags {
    pub C: Flag, // Carry
    pub Z: Flag, // Zero
    pub I: Flag, // Interrupt Disable
    pub D: Flag, // Decimal Mode
    pub B: Flag, // Break Command
    pub V: Flag, // Overflow
    pub N: Flag, // Negative
}