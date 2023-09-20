use crate::core::cpu::status_flags::CPUStatusFlags;
use crate::core::cpu::instructions;
use crate::core::cpu::instructions::{CPU_OPCODES, OpCode, ProcessorAction::*};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct Processor {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub register_s: u8,
    pub register_p: u8,
    pub status: CPUStatusFlags,
    pub program_counter: u16,
    memory: [u8; 0xffff]
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            register_s: 0,
            register_p: 0,
            status: CPUStatusFlags::default(),
            program_counter: 0,
            memory: [0x0; 0xffff],
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);
        u16::from_le_bytes([lo, hi])
    }

    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let bytes: [u8; 2] = data.to_le_bytes();
        self.mem_write(pos, bytes[0]);
        self.mem_write(pos + 1, bytes[1]);
    }

    fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.register_s = 0;
        self.register_p = 0;
        self.status = CPUStatusFlags::default();

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn branch(&mut self, condition: bool) {
        if !condition { return; }

        let jump = self.mem_read(self.program_counter) as i8;
        let jump_addr = self.program_counter
            .wrapping_add(1)
            .wrapping_add(jump as u16);

        self.program_counter = jump_addr;
    }

    pub fn run(&mut self) {


        loop {
            let next_byte = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let mut cpu_opcodes_iter = CPU_OPCODES.iter();
            let opcode: &OpCode = cpu_opcodes_iter.find(|oc| oc.hex == next_byte)
                .unwrap_or_else(|| panic!("invalid opcode: {next_byte:#04x}"));

            match &opcode.action {
                ADC => instructions::adc(self, opcode),
                AND => instructions::and(self, opcode),
                ASL => instructions::asl(self, opcode),
                BCC => instructions::bcc(self),
                BCS => instructions::bcs(self),
                BEQ => instructions::beq(self),
                BIT => instructions::bit(self, opcode),
                BRK => return,
                INX => instructions::inx(self),
                LDA => instructions::lda(self, opcode),
                SBC => instructions::sbc(self, opcode),
                STA => instructions::sta(self, opcode),
                TAX => instructions::tax(self),

                _ => todo!("{:#?} (hex {:#04x}) not yet implemented", opcode.action, opcode.hex)
            }
        }
    }

    pub fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => u16::from(self.mem_read(self.program_counter)),

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                
                u16::from(pos.wrapping_add(self.register_x))
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                
                u16::from(pos.wrapping_add(self.register_y))
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                
                base.wrapping_add(u16::from(self.register_x))
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                
                base.wrapping_add(u16::from(self.register_y))
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base).wrapping_add(self.register_x);
                let lo = self.mem_read(u16::from(ptr));
                let hi = self.mem_read(u16::from(ptr.wrapping_add(1)));
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(u16::from(base));
                let hi = self.mem_read(u16::from((base).wrapping_add(1)));
                let deref_base = u16::from_le_bytes([lo, hi]);
                
                deref_base.wrapping_add(u16::from(self.register_y))
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {mode:?} is not supported");
            }
        }
    }
}