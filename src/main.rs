fn main() {
    let mut enumerator = udev::Enumerator::new().unwrap();
    for dev in enumerator.scan_devices().unwrap() {
        if let Some(path) = dev.devnode() {
            println!("- {}", path.to_string_lossy());
        }
    }
}
