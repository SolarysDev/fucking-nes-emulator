use lazy_static::lazy_static;
use crate::core::cpu::processor::{AddressingMode, Processor};

#[derive(Debug, Copy, Clone)]
pub struct OpCode {
    pub hex: u8,
    pub human: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    pub fn new(hex: u8, human: &'static str, bytes: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            hex, human, bytes, cycles, mode
        }
    }
}

lazy_static! {
    pub static ref CPU_OPCODES: Vec<OpCode> = vec![
        /* LDA - Load Accumulator */
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 3, 4, AddressingMode::Indirect_Y),

        /* TAX - Transfer Accumulator to X */
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),

        /* INX - Increment X */
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),

        /* BRK - Force Interrupt */
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
    ];
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

    cpu.program_counter += (opcode.bytes - 1) as u16;
}

pub fn tax(cpu: &mut Processor) {
    cpu.register_x = cpu.register_a;
    cpu.status.zero_negative_flags(cpu.register_x);
}
