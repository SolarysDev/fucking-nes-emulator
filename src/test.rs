#[cfg(test)]

use crate::core::cpu::processor::Processor;

#[test]
fn test_adc() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0x10, 0x65, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x0a);
}

#[test]
fn test_adc_carry() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x0a, 0x00]);
    assert_eq!(cpu.register_a, 0x09);
    assert_eq!(cpu.status.carry().get_raw(), 1);
}

#[test]
fn test_adc_overflow() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x40, 0x69, 0x40, 0x0]);
    assert_eq!(cpu.status.overflow().get_raw(), 1);
}

#[test]
fn test_and() {
    let mut cpu = Processor::new();
    cpu.mem_write(0x10, 0b0010_0010);
    cpu.load_and_run(vec![0xa9, 0xff, 0x25, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0b0010_0010);
}

#[test]
fn test_asl_accumulator() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x40, 0x0a, 0x00]);
    assert_eq!(cpu.register_a, 0x80);
}

#[test]
fn test_asl_memory() {
    let mut cpu = Processor::new();
    cpu.mem_write(0x10, 0x40);
    cpu.load_and_run(vec![0x06, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0x80);
}

#[test]
fn test_bcc() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0x90, 0x02, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x00);
}

#[test]
fn test_bcs() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x01, 0xb0, 0x02, 0xa9, 0x01, 0x00]);
    assert_eq!(cpu.register_a, 0x00);
}

#[test]
fn test_beq() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x90, 0x02, 0xaa, 0xe8, 0x00]);
    assert_eq!(cpu.register_x, 0);
}

#[test]
fn test_bit() {
    let mut cpu = Processor::new();
    cpu.mem_write(0x10, 0x01);
    cpu.load_and_run(vec![0xa9, 0xff, 0x24, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_bit_overflow() {
    let mut cpu = Processor::new();
    cpu.mem_write(0x10, 0b0100_0000);
    cpu.load_and_run(vec![0xa9, 0xff, 0x24, 0x10, 0x00]);
    assert_eq!(cpu.status.overflow().get_raw(), 1);
}

#[test]
fn test_bit_negative() {
    let mut cpu = Processor::new();
    cpu.mem_write(0x10, 0b1000_0000);
    cpu.load_and_run(vec![0xa9, 0xff, 0x24, 0x10, 0x00]);
    assert_eq!(cpu.status.negative().get_raw(), 1);
}

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
fn test_sbc() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0x06, 0xe9, 0x04, 0x00]);
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_sta() {
    let mut cpu = Processor::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0x85, 0x10, 0x00]);
    assert_eq!(cpu.mem_read(0x10), 0xc0);
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
