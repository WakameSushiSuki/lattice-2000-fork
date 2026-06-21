pub fn lor(a: u8, b: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        let bit_a_b = match bit_a {
            0 => false,
            _ => true,
        };
        let bit_b_b = match bit_b {
            0 => false,
            _ => true,
        };
        if bit_a_b || bit_b_b {
            res |= 1 << i;
        }
        else {
            res |= 0 << i;
        }
    }

    return res;
}

pub fn land(a: u8, b: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        let bit_a_b = match bit_a {
            0 => false,
            _ => true,
        };
        let bit_b_b = match bit_b {
            0 => false,
            _ => true,
        };
        if bit_a_b && bit_b_b {
            res |= 1 << i;
        }
        else {
            res |= 0 << i;
        }
    }

    return res;
}

pub fn lxor(a: u8, b: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        let bit_a_b = match bit_a {
            0 => false,
            _ => true,
        };
        let bit_b_b = match bit_b {
            0 => false,
            _ => true,
        };
        if bit_a_b ^ bit_b_b {
            res |= 1 << i;
        }
        else {
            res |= 0 << i;
        }
    }

    return res;
}

pub fn lnot(a: u8) -> u8 {
    let mut res: u8 = 0;

    for i in 0..8 {
        let bit_a = (a >> i) & 1;
        let bit_a_b = match bit_a {
            0 => false,
            _ => true,
        };
        let bit_res = match bit_a_b {
            false => 1,
            true => 0,
        };
        res |= (bit_res) << i;
    }

    return res;
}