use std::io::{stdin, stdout, Read, Write};

pub fn flush() {
    stdout().flush().unwrap();
}
pub fn clear() {
    clearscreen::clear().unwrap();
}
pub fn wait() {
    println!("PRESS ENTER TO CONTINUE");
    let mut buf: Vec<u8> = Vec::new();
    stdin().read(&mut buf).unwrap();
}
