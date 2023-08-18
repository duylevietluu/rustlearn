use std::io::{self, Read, Write};

fn main() {
    // Calculate the rotation value
    let rotation_value = ((u8::MAX as u16 + 1u16) / 2) as u8;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read input characters and apply rot128 encryption
    loop {
        let mut input_byte: [u8; 1] = [0u8];
        let mut encrypted_byte: [u8; 1] = [0u8];

        match stdin.lock().read(&mut input_byte) {
            Ok(0) => break, // End of input
            Ok(_) => {
                encrypted_byte[0] = input_byte[0].wrapping_add(rotation_value);
                stdout.write_all(&encrypted_byte).expect("Write failed");
            }
            Err(_) => {
                eprintln!("An error occurred while reading input.");
                break;
            }
        }
    }
}
