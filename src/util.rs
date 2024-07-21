pub fn nth_bit(input: u32, nth_bit: u32) -> bool {
    if (a & (1 << n)) != 0 {
        1
    }
    else {
        0
    }
}

pub fn bit_set(input: &mut u32, nth_bit: u32, on: bool) {
    if on {
        *input |= 1 << nth_bit;
    }
    else {
        *input &= !(1<<nth_bit);
    }
}

fn between(a: i32, b: i32, c: i32) -> bool {
    a >= b && a <= c
} 
