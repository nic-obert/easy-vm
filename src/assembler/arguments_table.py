from typing import Dict, Tuple, Union

from shared.byte_code import ByteCodes


arguments_table : Dict[str, Tuple[Union[Tuple[Union[Tuple, ByteCodes]], ByteCodes]]] = \
{
    # Arithmetic
    'add': (
        (
            ByteCodes.ADD,
        )
    ),
    'sub': (
        (
            ByteCodes.SUB,
        )
    ),
    'mul': (
        (
            ByteCodes.MUL,
        )
    ),
    'div': (
        (
            ByteCodes.DIV,
        )
    ),
    'mod': (
        (
            ByteCodes.MOD,
        )
    ),

    'inc': (
        # Register
        ByteCodes.INC_REG
    ),
    'inc1': (
        None, # Register
        # Address in register
        ByteCodes.INC1_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.INC1_ADDR_LITERAL,
    ),
    'inc2': (
        None, # Register
        # Address in register
        ByteCodes.INC2_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.INC2_ADDR_LITERAL,
    ),
    'inc4': (
        None, # Register
        # Address in register
        ByteCodes.INC4_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.INC4_ADDR_LITERAL,
    ),
    'inc8': (
        None, # Register
        # Address in register
        ByteCodes.INC8_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.INC8_ADDR_LITERAL,
    ),

    'dec': (
        # Register
        ByteCodes.DEC_REG
    ),
    'dec1': (
        None, # Register
        # Address in register
        ByteCodes.DEC1_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.DEC1_ADDR_LITERAL,
    ),
    'dec2': (
        None, # Register
        # Address in register
        ByteCodes.DEC2_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.DEC2_ADDR_LITERAL,
    ),
    'dec4': (
        None, # Register
        # Address in register
        ByteCodes.DEC4_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.DEC4_ADDR_LITERAL,
    ),
    'dec8': (
        None, # Register
        # Address in register
        ByteCodes.DEC8_ADDR_IN_REG,
        None, # Constant
        # Address literal
        ByteCodes.DEC8_ADDR_LITERAL,
    ),

    # No operation

    'nop': ByteCodes.NO_OPERATION,

    # Memory

    'ld1': (
        # Register
        (
            ByteCodes.LOAD1_REG_REG,
            ByteCodes.LOAD1_REG_ADDR_IN_REG,
            ByteCodes.LOAD1_REG_CONST,
            ByteCodes.LOAD1_REG_ADDR_LITERAL,
        )
    ),
    'ld2': (
        # Register
        (
            ByteCodes.LOAD2_REG_REG,
            ByteCodes.LOAD2_REG_ADDR_IN_REG,
            ByteCodes.LOAD2_REG_CONST,
            ByteCodes.LOAD2_REG_ADDR_LITERAL,
        )
    ),
    'ld4': (
        # Register
        (
            ByteCodes.LOAD4_REG_REG,
            ByteCodes.LOAD4_REG_ADDR_IN_REG,
            ByteCodes.LOAD4_REG_CONST,
            ByteCodes.LOAD4_REG_ADDR_LITERAL,
        )
    ),
    'ld8': (
        # Register
        (
            ByteCodes.LOAD8_REG_REG,
            ByteCodes.LOAD8_REG_ADDR_IN_REG,
            ByteCodes.LOAD8_REG_CONST,
            ByteCodes.LOAD8_REG_ADDR_LITERAL,
        )
    ),

    'mov1': (
        # Register
        (
            ByteCodes.MOVE1_REG_REG,
            ByteCodes.MOVE1_REG_ADDR_IN_REG,
            ByteCodes.MOVE1_REG_CONST,
            ByteCodes.MOVE1_REG_ADDR_LITERAL,
        ),
        # Address in register
        (
            ByteCodes.MOVE1_ADDR_IN_REG_REG,
            ByteCodes.MOVE1_ADDR_IN_REG_ADDR_IN_REG,
            ByteCodes.MOVE1_ADDR_IN_REG_CONST,
            ByteCodes.MOVE1_ADDR_IN_REG_ADDR_LITERAL,
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.MOVE1_ADDR_LITERAL_REG,
            ByteCodes.MOVE1_ADDR_LITERAL_ADDR_IN_REG,
            ByteCodes.MOVE1_ADDR_LITERAL_CONST,
            ByteCodes.MOVE1_ADDR_LITERAL_ADDR_LITERAL,
        ),
    ),
    'mov2': (
        # Register
        (
            ByteCodes.MOVE2_REG_REG,
            ByteCodes.MOVE2_REG_ADDR_IN_REG,
            ByteCodes.MOVE2_REG_CONST,
            ByteCodes.MOVE2_REG_ADDR_LITERAL,
        ),
        # Address in register
        (
            ByteCodes.MOVE2_ADDR_IN_REG_REG,
            ByteCodes.MOVE2_ADDR_IN_REG_ADDR_IN_REG,
            ByteCodes.MOVE2_ADDR_IN_REG_CONST,
            ByteCodes.MOVE2_ADDR_IN_REG_ADDR_LITERAL,
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.MOVE2_ADDR_LITERAL_REG,
            ByteCodes.MOVE2_ADDR_LITERAL_ADDR_IN_REG,
            ByteCodes.MOVE2_ADDR_LITERAL_CONST,
            ByteCodes.MOVE2_ADDR_LITERAL_ADDR_LITERAL,
        ),
    ),
    'mov4': (
        # Register
        (
            ByteCodes.MOVE4_REG_REG,
            ByteCodes.MOVE4_REG_ADDR_IN_REG,
            ByteCodes.MOVE4_REG_CONST,
            ByteCodes.MOVE4_REG_ADDR_LITERAL,
        ),
        # Address in register
        (
            ByteCodes.MOVE4_ADDR_IN_REG_REG,
            ByteCodes.MOVE4_ADDR_IN_REG_ADDR_IN_REG,
            ByteCodes.MOVE4_ADDR_IN_REG_CONST,
            ByteCodes.MOVE4_ADDR_IN_REG_ADDR_LITERAL,
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.MOVE4_ADDR_LITERAL_REG,
            ByteCodes.MOVE4_ADDR_LITERAL_ADDR_IN_REG,
            ByteCodes.MOVE4_ADDR_LITERAL_CONST,
            ByteCodes.MOVE4_ADDR_LITERAL_ADDR_LITERAL,
        ),
    ),
    'mov8': (
        # Register
        (
            ByteCodes.MOVE8_REG_REG,
            ByteCodes.MOVE8_REG_ADDR_IN_REG,
            ByteCodes.MOVE8_REG_CONST,
            ByteCodes.MOVE8_REG_ADDR_LITERAL,
        ),
        # Address in register
        (
            ByteCodes.MOVE8_ADDR_IN_REG_REG,
            ByteCodes.MOVE8_ADDR_IN_REG_ADDR_IN_REG,
            ByteCodes.MOVE8_ADDR_IN_REG_CONST,
            ByteCodes.MOVE8_ADDR_IN_REG_ADDR_LITERAL,
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.MOVE8_ADDR_LITERAL_REG,
            ByteCodes.MOVE8_ADDR_LITERAL_ADDR_IN_REG,
            ByteCodes.MOVE8_ADDR_LITERAL_CONST,
            ByteCodes.MOVE8_ADDR_LITERAL_ADDR_LITERAL,
        ),
    ),

    
    'st1': (
        # Register
        (
            ByteCodes.STORE1_REG_REG
        ),
        # Address in register
        (
            ByteCodes.STORE1_ADDR_IN_REG_REG
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.STORE1_ADDR_LITERAL_REG
        ),
    ),
    'st2': (
        # Register
        (
            ByteCodes.STORE2_REG_REG
        ),
        # Address in register
        (
            ByteCodes.STORE2_ADDR_IN_REG_REG
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.STORE2_ADDR_LITERAL_REG
        ),
    ),
    'st4': (
        # Register
        (
            ByteCodes.STORE4_REG_REG
        ),
        # Address in register
        (
            ByteCodes.STORE4_ADDR_IN_REG_REG
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.STORE4_ADDR_LITERAL_REG
        ),
    ),
    'st8': (
        # Register
        (
            ByteCodes.STORE8_REG_REG
        ),
        # Address in register
        (
            ByteCodes.STORE8_ADDR_IN_REG_REG
        ),
        None, # Constant
        # Address literal
        (
            ByteCodes.STORE8_ADDR_LITERAL_REG
        ),
    ),

    # Control flow

    '@': (
        ByteCodes.LABEL
    ),

    'jmp': (
        None, None, None, None, # Register, Address in register, Constant, Address literal
        # Label
        ByteCodes.JUMP
    ),
    'cjmp': (
        None, None, None, None, # Register, Address in register, Constant, Address literal
        # Label
        (
            ByteCodes.JUMP_IF_TRUE_REG,
        ),
    ),
    'njmp': (
        None, None, None, None, # Register, Address in register, Constant, Address literal
        # Label
        (
            ByteCodes.JUMP_IF_FALSE_REG,
        ),
    ),

    # Comparison

    'cmp': (
        # Register
        (
            ByteCodes.COMPARE_REG_REG,
            None, # Address in register
            ByteCodes.COMPARE_REG_CONST
        ),
        None, # Address in register
        # Constant
        (
            ByteCodes.COMPARE_CONST_REG,
            None, # Address in register
            ByteCodes.COMPARE_CONST_CONST
        ),
    ),
    

}
