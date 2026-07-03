pub fn bsl(na: u8) -> u8 {
    let mut res = na;

    for i in (1..8).rev() {
        let bit = (na >> (i - 1)) & 1;
        if bit == 1 {
            res |= (1 << i);
        }
        else {
            res &= !(1 << i);
        }
    }

    res &= !(1 << 0);

    return res;
}

pub fn bsr(na: u8) -> u8 {
    let mut res = na;

    for i in 0..7 {
        let bit = (na >> (i + 1)) & 1;
        if bit == 1 {
            res |= (1 << i);
        }
        else {
            res &= !(1 << i);
        }
    }

    res &= !(1 << 7);

    return res;
}