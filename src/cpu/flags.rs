type Flag = u8;

#[derive(Debug)]
pub struct Flags {
    pub C: Flag,
    pub Z: Flag,
    pub I: Flag,
    pub D: Flag,
    pub B: Flag,
    pub V: Flag,
    pub N: Flag,
}