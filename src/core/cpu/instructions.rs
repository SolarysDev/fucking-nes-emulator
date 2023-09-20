use bit_struct::u1;
use lazy_static::lazy_static;
use crate::core::cpu::processor::{AddressingMode, Processor};
use std::sync::Arc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OpCode {
    pub hex: u8,
    pub action: ProcessorAction,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProcessorAction {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC,
    CLD, CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP,
    JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI,
    RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA
}

impl OpCode {
    pub fn new(hex: u8, human: ProcessorAction, bytes: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            hex,
            action: human, bytes, cycles, mode
        }
    }
}



lazy_static! {
    pub static ref CPU_OPCODES: Arc<[OpCode]> = Arc::from([
        /* ADC -  Add with Carry */
        OpCode::new(0x69, ProcessorAction::ADC, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, ProcessorAction::ADC, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, ProcessorAction::ADC, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6d, ProcessorAction::ADC, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, ProcessorAction::ADC, 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x79, ProcessorAction::ADC, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x61, ProcessorAction::ADC, 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, ProcessorAction::ADC, 2, 5, AddressingMode::Indirect_Y),

        /* AND - Logical AND */
        OpCode::new(0x29, ProcessorAction::AND, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, ProcessorAction::AND, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x23, ProcessorAction::AND, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x2d, ProcessorAction::AND, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3d, ProcessorAction::AND, 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x39, ProcessorAction::AND, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x21, ProcessorAction::AND, 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x31, ProcessorAction::AND, 2, 5, AddressingMode::Indirect_Y),

        /* ASL - Arithmetic Shift Left */
        OpCode::new(0x0a, ProcessorAction::ASL, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x06, ProcessorAction::ASL, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x16, ProcessorAction::ASL, 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x0e, ProcessorAction::ASL, 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1e, ProcessorAction::ASL, 3, 7, AddressingMode::Absolute_X),

        /* BCC - Branch if Carry Clear */
        OpCode::new(0x90, ProcessorAction::BCC, 2, 2, AddressingMode::NoneAddressing),

        /* BCS - Branch of Carry Set */
        OpCode::new(0xb0, ProcessorAction::BCS, 2, 2, AddressingMode::NoneAddressing),

        /* BEQ - Branch if Equal */
        OpCode::new(0xf0, ProcessorAction::BEQ, 2, 2, AddressingMode::NoneAddressing),

        /* BIT - Bit Test */
        OpCode::new(0x24, ProcessorAction::BIT, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x2c, ProcessorAction::BIT, 3, 4, AddressingMode::Absolute),

        /* BRK - Force Interrupt */
        OpCode::new(0x00, ProcessorAction::BRK, 1, 7, AddressingMode::NoneAddressing),

        /* INX - Increment X */
        OpCode::new(0xe8, ProcessorAction::INX, 1, 2, AddressingMode::NoneAddressing),

        /* LDA - Load Accumulator */
        OpCode::new(0xa9, ProcessorAction::LDA, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, ProcessorAction::LDA, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, ProcessorAction::LDA, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, ProcessorAction::LDA, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, ProcessorAction::LDA, 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0xb9, ProcessorAction::LDA, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, ProcessorAction::LDA, 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, ProcessorAction::LDA, 3, 4, AddressingMode::Indirect_Y),

        /* SBC - Subtract with Carry */
        OpCode::new(0xe9, ProcessorAction::SBC, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, ProcessorAction::SBC, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xf5, ProcessorAction::SBC, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xed, ProcessorAction::SBC, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, ProcessorAction::SBC, 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0xf9, ProcessorAction::SBC, 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0xe1, ProcessorAction::SBC, 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xf1, ProcessorAction::SBC, 2, 5, AddressingMode::Indirect_Y),

        /* STA - Store Accumulator in Memory */
        OpCode::new(0x85, ProcessorAction::STA, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, ProcessorAction::STA, 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8d, ProcessorAction::STA, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, ProcessorAction::STA, 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, ProcessorAction::STA, 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, ProcessorAction::STA, 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, ProcessorAction::STA, 2, 6, AddressingMode::Indirect_Y),

        /* TAX - Transfer Accumulator to X */
        OpCode::new(0xaa, ProcessorAction::TAX, 1, 2, AddressingMode::NoneAddressing),
    ]);
}

fn advance_program_counter(cpu: &mut Processor, opcode: &OpCode) {
    cpu.program_counter += (opcode.bytes - 1) as u16;
}

fn register_a_add(cpu: &mut Processor, data: u8) {
    let sum = cpu.register_a as u16
        + data as u16
        + cpu.status.carry().get_raw() as u16;

    if sum > 0xff { cpu.status.carry().set(u1!(1)); }
    else { cpu.status.carry().set(u1!(0)); }

    let result = sum as u8;
    if (data ^ result) & (result ^ cpu.register_a) & 0x80 != 0 { cpu.status.overflow().set(u1!(1)) }
    else { cpu.status.overflow().set(u1!(0)) }

    cpu.register_a = result;
}

pub fn adc(cpu: &mut Processor, opcode: &OpCode) {
    let addr = cpu.get_operand_address(&opcode.mode);
    let addend = cpu.mem_read(addr);

    register_a_add(cpu, addend);
    cpu.status.zero_negative_flags(cpu.register_a);

    advance_program_counter(cpu, opcode);
}

pub fn and(cpu: &mut Processor, opcode: &OpCode) {
    let addr = cpu.get_operand_address(&opcode.mode);
    let operand = cpu.mem_read(addr);

    cpu.register_a &= operand;
    cpu.status.zero_negative_flags(cpu.register_a);

    advance_program_counter(cpu, opcode);
}

pub fn asl(cpu: &mut Processor, opcode: &OpCode) {
    // check if we're just doing this on the accumulator
    if opcode.mode == AddressingMode::NoneAddressing {
        asl_accumulator(cpu, opcode);
        return;
    }

    let addr = cpu.get_operand_address(&opcode.mode);
    let mut value = cpu.mem_read(addr);

    if value >> 7 == 1 { cpu.status.carry().set(u1!(1)) }
    else { cpu.status.carry().set(u1!(0)) }

    value <<= 1;
    cpu.mem_write(addr, value);

    cpu.status.zero_negative_flags(value);

    advance_program_counter(cpu, opcode);
}

fn asl_accumulator(cpu: &mut Processor, opcode: &OpCode) {
    if cpu.register_a >> 7 == 1 { cpu.status.carry().set(u1!(1)) }
    else { cpu.status.carry().set(u1!(0)) }

    cpu.register_a <<= 1;

    cpu.status.zero_negative_flags(cpu.register_a);

    advance_program_counter(cpu, opcode);
}

pub fn bcc(cpu: &mut Processor) {
    let carry = cpu.status.carry().get_raw();
    cpu.branch(carry == 0);
}

pub fn bcs(cpu: &mut Processor) {
    let carry = cpu.status.carry().get_raw();
    cpu.branch(carry == 1);
}

pub fn beq(cpu: &mut Processor) {
    let zero = cpu.status.zero().get_raw();
    cpu.branch(zero == 1);
}

pub fn bit(cpu: &mut Processor, opcode: &OpCode) {
    let addr = cpu.get_operand_address(&opcode.mode);
    let operand = cpu.mem_read(addr);

    let result = cpu.register_a & operand;

    if result & 0b0100_0000 == 0b0100_0000 { cpu.status.overflow().set(u1!(1)); }
    else { cpu.status.overflow().set(u1!(0)); }

    if result & 0b1000_0000 == 0b1000_0000 { cpu.status.negative().set(u1!(1)); }
    else { cpu.status.negative().set(u1!(0)); }

    cpu.register_a = result;

    cpu.status.zero_negative_flags(cpu.register_a);

    advance_program_counter(cpu, opcode);
}

pub fn inx(cpu: &mut Processor) {
    let result = cpu.register_x.overflowing_add(1);
    cpu.register_x = result.0;
    cpu.status.zero_negative_flags(cpu.register_x);
}

pub fn lda(cpu: &mut Processor, opcode: &OpCode) {
    let addr = cpu.get_operand_address(&opcode.mode);
    let value = cpu.mem_read(addr);
    cpu.register_a = value;
    cpu.status.zero_negative_flags(cpu.register_a);

    advance_program_counter(cpu, opcode);
}

pub fn sbc(cpu: &mut Processor, opcode: &OpCode) {
    let addr = cpu.get_operand_address(&opcode.mode);
    let addend = cpu.mem_read(addr);
                                                    /* handle NOT-ing the carry flag here */
    register_a_add(cpu, ((addend as i8).wrapping_neg().wrapping_sub(1)) as u8);
    cpu.status.zero_negative_flags(cpu.register_a);

    advance_program_counter(cpu, opcode);
}

pub fn sta(cpu: &mut Processor, opcode: &OpCode) {
    let addr = cpu.get_operand_address(&opcode.mode);
    cpu.mem_write(addr, cpu.register_a);

    cpu.program_counter += u16::from(opcode.bytes - 1);
}

pub fn tax(cpu: &mut Processor) {
    cpu.register_x = cpu.register_a;
    cpu.status.zero_negative_flags(cpu.register_x);
}
