use std::io;

fn main() {
    let mut lin = String::new();
    match io::stdin().read_line(&mut lin) {
        Ok(_) => {
            println!("you said: {}", lin);
        }
        Err(e) => {
            println!("failed: {}", e);
        }
    }
}