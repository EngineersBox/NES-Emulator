type Opcode = u32;

#[derive(Debug)]
pub struct Registers {
    pub a: u32,
    pub b: u32,
    pub c: u32
}

fn char_to_u32(c: char) -> u32 {
    c as u32
}

fn decode_base64(value: u32) -> u32 {
    if value >= char_to_u32('A') && value <= char_to_u32('Z') {
        return value - char_to_u32('A') + 0u32;
    } else if value >= char_to_u32('a') && value <= char_to_u32('z') {
        return value - char_to_u32('a') + 26u32;
    } else if value >= char_to_u32('0') && value <= char_to_u32('9') {
        return value - char_to_u32('0') + 52u32;
    } else if value == char_to_u32('+') {
        return 62u32;
    } else {
        return 63u32;
    }
}

macro_rules! t {
    ($pattern:expr, $index:expr, $bitmask:expr, $result:stmt) => {
        if (decode_base64(char_to_u32($pattern.chars().nth($index as usize).unwrap())) & $bitmask) == 0 {
            println!("{}", $pattern);
            $result
        }
    };
}

// For example:
// 0b00000011:
// - zAAAAAAAAAAA = ADD b
// - iAAAAAAAAAAA = ADD c
pub fn execute_instruction(registers: &mut Registers, opcode: Opcode) {
    let mut temp: u32 = 0;
    let index: u32 = opcode / 6u32;
    let bitmask: u32 = 1u32 << (opcode % 6u32);

    t!(String::from("/DAAAAAAAAAA"), index, bitmask, temp = registers.a);  // LDR a  (opcodes: 0,1,2,3,4,5,6,7)
    t!(String::from("zAAAAAAAAAAA"), index, bitmask, temp += registers.b); // ADD b  (opcodes: 0,1,    4,5,   )
    t!(String::from("MDAAAAAAAAAA"), index, bitmask, temp -= registers.b); // SUB b  (opcodes:     2,3,    6,7)
    t!(String::from("iAAAAAAAAAAA"), index, bitmask, temp += registers.c); // ADD c  (opcodes:   1,      5,   )
    t!(String::from("ICAAAAAAAAAA"), index, bitmask, temp -= registers.c); // SUB c  (opcodes:       3,      7)
    t!(String::from("/DAAAAAAAAAA"), index, bitmask, registers.a = temp);  // STR a  (opcodes: 0,1,2,3,4,5,6,7)
    t("PAAAAAAAAAAA", index, bitmask, temp = registers.a);                 // UPD F  (opcodes: 0,1,2,3        )
}