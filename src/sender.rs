use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::env;

const BASE_DELAY_MS: u64 = 1000;
const BIT_1_OFFSET: u64 = 75;
const BIT_0_OFFSET: u64 = 25;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let original_msg = args.get(1).expect("usage: sender \"message\"");

    let msg = format!("{}\n", original_msg); 

    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let target_addr = "127.0.0.1:9999";
    
    let mut bits = Vec::new();
    for byte in msg.as_bytes() {
        bits.push(1);
        let mut parity = 0;
        for i in 0..8 {
            let bit = (byte >> i) & 1;
            bits.push(bit);
            if bit == 1 { parity += 1; }
        }
        bits.push(parity % 2);
    }

    println!("[!] heartbit started. sending: '{}'", original_msg);
    for bit in bits {
        let start = Instant::now();

        socket.send_to(b"HEARTBEAT", target_addr)?;

        let offset = if bit == 1 { BIT_1_OFFSET } else { BIT_0_OFFSET };
        let target_time = start + Duration::from_millis(BASE_DELAY_MS + offset);
        
        while Instant::now() < target_time {
            std::hint::spin_loop();
        }
    }

    std::thread::sleep(Duration::from_millis(1500));
    println!("[+] transmission complete.");
    Ok(())
}