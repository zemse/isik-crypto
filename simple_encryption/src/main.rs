mod lib;
use secure_random;

fn main() {
    let message = secure_random::u8();
    let key = secure_random::u8();
    let cipher = lib::encrypt(message, key);
    let d_message = lib::decrypt(cipher, key);
    println!("message = {}", message);
    println!("key = {}", key);
    println!("cipher = {}", cipher);
    println!("d_message = {}", d_message);
    //
    let message2 = secure_random::u8();
    let cipher2 = lib::encrypt(message2, key);
    let d_message2 = lib::decrypt(cipher, key);
    println!("\nmessage2 = {}", message2);
    println!("key = {}", key);
    println!("cipher2 = {}", cipher2);
    println!("d_message2 = {}", d_message2);
}
