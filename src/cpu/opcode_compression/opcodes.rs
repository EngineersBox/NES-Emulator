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

// For example:
// 0b00000011:
// - zAAAAAAAAAAA = ADD b
// - iAAAAAAAAAAA = ADD c
pub fn execute_instruction(registers: &mut Registers, opcode: Opcode) {
    let mut temp: u32 = 0;

    macro_rules! t {
        ($pattern:expr, $result:stmt) => {
            if (decode_base64(char_to_u32($pattern.chars().nth((opcode / 6u32) as usize).unwrap())) & (1u32 << (opcode % 6u32))) == 0 {
                $result
            }
        };
    }

    t!(String::from("/DAAAAAAAAAA"), temp = registers.a);  // LDR a  (opcodes: 0,1,2,3,4,5,6,7)
    t!(String::from("zAAAAAAAAAAA"), temp += registers.b); // ADD b  (opcodes: 0,1,    4,5,   )
    t!(String::from("MDAAAAAAAAAA"), temp -= registers.b); // SUB b  (opcodes:     2,3,    6,7)
    t!(String::from("iAAAAAAAAAAA"), temp += registers.c); // ADD c  (opcodes:   1,      5,   )
    t!(String::from("ICAAAAAAAAAA"), temp -= registers.c); // SUB c  (opcodes:       3,      7)
    t!(String::from("/DAAAAAAAAAA"), registers.a = temp);  // STR a  (opcodes: 0,1,2,3,4,5,6,7)
    t!(String::from("PAAAAAAAAAAA"), temp = registers.a);  // UPD F  (opcodes: 0,1,2,3        )
}

// AN EXAMPLE USAGE:
// ------------------------------------------------------------------------
// fn main() {
//     let mut reg: Registers = Registers{a: 0u32, b: 1u32, c: 1u32};
//     let opcode: Opcode = 0b00000011u32; // 3: LDR a, ADD b, ADD c, STR a
//     execute_instruction(&mut reg, opcode);
//     println!("{:?}", reg);
// }