#![allow(arithmetic_overflow)]

pub fn encrypt(num: u8, key: u8) -> u8 {
    num + key
}

pub fn decrypt(num: u8, key: u8) -> u8 {
    num - key
}
