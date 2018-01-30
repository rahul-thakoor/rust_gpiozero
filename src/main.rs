extern crate gpiozero;

use gpiozero::output_devices::OutputDevice;

fn main() {
    
    let mut d = OutputDevice::new(17);
    d.off();
}
