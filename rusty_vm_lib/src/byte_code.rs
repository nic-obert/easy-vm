use std::fmt;


/// String representation of byte code instructions
pub const BYTE_CODE_NAMES: [&str; BYTE_CODE_COUNT] = [
    "INTEGER_ADD",
    "INTEGER_SUB",
    "INTEGER_MUL",
    "INTEGER_DIV",
    "INTEGER_MOD",

    "FLOAT_ADD",
    "FLOAT_SUB",
    "FLOAT_MUL",
    "FLOAT_DIV",
    "FLOAT_MOD",

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

    "PUSH_STACK_POINTER_REG",
    "PUSH_STACK_POINTER_ADDR_IN_REG",
    "PUSH_STACK_POINTER_CONST",
    "PUSH_STACK_POINTER_ADDR_LITERAL",

    "POP_REG",
    "POP_ADDR_IN_REG",
    "POP_ADDR_LITERAL",

    "POP_STACK_POINTER_REG",
    "POP_STACK_POINTER_ADDR_IN_REG",
    "POP_STACK_POINTER_CONST",
    "POP_STACK_POINTER_ADDR_LITERAL",

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
    "SHIFT_LEFT",
    "SHIFT_RIGHT",

    "INTERRUPT_REG",
    "INTERRUPT_ADDR_IN_REG",
    "INTERRUPT_CONST",
    "INTERRUPT_ADDR_LITERAL",

    "EXIT"
];


/// Represents the byte code instruction set
#[derive(Debug, Clone, Copy)]
#[allow(dead_code, non_camel_case_types)]
pub enum ByteCodes {
    /**
     * Add `r1` and `r2` and store the result in `r1`.
     * Set `zf` if the result is zero.
     * Set `sf` if the most significant bit of the result is 1.
     * Set `rf` to 0.
     * Set `cf` if the operation overflowed.
     * Set `of` to `sf` xor `cf`.
    */
    INTEGER_ADD = 0,
    /**
     * Subtract `r2` from `r1` and store the result in `r1`.
     * Set `zf` if the result is zero.
     * Set `sf` if the most significant bit of the result is 1.
     * Set `rf` to 0.
     * Set `cf` if the operation overflowed.
     * Set `of` to `sf` xor `cf`.
    */
    INTEGER_SUB,
    INTEGER_MUL,
    INTEGER_DIV,
    INTEGER_MOD,

    FLOAT_ADD,
    FLOAT_SUB,
    FLOAT_MUL,
    FLOAT_DIV,
    FLOAT_MOD,

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

    PUSH_STACK_POINTER_REG,
    PUSH_STACK_POINTER_ADDR_IN_REG,
    PUSH_STACK_POINTER_CONST,
    PUSH_STACK_POINTER_ADDR_LITERAL,

    POP_INTO_REG,
    POP_INTO_ADDR_IN_REG,
    POP_INTO_ADDR_LITERAL,

    POP_STACK_POINTER_REG,
    POP_STACK_POINTER_ADDR_IN_REG,
    POP_STACK_POINTER_CONST,
    POP_STACK_POINTER_ADDR_LITERAL,

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
    SHIFT_LEFT,
    SHIFT_RIGHT,

    INTERRUPT_REG,
    INTERRUPT_ADDR_IN_REG,
    INTERRUPT_CONST,
    INTERRUPT_ADDR_LITERAL,

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

