

# ❤️‍🔥 heartbit: Temporal Jitter Steganography

My own novel PoC steganography tool written in Rust. Unlike traditional steganography that hides data within the *static* bits of a file (like an image or audio), `heartbit` hides data within time frames. It uses millisecond-level latency between network pulses to exfiltrate information without ever placing the secret text into a packet payload.

⚠️ **Please Note:** This project is strictly for **Educational and Authorized Penetration Testing**. I am not responsible for any of the shenanigans you guys pull.

---

## 🛠️ How It Works
`heartbit` operates on the principle of **Differential Pulse Position Modulation (DPPM)**. 

*   **Medium:** The script sends standard UDP *Heartbeat* packets that appear to be routine network health checks.
*   **Encoding:** Information is stored in the IAT—the specific delay between two pulses. 
*   **Protocol:** Each character is wrapped in a 10-bit frame.
    *   **1 Start Bit:** Signals the receiver to begin decoding.
    *   **8 Data Bits:** ASCII representation of the character.
    *   **1 Parity Bit:** Even-parity check to ensure complete reconstruction even if network jitter occurs.

---

## 📡 Transmitter & Receiver

### Transmitter
Converts a string into a bitstream and *rhythms* the outbound UDP packets. It uses a **High-Precision Spin-Lock** to bypass the standard OS sleep timers, making sure the timing is accurate. Moreover, to make sure the last character is never dropped, it automatically appends a padding frame.

### Receiver
Sits in wait, measuring the exact arrival time of every packet. It ignores the content of the packets and only looks at the *silence* between them. It determines if the sender was signaling a binary `0` or `1` by comparing the arrival of `Pulse B` against `Pulse A`. Uses a **safety buffer** (typically 30ms) to distinguish between intentional signaling and natural network lag.

---

## 🕵️ Steganography
This is a form of **Network Covert Channel** steganography. Most security tools are designed to scan *what* is inside a packet. `heartbit` is written around *when* the packet arrives. 
*   **Bit 0:** Represented by a ~1025ms delay.
*   **Bit 1:** Represented by a ~1075ms delay.

---

## 🛡️ Why This Method Works
It's virtually impossible for standard DPI or EDR systems to figure out the text because:
- **Zero Payload:** Packets contain dummy data (e.g. "HEARTBEAT"). Even if intercepted and decrypted, there is no secret text to find.
- **Camouflage:** On a real network, pings naturally fluctuate. A `50ms` difference looks like standard network jitter or minor congestion. To an admin, it just looks like a slightly laggy but normal monitoring script.
- **No Signatures:** There are no file headers or encryption strings for an AV to flag. Data only exists in the transition of time.

---

## 💼 Use Cases
*   **Low-and-Slow Exfiltration:** Moving high-value credentials out of an hardened environment where large file transfers are blocked.
*   **Dead Signaling:** Triggering a remote action on a dormant piece of software without establishing a long-term connection.
*   **Internal Persistence:** Maintaining a *heartbeat* between a compromised server and a controller that evades detection by looking like routine system management.

---

## 🧪 Testing heartbit
Follow these steps to run a local test in your environment.

- **Compile the Project**
    ```powershell
    cargo build --release
    ```
- **Start the Receiver**

    Open a PowerShell window and run:
    ```powershell
    .\target\release\receiver.exe
    ```
- **Start the Transmitter**

    Open a second PowerShell window and send your message:
    ```powershell
    .\target\release\sender.exe "Test@123"
    ```
- **Observe**

    The receiver will begin reconstructing the string character-by-character. Note that because each character takes ~10 seconds to transmit, this is a slow-drip method implemented for substantial stealth.

---
