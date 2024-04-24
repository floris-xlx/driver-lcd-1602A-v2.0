use rppal::gpio::{Gpio, Mode, Level};
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let rs = gpio.get(7)?.into_output();
    let enable = gpio.get(8)?.into_output();
    let d4 = gpio.get(25)?.into_output();
    let d5 = gpio.get(24)?.into_output();
    let d6 = gpio.get(23)?.into_output();
    let d7 = gpio.get(18)?.into_output();

    // Initialize display (assuming a 4-bit interface)
    lcd_init(&rs, &enable, &d4, &d5, &d6, &d7)?;

    // Write a message
    lcd_write_str("Hello, Rust!", &rs, &enable, &d4, &d5, &d6, &d7)?;

    Ok(())
}

fn lcd_init(rs: &OutputPin, enable: &OutputPin, d4: &OutputPin, d5: &OutputPin, d6: &OutputPin, d7: &OutputPin) -> Result<(), Box<dyn std::error::Error>> {
    // Initialization sequence
    thread::sleep(time::Duration::from_millis(15)); // Wait for more than 15ms after VCC rises to 4.5V
    lcd_command(0x03, rs, enable, d4, d5, d6, d7); // Function set (8-bit interface)
    thread::sleep(time::Duration::from_millis(5)); // Wait for more than 4.1ms
    lcd_command(0x03, rs, enable, d4, d5, d6, d7); // Function set (8-bit interface)
    thread::sleep(time::Duration::from_millis(1)); // Wait for more than 100μs
    lcd_command(0x03, rs, enable, d4, d5, d6, d7); // Function set (8-bit interface)
    lcd_command(0x02, rs, enable, d4, d5, d6, d7); // Function set (4-bit interface)
    lcd_command(0x28, rs, enable, d4, d5, d6, d7); // Function set: 4-bit/2-line
    lcd_command(0x0C, rs, enable, d4, d5, d6, d7); // Display ON/OFF control: Display ON, Cursor OFF, Blink OFF
    lcd_command(0x06, rs, enable, d4, d5, d6, d7); // Entry Mode Set: Increment cursor, No display shift
    lcd_command(0x01, rs, enable, d4, d5, d6, d7); // Clear Display
    Ok(())
}

fn lcd_command(command: u8, rs: &OutputPin, enable: &OutputPin, d4: &OutputPin, d5: &OutputPin, d6: &OutputPin, d7: &OutputPin) {
    // Send command to LCD
    rs.set_low();
    send_nibble(command >> 4, enable, d4, d5, d6, d7);
    send_nibble(command & 0x0F, enable, d4, d5, d6, d7);
}

fn lcd_write_str(text: &str, rs: &OutputPin, enable: &OutputPin, d4: &OutputPin, d5: &OutputPin, d6: &OutputPin, d7: &OutputPin) {
    rs.set_high();
    for byte in text.bytes() {
        send_nibble(byte >> 4, enable, d4, d5, d6, d7);
        send_nibble(byte & 0x0F, enable, d4, d5, d6, d7);
    }
}

fn send_nibble(data: u8, enable: &OutputPin, d4: &OutputPin, d5: &OutputPin, d6: &OutputPin, d7: &OutputPin) {
    d4.write(if data & 0x01 > 0 { Level::High } else { Level::Low });
    d5.write(if data & 0x02 > 0 { Level::High } else { Level::Low });
    d6.write(if data & 0x04 > 0 { Level::High } else { Level::Low });
    d7.write(if data & 0x08 > 0 { Level::High } else { Level::Low });
    enable_pulse(enable);
}

fn enable_pulse(enable: &OutputPin) {
    enable.set_high();
    thread::sleep(time::Duration::from_micros(1)); // Enable pulse must be at least 450ns
    enable.set_low();
    thread::sleep(time::Duration::from_micros(50)); // Commands need > 37μs to settle
}

