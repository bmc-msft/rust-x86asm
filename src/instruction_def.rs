use std::collections::HashMap;
use std::io::Write;
use std::sync::RwLock;
use ::{Mnemonic, OperandSize, Reg, RegType};
use ::instruction_defs::INSTR_DEFS;

lazy_static! {
    static ref INSTR_MNEMONIC_MAP : RwLock<HashMap<Mnemonic, Vec<&'static InstructionDefinition>>> = {
        let mut lock = RwLock::new(HashMap::new());
        {
            let mut map = lock.write().unwrap();
            load_instructions(&mut map);
        }
        lock
    };
}

pub fn load_instructions(map: &mut HashMap<Mnemonic, Vec<&'static InstructionDefinition>>) {
    for instr in INSTR_DEFS.iter() {
        if !map.contains_key(&instr.mnemonic) {
            let new_list = Vec::new();
            map.insert(instr.mnemonic, new_list);
        };
        let mut list = map.get_mut(&instr.mnemonic).unwrap();
        list.push(&instr);
    }
}

pub struct InstructionDefinition {
    pub mnemonic: Mnemonic,
    pub allow_prefix: bool,
    pub operand_size_prefix: Option<bool>,
    pub address_size_prefix: Option<bool>,
    pub fixed_prefix: Option<u8>,
    pub composite_prefix: Option<CompositePrefix>,

    pub two_byte_opcode: bool,
    pub primary_opcode: u8,
    pub secondary_opcode: Option<u8>,
    pub opcode_ext: Option<u8>,

    pub fixed_post: Option<u8>,
    pub has_mod_rm: bool,
    pub allow_mask: bool,
    pub allow_merge_mode: bool,
    pub allow_rounding: bool,
    pub allow_sae: bool,
    
    pub operands: [Option<OperandDefinition>; 4],

    pub feature_set: Option<&'static [FeatureSet]>,
    pub valid_legacy: bool,
    pub valid_long: bool,
    pub desc: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub enum CompositePrefix {
    Rex {
        size_64: Option<bool>,
    },

    Vex {
        vector_size: Option<OperandSize>,
        operand_behavior: Option<VexOperandBehavior>,
        we: Option<bool>
    },

    Evex {
        vector_size: Option<OperandSize>,
        operand_behavior: Option<VexOperandBehavior>,
        we: Option<bool>,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FeatureSet {
    Sse,
    Sse2,
    Sse3,
    Sse41,
    Sse42,
    Aesni,
    Pclmulqdq,
    Avx,
    Avx512vl,
    Avx512f,
    Rdrand
}

#[derive(Clone, Debug)]
pub struct OperandDefinition {
    pub encoding: OperandEncoding,
    pub access: OperandAccess,
    pub size: OperandSize,
    pub op_type: OperandType
}

#[derive(Clone, Debug)]
pub enum OperandType {
    Reg(RegType),
    Mem,
    Imm,
    Constant,
    Offset,
    Mib,
    Bcst(OperandSize),
    Fixed(FixedOperand),
    Set(&'static [OperandType])
}

#[derive(Clone, Copy, Debug)]
pub enum OperandEncoding {
    ModRmReg,
    ModRmRm,
    Vex,
    Imm,
    OpcodeAddend,
    Offset,
    Mib,
    Fixed
}

#[derive(Clone, Copy, Debug)]
pub enum OperandAccess {
    Read,
    Write,
    ReadWrite
}

#[derive(Clone, Copy, Debug)]
pub enum FixedOperand {
    Reg(Reg),
    Constant(u32)
}

#[derive(Clone, Copy, Debug)]
pub enum VexOperandBehavior {
    Nds,
    Ndd,
    Dds
}
