use evdev::*;

fn main() {
    let mut device = Device::open("/dev/input/event0").unwrap();
    loop {
        for event in device.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(ev, KeyCode::KEY_A, 1) => {
                    println!("Key 'a' was pressed, got event: {:?}", ev);
                }
                EventSummary::Key(_, key_type, 0) => {
                    println!("Key {:?} was released", key_type);
                }
                EventSummary::AbsoluteAxis(_, axis, value) => {
                    println!("The Axis {:?} was moved to {}", axis, value);
                }
                _ => println!("got a different event!"),
            }
        }
    }
}
