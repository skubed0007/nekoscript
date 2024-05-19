fn main() {
    let mut buf = String::new();
    println!("PRESS ENTER");
    std::io::stdin().read_line(&mut buf).unwrap();
    std::process::exit(0);
}
