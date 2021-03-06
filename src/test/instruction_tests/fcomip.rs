use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcomip_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMIP, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 243], OperandSize::Word)
}

fn fcomip_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMIP, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 241], OperandSize::Dword)
}

fn fcomip_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMIP, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 245], OperandSize::Qword)
}

