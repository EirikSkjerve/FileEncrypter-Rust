use std::io;

fn main() {
    println!("Select a file to encrypt: ");
    let mut filepath = String::new();

    io::stdin().read_line(&mut filepath).expect("failed to read line");
    
    filepath.pop();
    println!("Now encrypting {}", filepath);

}
