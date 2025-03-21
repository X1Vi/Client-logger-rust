use evdev::*; // Import evdev crate
use std::fs::OpenOptions;

fn keycode_to_char(key: KeyCode, shift: bool) -> Option<char> {
    match key {
        KeyCode::KEY_A => Some(if shift { 'A' } else { 'a' }),
        KeyCode::KEY_B => Some(if shift { 'B' } else { 'b' }),
        KeyCode::KEY_C => Some(if shift { 'C' } else { 'c' }),
        KeyCode::KEY_D => Some(if shift { 'D' } else { 'd' }),
        KeyCode::KEY_E => Some(if shift { 'E' } else { 'e' }),
        KeyCode::KEY_F => Some(if shift { 'F' } else { 'f' }),
        KeyCode::KEY_G => Some(if shift { 'G' } else { 'g' }),
        KeyCode::KEY_H => Some(if shift { 'H' } else { 'h' }),
        KeyCode::KEY_I => Some(if shift { 'I' } else { 'i' }),
        KeyCode::KEY_J => Some(if shift { 'J' } else { 'j' }),
        KeyCode::KEY_K => Some(if shift { 'K' } else { 'k' }),
        KeyCode::KEY_L => Some(if shift { 'L' } else { 'l' }),
        KeyCode::KEY_M => Some(if shift { 'M' } else { 'm' }),
        KeyCode::KEY_N => Some(if shift { 'N' } else { 'n' }),
        KeyCode::KEY_O => Some(if shift { 'O' } else { 'o' }),
        KeyCode::KEY_P => Some(if shift { 'P' } else { 'p' }),
        KeyCode::KEY_Q => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::KEY_R => Some(if shift { 'R' } else { 'r' }),
        KeyCode::KEY_S => Some(if shift { 'S' } else { 's' }),
        KeyCode::KEY_T => Some(if shift { 'T' } else { 't' }),
        KeyCode::KEY_U => Some(if shift { 'U' } else { 'u' }),
        KeyCode::KEY_V => Some(if shift { 'V' } else { 'v' }),
        KeyCode::KEY_W => Some(if shift { 'W' } else { 'w' }),
        KeyCode::KEY_X => Some(if shift { 'X' } else { 'x' }),
        KeyCode::KEY_Y => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::KEY_Z => Some(if shift { 'Z' } else { 'z' }),
        KeyCode::KEY_1 => Some(if shift { '!' } else { '1' }),
        KeyCode::KEY_2 => Some(if shift { '@' } else { '2' }),
        KeyCode::KEY_3 => Some(if shift { '#' } else { '3' }),
        KeyCode::KEY_4 => Some(if shift { '$' } else { '4' }),
        KeyCode::KEY_5 => Some(if shift { '%' } else { '5' }),
        KeyCode::KEY_6 => Some(if shift { '^' } else { '6' }),
        KeyCode::KEY_7 => Some(if shift { '&' } else { '7' }),
        KeyCode::KEY_8 => Some(if shift { '*' } else { '8' }),
        KeyCode::KEY_9 => Some(if shift { '(' } else { '9' }),
        KeyCode::KEY_0 => Some(if shift { ')' } else { '0' }),
        KeyCode::KEY_SPACE => Some(' '),
        _ => None, // Return None for unsupported keys
    }
}

fn main() {
    let mut device = Device::open("/dev/input/event0").unwrap();
    let mut shift_pressed = false;

    loop {
        for event in device.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(_, key, 1) => {
                    if key == KeyCode::KEY_LEFTSHIFT || key == KeyCode::KEY_RIGHTSHIFT {
                        shift_pressed = true; // Track Shift key state
                    } else if let Some(ch) = keycode_to_char(key, shift_pressed) {
                        print!("{}", ch);
                    } else {
                        println!("Unknown key: {:?}", key);
                    }
                }
                EventSummary::Key(_, key, 0) => {
                    if key == KeyCode::KEY_LEFTSHIFT || key == KeyCode::KEY_RIGHTSHIFT {
                        shift_pressed = false; // Reset Shift key state
                    }
                }
                _ => {}
            }
        }
    }
}
