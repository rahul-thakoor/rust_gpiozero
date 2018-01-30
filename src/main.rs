extern crate gpiozero;

use gpiozero::output_devices::OutputDevice;

fn main() {
    
    let d = OutputDevice::new(17);
    d.on();
}
