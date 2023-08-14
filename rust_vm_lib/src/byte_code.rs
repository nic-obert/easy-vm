use std::fmt;


/// String representation of byte code instructions
pub const BYTE_CODE_NAMES: [&str; BYTE_CODE_COUNT] = [
    "ADD",
    "SUB",
    "MUL",
    "DIV",
    "MOD",

    "INC_REG",
    "INC_ADDR_IN_REG",
    "INC_ADDR_LITERAL",

    "DEC_REG",
    "DEC_ADDR_IN_REG",
    "DEC_ADDR_LITERAL",

    "NO_OPERATION",

    "MOVE_REG_REG",
    "MOVE_REG_ADDR_IN_REG",
    "MOVE_REG_CONST",
    "MOVE_REG_ADDR_LITERAL",
    "MOVE_ADDR_IN_REG_REG",
    "MOVE_ADDR_IN_REG_ADDR_IN_REG",
    "MOVE_ADDR_IN_REG_CONST",
    "MOVE_ADDR_IN_REG_ADDR_LITERAL",
    "MOVE_ADDR_LITERAL_REG",
    "MOVE_ADDR_LITERAL_ADDR_IN_REG",   
    "MOVE_ADDR_LITERAL_CONST",
    "MOVE_ADDR_LITERAL_ADDR_LITERAL",

    "PUSH_REG",
    "PUSH_ADDR_IN_REG",
    "PUSH_CONST",
    "PUSH_ADDR_LITERAL",

    "POP_REG",
    "POP_ADDR_IN_REG",
    "POP_ADDR_LITERAL",

    "LABEL",

    "JUMP",
    "JUMP_NOT_ZERO",
    "JUMP_ZERO",
    "JUMP_GREATER",
    "JUMP_LESS",
    "JUMP_GREATER_OR_EQUAL",
    "JUMP_LESS_OR_EQUAL",
    "JUMP_CARRY",
    "JUMP_NOT_CARRY",
    "JUMP_OVERFLOW",
    "JUMP_NOT_OVERFLOW",
    "JUMP_SIGN",
    "JUMP_NOT_SIGN",

    "CALL",
    "RETURN",

    "COMPARE_REG_REG",
    "COMPARE_REG_ADDR_IN_REG",
    "COMPARE_REG_CONST",
    "COMPARE_REG_ADDR_LITERAL",
    "COMPARE_ADDR_IN_REG_REG",
    "COMPARE_ADDR_IN_REG_ADDR_IN_REG",
    "COMPARE_ADDR_IN_REG_CONST",
    "COMPARE_ADDR_IN_REG_ADDR_LITERAL",
    "COMPARE_CONST_REG",
    "COMPARE_CONST_ADDR_IN_REG",
    "COMPARE_CONST_CONST",
    "COMPARE_CONST_ADDR_LITERAL",
    "COMPARE_ADDR_LITERAL_REG",
    "COMPARE_ADDR_LITERAL_ADDR_IN_REG",
    "COMPARE_ADDR_LITERAL_CONST",
    "COMPARE_ADDR_LITERAL_ADDR_LITERAL",

    "AND",
    "OR",
    "XOR",
    "NOT",

    "PRINT_SIGNED",
    "PRINT_UNSIGNED",
    "PRINT_CHAR",
    "PRINT_STRING",
    "PRINT_BYTES",

    "INPUT_SIGNED_INT",
    "INPUT_UNSIGNED_INT",
    "INPUT_STRING",

    "EXIT"
];


/// Represents the byte code instruction set
#[derive(Debug, Clone, Copy)]
#[allow(dead_code, non_camel_case_types)]
pub enum ByteCodes {
    ADD = 0,
    SUB,
    MUL,
    DIV,
    MOD,

    INC_REG,
    INC_ADDR_IN_REG,
    INC_ADDR_LITERAL,

    DEC_REG,
    DEC_ADDR_IN_REG,
    DEC_ADDR_LITERAL,

    NO_OPERATION,

    MOVE_INTO_REG_FROM_REG,
    MOVE_INTO_REG_FROM_ADDR_IN_REG,
    MOVE_INTO_REG_FROM_CONST,
    MOVE_INTO_REG_FROM_ADDR_LITERAL,
    MOVE_INTO_ADDR_IN_REG_FROM_REG,
    MOVE_INTO_ADDR_IN_REG_FROM_ADDR_IN_REG,
    MOVE_INTO_ADDR_IN_REG_FROM_CONST,
    MOVE_INTO_ADDR_IN_REG_FROM_ADDR_LITERAL,
    MOVE_INTO_ADDR_LITERAL_FROM_REG,
    MOVE_INTO_ADDR_LITERAL_FROM_ADDR_IN_REG,
    MOVE_INTO_ADDR_LITERAL_FROM_CONST,
    MOVE_INTO_ADDR_LITERAL_FROM_ADDR_LITERAL,

    PUSH_FROM_REG,
    PUSH_FROM_ADDR_IN_REG,
    PUSH_FROM_CONST,
    PUSH_FROM_ADDR_LITERAL,

    POP_INTO_REG,
    POP_INTO_ADDR_IN_REG,
    POP_INTO_ADDR_LITERAL,

    LABEL,

    JUMP,
    JUMP_NOT_ZERO,
    JUMP_ZERO,
    JUMP_GREATER,
    JUMP_LESS,
    JUMP_GREATER_OR_EQUAL,
    JUMP_LESS_OR_EQUAL,
    JUMP_CARRY,
    JUMP_NOT_CARRY,
    JUMP_OVERFLOW,
    JUMP_NOT_OVERFLOW,
    JUMP_SIGN,
    JUMP_NOT_SIGN,

    CALL,
    RETURN,

    COMPARE_REG_REG,
    COMPARE_REG_ADDR_IN_REG,
    COMPARE_REG_CONST,
    COMPARE_REG_ADDR_LITERAL,
    COMPARE_ADDR_IN_REG_REG,
    COMPARE_ADDR_IN_REG_ADDR_IN_REG,
    COMPARE_ADDR_IN_REG_CONST,
    COMPARE_ADDR_IN_REG_ADDR_LITERAL,
    COMPARE_CONST_REG,
    COMPARE_CONST_ADDR_IN_REG,
    COMPARE_CONST_CONST,
    COMPARE_CONST_ADDR_LITERAL,
    COMPARE_ADDR_LITERAL_REG,
    COMPARE_ADDR_LITERAL_ADDR_IN_REG,
    COMPARE_ADDR_LITERAL_CONST,
    COMPARE_ADDR_LITERAL_ADDR_LITERAL,

    AND,
    OR,
    XOR,
    NOT,

    PRINT_SIGNED,
    PRINT_UNSIGNED,
    PRINT_CHAR,
    PRINT_STRING,
    PRINT_BYTES,

    INPUT_SIGNED_INT,
    INPUT_UNSIGNED_INT,
    INPUT_STRING,

    // This has to be the last variant
    EXIT
}


pub const BYTE_CODE_COUNT: usize = {
    assert!((ByteCodes::EXIT as usize) < 256);
    ByteCodes::EXIT as usize + 1
};


impl fmt::Display for ByteCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", BYTE_CODE_NAMES[*self as usize])
    }
}


impl std::convert::From<u8> for ByteCodes {

    fn from(value: u8) -> Self {
        if value < BYTE_CODE_COUNT as u8 {
            unsafe { std::mem::transmute(value) }
        } else {
            panic!("Invalid byte code: {}", value);
        }
    }
}


/// Return whether the given instruction is a jump instruction
pub fn is_jump_instruction(instruction: ByteCodes) -> bool {
    ByteCodes::JUMP as usize <= instruction as usize && instruction as usize <= ByteCodes::RETURN as usize
}

