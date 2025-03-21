use evdev::*;


fn main() {
    let keylogs: String = String::new();

    let mut device = Device::open("/dev/input/event0").unwrap();
    
    loop {
        for event in device.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(ev, key, 1) => {
                    println!("Key '{:?}' was pressed, got event: {:?}", key, ev);
                }
                EventSummary::Key(ev, key, 0) => {
                    println!("Key '{:?}' was released, got event: {:?}", key, ev);
                }
                _ => {
                    println!("Unhandled event: {:?}", event);
                }
            }
        }
    }
}
