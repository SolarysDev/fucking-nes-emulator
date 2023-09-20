use bit_struct::*;

bit_struct! {
    pub struct CPUStatusFlags(u8) {
        negative: u1,
        overflow: u1,
        dummy_flag: u1, // always set to 1
        b_flag: u1, // this thing isn't used
        decimal: u1,
        interrupt_disable: u1,
        zero: u1,
        carry: u1
    }
}

impl Default for CPUStatusFlags {
    fn default() -> Self {
        CPUStatusFlags::new(
            u1!(0), // negative
            u1!(0), // overflow
            u1!(1), // dummy_flag
            u1!(0), // b_flag
            u1!(0), // decimal
            u1!(0), // interrupt_disable
            u1!(0), // zero
            u1!(0) // carry
        )
    }
}

impl CPUStatusFlags {
    pub fn zero_negative_flags(&mut self, target: u8) {
        if target == 0 { self.zero().set(u1!(1)); }
        else { self.zero().set(u1!(0)); }

        if target & 0b1000_0000 != 0 { self.negative().set(u1!(1)); }
        else { self.negative().set(u1!(0)); }
    }
}
