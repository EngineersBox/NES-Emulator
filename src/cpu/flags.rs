// Defines the 8 flags for the status register
#[derive(Debug)]
pub enum StatusRegFlags {
    C = (1 << 0), // Carry bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable interrupt
    D = (1 << 3), // Decimal mode
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}
