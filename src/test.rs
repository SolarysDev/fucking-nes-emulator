use crate::core::cpu::processor::Processor;

#[test]
fn test_lda_immediate_load_data() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert_eq!(cpu.status.raw() & 0b0000_0010, 0b00);
    assert_eq!(cpu.status.raw() & 0b1000_0000, 0);
}

#[test]
fn test_lda_zero_flag() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert_eq!(cpu.status.raw() & 0b0000_0010, 0b10);
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = Processor::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_tax() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10)
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
}