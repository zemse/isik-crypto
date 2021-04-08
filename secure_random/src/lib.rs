use rand::Rng;

pub fn bytes(n_bytes: usize) -> Vec<u8> {
    let mut bytes = vec![0; n_bytes];
    for i in 0..n_bytes {
        // generating a uniform u8
        let mut secure_u8: u8 = 0;
        let mut bits_added: usize = 0;
        while bits_added != 8 {
            // Security consideration: Using same thread_rng might cause value of next rng to depend
            //  on previous one, which means bit1 and bit2 may not be truely independent events. That's
            //  why thread_rng is created again.
            let bit1: bool = {
                let mut rng = rand::thread_rng();
                rng.gen() // non-uniform rng
            };
            let bit2: bool = {
                let mut rng = rand::thread_rng();
                rng.gen() // non-uniform rng
            };

            // considering only two cases when probability is equal
            if bit1 != bit2 {
                // shifting left and adding the random bit
                secure_u8 <<= 1;
                secure_u8 += bit1 as u8;
                bits_added += 1;
            }
        }
        // println!("secure_u8 is {}", secure_u8);
        bytes[i] = secure_u8;
    }
    return bytes;
}

pub fn u8() -> u8 {
    bytes(1)[0]
}

pub fn u16() -> u16 {
    let generated = bytes(2);
    let mut ret: u16 = 0;
    ret += generated[0] as u16;
    ret <<= 8;
    ret += generated[1] as u16;
    ret
}

pub fn u32() -> u32 {
    let generated = bytes(4);
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret <<= 8;
        ret += generated[i] as u32;
    }
    ret
}

pub fn u64() -> u64 {
    let generated = bytes(8);
    let mut ret: u64 = 0;
    for i in 0..8 {
        ret <<= 8;
        ret += generated[i] as u64;
    }
    ret
}
