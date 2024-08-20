pub fn nth_bit(input: u32, nth_bit: u32) -> bool {
    (input & (1 << nth_bit)) != 0
}

pub fn bit_set(mut input: u8, nth_bit: u8, on: bool) -> u8 {
    if on {
        input |= 1 << nth_bit;
    }
    else {
        input &= !(1<<nth_bit);
    }
    input
}