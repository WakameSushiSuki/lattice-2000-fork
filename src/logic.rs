pub fn lor(a: u8, b: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        match (bit_a, bit_b) {
            (0, 0) => res &= !(1 << i),
            (1, 0) => res |= (1 << i),
            (0, 1) => res |= (1 << i),
            (1, 1) => res |= (1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}

pub fn land(a: u8, b: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        match (bit_a, bit_b) {
            (0, 0) => res &= !(1 << i),
            (1, 0) => res &= !(1 << i),
            (0, 1) => res &= !(1 << i),
            (1, 1) => res |= (1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}

pub fn lxor(a: u8, b: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        match (bit_a, bit_b) {
            (0, 0) => res &= !(1 << i),
            (1, 0) => res |= (1 << i),
            (0, 1) => res |= (1 << i),
            (1, 1) => res &= !(1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}

pub fn lnot(a: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        match bit_a {
            1 => res &= !(1 << i),
            0 => res |= (1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}