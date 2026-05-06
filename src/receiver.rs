use std::net::UdpSocket;
use std::time::Instant;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9999")?;
    let mut buf = [0u8; 10];
    let mut last_arrival = Instant::now();
    let mut bit_buffer = Vec::new();

    println!("[!] receiver active. listening for jitter...");

    loop {
        socket.recv_from(&mut buf)?;
        let now = Instant::now();
        let iat = now.duration_since(last_arrival).as_millis();
        last_arrival = now;

        let bit = if iat >= 1060 && iat <= 1095 { Some(1) }
                  else if iat >= 1010 && iat <= 1045 { Some(0) }
                  else { None };

        if let Some(b) = bit {
            bit_buffer.push(b);
            if bit_buffer.len() == 10 {
                decode_frame(&bit_buffer);
                bit_buffer.clear();
            }
        }
    }
}

fn decode_frame(frame: &[u8]) {
    if frame[0] != 1 { return; }
    
    let mut byte = 0u8;
    let mut ones = 0;
    for i in 0..8 {
        if frame[i+1] == 1 {
            byte |= 1 << i;
            ones += 1;
        }
    }

    if (ones % 2) == frame[9] as i32 {
        print!("{}", byte as char);
    } else {
        print!("[?]"); 
    }
    
    io::stdout().flush().unwrap();
}