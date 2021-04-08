mod lib;

fn main() {
    let secure_u8 = lib::u8();
    println!("secure_u8 = {}", secure_u8);
    let secure_u16 = lib::u16();
    println!("secure_u16 = {}", secure_u16);
    let secure_u32 = lib::u32();
    println!("secure_u32 = {}", secure_u32);
    let secure_u64 = lib::u64();
    println!("secure_u64 = {}", secure_u64);

    let secure_bytes = lib::bytes(32);
    print!("Bytes32=[");
    for i in 0..secure_bytes.len() {
        let byte = secure_bytes[i];
        print!("{}", byte);
        if i != secure_bytes.len() - 1 {
            print!(",");
        }
    }
    print!("]\n");
}
