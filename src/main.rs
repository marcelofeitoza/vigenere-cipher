use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: vigenere <encrypt|decrypt> <key> <text>");
        std::process::exit(1);
    }

    let mode = &args[1];
    let key = args[2].to_uppercase();
    let text = args[3].to_uppercase();

    if mode != "encrypt" && mode != "decrypt" {
        eprintln!("Invalid mode: {}", mode);
        std::process::exit(1);
    }

    let key_bytes: Vec<u8> = key.bytes().collect();
    let text_bytes: Vec<u8> = text.bytes().collect();
    let key_len = key_bytes.len();
    let text_len = text_bytes.len();
    
    let mut result = vec![0u8; text_len];
    
    let start = Instant::now();
    
    unsafe {
        let key_ptr = key_bytes.as_ptr();
        let text_ptr = text_bytes.as_ptr();
        let result_ptr = result.as_mut_ptr();
        
        for i in 0..text_len {
            let b = *text_ptr.add(i);
            if b'A' <= b && b <= b'Z' {
                let mut shift = *key_ptr.add(i % key_len) - b'A';
                if mode == "decrypt" {
                    shift = 26 - shift;
                }
                *result_ptr.add(i) = ((b - b'A' + shift) % 26) + b'A';
            } else {
                *result_ptr.add(i) = b;
            }
        }
    }
    
    let duration = start.elapsed();
    
    let result = String::from_utf8(result).unwrap();
    println!("{}", result);
    println!("Time taken: {} nanoseconds", duration.as_nanos());
}