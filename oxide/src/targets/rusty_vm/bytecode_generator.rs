use std::collections::HashMap;
use std::mem;

use rusty_vm_lib::assembly::ByteCode;
use rusty_vm_lib::byte_code::ByteCodes;
use rusty_vm_lib::vm::Address;
use rusty_vm_lib::registers::Registers;

use crate::irc::{IROperator, LabelID, TnID};
use crate::symbol_table::{StaticID, SymbolTable};
use crate::flow_analyzer::FunctionGraph;


type LabelAddressMap = HashMap<LabelID, Address>;
type StaticAddressMap = HashMap<StaticID, Address>;


// struct GeneralPurposeRegisterSet {
//     registers: Vec<bool>,
// }

// impl GeneralPurposeRegisterSet {

//     pub fn new() -> Self {
//         Self {
//             registers: vec![false; GENERAL_PURPOSE_REGISTER_COUNT]
//         }
//     }


//     pub fn set(&mut self, reg: Registers) {
//         self.registers[reg as usize] = true;
//     }


//     pub fn is_set(&mut self, reg: Registers) -> bool {
//         self.registers[reg as usize]
//     }


//     pub fn clear(&mut self, reg: Registers) {
//         self.registers[reg as usize] = false;
//     }

// }

type Offset = usize;

enum TnLocation {

    /// The value of the Tn is stored inside a register
    Register (Registers),
    /// The value of the Tn is stores on the stack at an offset
    Stack (Offset)

}


/// Generate static data section, equivalent to .data section in assembly
fn generate_static_data_section(symbol_table: &SymbolTable, static_address_map: &mut StaticAddressMap, bytecode: &mut ByteCode) {

    for (static_id, static_value) in symbol_table.get_statics() {

        let static_size = static_value.data_type.static_size().unwrap_or_else(
            |()| panic!("Could not determine static size of {:?}. This is a bug.", static_value.data_type)
        );

        let byte_repr = static_value.value.as_bytes();

        assert_eq!(static_size, byte_repr.len());

        static_address_map.insert(static_id, bytecode.len());

        bytecode.extend(byte_repr);
    }

}


/// Generate the code section, equivalent to .text in assembly
fn generate_text_section(function_graphs: Vec<FunctionGraph>, labels_to_resolve: &mut Vec<Address>, bytecode: &mut ByteCode, label_address_map: &mut LabelAddressMap) {

    for function_graph in function_graphs {

        // TODO: we need to load the function arguments, or at least keep track of where they are.
        // defining a calling convention is thus necessary at this point.
        // This function should have access to the function's signature

        // Keeps track of where the actual value of Tns is stored
        let mut tn_location: HashMap<TnID, TnLocation> = HashMap::new();

        for block in function_graph.code_blocks {

            for ir_node in block.borrow().code.iter() {

                macro_rules! pushbc {
                    ($instruction:path) => {
                        bytecode.push($instruction as u8);
                    }
                }

                macro_rules! set_reg_const {
                    ($reg:path, $val:expr) => {
                        pushbc!(ByteCodes::MOVE_INTO_REG_FROM_CONST);
                        bytecode.extend(mem::size_of::<usize>().to_le_bytes());
                        pushbc!($reg);
                        bytecode.extend(($val).to_le_bytes());
                    }
                }

                macro_rules! move_into_reg_from_reg {
                    ($reg1:path, $reg2:path) => {
                        pushbc!(ByteCodes::MOVE_INTO_REG_FROM_REG);
                        pushbc!($reg1);
                        pushbc!($reg2);
                    }
                }

                macro_rules! placeholder_label {
                    ($label:ident) => {
                        labels_to_resolve.push(bytecode.len());
                        bytecode.extend(($label.0.0).to_le_bytes());
                    }
                }

                match &ir_node.op {

                    IROperator::Add { target, left, right } => {
                        // TODO: we need to keep a record of which Tns map to which memory address in the stack.
                    },
                    IROperator::Sub { target, left, right } => todo!(),
                    IROperator::Mul { target, left, right } => todo!(),
                    IROperator::Div { target, left, right } => todo!(),
                    IROperator::Mod { target, left, right } => todo!(),
                    IROperator::Assign { target, source } => todo!(),
                    IROperator::Deref { target, ref_ } => todo!(),
                    IROperator::DerefAssign { target, source } => todo!(),
                    IROperator::Ref { target, ref_ } => todo!(),
                    IROperator::Greater { target, left, right } => todo!(),
                    IROperator::Less { target, left, right } => todo!(),
                    IROperator::GreaterEqual { target, left, right } => todo!(),
                    IROperator::LessEqual { target, left, right } => todo!(),
                    IROperator::Equal { target, left, right } => todo!(),
                    IROperator::NotEqual { target, left, right } => todo!(),
                    IROperator::BitShiftLeft { target, left, right } => todo!(),
                    IROperator::BitShiftRight { target, left, right } => todo!(),
                    IROperator::BitNot { target, operand } => todo!(),
                    IROperator::BitAnd { target, left, right } => todo!(),
                    IROperator::BitOr { target, left, right } => todo!(),
                    IROperator::BitXor { target, left, right } => todo!(),
                    IROperator::Copy { target, source } => todo!(),
                    IROperator::DerefCopy { target, source } => todo!(),

                    IROperator::Jump { target } => {
                        pushbc!(ByteCodes::JUMP);
                        placeholder_label!(target);
                    },

                    IROperator::JumpIf { condition, target } => todo!(),
                    IROperator::JumpIfNot { condition, target } => todo!(),

                    IROperator::Label { label } => {
                        // Labels are not actual instructions and don't get translated to any bytecode.
                        // Mark the label as pointing to this specific real location in the bytecode
                        label_address_map.insert(label.0, bytecode.len());
                    },

                    IROperator::Call { return_target, return_label, callable, args } => todo!(),
                    IROperator::Return => todo!(),

                    IROperator::PushScope { bytes } => {
                        set_reg_const!(Registers::R1, bytes);
                        move_into_reg_from_reg!(Registers::R2, Registers::STACK_TOP_POINTER);
                        // The stack grows downwards
                        pushbc!(ByteCodes::INTEGER_SUB);
                        move_into_reg_from_reg!(Registers::STACK_TOP_POINTER, Registers::R1);
                    },
                    IROperator::PopScope { bytes } => {
                        set_reg_const!(Registers::R1, bytes);
                        move_into_reg_from_reg!(Registers::R2, Registers::STACK_TOP_POINTER);
                        pushbc!(ByteCodes::INTEGER_ADD);
                        move_into_reg_from_reg!(Registers::STACK_TOP_POINTER, Registers::R1);
                    },

                    IROperator::Nop => {
                        pushbc!(ByteCodes::NO_OPERATION);
                    },
                }
            }
        }
    }

}


fn resolve_unresolved_addresses(labels_to_resolve: Vec<Address>, label_address_map: LabelAddressMap, bytecode: &mut ByteCode) {

    // Substitute labels with actual addresses
    for label_location in labels_to_resolve {
        let label_id = LabelID(usize::from_le_bytes(
            bytecode[label_location..label_location + mem::size_of::<LabelID>()].try_into().unwrap()
        ));
        let address = label_address_map.get(&label_id).unwrap();
        bytecode[label_location..label_location + mem::size_of::<LabelID>()].copy_from_slice(&address.to_le_bytes());
    }

}


pub fn generate_bytecode(symbol_table: &SymbolTable, function_graphs: Vec<FunctionGraph>) -> ByteCode {
    /*
        Generate a static section for static data
        Generate a text section for the code
        Substitute labels with actual addresses
    */

    // Map a label to an actual memory address in the bytecode
    let mut label_address_map = LabelAddressMap::new();
    // Maps a static data id to an actual memory address in the bytecode
    let mut static_address_map = StaticAddressMap::new();
    // List of labels that will need to be filled in later, when all label addresses are known.
    let mut labels_to_resolve: Vec<Address> = Vec::new();

    let mut bytecode = ByteCode::new();

    generate_static_data_section(symbol_table, &mut static_address_map, &mut bytecode);

    generate_text_section(function_graphs, &mut labels_to_resolve, &mut bytecode, &mut label_address_map);

    resolve_unresolved_addresses(labels_to_resolve, label_address_map, &mut bytecode);

    // Specify the entry point of the program (main function or __init__ function)
    todo!();

    bytecode
}
