mod textractor_ws;

use std::io;

fn main() {
    let mut buf = String::new();
    println!("Starting server at {}", textractor_ws::ADDRESS);
    println!("Input any line and press ENTER to send it to all clients");
    loop {
        match io::stdin().read_line(&mut buf) {
            Ok(_n) => {
                textractor_ws::handle(buf.clone());
                buf.clear();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
