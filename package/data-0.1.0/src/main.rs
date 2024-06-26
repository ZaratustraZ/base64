use std::process;
use std::io::{self, Write};
use base64::{encode, decode};

fn main() {
    let mut option = String::new();
    println!("Select:");
    println!("1. Text to Base64");
    println!("2. Base64 to Text");
    println!("3. Exit");
    print!(">> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();

    match option.trim() {
        "1" => txt_to_base(),
        "2" => base_to_txt(),
        "3" => process::exit(0),
        &_  => main(),
    }
}

fn txt_to_base() {
    let mut input = String::new();

    println!("");
    print!("Type something as a String: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!("");

    let encoded = encode(input.trim().as_bytes());
    println!("========================================================");
    println!("                    {}", encoded);
    println!("========================================================");

    main();
}

fn base_to_txt() {
    let mut input = String::new();

    println!("");
    print!("Type something in Base64: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!("");

    let decoded = decode(input.trim().as_bytes()).unwrap();
    let decoded_str = String::from_utf8_lossy(&decoded);
    println!("========================================================");
    println!("                    {}", decoded_str);
    println!("========================================================");

    main();
}