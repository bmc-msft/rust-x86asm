use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 221], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 763893413, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 4, 205, 165, 22, 136, 45], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 251], OperandSize::Qword)
}

#[test]
fn cvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 33], OperandSize::Qword)
}
