//! Slicky HID probe v3 — focused on color state polling.
//!
//! Findings so far:
//! - CMD 0x00: device info (firmware version?)
//! - CMD 0x01: serial number
//! - CMD 0x0B: current color state (BGR at bytes 5-7?)
//! - No unsolicited input reports on button press
//! - Can poll color state to detect button-driven changes

use hidapi::HidApi;
use std::time::{Duration, Instant};

const VID: u16 = 0x04D8;
const PID: u16 = 0xEC24;

fn send_cmd(device: &hidapi::HidDevice, cmd: u8) -> Option<[u8; 64]> {
    let mut report = [0u8; 65];
    report[1] = cmd;
    device.write(&report).ok()?;
    let mut buf = [0u8; 64];
    match device.read_timeout(&mut buf, 200) {
        Ok(n) if n > 0 => Some(buf),
        _ => None,
    }
}

fn set_color(device: &hidapi::HidDevice, r: u8, g: u8, b: u8) {
    let mut report = [0u8; 65];
    report[1] = 0x0A;
    report[2] = 0x04;
    report[6] = b;
    report[7] = g;
    report[8] = r;
    let _ = device.write(&report);
}

fn read_color(device: &hidapi::HidDevice) -> Option<[u8; 64]> {
    send_cmd(device, 0x0B)
}

fn main() {
    let api = HidApi::new().expect("Failed to init HidApi");
    let info = api
        .device_list()
        .find(|d| d.vendor_id() == VID && d.product_id() == PID)
        .expect("No Slicky found");
    let device = info.open_device(&api).expect("Failed to open");

    println!("=== Slicky Color State Probe ===");
    println!("  Serial: {:?}", info.serial_number());
    println!();

    // --- Read initial state ---
    println!("--- Initial State ---");
    if let Some(resp) = read_color(&device) {
        print!("  0x0B response: ");
        for b in &resp[..16] { print!("{:02X} ", b); }
        println!();
    }
    println!();

    // --- Set specific colors and read back ---
    let colors = [
        ("RED",     255, 0,   0  ),
        ("GREEN",   0,   255, 0  ),
        ("BLUE",    0,   0,   255),
        ("WHITE",   255, 255, 255),
        ("YELLOW",  255, 255, 0  ),
        ("OFF",     0,   0,   0  ),
    ];

    println!("--- Set Colors & Read Back ---");
    for (name, r, g, b) in &colors {
        set_color(&device, *r, *g, *b);
        std::thread::sleep(Duration::from_millis(100));
        if let Some(resp) = read_color(&device) {
            println!(
                "  Set {:<7} (R={:3} G={:3} B={:3}) → response bytes [4..8]: {:02X} {:02X} {:02X} {:02X}",
                name, r, g, b, resp[4], resp[5], resp[6], resp[7]
            );
            // Show more context
            print!("    Full [0..12]: ");
            for byte in &resp[..12] { print!("{:02X} ", byte); }
            println!();
        } else {
            println!("  Set {name}: no response to 0x0B");
        }
    }
    println!();

    // --- Poll for button presses by watching color changes ---
    println!("--- Button Detection via Color Polling ---");
    println!("  Setting BLUE, then polling color state for 20s.");
    println!("  Press the button and watch for color changes!");
    println!();

    set_color(&device, 0, 0, 255);
    std::thread::sleep(Duration::from_millis(100));

    let initial = read_color(&device);
    let mut prev_bytes = initial.map(|r| [r[4], r[5], r[6], r[7]]).unwrap_or([0; 4]);
    println!("  Initial state: {:02X} {:02X} {:02X} {:02X}", prev_bytes[0], prev_bytes[1], prev_bytes[2], prev_bytes[3]);

    let start = Instant::now();
    let duration = Duration::from_secs(20);
    let mut change_count = 0;

    while start.elapsed() < duration {
        if let Some(resp) = read_color(&device) {
            let curr = [resp[4], resp[5], resp[6], resp[7]];
            if curr != prev_bytes {
                change_count += 1;
                let elapsed = start.elapsed();
                println!(
                    "  [{:5.1}s] CHANGE #{}: {:02X} {:02X} {:02X} {:02X} → {:02X} {:02X} {:02X} {:02X}",
                    elapsed.as_secs_f64(), change_count,
                    prev_bytes[0], prev_bytes[1], prev_bytes[2], prev_bytes[3],
                    curr[0], curr[1], curr[2], curr[3]
                );

                // Also show full response for analysis
                print!("           Full [0..12]: ");
                for byte in &resp[..12] { print!("{:02X} ", byte); }
                println!();

                prev_bytes = curr;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    if change_count == 0 {
        println!("  No color changes detected in 20s.");
    } else {
        println!("  Total changes: {change_count}");
    }

    // Turn off
    set_color(&device, 0, 0, 0);
    println!();
    println!("=== Probe Complete ===");
}
